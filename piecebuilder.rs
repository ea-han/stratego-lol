
struct PieceBuilder {
    piece:Piece,
}
impl PieceBuilder{
    fn new(&mut self){
        self.piece = Piece::new();
    }

    fn build_piece(&self) -> Piece{
        return self.piece.clone();
    }

    fn set_move_set(&mut self, move_set:Vec<Position>){
        self.piece.move_set = move_set;
    }
    
    fn set_icon_path(&mut self, icon_path:String){
        self.piece.icon_path = icon_path;
    }

    fn set_num(&mut self, num:u16){
        self.piece.num = num;
    }
}
