#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new() -> Self {
        Position {
            line: 1,
            column: 0,
            offset: 0,
        }
    }
}
