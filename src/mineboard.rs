use crossterm::execute;
use crossterm::style::{Color, Print};
use std::error::Error;
use std::io::Write;
use std::path::Display;

use crate::utils::Coordinates;
#[derive(Debug, Clone)]
pub struct BoardConfig {
    pub height: usize,
    pub width: usize,
    pub char_width: i32,
    pub border_top_right_symbol: &'static str,
    pub border_top_left_symbol: &'static str,
    pub border_bottom_right_symbol: &'static str,
    pub border_bottom_left_symbol: &'static str,
    pub border_top: &'static str,
    pub border_bottom: &'static str,
    pub border_right: &'static str,
    pub border_left: &'static str,
    pub board_background_colour: Color,
}

impl BoardConfig {
    pub fn new(
        height: usize,
        width: usize,
        char_width: i32,
        border_top_right_symbol: &'static str,
        border_top_left_symbol: &'static str,
        border_bottom_right_symbol: &'static str,
        border_bottom_left_symbol: &'static str,
        border_top: &'static str,
        border_bottom: &'static str,
        border_right: &'static str,
        border_left: &'static str,
        board_background_colour: Color,
    ) -> Self {
        BoardConfig {
            height,
            width,
            char_width,
            border_top_right_symbol,
            border_top_left_symbol,
            border_bottom_right_symbol,
            border_bottom_left_symbol,
            border_top,
            border_bottom,
            border_right,
            border_left,
            board_background_colour,
        }
    }

    pub fn unicode(width: usize, height: usize, board_background_colour: Color) -> Self {
        BoardConfig::new(
            height,
            width,
            2,
            "─┐",
            "┌─",
            "─┘",
            "└─",
            "──",
            "──",
            " │",
            "│ ",
            board_background_colour,
        )
    }
}

#[derive(Debug)]
pub enum BoardError {
    CoordinatesOutOffRange,
}

impl Error for BoardError {}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::CoordinatesOutOffRange => write!(f, "coordinates out off range"),
        }
    }
}

impl fmt::Debug for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::CoordinatesOutOffRange => write!(f, "coordinates out off range"),
        }
    }
}

pub trait Field<T> {
    fn set_field(
        set_values: &Vec<(Vec<Coordinates>, T),
        width: usize,
        height: usize,
        default_value: T,
    ) -> Result<Vec<Vec<T>>, BoardError> {
        let mut field = vec![vec![defualt_value; width]; height];
        for (pos, value) in set_values {
            if pos.x >= width || pos.y >= height {
                return Err(BoardError::CoordinatesOutOffRange);
            }
            field[y][x] = value;
        }
        Ok(field)
    }

    fn get_width(&self) -> &usize;
    fn get_height(&self) -> &usize;
    fn get_field(&self) -> &Vec<Vec<T>>;
    fn get_count(&self) -> &usize;
    fn get(&self, pos: &Coordinates) -> &T;
}

pub trait AddField {
    fn add(&mut self, pos: &Coordinates) -> Result<(), BoardError>;
}

#[derive(Debug, Clone)]
pub struct LandmineField {
    width: usize,
    height: usize,
    field: Vec<Vec<bool>>,
    count: usize,
    symbol: &'static str,
}

impl LandmineField {
    pub fn new(
        landmine_positions: &Vec<Coordinates>,
        width: usize,
        height: usize,
    ) -> Result<LandmineField, BoardError> {
        let count = landmine_positions.len();
        let field = LandmineField::set_field(landmine_positions.into_iter().map(|pos| (pos, false)).collect(), width, height, false)?;
        let symbol = "💣";
        Ok(LandmineField {
            width,
            height,
            field,
            count,
            symbol,
        })
    }
}

impl Field<bool> for LandmineField {
    fn get_width(&self) -> &usize {
        &self.width
    }

    fn get_height(&self) -> &usize {
        &self.height
    }

    fn get_field(&self) -> &Vec<Vec<bool>> {
        &self.field
    }

    fn get_count(&self) -> &usize {
        &self.count
    }

    fn get(&self, pos: &Coordinates) -> &bool {
        &self.field[pos.x][pos.y]
    }
}

impl Error for LandmineField {}

impl std::fmt::Display for LandmineField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something bad happened")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FlagsField {
    width: usize,
    height: usize,
    field: Vec<Vec<bool>>,
    count: usize,
    symbol: &'static str,
}

impl FlagsField {
    pub fn new(width: usize, height: usize) -> Result<FlagsField, BoardError> {
        let count = 0;
        let nothin = vec![];
        let field = FlagsField::set_field(&nothing, width, height, false)?;
        let symbol = "🚩";
        Ok(FlagsField {
            width,
            height,
            field,
            count,
            symbol,
        })
    }
}

