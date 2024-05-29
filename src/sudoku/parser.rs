use super::*;
use num_traits::ToPrimitive;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<T> {
    #[error("String length is invalid: {0}")]
    InvalidLength(String),

    #[error("Number is not in the range 0..=9: {0}")]
    OutOfRange(T),
}


impl Sudoku {
    pub fn from_array<T>(array_sudoku: [[T; 9]; 9]) -> Result<Sudoku, Error<T>>
    where
        T: ToPrimitive + Copy,
    {
        let mut cell_ = new_cells();
        for i in 0..9 {
            for j in 0..9 {
                let index = i * 9 + j;
                let value_u8 = match array_sudoku[i][j].to_u8() {
                    Some(v) => v,
                    None => return Err(Error::OutOfRange(array_sudoku[i][j])),
                };
                let value = match value_u8 {
                    0 => Value::Null,
                    1..=9 => Value::Num(value_u8),
                    _ => return Err(Error::OutOfRange(array_sudoku[i][j])),
                };


                let region = (i / 3) * 3 + (j / 3) + 1;

                cell_[index].x = i as u8;
                cell_[index].y = j as u8;
                cell_[index].region = region as u8;
                cell_[index].value = value;
            }
        }
        Ok(Sudoku { cells: cell_ })
    }

    pub fn from_string(sudoku_string: &str) -> Result<Sudoku, Error<String>> {
        if sudoku_string.len() != 81 {
            return Err(Error::InvalidLength(format!(
                "Expected length {}, but got length {}",
                81,
                sudoku_string.len()
            )));
        }

        let mut cell_ = new_cells();
        let mut str_iter = sudoku_string.chars();
        for i in 0..9 {
            for j in 0..9 {
                let index = i * 9 + j;
                let char = str_iter.next().unwrap();
                let value = match char.to_digit(10) {
                    Some(num) if num == 0 => Value::Null,
                    Some(num) => Value::Num(num as u8),
                    None => Value::Null,
                };

                let region = (i / 3) * 3 + (j / 3) + 1;
                cell_[index].x = i as u8;
                cell_[index].y = j as u8;
                cell_[index].region = region as u8;
                cell_[index].value = value;
            }
        }
        Ok(Sudoku { cells: cell_ })
    }
}


fn new_cells() -> [Cell; 81] {
    let cell_: [Cell; 81] = [Cell {
        x: 0,
        y: 0,
        region: 0,
        value: Value::Null,
        grid_guess: CellGuess {
            guess_state: CellGuessState::Unattempted,
            len: 0,
            one: false,
            two: false,
            three: false,
            four: false,
            five: false,
            six: false,
            seven: false,
            eight: false,
            nine: false,
        },
    }; 81];
    cell_
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        const _H: [[usize; 9]; 9] = [
            [8, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 3, 6, 0, 0, 0, 0, 0],
            [0, 7, 0, 0, 9, 0, 2, 0, 0],
            [0, 5, 0, 0, 0, 7, 0, 0, 0],
            [0, 0, 0, 0, 4, 5, 7, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 3, 0],
            [0, 0, 1, 0, 0, 0, 0, 6, 8],
            [0, 0, 8, 5, 0, 0, 0, 1, 0],
            [0, 9, 0, 0, 0, 0, 4, 0, 0],
        ];

        let _sudoku = Sudoku::from_array(_H).unwrap();

        _sudoku.print_grid_guess();
        _sudoku.print_sudoku();
    }
}
