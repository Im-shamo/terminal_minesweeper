use crate::mineboard::{Board, BoardConfig, BoardError};
use crate::utils::{Coordinates};
use rand::prelude::*;

#[derive(Clone, Debug, Copy)]
pub enum GameState {
    Playing,
    Win,
    HitMine,
    Quit
}

#[derive(Clone, Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(config: BoardConfig, landmine_count: usize) -> Result<Self, BoardError> {
        let mines = Self::gen_landmine(landmine_count, &config);
        let mut board: Board = Board::new(config, &mines)?;
        board.draw_border();
        Ok(Self {board})
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
        self.board.print();
    }

    pub fn landmine_count(&self) -> &usize {
        self.board.landmine_count()
    }

    fn get_cursor_location_in_board(&self) -> Option<Coordinates> {
        Some(Coordinates {x: 0, y: 0})
    }
}