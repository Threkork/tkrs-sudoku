#[derive(Debug, Clone)]
pub struct Sudoku {
    pub cells: Vec<Grid>,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Num(i8),
    Null,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub x: i8,
    pub y: i8,
    pub region: i8,
    pub value: Value,
    pub po: Vec<i8>,
}

struct GridHome {
    x_home: [usize; 9],
    y_home: [usize; 9],
    region_home: [usize; 9],
}

///
/// sudoku的普通部分
impl Sudoku {
    pub fn string_to_sudoku(sudoku_str: String) -> Sudoku {
        if sudoku_str.len() != 81 {
            panic!()
        }
        let mut sudoku = Sudoku { cells: Vec::new() };

        for index in 0..81 {
            let y = index % 9 + 1;
            let x = index / 9 + 1;
            /* let value = */
            let region = ((x - 1) / 3) * 3 + ((y - 1) / 3) + 1;

            let char_at_index = sudoku_str.chars().nth(index).unwrap();
            let value = match char_at_index {
                '.' => Value::Null,
                _ => {
                    let num = char_at_index.to_digit(10).unwrap() as i8;
                    Value::Num(num)
                }
            };

            sudoku.cells.push(Grid {
                x: x as i8,
                y: y as i8,
                region: region as i8,
                value,
                po: Vec::new(),
            })
        }

        sudoku
    }

    pub fn array_to_sudoku(a: [[i8; 9]; 9]) -> Sudoku {
        let mut sudoku = Sudoku { cells: Vec::new() };

        for i in 0..9 {
            for j in 0..9 {
                let value = if a[i][j] != 0 {
                    Value::Num(a[i][j])
                } else {
                    Value::Null
                };
                let region = (i / 3) * 3 + (j / 3) + 1;

                sudoku.cells.push(Grid {
                    x: (i + 1) as i8,
                    y: (j + 1) as i8,
                    region: region as i8,
                    value,
                    po: Vec::new(),
                })
            }
        }

        sudoku
    }

    pub fn print_po(&self) {
        let mut po_len = 0;
        for index in 0..81 {
            match self.cells[index].value {
                Value::Num(_) => {}
                Value::Null => {
                    if po_len < self.cells[index].po.len() {
                        po_len = self.cells[index].po.len()
                    }
                }
            }
        }
        let mut index = 0;

        for _i in 0..9 {
            for _j in 0..9 {
                match self.cells[index].value {
                    Value::Null => {
                        if self.cells[index].po.len() != po_len {
                            for _i in 0..((po_len - self.cells[index].po.len()) * 3 / 2) {
                                print!(" ");
                            }
                        }

                        print!("{:?}", self.cells[index].po);
                        if self.cells[index].po.len() != po_len {
                            for _i in 0..((po_len - self.cells[index].po.len()) * 3 / 2) {
                                print!(" ");
                            }
                        }

                        print!("|");
                    }
                    Value::Num(num) => {
                        for _i in 0..po_len as isize * 3 / 2 - 1 {
                            print!(" ");
                        }

                        print!("{:?}", num);

                        for _i in 0..po_len as isize * 3 / 2 {
                            print!(" ");
                        }
                        print!("|");
                    }
                }
                index += 1;
            }
            println!("");
            if _i == 2 || _i == 5 {
                for _i in 0..(po_len * 3 + 3) * 8 {
                    print!("-");
                }
                println!("");
            }
        }
    }

    pub fn print_sudoku(&self) {
        let mut index = 0;
        for _i in 0..9 {
            for _j in 0..9 {
                match self.cells[index].value {
                    Value::Null => {
                        print!("0, ")
                    }
                    Value::Num(num) => {
                        print!("{:?}, ", num)
                    }
                }
                index += 1;
            }
            println!("")
        }
    }

    fn sudoku_end(&self) -> bool {
        let mut i = 0;
        for index in 0..81 {
            if let Value::Null = self.cells[index].value {
                i += 1;
            }
        }
        if i != 0 { false } else { true }
    }

