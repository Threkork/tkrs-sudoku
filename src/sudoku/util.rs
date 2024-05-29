use super::*;

impl CellGuess {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::default();
        for num in self.to_iter() {
            vec.push(num as u8);
        }
        vec
    }
}

pub struct CellGuessIter<'a> {
    cell_guess: &'a CellGuess,
    index: usize,
}
impl<'a> Iterator for CellGuessIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut count = 0;

        for (num, boo) in [
            self.cell_guess.one,
            self.cell_guess.two,
            self.cell_guess.three,
            self.cell_guess.four,
            self.cell_guess.five,
            self.cell_guess.six,
            self.cell_guess.seven,
            self.cell_guess.eight,
            self.cell_guess.nine,
        ]
        .iter()
        .enumerate()
        {
            if *boo {
                count += 1;
            }

            if count == self.index {
                self.index += 1;
                return Some(num + 1);
            }
        }

        None
    }
}
impl CellGuess {
    pub fn to_iter(&self) -> CellGuessIter {
        CellGuessIter {
            cell_guess: self,
            index: 1,
        }
    }
}


impl Sudoku {
    pub fn print_grid_guess(&self) {
        let mut grid_guess_len = 0;
        for index in 0..81 {
            match self.cells[index].value {
                Value::Num(_) => {}
                Value::Null => {
                    if grid_guess_len < self.cells[index].grid_guess.len() {
                        grid_guess_len = self.cells[index].grid_guess.len()
                    }
                }
            }
        }


        for index in 0..81 {
            match self.cells[index].value {
                Value::Null => {
                    let a = |index: usize| -> String {
                        let mut str = String::default();
                        for _i in 0..((grid_guess_len - self.cells[index].grid_guess.len()) * 3 / 2)
                        {
                            str.push(' ')
                        }
                        str
                    };
                    if self.cells[index].grid_guess.len() != grid_guess_len {
                        print!("{}", a(index));
                    }
                    let guess_vec = self.cells[index].grid_guess.to_vec();
                    print!("{:?}", guess_vec);
                    if self.cells[index].grid_guess.len() != grid_guess_len {
                        print!("{}", a(index));
                    }

                    print!("|");
                }
                Value::Num(num) => {
                    for _i in 0..grid_guess_len as isize * 3 / 2 - 1 {
                        print!(" ");
                    }

                    print!("{:?}", num);

                    for _i in 0..grid_guess_len as isize * 3 / 2 {
                        print!(" ");
                    }
                    print!(" |");
                }
            }

            if index % 9 == 8 {
                println!("");
            }
            if index == 26 || index == 53 {
                for _i in 0..(grid_guess_len * 3 + 3) * 9 {
                    print!("-");
                }
                println!("");
            }
        }
    }

    pub fn print_sudoku(&self) {
        for index in 0..81 {
            match self.cells[index].value {
                Value::Null => {
                    print!("0,")
                }
                Value::Num(num) => {
                    print!("{},", num)
                }
            }
            if index % 9 == 8 {
                println!("");
            }
        }
        println!("")
    }
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

        let sudoku = Sudoku::from_array(_H).unwrap();

        sudoku.print_grid_guess();
        sudoku.print_sudoku();
    }

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
