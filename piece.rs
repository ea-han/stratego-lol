use crate::position::Position;

//Piece struct for game pieces

pub struct Piece { // must have move_set only in 2 directions or else will break stuff. 
    pub num: u16,
    pub icon_path: String,
    pub move_set: Vec<Position>,
    pub owner: u16,
}
impl Piece {
    pub fn new() -> Piece{
        Piece { num: 0, icon_path: String::from("None"), move_set: vec![], owner: 0}
    }

    pub fn get_moves(&self, pos: &Position) -> Vec<Position> {
        let mut relational_move_set:Vec<Position> = self.move_set.clone();
        let relational_move_set_iter = relational_move_set.iter_mut();
        for move_pos in relational_move_set_iter {
            move_pos.row += pos.row;
            move_pos.col += pos.col;
        }
        return relational_move_set;
    }
}

impl Clone for Piece{
    fn clone(&self) -> Self {
        return Piece { num: (self.num), icon_path: (self.icon_path.clone()), move_set: (self.move_set.clone()), owner: self.owner}
    }   
}