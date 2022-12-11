use crate::board::Board;

enum Direction {
    West,
    South,
    SouthWest,
}

pub fn check(board: &Board) -> char {
    let total = board.size * 2 + 1;
    for i in 0 .. total {
        let mut x = 0;
        let mut y = 0;

        let mut dir = Direction::SouthWest;
        if i < board.size {
            dir = Direction::South;
            x += i % board.size;
        } else if i < total - 1 {
            dir = Direction::West;
            y += i % board.size;
        }

        let mut c = '_';
        let won = match board.get_val(x, y) {
            Some('x') => { c = 'x'; 
                check_helper(board, (x, y), 'x', dir)},
            Some('o') => { c = 'o';
                check_helper(board, (x, y), 'o', dir)},
            _ => false,
        };
        if won {
            return c
        }
    }
    '_'
}

fn check_helper(board: &Board, loc: (usize, usize), c: char, dir: Direction ) -> bool {
    let mut x = loc.0;
    let mut y = loc.1;
    match dir {
        Direction::South => y+=1,
        Direction::West => x+=1,
        Direction::SouthWest => { y+=1; x+=1 },
    };
    if x >= board.size || y >= board.size {
        return true;
    }
    match board.get_val(x, y) {
        Some(s) => (check_helper(board, (x, y), c, dir)) && (s == c),
        _ => false,
    }
}
