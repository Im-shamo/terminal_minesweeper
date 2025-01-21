use terminal_minesweeper::mineboard::{test1, Errors};

fn main() -> Result<(), Errors>{
    test1()?;
    Ok(())
}