impl Field<bool> for FlagsField {
    fn get_width(&self) -> &usize {
        &self.width
    }

    fn get_height(&self) -> &usize {
        &self.height
    }

    fn get_count(&self) -> &usize {
        &self.count
    }

    fn get_field(&self) -> &Vec<Vec<bool>> {
        &self.field
    }

    fn get(&self, pos: &Coordinates) -> &bool {
        &self.field[pos.x][pos.y]
    }
}

impl AddField for FlagsField {
    fn add(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            return Err(BoardError::CoordinatesOutOffRange);
        }

        self.count += 1;
        self.field[pos.x][pos.y] = true;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OpenedField {
    width: usize,
    height: usize,
    field: Vec<Vec<bool>>,
    count: usize,
    symbol_open: &'static str,
    symbol_closed: &'static str,
}

impl OpenedField {
    pub fn new(width: usize, height: usize) -> Result<OpenedField, BoardError> {
        let count = 0;
        let nothing: = vec![];
        let field = FlagsField::set_field(&nothing, width, height, false)?;
        let symbol_open = "░░";
        let symbol_closed = "██";
        Ok(OpenedField {
            width,
            height,
            field,
            count,
            symbol_open,
            symbol_closed,
        })
    }
}

impl Field<bool> for OpenedField {
    fn get_width(&self) -> &usize {
        &self.width
    }

    fn get_height(&self) -> &usize {
        &self.height
    }

    fn get_count(&self) -> &usize {
        &self.count
    }

    fn get_field(&self) -> &Vec<Vec<bool>> {
        &self.field
    }

    fn get(&self, pos: &Coordinates) -> &bool {
        &self.field[pos.y][pos.x]
    }
}

impl AddField for OpenedField {
    fn add(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            return Err(BoardError::CoordinatesOutOffRange);
        }

        self.count += 1;
        self.field[pos.y][pos.x] = true;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct NumberField {
    width: usize,
    height: usize,
    field: Vec<Vec<i32>>,
    symbol: &'static str,
    count: usize,
}

impl NumberField {
    pub fn new(width: usize, height: usize, landmines: &LandmineField) -> Self {
        let field = NumberField::calculate_numbers(width, height, landmines);
        let count = Self::count(&field);
        NumberField {
            width,
            height,
            field: vec![vec![]],
            symbol: "░",
            count,
        }
    }


    fn calculate_numbers(width: usize, height: usize, landmines: &LandmineField) -> Vec<Vec<i32>> {
        let set_values = vec![];
        for i in 0..width as usize {
            for j in 0..height as usize {
                let mut count = 0;
                if i - 1 >= 0 && j - 1 >= 0 && i - 1 < width && j - 1 < height {
                    if landmines.get(&Coordinates {x: i - 1, y: j - 1}) {
                        count += 1;
                    }
                }

                if i - 1 >= 0 && i - 1 < width {
                    if landmines.get(&Coordinates {x: i - 1, y: j}) {
                        count += 1;
                    }
                }
                
                if i - 1 >= 0 && j + 1 >= 0 && i - 1 < width && j + 1 < height {
                    if landmines.get(&Coordinates {x: i - 1, y: j + 1}) {
                        count += 1;
                    }
                }
                
                if j - 1 >= 0 && j - 1 < height {
                    if landmines.get(&Coordinates {x: i , y: j - 1}) {
                        count += 1;
                    }
                }
                
                if j + 1 >= 0 && j + 1 < height {
                    if landmines.get(&Coordinates {x: i, y: j + 1}) {
                        count += 1;
                    }
                }
                
                if i + 1 >= 0 && i + 1 < width && j - 1 >= 0 && j - 1 < height{
                    if landmines.get(&Coordinates {x: i + 1, y: j - 1}) {
                        count += 1;
                    }
                }
                
                if i + 1 >= 0 && i + 1 < width {
                    if landmines.get(&Coordinates {x: i + 1, y: j}) {
                        count += 1;
                    }
                }
                
                if i + 1 >= 0 && i + 1 < width && j + 1 >= 0 && j + 1 < height {
                    if landmines.get(&Coordinates {x: i + 1, y: j + 1}) {
                        count += 1;
                    }
                }
                
                set_values.push((Coordinates {x: i, y: j}, count));
            }

        }
        let mut field = numberfield::set_field(set_values, width, height, 0);
        field
    }

