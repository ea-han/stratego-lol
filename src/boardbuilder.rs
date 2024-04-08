use crate::board::Board;
use crate::piece::Piece;
use crate::position::Position;
use crate::piecebuilder::PieceBuilder;

pub struct BoardBuilder {
    board: Board,
}
impl BoardBuilder {
    pub fn new() -> BoardBuilder{
        let board_array: [[Option<Piece>; 10]; 10] = Default::default();
        return BoardBuilder { board: Board::new(board_array, 0)};
    }
    
    pub fn place_1(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
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
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 1 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_2(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(9);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 2 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_3(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(8);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 3 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_4(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(7);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 4 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_5(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(6);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 5 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_6(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(5);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 6 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_7(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(4);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 7 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_8(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(3);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 8 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_9(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1),
            Position::new(2,0),
            Position::new(-2,0),
            Position::new(0,2),
            Position::new(0,-2),
            Position::new(3,0),
            Position::new(-3,0),
            Position::new(0,3),
            Position::new(0,-3),
            Position::new(4,0),
            Position::new(-4,0),
            Position::new(0,4),
            Position::new(0,-4),
            Position::new(5,0),
            Position::new(-5,0),
            Position::new(0,5),
            Position::new(0,-5),
            Position::new(6,0),
            Position::new(-6,0),
            Position::new(0,6),
            Position::new(0,-6),
            Position::new(7,0),
            Position::new(-7,0),
            Position::new(0,7),
            Position::new(0,-7),
            Position::new(8,0),
            Position::new(-8,0),
            Position::new(0,8),
            Position::new(0,-8),
            Position::new(9,0),
            Position::new(-9,0),
            Position::new(0,9),
            Position::new(0,-9)
        ]);
        p_builder.set_num(2);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" 9 "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_spy(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![
            Position::new(1,0),
            Position::new(-1,0),
            Position::new(0,1),
            Position::new(0,-1)
        ]);
        p_builder.set_num(1);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" S "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_flag(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![]);
        p_builder.set_num(0);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" F "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn place_bomb(&mut self, pos:Position, owner: u16) -> bool{
        //check if empty position
        if self.board.get_piece(&pos).is_some() {
            return false;
        }

        let mut p_builder: PieceBuilder = PieceBuilder::new();
        p_builder.set_move_set(vec![]);
        p_builder.set_num(11);
        p_builder.set_owner(owner);
        p_builder.set_icon_path(String::from(" B "));
        let piece: Piece = p_builder.build_piece();
        self.board.set_piece(&pos, Some(piece));
        return true;
    }

    pub fn build_board(self) -> Board{
        self.board
    }

}