use crate::board::Board;
use crate::piece::Piece;
use crate::position::Position;
use crate::piecebuilder::PieceBuilder;

const ARRAY_REPEAT_VALUE: Option<Piece> = None;
pub struct BoardBuilder {
    board: Board,
}
impl BoardBuilder {
    pub fn new() -> BoardBuilder{
        let board_array: [[Option<Piece>; 10]; 10] = Default::default();
        return BoardBuilder { board: Board::new(board_array, 0)};
    }
    
    pub fn place_marshall(&mut self, pos:Position) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_none() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(10);
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn build_board(self) -> Board{
        self.board
    }

}