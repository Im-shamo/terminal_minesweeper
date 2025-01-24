#![cfg(feature = "bracketed-paste")]
use crate::mineboard::{Board, BoardConfig, BoardError, ItemType};
use crate::utils::Coordinates;
use crossterm::cursor::{self, EnableBlinking};
use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
};
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
};

use rand::prelude::*;
use std::io::stdout;

#[derive(Clone, Debug)]
pub enum GameState {
    Playing,
    Win,
    HitMine,
    Quit,
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new(config: BoardConfig, landmine_count: usize) -> Result<Self, BoardError> {
        let mines = Self::gen_landmine(landmine_count, &config);
        let mut board: Board = Board::new(config, &mines)?;
        board.draw_border();
        Ok(Self {
            board,
            state: GameState::Playing,
        })
    }

    pub fn game_loop(&mut self) {
        enable_raw_mode().unwrap();

        let mut stdout = stdout();
        execute!(stdout, EnableMouseCapture, EnableBlinking).unwrap();
        execute!(stdout, cursor::Show).unwrap();
        self.rander();
        execute!(stdout, cursor::MoveTo(2, 1)).unwrap();
        loop {
            self.get_and_proccess_input();
            self.rander();
            match self.state {
                GameState::Quit | GameState::HitMine | GameState::Win => break,
                GameState::Playing => {}
            }
        }
        execute!(stdout, Clear(ClearType::All), DisableMouseCapture).unwrap();
        disable_raw_mode().unwrap();
    }

    fn gen_landmine(landmine_count: usize, config: &BoardConfig) -> Vec<Coordinates> {
        let mut rng = rand::thread_rng();

        let mut mines = vec![];

        let height = config.height;
        let width = config.width;

        for _ in 0..landmine_count {
            let pos = loop {
                let pos = Coordinates::new(rng.gen_range(0..width), rng.gen_range(0..height));
                if mines.iter().find(|&&p| p == pos).is_some() {
                    continue;
                }
                break pos;
            };
            mines.push(pos);
        }
        mines
    }

    fn rander(&mut self) {
        let mut stdout = stdout();
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::SavePosition,
            cursor::MoveTo(0, 0)
        )
        .unwrap();
        self.board.print();
        execute!(stdout, cursor::RestorePosition).unwrap();
    }

    pub fn landmine_count(&self) -> &usize {
        self.board.landmine_count()
    }

    fn get_input(&self) -> std::io::Result<Event> {
        let key_event = read()?;
        Ok(key_event)
    }

    fn get_and_proccess_input(&mut self) {
        let mut stdout = stdout();
        loop {
            match self.get_input().unwrap() {
                Event::Mouse(mouse_event) => {
                    self.process_mouse(&mut stdout, mouse_event);
                    break;
                }
                Event::Key(key_event) => {
                    self.process_key(&mut stdout, key_event);
                    break;
                }
                _ => (),
            }
        }
    }

    fn process_key(&mut self, stdout: &mut std::io::Stdout, key_event: KeyEvent) {
        if key_event.modifiers != KeyModifiers::NONE {
            return;
        }

        let pos = match key_event.code {
            KeyCode::Char('W') | KeyCode::Char('w') | KeyCode::Up => {
                self.update_cursor_location(stdout, Direction::Up);
                return;
            }
            KeyCode::Char('S') | KeyCode::Char('s') | KeyCode::Down => {
                self.update_cursor_location(stdout, Direction::Down);
                return;
            }
            KeyCode::Char('D') | KeyCode::Char('d') | KeyCode::Right => {
                self.update_cursor_location(stdout, Direction::Right);
                return;
            }
            KeyCode::Char('A') | KeyCode::Char('a') | KeyCode::Left => {
                self.update_cursor_location(stdout, Direction::Left);
                return;
            }
            KeyCode::Char('Q') | KeyCode::Char('q') => {
                self.state = GameState::Quit;
                return;
            }
            _ => match self.get_coordinates_from_cursor() {
                Some(p) => p,
                _ => return,
            },
        };

        match key_event.code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.click(&pos);
                return;
            }
            KeyCode::Char('F') | KeyCode::Char('f') => {
                self.click_flag(&pos);
                return;
            }
            _ => return,
        }
    }

    fn process_mouse(&mut self, stdout: &mut std::io::Stdout, mouse_event: MouseEvent) {
        let mut x = mouse_event.column;
        if x % 2 == 1 {
            x -= 1;
        }
        let y = mouse_event.row;
        execute!(stdout, cursor::MoveTo(x, y)).unwrap();

        let pos = match self.get_coordinates_from_cursor() {
            Some(pos) => pos,
            None => return,
        };

        let button_pressed = match mouse_event.kind {
            MouseEventKind::Down(b) => b,
            _ => return,
        };

        match button_pressed {
            MouseButton::Left => self.click(&pos),
            MouseButton::Right => self.click_flag(&pos),
            _ => return,
        }
    }

    fn click_flag(&mut self, pos: &Coordinates) {
        if self.board.have_flag(pos).unwrap() {
            self.board.remove_flag(pos).unwrap();
        } else {
            self.board.add_flag(pos).unwrap();
        }
    }

    fn click(&mut self, pos: &Coordinates) {
        if self.board.get(pos).unwrap() == ItemType::Landmine {
            self.state = GameState::HitMine;
        }
        self.board.click(pos).unwrap();
    }

    fn update_cursor_location(&self, stdout: &mut std::io::Stdout, dir: Direction) {
        let size = self.board.get_config().char_width as u16;
        match dir {
            Direction::Up => execute!(stdout, cursor::MoveUp(1)).unwrap(),
            Direction::Down => execute!(stdout, cursor::MoveDown(1)).unwrap(),
            Direction::Left => execute!(stdout, cursor::MoveLeft(size)).unwrap(),
            Direction::Right => execute!(stdout, cursor::MoveRight(size)).unwrap(),
        }
    }

    pub fn pos_in_range(&self, x: u32, y: u32) -> bool {
        let config = self.board.get_config();
        x < config.width as u32 && y < config.height as u32
    }

    fn terminal_pos_to_coordinates(&self, x: u16, y: u16) -> Option<Coordinates> {
        let config = self.board.get_config();
        let x = x as usize;
        let y = y as usize;
        if x < 2 || y < 1 {
            None
        } else {
            let x = x / 2 - 1;
            let y = y - 1;
            if x >= config.width || y >= config.height {
                None
            } else {
                Some(Coordinates { x, y })
            }
        }
    }

    pub fn get_coordinates_from_cursor(&self) -> Option<Coordinates> {
        let (x, y) = cursor::position().unwrap();

        self.terminal_pos_to_coordinates(x, y)
    }

    pub fn get_coordinates_from_mouse(&self, mouse_event: &MouseEvent) -> Option<Coordinates> {
        self.terminal_pos_to_coordinates(mouse_event.row, mouse_event.column)
    }
}

pub fn test1() {
    let board_config = BoardConfig::unicode(20, 20, Color::Reset);
    let mut game = Game::new(board_config, 20).unwrap();
    game.game_loop();
}
