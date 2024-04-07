//Position struct for recording position. 
pub struct Position{
    pub row: isize,
    pub col: isize,
}
impl Position {
    pub fn new(row:isize, col:isize) -> Position{
        Position{row, col}
    }
}
impl Clone for Position{
    fn clone(&self) -> Self {
        return Position::new(self.row, self.col);
    }
}


