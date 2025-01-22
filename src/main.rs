use terminal_minesweeper::mineboard::{test1, BoardError};

fn main() -> Result<(), BoardError>{
    test1()?;
    Ok(())
}