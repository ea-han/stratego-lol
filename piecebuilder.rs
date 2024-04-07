use crate::piece::Piece;
use crate::position::Position;

pub struct PieceBuilder {
    pub num: u16,
    pub icon_path: String,
    pub move_set: Vec<Position>,
    pub owner: u16,
}
impl PieceBuilder{
    pub fn new() -> PieceBuilder{
        PieceBuilder { num: (0), icon_path: (String::from("None")), move_set: (vec![]), owner: (0) }
    }

    pub fn build_piece(&self) -> Piece{
        Piece { num: (self.num), icon_path: (self.icon_path.clone()), move_set: (self.move_set.clone()), owner: (self.owner) }
    }

    pub fn set_move_set(&mut self, move_set:Vec<Position>){
        self.move_set = move_set;
    }
    
    pub fn set_icon_path(&mut self, icon_path:String){
        self.icon_path = icon_path;
    }

    pub fn set_num(&mut self, num:u16){
        self.num = num;
    }

    pub fn set_owner(&mut self, owner: u16){
        self.owner = owner;
    }
}
