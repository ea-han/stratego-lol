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
    board_builder.place_1(Position::new(3,3),0);
    board_builder.place_2(Position::new(2,1), 0);
    board_builder.place_3(Position::new(2,2),0);
    board_builder.place_4(Position::new(0,0), 0);
    board_builder.place_bomb(Position::new(1,1), 0);
    board_builder.place_flag(Position::new(0,2), 0);

    board_builder.place_1(Position::new(6,0),1);
    board_builder.place_2(Position::new(6,1), 1);
    board_builder.place_3(Position::new(6,2),1);
    board_builder.place_4(Position::new(6,3), 1);
    board_builder.place_bomb(Position::new(7,1), 1);
    board_builder.place_flag(Position::new(7,8), 1);

    let mut game: Board = board_builder.build_board();
    
    let mut done:bool = false;

    let mut input:String = String::new();
    let mut player:u16 = 0;

    while !done {
        game.print_board();
        
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
            player += 1;
            player %= 2;
            print!("{}[2J", 27 as char);
            let p:  &mut String = &mut String::from("");
            println!("{player}'s turn! Press a key to continue...");
            io::stdin().read_line(p);
        }
        
    }
}