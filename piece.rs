//Piece struct for game pieces
struct Pieces {
    num: u16,
    icon_path: String,
    move_set: Vec<Position>,
    owner: u16,
}
impl Piece {
    fn new() -> Piece{
        Piece { num: 0, icon_path: String::from("None"), move_set: vec![], owner: 0}
    }

    fn get_moves(&self, pos: Position) -> Vec<Position> {
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