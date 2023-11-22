#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub char: usize,
}

impl Position {
    pub fn new() -> Self {
        Position { line: 1, char: 0 }
    }
}
