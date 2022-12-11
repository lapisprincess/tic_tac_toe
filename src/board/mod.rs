mod win_check;
mod cell;

use crate::board::cell::Cell;

pub struct Board {
    cells: Vec<Vec<Cell>>,
    size: usize,
}

impl Board {
    fn get_val(&self, x: usize, y: usize) -> Option<char> {
        if (x >= self.size) || (y >= self.size) {
            None
        } else {
            let cell = &self.cells[x][y];
            match cell {
                Cell::X => Some('x'),
                Cell::O => Some('o'),
                Cell::Empty => Some('_'),
            }
        }
    }

    pub fn print_board(&self) {
        for y in 0 .. self.size {
            for x in 0 .. self.size {
                let val = self.get_val(x, y);
                match val {
                    Some(c) => print!("|{c}|"),
                    None => print!("error!"),
                }
            }
            println!();
        }
    }

    pub fn get_size(&self) -> i32 {
        self.size as i32
    }

    pub fn set_val(mut self, x: usize, y: usize, val: char) -> Self {
        let cell = match val {
            'x' => Cell::X,
            'o' => Cell::O,
            _ => Cell::Empty,
        };

        if (x >= self.size) || (y >= self.size) {
            println!("can't change that value!");
        } else {
            if self.cells[x][y] == Cell::Empty {
                self.cells[x][y] = cell;
            } else {
                println!("can't change that value!");
            }
        }
        self
    }

    pub fn won(&self) -> char {
        win_check::check(self)
    }
}

pub fn make_board(size: usize) -> Board {
    let mut board = Board {
        cells: Vec::new(),
        size: size,
    };
    for _x in 0 .. size {
        let mut row = Vec::new();
        for _y in 0 .. size {
            row.push(Cell::Empty);
        }
        board.cells.push(row);
    }
    board
}
