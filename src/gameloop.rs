use crate::mineboard::{Board, BoardConfig, Field, AddField};
use crate::utils::{Coordinates};
use rang::prelude::*;
use crossterm::execute;
use crossterm::style::Print;

#[derived(Clone, Eq, Debug)]
pub enum GameState {
    Playing,
    Win,
    HitMine,
    Quit
}

#[derived(Clone, Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(config: BoardConfig, landmine_count: usize) -> Result<Self, BoardError> {
        let mines = gen_landmine(landmine_count, &config);
        let board = Board::new(config, &mines);
        let game = Self {board};
        board.draw_border();
        game
    }

    fn gen_landmine(landmine_count: usize, config: &BoardConfig) -> Vec<Coordinates> {
        let rng = rang::thread_rang();

        let mines = vec![];

        let height = config.height();
        let width = config.width();

        for _ in 0..landmine_count {
            let pos = loop {
                let pos = Coordinates::new(rng.gen_range(0..width), rng.gen_range(0..height));
                if mines.find(|&&p| p == pos) {
                    Some(_) => continue;
                    None => break pos;
                }
            }
            mines.push(pos);
        }
        mines
    }

    fn rander(&mut self) {
        self.board.print();
    }

    pub fn landmine_count(&self) -> &usize {
        self.board.landmine_count()
    }

    fn get_cursor_location_in_board(&self) -> Option(Coordinates) {
        
    }
}