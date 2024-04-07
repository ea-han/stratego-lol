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
        return true;
    }

    pub fn get_piece(&self,pos:&Position) -> &Option<Piece>{
        return &self.board[pos.row as usize][pos.col as usize];
    }

    pub fn set_piece(&mut self, pos: &Position, piece:Option<Piece>) {
        self.board[pos.row as usize][pos.col as usize] = piece;
    }

    fn try_move_piece(&mut self, start: &Position, end:&Position, owner: u16) -> bool {
        //if the starting piece is the owners
        if self.get_piece(start).clone().unwrap().owner != owner {
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
        if self.get_piece(end).is_some() {
            //self.do_battle();
        } else { //end is empty 
            self.set_piece(end, Some(self.get_piece(end).clone().unwrap().clone()));
            self.set_piece(start, None);
        }

        //conditions met, moving piece.
        self.advance_turn();
        return true;
    }


    fn advance_turn(&mut self){
        if self.turn == 1 {
            self.turn = 0;
        } else {
            self.turn = 1;
        }
    }
}

