
//Board: has pieces, manages game state. Is initialized separately.
struct Board {
    board: [[Piece; 10]; 10],
    turn: u16,
}

impl Board {
    fn new(board: [[Piece; 10]; 10], turn: u16) -> Board{
        Board{board, turn}
    }

    fn get_valid_moves(&self, pos:Position) -> Vec<Position>{
        let piece = self.get_piece(pos);
        let mut valid_moves: Vec<Position> = piece.get_moves(pos);
        let valid_moves_iter = valid_moves.iter_mut();
        for move_pos in valid_moves_iter{
        //    if self.get_piece(move_pos) 
        }
    }

    fn get_piece(&self,pos:Position) -> &Piece{
        return &self.board[pos.row][pos.col];
    }

    fn try_move_piece(&mut self, start: Position, end:Position, owner: u16) -> bool {
        
        if self.get_piece(start).owner != owner {
            return false;
        }
        //if self.is_p1_turn


        
        self.set_turn();
        return true;
    }

    fn set_turn(&mut self, turn:u16){
        self.is_p0_turn = turn;
    }
}

