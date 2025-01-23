use crossterm::execute;
use crossterm::style::{Color, Print};
use crossterm::terminal::{Clear, ClearType};
use std::error::Error;
use std::fmt;

use crate::utils::Coordinates;

#[derive(Debug, Clone)]
pub enum ItemType {
    Nothing,
    Landmine,
    Number(i32),
}

#[derive(Debug, Clone)]
pub struct BoardConfig {
    pub height: usize,
    pub width: usize,
    pub char_width: u32,
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
        char_width: u32,
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

pub trait Field<T: Clone + Copy, E: std::error::Error> {
    fn set_field(
        set_values: &[(Coordinates, T)],
        width: usize,
        height: usize,
        default_value: T,
    ) -> Result<Vec<Vec<T>>, BoardError> {
        let mut field = vec![vec![default_value; width]; height];
        for (pos, value) in set_values {
            if pos.x >= width || pos.y >= height {
                return Err(BoardError::CoordinatesOutOffRange);
            }
            field[pos.y][pos.x] = *value;
        }
        Ok(field)
    }

    fn get_width(&self) -> &usize;
    fn get_height(&self) -> &usize;
    fn get_field(&self) -> &Vec<Vec<T>>;
    fn get_count(&self) -> &usize;
    fn get(&self, pos: &Coordinates) -> Result<&T, E>;
}

pub trait ChangeableField {
    fn add(&mut self, pos: &Coordinates) -> Result<(), BoardError>;
    fn remove(&mut self, pos: &Coordinates) -> Result<(), BoardError>;
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
        landmine_positions: &[Coordinates],
        width: usize,
        height: usize,
    ) -> Result<LandmineField, BoardError> {
        let count = landmine_positions.len();
        let set_values: Vec<(Coordinates, bool)> =
            landmine_positions.iter().map(|pos| (*pos, true)).collect();
        let field = LandmineField::set_field(&set_values, width, height, false)?;
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

impl Field<bool, BoardError> for LandmineField {
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

    fn get(&self, pos: &Coordinates) -> Result<&bool, BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            Err(BoardError::CoordinatesOutOffRange)
        } else {
            Ok(&self.field[pos.y][pos.x])
        }
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
        let nothing = vec![];
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

impl Field<bool, BoardError> for FlagsField {
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

    fn get(&self, pos: &Coordinates) -> Result<&bool, BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            Err(BoardError::CoordinatesOutOffRange)
        } else {
            Ok(&self.field[pos.y][pos.x])
        }
    }
}

impl ChangeableField for FlagsField {
    fn add(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            return Err(BoardError::CoordinatesOutOffRange);
        }

        self.count += 1;
        self.field[pos.y][pos.x] = true;
        Ok(())
    }

    fn remove(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            return Err(BoardError::CoordinatesOutOffRange);
        }

        self.count += 1;
        self.field[pos.y][pos.x] = false;
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
        let nothing: Vec<(Coordinates, bool)> = vec![];
        let field = FlagsField::set_field(&nothing, width, height, false)?;
        let symbol_open = "  ";
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

impl Field<bool, BoardError> for OpenedField {
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

    fn get(&self, pos: &Coordinates) -> Result<&bool, BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            Err(BoardError::CoordinatesOutOffRange)
        } else {
            Ok(&self.field[pos.y][pos.x])
        }
    }
}

impl ChangeableField for OpenedField {
    fn add(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            return Err(BoardError::CoordinatesOutOffRange);
        }

