#![cfg(feature = "bracketed-paste")]
use crate::mineboard::{Board, BoardConfig, BoardError};
use crate::utils::Coordinates;
use crossterm::cursor;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode};
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
use std::io::{stdout, Cursor};

#[derive(Clone, Debug, Copy)]
pub enum GameState {
    Playing,
    Win,
    HitMine,
    Quit,
}

#[derive(Clone, Debug)]
pub struct Game {
    board: Board,
    state: GameState,
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        execute!(stdout, cursor::Show).unwrap();
        self.rander();
        execute!(stdout, cursor::MoveTo(2, 1)).unwrap();
        loop {
            self.proccess_input();
            self.rander();
            match self.state {
                GameState::Quit | GameState::HitMine | GameState::Win => break,
                GameState::Playing => {}
            }
        }
        execute!(stdout, Clear(ClearType::All)).unwrap();
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

    fn get_cursor_location_in_board(&self) -> Option<Coordinates> {
        Some(Coordinates { x: 1, y: 1 })
    }

    fn get_input(&self) -> std::io::Result<Event> {
        let key_event = read()?;
        Ok(key_event)
    }

    fn proccess_input(&mut self) {
        loop {
            let event = self.get_input().unwrap();
            let key_event = match event {
                Event::Key(event) => event,
                _ => continue,
            };
            if key_event.modifiers == KeyModifiers::NONE {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => self.update_cursor_location(Direction::Up),
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S')=> self.update_cursor_location(Direction::Down),
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D')=> self.update_cursor_location(Direction::Right),
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A')=> self.update_cursor_location(Direction::Left),
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        if let Some(pos) = self.get_coordinates_from_cursor() {
                            self.board.add_flag(&pos).unwrap();
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(pos) = self.get_coordinates_from_cursor() {
                            self.board.click(&pos).unwrap();
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => self.state = GameState::Quit,
                    _ => continue,
                }
                break;
            } else if key_event.modifiers == KeyModifiers::SHIFT {
                match key_event.code {
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        if let Some(pos) = self.get_coordinates_from_cursor() {
                            self.board.remove_flag(&pos).unwrap();
                        }
                    }
                    _ => continue,
                }
                break;
            }
        }
    }

    fn update_cursor_location(&self, dir: Direction) {
        let size = self.board.get_config().char_width as u16;
        let mut stdout = stdout();
        match dir {
            Direction::Up => execute!(stdout, cursor::MoveUp(1)).unwrap(),
            Direction::Down => execute!(stdout, cursor::MoveDown(1)).unwrap(),
            Direction::Left => execute!(stdout, cursor::MoveLeft(size)).unwrap(),
            Direction::Right => execute!(stdout, cursor::MoveRight(size)).unwrap(),
        }
    }

    pub fn pos_in_range(&self, x: i32, y: i32) -> bool {
        let config = self.board.get_config();
        x >= 0 && x < config.width as i32 && y >= 0 && y < config.height as i32
    }

    pub fn get_coordinates_from_cursor(&self) -> Option<Coordinates> {
        let (x, y) = cursor::position().unwrap();

        if self.pos_in_range(x as i32 / 2 - 1, y as i32 - 1) {
            Some(Coordinates::new(x as usize / 2 - 1, y as usize - 1))
        } else {
            None
        }
    }
}

pub fn test1() {
    let board_config = BoardConfig::unicode(7, 7, Color::Reset);
    let mut game = Game::new(board_config,4).unwrap();
    game.game_loop();
}
