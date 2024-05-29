use std::ops::Index;

use super::*;

// 1
impl Sudoku {
    fn simple_guess_sudoku(&mut self) {
    }
    fn simple_guess_cell(&mut self, index: usize) {
    }
}


impl Sudoku {
    fn guess_grid_init(&mut self) {
        for (index, cell) in self.cells.iter_mut().enumerate() {
            self.cells[index].grid_guess
        }
    }


    ///传入index,检查猜测数是否就剩一个，修改原数独，返回Result
    fn try_fix_guess(&mut self, index: usize) -> Result<(), ()> {
        if self.cells[index].grid_guess.len() == 1 {}
        todo!()
    }

    fn fix_guess(&mut self, index: usize) {
        self.cells[index].grid_guess.guess_state = CellGuessState::Correct;

        self.cells[index].value = Value::Num(
            self.cells[index]
                .grid_guess
                .to_iter()
                .next()
                .unwrap()
                .try_into()
                .unwrap(),
        );
        self.cells[index].grid_guess.len = 0;
    }


    fn find_grid_home(x: usize, y: usize) -> CellHome {
        let x_home = if x == 1 {
            [0, 1, 2, 3, 4, 5, 6, 7, 8]
        } else if x == 2 {
            [9, 10, 11, 12, 13, 14, 15, 16, 17]
        } else if x == 3 {
            [18, 19, 20, 21, 22, 23, 24, 25, 26]
        } else if x == 4 {
            [27, 28, 29, 30, 31, 32, 33, 34, 35]
        } else if x == 5 {
            [36, 37, 38, 39, 40, 41, 42, 43, 44]
        } else if x == 6 {
            [45, 46, 47, 48, 49, 50, 51, 52, 53]
        } else if x == 7 {
            [54, 55, 56, 57, 58, 59, 60, 61, 62]
        } else if x == 8 {
            [63, 64, 65, 66, 67, 68, 69, 70, 71]
        } else {
            [72, 73, 74, 75, 76, 77, 78, 79, 80]
        };

        let y_home = if y == 1 {
            [0, 9, 18, 27, 36, 45, 54, 63, 72]
        } else if y == 2 {
            [1, 10, 19, 28, 37, 46, 55, 64, 73]
        } else if y == 3 {
            [2, 11, 20, 29, 38, 47, 56, 65, 74]
        } else if y == 4 {
            [3, 12, 21, 30, 39, 48, 57, 66, 75]
        } else if y == 5 {
            [4, 13, 22, 31, 40, 49, 58, 67, 76]
        } else if y == 6 {
            [5, 14, 23, 32, 41, 50, 59, 68, 77]
        } else if y == 7 {
            [6, 15, 24, 33, 42, 51, 60, 69, 78]
        } else if y == 8 {
            [7, 16, 25, 34, 43, 52, 61, 70, 79]
        } else {
            [8, 17, 26, 35, 44, 53, 62, 71, 80]
        };

        let region = ((x - 1) / 3) * 3 + (y - 1) / 3 + 1;

        let region_home = if region == 1 {
            [0, 1, 2, 9, 10, 11, 18, 19, 20]
        } else if region == 2 {
            [3, 4, 5, 12, 13, 14, 21, 22, 23]
        } else if region == 3 {
            [6, 7, 8, 15, 16, 17, 24, 25, 26]
        } else if region == 4 {
            [27, 28, 29, 36, 37, 38, 45, 46, 47]
        } else if region == 5 {
            [30, 31, 32, 39, 40, 41, 48, 49, 50]
        } else if region == 6 {
            [33, 34, 35, 42, 43, 44, 51, 52, 53]
        } else if region == 7 {
            [54, 55, 56, 63, 64, 65, 72, 73, 74]
        } else if region == 8 {
            [57, 58, 59, 66, 67, 68, 75, 76, 77]
        } else {
            [60, 61, 62, 69, 70, 71, 78, 79, 80]
        };

        CellHome {
            x_home,
            y_home,
            region_home,
        }
    }
}
