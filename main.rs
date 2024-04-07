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
    // let mut board_builder: BoardBuilder = BoardBuilder::new();
    // board_builder.place_1(Position::new(0,0),0);
    // board_builder.place_2(Position::new(9,0), 1);
    // let mut game: Board = board_builder.build_board();
    // game.print_board();

    // let mut done:bool = false;

    // let mut input:String = String::new();
    // let mut player:u16 = 0;

    // while !done {
    //     // println!("input starting square... ");
    
    //     // io::stdin()
    //     // .read_line(&mut input)
    //     // .expect("Failed to read line");

    //     // let mut words = input.split_whitespace();

    //     // // Parse the first integer
    //     // let first_num: isize = words
    //     //     .next()
    //     //     .expect("Expected two integers separated by space")
    //     //     .parse()
    //     //     .expect("Failed to parse first integer");
    
    //     // // Parse the second integer
    //     // let second_num: isize = words
    //     //     .next()
    //     //     .expect("Expected two integers separated by space")
    //     //     .parse()
    //     //     .expect("Failed to parse second integer");

    //     // println!("input ending square... ");

    //     // io::stdin()
    //     // .read_line(&mut input)
    //     // .expect("Failed to read line");

    //     // let mut words = input.split_whitespace();

    //     // // Parse the first integer
    //     // let third_num: isize = words
    //     //     .next()
    //     //     .expect("Expected two integers separated by space")
    //     //     .parse()
    //     //     .expect("Failed to parse first integer");
    
    //     // // Parse the second integer
    //     // let fourth_num: isize = words
    //     //     .next()
    //     //     .expect("Expected two integers separated by space")
    //     //     .parse()
    //     //     .expect("Failed to parse second integer");
    
    //     let bof = game.try_move_piece(&Position::new(0,0),&Position::new(0,1),player);
        
    //     if bof {
    //         println!("Valid");
    //         player += 1;
    //         player %= 2;
    //         game.print_board();
    //     }
    //}
}