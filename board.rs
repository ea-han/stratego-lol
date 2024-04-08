use crate::piece::Piece;
use crate::position::Position;

//Board: has pieces, manages game state. Is initialized separately. Each board cell has None, or a piece.
pub struct Board {
    board: [[Option<Piece>; 10]; 10],
    turn: u16,
}

impl Board {

    pub fn new(board: [[Option<Piece>; 10]; 10], turn: u16) -> Board{
        Board{board, turn}
    }

    //assume position is valid piece, return valid move spots in board
    fn get_valid_moves(&self, pos:&Position) -> Vec<Position>{
        let piece = self.get_piece(pos);
        let mut valid_moves: Vec<Position> = piece.clone().unwrap().get_moves(pos);

        valid_moves.retain(|move_pos| //first checks if positions are negative, then checks if within board constraints, then checks if line of sight.
                (move_pos.row >= 0 && move_pos.col >= 0) || 
                    (move_pos.row < self.board.len() as isize && 
                        move_pos.col < self.board[0].len() as isize && 
                        self.has_los(pos,move_pos))
            );
        
        return valid_moves;
    }


    //assume start and end lie on same axis, and are valid indicies. returns if start has clear LOS up to but not including end
    fn has_los(&self, start:&Position, end:&Position) -> bool {
        let mut start_ptr = start.clone();
        //if rows are same
        if start.row == end.row{
            while start_ptr.row != end.row {
                if start_ptr.row < end.row {
                    start_ptr.row+=1;
                } else {
                    start_ptr.row-=1;
                }
                if self.get_piece(&start_ptr).is_none(){
                    return false;
                }
            } 
        } else { //if cols are same
            while start_ptr.col != end.col {
                if start_ptr.col < end.col {
                    start_ptr.col+=1;
                } else {
                    start_ptr.col-=1;
                }
                if self.get_piece(&start_ptr).is_none(){
                    return false;
                }
            } 
        }
        println!("has LOS.");
        return true;
    }

    pub fn get_piece(&self,pos:&Position) -> &Option<Piece>{
        return &self.board[pos.row as usize][pos.col as usize];
    }

    pub fn set_piece(&mut self, pos: &Position, piece:Option<Piece>) {
        self.board[pos.row as usize][pos.col as usize] = piece;
    }
    
    pub fn try_move_piece(&mut self, start: &Position, end:&Position, owner: u16) -> bool {
        //if the piece is non-none
        let start_piece = self.get_piece(start);
        let end_piece =  self.get_piece(end);
        if start_piece.is_none(){
            return false;
        }
        
        //if the starting piece is the owners
        if start_piece.clone().unwrap().owner != owner {
            return false;
        }
        if self.turn != owner {
            return false;
        }

        //check if end position is in valid moves
        let valid_moves: Vec<Position> = self.get_valid_moves(start);
        let valid_moves_iter = valid_moves.iter();
        let mut in_moves:bool = false;
        for m in valid_moves_iter {
            if end.row == m.row && end.col == m.col {
                in_moves = true;
                break;
            }
        }
        if !in_moves {
            return false;
        }
        //resolve collision/battle
        if end_piece.is_some() {
            self.do_battle(&start, &end);
        } else { //end is empty 
            self.set_piece(end, Some(start_piece.clone().unwrap().clone()));
            self.set_piece(start, None);
        }
        return true;
    }


    pub fn advance_turn(&mut self){
        self.turn += 1;
        self.turn %= 2;
    }

    fn do_battle(&mut self, attacker: &Position, defender: &Position){
        let attack_piece: &Piece = self.get_piece(attacker).as_ref().unwrap();
        let defend_piece: &Piece =self.get_piece(defender).as_ref().unwrap();
        if defend_piece.num == 0 {
            self.end_game();
        } else if attack_piece.num == 8 && defend_piece.num == 11 {
            self.set_piece(defender, Some(attack_piece.clone()));
            self.set_piece(attacker, None);
        } else if attack_piece.num == 1 && defend_piece.num == 10 {
            self.set_piece(defender, Some(attack_piece.clone()));
            self.set_piece(attacker, None);
        } else {
            if attack_piece.num > defend_piece.num {
                self.set_piece(defender, Some(attack_piece.clone()));
                self.set_piece(attacker, None);
            } else {
                self.set_piece(attacker, None);
            }
        }
    }

    fn end_game(&self){
        let winner = self.turn + 1;
        println!("{winner} Wins!")
    }

    pub fn get_board_for(&self,player: u16) -> [[Option<Piece>;10];10]{
        let mut new_board: [[Option<Piece>;10];10] = Default::default();
        let hidden_piece: Piece = Piece::new(0, String::from(" * "), vec![], (player + 1)%2);
        
        for i in 0..self.board.len(){
            for j in 0..self.board.len(){
                if self.board[i][j].is_some(){
                    if self.board[i][j].as_ref().unwrap().get_owner() == player {
                        new_board[i][j] = self.board[i][j].clone();
                    } else {
                        new_board[i][j] = Some(hidden_piece.clone());
                    }
                } else {
                    new_board[i][j] = None;
                }
            }
        }

        return new_board;
    }

    pub fn print_board(&self){
        let show_board = self.get_board_for(self.turn);
        println!("   0  1  2  3  4  5  6  7  8  9 ");
        for i in 0..show_board.len(){
            print!("{i} ");
            for j in 0..show_board[i].len(){
                if show_board[i][j].is_none(){
                    print!(" . ");
                } else {
                    let p = &show_board[i][j].as_ref().unwrap().icon_path;
                    print!("{p}");
                }
            }
            println!();
        }
    }
}

