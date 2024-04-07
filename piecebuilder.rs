use crate::piece::Piece;
use crate::position::Position;

pub struct PieceBuilder {
    piece:Piece,
}
impl PieceBuilder{
    pub fn new() -> PieceBuilder{
        return PieceBuilder { piece: Piece::new()};
    }

    pub fn build_piece(&self) -> Piece{
        return self.piece.clone();
    }

    pub fn set_move_set(&mut self, move_set:Vec<Position>){
        self.piece.move_set = move_set;
    }
    
    pub fn set_icon_path(&mut self, icon_path:String){
        self.piece.icon_path = icon_path;
    }

    pub fn set_num(&mut self, num:u16){
        self.piece.num = num;
    }
}
