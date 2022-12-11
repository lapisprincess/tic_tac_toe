use crate::board::Board;
use std::io;

pub fn play(mut b: Board) -> bool {
    // true: x  |  false: o
    let mut player = true;

    while b.won() == '_' {

        let letter = 
            if player { 'x' }
            else { 'o' };

        println!("Player ({letter})!");
        b.print_board();
        
        let mut coord = (-1, -1);
        let mut issue = true;
        while issue {
            for i in 0 .. 2 {
                if i == 0 {
                    println!("X: ");
                } else if i == 1 {
                    println!("Y: ");
                }
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line!");
                
                let trimmed = guess.trim();
                if trimmed == "exit" { return false; }
                else {
                    match trimmed.parse::<i32>() {
                        Ok(u) => {
                            if u <= b.get_size() as i32 { issue = false; }
                            else { 
                                println!("Out of bounds :("); 
                                issue = true; 
                                break; 
                            }

                            if i == 0 { coord.0 = u - 1; }
                            else { coord.1 = u - 1; }
                        },
                        Err(..) => println!("Not integer :("),
                    }
                }
            }
        }
        
        b = b.set_val(coord.0 as usize, coord.1 as usize, letter);
        player = !player;
        println!();
    }
    let winner = b.won();
    println!("{} is the winner!", winner);
    b.print_board();
    true
}
