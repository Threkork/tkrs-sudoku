pub mod parser;
pub mod sort;
pub mod util;


#[derive(Debug, Clone, Copy)]
pub struct Sudoku {
    pub cells: [Cell; 81],
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub region: u8,
    pub value: Value,
    pub grid_guess: CellGuess,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Num(u8),
    Null,
}


struct CellHome {
    x_home: [usize; 9],
    y_home: [usize; 9],
    region_home: [usize; 9],
}

#[derive(Debug, Clone, Copy)]
pub struct CellGuess {
    guess_state: CellGuessState,
    len: usize,
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    five: bool,
    six: bool,
    seven: bool,
    eight: bool,
    nine: bool,
}

#[derive(Debug, Clone, Copy)]
enum CellGuessState {
    ///未开始
    Unattempted,
    ///拥有猜测
    WithGuess,
    ///确定
    Correct,
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn some() {
        let a = CellGuess {
            guess_state: CellGuessState::WithGuess,
            len: 8,
            one: true,
            two: false,
            three: true,
            four: true,
            five: true,
            six: true,
            seven: true,
            eight: true,
            nine: true,
        };
        for j in a.to_iter() {
            println!("{}", j);
        }
    }
}
