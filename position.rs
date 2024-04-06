//Position struct for recording position. 
struct Position{
    row: usize,
    col: usize,
}
impl Position {
    fn new(row:usize, col:usize) -> Position{
        Position{row, col}
    }
}
impl Clone for Position{
    fn clone(&self) -> Self {
        return Position::new(self.row, self.col);
    }
}