    fn sudoku_err(&self) -> bool {
        for index in 0..81 {
            match self.cells[index].value {
                Value::Num(_) => {
                    continue;
                }
                Value::Null => {
                    if self.cells[index].po.len() == 0 {
                        return true;
                    }
                }
            }
        }
        let find_any_arr: [[usize; 2]; 9] = [
            [1, 1],
            [2, 4],
            [3, 7],
            [4, 2],
            [5, 5],
            [6, 8],
            [7, 3],
            [8, 6],
            [9, 9],
        ];
        let check_duplicates = |indexes: &[usize]| -> bool {
            let mut numbers = [false; 10];
            for &index in indexes.iter() {
                let value = self.cells[index].value;
                if let Value::Num(number) = value {
                    let number = number as usize;
                    if numbers[number] {
                        return true; // Duplicate number found
                    } else {
                        numbers[number] = true;
                    }
                }
            }
            false // No duplicates found
        };

        // Check each region in the grid
        for i in find_any_arr.iter() {
            let some = Self::find_grid_home(i[0], i[1]);
            if !check_duplicates(&some.x_home)
                || !check_duplicates(&some.y_home)
                || !check_duplicates(&some.region_home)
            {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
struct StackGrid {
    x: usize,
    y: usize,
    index: usize,
    value: Value,
    po: Vec<i8>,
    po_index: isize,
}

#[derive(Debug, Clone)]
struct PoPop {
    po_value: i8,
    index: usize,
    stack: Vec<StackGrid>,
}

struct ReturnSudoku {
    test: usize,
    sudoku: Sudoku,
    stack: Vec<StackGrid>,
}

impl Sudoku {
    pub fn solve_sudoku(&mut self) {
        match Self::backtracking_algorithm(self, Vec::new(), 0) {
            Ok(return_sudoku) => {
                if return_sudoku.test == 0 {
                    /* println!("没有调用回溯算法就解决了呢"); */
                    println!("\n\n\n没用回溯算法就解决了呢，杂鱼哥哥果然是个废柴呢，这都不会\n");
                } else {
                    /* println!("穷举了{}次", return_sudoku.test); */
                    println!(
                        "\n\n\n回溯算法穷举了{}次就解决了，杂鱼哥哥果然是个笨蛋呢\n",
                        return_sudoku.test
                    );
                }

                /* println!("数独解决完毕！！！"); */
                println!("哼哼~杂鱼哥哥，居然需要我才能解决数独，真是没用呢~");

                return_sudoku.sudoku.print_sudoku();
            }
            Err(_) => todo!(),
        };
    }

    fn find_min_po_index(sudoku: Sudoku, stack: Vec<StackGrid>) -> Option<usize> {
        let mut grid_min = 404;
        let mut min_grid_len = 10;
        for index_ in 0..81 {
            if let Value::Num(_) = sudoku.cells[index_].value {
                continue;
            }

            let mut bool: bool = false;
            for i in stack.iter() {
                if i.index == index_ {
                    bool = true;
                    break;
                }
            }
            if bool {
                continue;
            }

            if sudoku.cells[index_].po.len() < min_grid_len {
                print!(" {:?}", index_);
                min_grid_len = sudoku.cells[index_].po.len();
                println!("len={:?}", min_grid_len);
                grid_min = index_;
            }
        }
        if grid_min == 404 {
            return None;
        }
        Some(grid_min)
    }

    ///
    /// 控制stack和stack里面po，返回下一个po
    fn po_pop(stack: Vec<StackGrid>) -> PoPop {
        let mut stack_ = stack.clone();

        let end_stack_index = stack_.len() - 1;
        stack_[end_stack_index].po_index += 1;

        if stack_[end_stack_index].po_index as usize != stack_[end_stack_index].po.len() {
            let po_value_: i8 =
                stack_[end_stack_index].po[stack_[end_stack_index].po_index as usize];

            return PoPop {
                po_value: po_value_,
                index: end_stack_index,
                stack: stack_,
            };
        } else {
            if stack_.len() == 0 {
                panic!()
            }
            stack_[end_stack_index].po_index = 0;

            stack_.pop();

            return Self::po_pop(stack_);
        }
    }

    fn backtracking_algorithm(
        sudoku_: &mut Sudoku,
        mut stack_: Vec<StackGrid>,
        mut test: usize,
    ) -> Result<ReturnSudoku, ()> {
        let mut old_sudoku = sudoku_.clone();

        if test == 0 {
            if let Ok(_) = sudoku_.guess_sudoku() {
                return Ok(ReturnSudoku {
                    test,
                    sudoku: sudoku_.clone(),
                    stack: stack_.clone(),
                });
            }
        }

        /* ************************************************************* */
        let mut sudoku: Sudoku = sudoku_.clone();
        let mut stack = stack_.clone();
        if stack.len() == 0 {
            if let Ok(_) = sudoku_.guess_sudoku() {
                return Ok(ReturnSudoku {
                    test,
                    sudoku: sudoku_.clone(),
                    stack,
                });
            }
            sudoku = sudoku_.clone();
        }

        let min_index;
        println!("\n\n\n\n");
        sudoku.print_po();
        if let Some(index_) = Self::find_min_po_index(sudoku.clone(), stack.clone()) {
            min_index = index_
        } else {
            panic!()
        }

        let stack_grid = StackGrid {
            x: sudoku.cells[min_index].x as usize,
            y: sudoku.cells[min_index].y as usize,
            index: ((sudoku.cells[min_index].x - 1) * 9 + sudoku.cells[min_index].y - 1) as usize,
            value: Value::Null,
            po: sudoku.cells[min_index].po.clone(),
            po_index: -1,
        };
        stack.push(stack_grid);

        loop {
            let po_pop = Self::po_pop(stack);
            test += 1;
            stack = po_pop.stack;

            stack[po_pop.index].value = Value::Num(po_pop.po_value);

            match sudoku.fix_backtracking_algorithm_sudoku(stack.clone()) {
                Ok(_) => {
                    return Ok(ReturnSudoku {
                        test,
                        sudoku: sudoku_.clone(),
                        stack,
                    });
                }
                Err(sudoku__) => {
                    sudoku.fix_backtracking_algorithm_sudoku_err(sudoku_);
                    return Self::backtracking_algorithm(&mut sudoku, stack, test);
                }
            }
        }

        /****************************************************************/

        /****************************************************************/
    }

    fn fix_backtracking_algorithm_sudoku_err(&mut self, old_self: &Sudoku) {
        *self = old_self.clone();
    }

    fn fix_backtracking_algorithm_sudoku(
        &mut self,
        stack: Vec<StackGrid>,
    ) -> Result<(), GuessSudokuErr> {
        /* let mut sudoku = self.clone(); */

        for stack_grid in stack.iter() {
            self.cells[stack_grid.index].value = stack_grid.value;
            self.cells[stack_grid.index].po.clear();
        }
        if self.sudoku_err() {
            return Err(GuessSudokuErr::PoVecErr);
        }

        let err;

        match self.guess_sudoku() {
            Ok(_) => {
                return Ok(());
            }
            Err(return_err) => err = return_err,
        }
        match err {
            GuessSudokuErr::PoVecErr => return Err(GuessSudokuErr::PoVecErr),
            GuessSudokuErr::UnableContinue => return Err(GuessSudokuErr::UnableContinue),
            GuessSudokuErr::UserErr => return Err(GuessSudokuErr::UserErr),
        }
    }
}

pub enum GuessSudokuErr {
    PoVecErr,
    UnableContinue,
    UserErr,
}

///
/// sudoku的算法部分
impl Sudoku {
    pub fn guess_sudoku(&mut self) -> Result<(), GuessSudokuErr> {
        self.guess_sudoku_grid();

        let mut test = 0;
        let mut stage_counter = 0;
        loop {
            if self.sudoku_end() {
                /* println!("数独解题完毕"); */
                return Ok(());
            }

            /////////
            // 读取控制台的输出
            /* let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let trim_input = input.trim();
            if trim_input == "abc".to_string() {
                println!("结束");
                return Err(GuessSudokuErr::UserErr);
            }
            if trim_input == "po".to_string() {
                self.print_po();
                continue;
            } */
            //////////
            test += 1;
            /* println!("本次方法阶段共使用{:?}步", test); */
            if !self.fix_sole_po() {
                stage_counter += 1;
            } else {
                stage_counter = 0;
            }

            match stage_counter {
                0 => {
                    self.guess_sudoku_grid();
                }
                1 => {
                    self.guess_sudoku_phase2();
                }
                2 => {
                    self.guess_sudoku_phase3();
                }

                _ => {
                    return Err(GuessSudokuErr::UnableContinue);
                }
            }
        }
    }

    pub fn guess_sudoku_grid(&mut self) {
        for index in 0..81 {
            let x = self.cells[index].x.clone();
            let y = self.cells[index].y.clone();
            self.guess_grid(x as usize, y as usize);
        }
    }

    pub fn guess_grid(&mut self, x: usize, y: usize) {
        let index = (x - 1) * 9 + y - 1;

        if let Value::Num(_) = self.cells[index].value {
            return;
        }

        /* println!(
            "---{:?},{:?},{:?}---",
            self.cells[index].x, self.cells[index].y, self.cells[index].region
        ); */

        let grid_home = Self::find_grid_home(x, y);

        let mut guess_vec: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        //删除元素的闭包
        let mut del_guess_vec = |num: i8| {
            if guess_vec.contains(&num) {
                guess_vec.retain(|&x| x != num);
            }
        };

        //价值判断,然后调用上面删除元素的闭包
        let mut value_determine = |home: [usize; 9]| {
            for index_ in home.iter() {
                if let Value::Num(num) = self.cells[*index_].value {
                    del_guess_vec(num);
                }
            }
        };

        value_determine(grid_home.x_home);
        value_determine(grid_home.y_home);
        value_determine(grid_home.region_home);

        self.cells[index].po = guess_vec.clone();

        /* println!("{:?}", self.cells[index].po); */
    }

    fn guess_sudoku_phase2(&mut self) {
        /* println!("调用了guess_grid_phase2"); */
        for index in 0..81 {
            let x = self.cells[index].x.clone();
            let y = self.cells[index].y.clone();
            self.guess_grid_phase2(x as usize, y as usize);
        }
    }

    ///naked_single
    ///第二阶段是使用共同余数
    pub fn guess_grid_phase2(&mut self, x: usize, y: usize) {
        let index = (x - 1) * 9 + y - 1;

        if let Value::Num(_) = self.cells[index].value {
            return;
        }

        /* println!(
            "---{:?},{:?},{:?}---",
            self.cells[index].x, self.cells[index].y, self.cells[index].region
        ); */
        /* print!("---{:?}", self.cells[index].po); */

        let grid_home = Self::find_grid_home(x, y);

        let po_vec = self.cells[index].po.clone();
        let po_len = po_vec.len();

        let mut find_grid_same_po = |home: [usize; 9]| {
            let mut i = 1;
            let mut same_index: Vec<usize> = Vec::new();
            for index_ in home.iter() {
                if index_ == &index {
                    continue;
                }
                //寻找同区的其他猜数是否与自己相同
                if self.cells[*index_].po == po_vec {
                    same_index.push(*index_);
                    i += 1;
                }
            }
            //
            //todo!()
            //此处应该有个检测 i 是否大于po_len，然后调用数独错误，此数独为错误
            //
            //

            if i != po_len {
                return;
            }
            for index_ in home.iter() {
                if let Value::Num(_) = self.cells[*index_].value {
                    continue;
                }
                if index_ == &index || same_index.contains(index_) {
                    continue;
                }

                for vec_num in po_vec.iter() {
                    if self.cells[*index_].po.contains(vec_num) {
                        self.cells[*index_].po.retain(|&x| x != *vec_num);
                    }
                }
            }
        };
        find_grid_same_po(grid_home.x_home);
        find_grid_same_po(grid_home.y_home);
        find_grid_same_po(grid_home.region_home);
        /* println!("{:?}", po_vec); */
    }

    fn guess_sudoku_phase3(&mut self) {
        /* println!("调用了guess_grid_phase3"); */
        for index in 0..81 {
            let x = self.cells[index].x.clone();
            let y = self.cells[index].y.clone();
            self.guess_grid_phase3(x as usize, y as usize);
        }
    }
    pub fn guess_grid_phase3(&mut self, x: usize, y: usize) {
        let index = (x - 1) * 9 + y - 1;

        if let Value::Num(_) = self.cells[index].value {
            return;
        }

        let grid_home = Self::find_grid_home(x, y);

        let mut phase3_fn = |home: [usize; 9]| -> bool {
            let mut po_vec = self.cells[index].po.clone();
            for index_ in home.iter() {
                if let Value::Num(_) = self.cells[*index_].value {
                    continue;
                }
                if index_ == &index {
                    continue;
                }

                for vec_num in self.cells[*index_].po.iter() {
                    if po_vec.contains(vec_num) {
                        po_vec.retain(|&x| x != *vec_num);
                    }
                }
            }
            if po_vec.len() != 1 {
                return false;
            } else {
                self.cells[index].po = po_vec.clone();
                return true;
            }
        };
        if phase3_fn(grid_home.x_home) {
            return;
        }
        if phase3_fn(grid_home.y_home) {
            return;
        }
        if phase3_fn(grid_home.region_home) {
            return;
        }
    }
    fn fix_sole_po(&mut self) -> bool {
        let mut i = 0;
        for index in 0..81 {
            if self.cells[index].po.len() != 1 {
                continue;
            }
            self.cells[index].value = Value::Num(self.cells[index].po[0]);
            self.cells[index].po = vec![];
            i = i + 1;
        }
        if i == 0 { false } else { true }
    }

    fn find_grid_home(x: usize, y: usize) -> GridHome {
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

        GridHome {
            x_home,
            y_home,
            region_home,
        }
    }

    fn _find_grid_home(x: usize, y: usize) -> GridHome {
        let mut x_home = [0; 9];
        let mut y_home = [0; 9];
        let region = ((x - 1) / 3) * 3 + (y - 1) / 3 + 1;
        let mut region_home = [0; 9];

        for i in 0..9 {
            x_home[i] = 9 * (x - 1) + i;
            y_home[i] = (i / 3) * 9 + (y - 1) % 3;
            region_home[i] = 27 * ((region - 1) / 3) + 3 * ((i / 3) % 3) + (i % 3);
        }

        GridHome {
            x_home,
            y_home,
            region_home,
        }
    }
}
