use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use ansi_escapes::{CursorUp, EraseLine};
use connect4::board::{Board, State};

fn input() -> String {
    stdout().flush().unwrap();
    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    String::from(out.trim())
}

fn main() {
    let mut board = Board::new();
    let mut current = State::X;
    let mut move_up_num = 0;
    loop {
        println!("{}", CursorUp(move_up_num));
        println!("{board}");
        print!("{}", EraseLine);
        if let Some(winner) = board.wincheck() {
            println!("{} wins!", winner);
            exit(0);
        }
        print!("Your move: ");
        let mov = input();
        match mov.parse::<usize>() {
            Ok(n) => match board.place_obj(current, n.clamp(1, 8) - 1) {
                Ok(()) => (),
                Err(e) => {
                    println!("{e}");
                    move_up_num += 1;
                    continue;
                }
            },
            Err(_) => {
                println!("Enter a valid number.");
                continue;
            }
        }

        match current {
            State::O => current = State::X,
            State::X => current = State::O,
            _ => unreachable!(),
        }
        move_up_num = 10;
    }
}
