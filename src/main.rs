mod board;
mod boardbuilder;
mod piece;
mod piecebuilder;
mod position;

use std::io;

use crate::boardbuilder::BoardBuilder;
use crate::position::Position;
use crate::board::Board;
fn main(){
    let mut board_builder: BoardBuilder = BoardBuilder::new();
    board_builder.place_1(Position::new(0,0),0);
    board_builder.place_2(Position::new(2,0), 1);
    let mut game: Board = board_builder.build_board();
    game.print_board();

    let mut done:bool = false;

    let mut input:String = String::new();
    let mut player:u16 = 0;

    while !done {
        
        println!("Enter two pairs of integers separated by spaces:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        // Split the input string by spaces
        let numbers: Vec<isize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok()) // Parse each substring into an integer
            .collect();
    
        // Ensure we have exactly 4 numbers (2 pairs)
        if numbers.len() != 4 {
            panic!("Invalid input: Expected two pairs of integers");
        }

        if game.try_move_piece(&Position::new(numbers[0],numbers[1]),&Position::new(numbers[2],numbers[3]),player) {
            game.advance_turn();
            game.print_board();
            player += 1;
            player %= 2;
            println!("{player}'s turn!");
        }
    }
}