    fn count(field: &Vec<Vec<i32>>) -> usize {
        let mut count: usize = 0;
        for row in field {
            for n in row {
                if *n > 0 {
                    count += 1;
                }
            }
        }
        count
    }
    
}

impl Field<i32> for NumberField {
    fn get_count(&self) -> &usize {
        &self.count
    }

    fn get_field(&self) -> &Vec<Vec<i32>> {
        &self.field
    }

    fn get_height(&self) -> &usize {
        &self.height
    }

    fn get_width(&self) -> &usize {
        &self.width
    }

    fn get(&self, pos: &Coordinates) -> &i32 {
        &self.field[pos.y][pos.x]
    }
}

#[derive(Debug, Clone)]
pub struct Board<'a> {
    config: BoardConfig,
    landmines: LandmineField,
    flags: FlagsField,
    opened: OpenedField,
    numbers: NumberField,
    framebuffer: Vec<Vec<&'a str>>,
}

impl<'a> Board<'a> {
    pub fn new(config: BoardConfig, landmine_pos: &Vec<Coordinates>) -> Result<Board, BoardError> {
        let landmines = LandmineField::new(landmine_pos, config.width, config.height)?;
        let flags = FlagsField::new(config.width, config.height)?;
        let opened = OpenedField::new(config.width, config.height)?;
        let framebuffer = vec![vec![""; config.width + 2]; config.height + 2];
        let numbers = NumberField::new(config.width, config.height, &landmines);
        Ok(Board {
            config,
            landmines,
            flags,
            opened,
            numbers,
            framebuffer,
        })
    }

    pub fn landmine_count(&self) -> &usize {
        self.landmines.count();
    }

    pub fn get_config(&self) -> &BoardConfig {
        &self.config
    }

    pub fn draw_border(&mut self) {
        // Top
        self.framebuffer[0][0] = self.config.border_top_left_symbol;
        for i in 1..self.config.width + 1 {
            self.framebuffer[0][i] = self.config.border_top;
        }
        self.framebuffer[0][self.config.width + 1] = self.config.border_top_right_symbol;

        // Side
        for j in 1..self.config.height + 1 {
            self.framebuffer[j][0] = self.config.border_left;
            self.framebuffer[j][self.config.width + 1] = self.config.border_right;
        }

        // Bottom
        self.framebuffer[self.config.height + 1][0] = self.config.border_bottom_left_symbol;
        for i in 1..self.config.width + 1 {
            self.framebuffer[self.config.height + 1][i] = self.config.border_bottom;
        }
        self.framebuffer[self.config.height + 1][self.config.width + 1] =
            self.config.border_bottom_right_symbol;
    }

    pub fn draw_field(&mut self) {
        for i in 0..self.config.width {
            for j in 0..self.config.height {
                let pos = Coordinates::new(i, j);
                self.framebuffer[j + 1][i + 1] = if self.opened.get(&pos) {
                    if self.flags.get(&pos) {
                        self.flags.symbol
                    } else if self.numbers.get(&pos) > 0 {
                        self.numbers.symbol
                    } else if self.landmines.get(&pos) {
                        self.landmines.symbol
                    } else {
                        self.opened.symbol_open
                    }
                } else {
                    self.opened.symbol_closed
                }
            }
        }
    }

    pub fn print(&mut self) {
        let mut f = std::io::stdout();
        self.draw_field();
        execute!(f, Print(self)).unwrap();
    }

    pub fn add_flag(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        self.flags.add(pos)?;
        Ok(())
    }

    pub fn click(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        self.opened.add(pos)?;
        Ok(())
    }
}

impl<'a> std::fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.framebuffer.iter().enumerate() {
            for (j, symbol) in row.iter().enumerate() {
                if symbol.len() == 1 && self.config.char_width == 2 {
                    write!(
                        f,
                        "{}{}",
                        self.numbers.get(&Coordinates { x: i, y: j }),
                        self.numbers.symbol
                    )?;
                } else {
                    write!(f, "{}", symbol)?;
                }
            }
            write!(f, "\n\r")?;
        }
        Ok(())
    }
}

pub fn test1() -> Result<(), BoardError> {
    let config = BoardConfig::unicode(12, 10, Color::Reset);
    let mine = vec![Coordinates { x: 1, y: 2 }];
    let mut board = Board::new(config, &mine)?;
    board.add_flag(&Coordinates { x: 1, y: 2 })?;
    board.click(&Coordinates { x: 1, y: 2 })?;
    board.click(&Coordinates { x: 1, y: 1 })?;
    board.draw_border();
    board.print();
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

}