        self.count += 1;
        self.field[pos.y][pos.x] = true;
        Ok(())
    }

    fn remove(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            return Err(BoardError::CoordinatesOutOffRange);
        }

        self.count += 1;
        self.field[pos.y][pos.x] = false;
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
    pub fn new(width: usize, height: usize, landmines: &LandmineField) -> Result<Self, BoardError> {
        let field = NumberField::calculate_numbers(width, height, landmines)?;
        let count = Self::get_count(&field);
        Ok(NumberField {
            width,
            height,
            field,
            symbol: " ",
            count,
        })
    }

    fn calculate_numbers(
        width: usize,
        height: usize,
        landmines: &LandmineField,
    ) -> Result<Vec<Vec<i32>>, BoardError> {
        let mut set_values = vec![];
        for i in 0..width as usize {
            for j in 0..height as usize {
                let mut count = 0;
                if i >= 1 && j >= 1 && i - 1 < width && j - 1 < height {
                    if *landmines.get(&Coordinates { x: i - 1, y: j - 1 })? {
                        count += 1;
                    }
                }

                if i >= 1 && i - 1 < width {
                    if *landmines.get(&Coordinates { x: i - 1, y: j })? {
                        count += 1;
                    }
                }

                if i >= 1 && i - 1 < width && j + 1 < height {
                    if *landmines.get(&Coordinates { x: i - 1, y: j + 1 })? {
                        count += 1;
                    }
                }

                if j >= 1 && j - 1 < height {
                    if *landmines.get(&Coordinates { x: i, y: j - 1 })? {
                        count += 1;
                    }
                }

                if j + 1 < height {
                    if *landmines.get(&Coordinates { x: i, y: j + 1 })? {
                        count += 1;
                    }
                }

                if i + 1 < width && j >= 1 && j - 1 < height {
                    if *landmines.get(&Coordinates { x: i + 1, y: j - 1 })? {
                        count += 1;
                    }
                }

                if i + 1 < width {
                    if *landmines.get(&Coordinates { x: i + 1, y: j })? {
                        count += 1;
                    }
                }

                if i + 1 < width && j + 1 < height {
                    if *landmines.get(&Coordinates { x: i + 1, y: j + 1 })? {
                        count += 1;
                    }
                }

                set_values.push((Coordinates { x: i, y: j }, count));
            }
        }
        let field = NumberField::set_field(&set_values, width, height, 0)?;
        Ok(field)
    }

    fn get_count(field: &Vec<Vec<i32>>) -> usize {
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

impl Field<i32, BoardError> for NumberField {
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

    fn get(&self, pos: &Coordinates) -> Result<&i32, BoardError> {
        if pos.x >= self.width || pos.y >= self.height {
            Err(BoardError::CoordinatesOutOffRange)
        } else {
            Ok(&self.field[pos.y][pos.x])
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    config: BoardConfig,
    landmines: LandmineField,
    flags: FlagsField,
    opened: OpenedField,
    numbers: NumberField,
    framebuffer: Vec<Vec<String>>,
}

impl Board {
    pub fn new<'b>(
        config: BoardConfig,
        landmine_pos: &'b [Coordinates],
    ) -> Result<Board, BoardError> {
        let landmines = LandmineField::new(landmine_pos, config.width, config.height)?;
        let flags = FlagsField::new(config.width, config.height)?;
        let opened = OpenedField::new(config.width, config.height)?;
        let framebuffer = vec![vec![String::new(); config.width + 2]; config.height + 3];
        let numbers = NumberField::new(config.width, config.height, &landmines)?;
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
        self.landmines.get_count()
    }

    pub fn get_config(&self) -> &BoardConfig {
        &self.config
    }

    pub fn draw_border(&mut self) {
        // Top
        self.framebuffer[0][0] = self.config.border_top_left_symbol.to_string();
        for i in 1..self.config.width + 1 {
            self.framebuffer[0][i] = self.config.border_top.to_string();
        }
        self.framebuffer[0][self.config.width + 1] =
            self.config.border_top_right_symbol.to_string();

        // Side
        for j in 1..self.config.height + 1 {
            self.framebuffer[j][0] = self.config.border_left.to_string();
            self.framebuffer[j][self.config.width + 1] = self.config.border_right.to_string();
        }

        // Bottom
        self.framebuffer[self.config.height + 1][0] =
            self.config.border_bottom_left_symbol.to_string();
        for i in 1..self.config.width + 1 {
            self.framebuffer[self.config.height + 1][i] = self.config.border_bottom.to_string();
        }
        self.framebuffer[self.config.height + 1][self.config.width + 1] =
            self.config.border_bottom_right_symbol.to_string();
    }

    pub fn draw_field(&mut self) -> Result<(), BoardError> {
        for i in 0..self.config.width {
            for j in 0..self.config.height {
                let pos = Coordinates::new(i, j);
                self.framebuffer[j + 1][i + 1] = if *self.flags.get(&pos)? {
                    self.flags.symbol.to_string()
                } else if *self.opened.get(&pos)? {
                    if *self.landmines.get(&pos)? {
                        self.landmines.symbol.to_string() 
                    } else if *self.numbers.get(&pos)? > 0 {
                        format!("{}{}", self.numbers.get(&pos)?, self.numbers.symbol)
                    } else {
                        self.opened.symbol_open.to_string()
                    }
                } else {
                    self.opened.symbol_closed.to_string()
                }
            }
        }
        Ok(())
    }

    pub fn print(&mut self) {
        let mut f = std::io::stdout();
        self.draw_field().unwrap();
        execute!(f, Print(self)).unwrap();
    }

    pub fn add_flag(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        self.flags.add(pos)?;
        Ok(())
    }

    pub fn remove_flag(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        self.flags.remove(pos)?;
        Ok(())
    }

    pub fn have_flag(&self, pos: &Coordinates) -> Result<bool, BoardError> {
        self.flags.get(pos)?
    }

    pub fn click(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        self.opened.add(pos)?;
        Ok(())
    }

    pub fn unclick(&mut self, pos: &Coordinates) -> Result<(), BoardError> {
        self.opened.remove(pos)?;
        Ok(())
    }

    pub fn get(&self, &pos) -> Result<ItemType, BoardError> {
        if self.board.landmines.get(pos)? {
            Ok(ItemType::Landmine)
        } else if self.board.numbers.get(pos)? {
            Ok(ItemType::Number(self.board.number.get(pos)?))
        } else {
            Ok(ItemType::Nothing)
        }
    }
}

impl<'a> std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.framebuffer.iter().enumerate() {
            for (j, symbol) in row.iter().enumerate() {
                if symbol.len() == 1 && self.config.char_width == 2 {
                    write!(f, "1{}", self.numbers.symbol)?;
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
    let config = BoardConfig::unicode(10, 10, Color::Reset);
    let mine = vec![Coordinates { x: 1, y: 2 }];
    let mut board = Board::new(config, &mine)?;

    board.add_flag(&Coordinates { x: 1, y: 2 })?;
    board.click(&Coordinates { x: 1, y: 2 })?;
    board.click(&Coordinates { x: 1, y: 1 })?;
    board.draw_border();
    board.print();
    println!();
    Ok(())
}

#[cfg(test)]
mod test {}
