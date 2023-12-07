use crate::position::Position;
use crate::token_kind::TokenKind;

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'s> {
    pub kind: TokenKind,
    pub pos: Position,
    pub length: usize,
    pub src: &'s str,
}

impl<'s> Token<'s> {
    pub fn get_lexeme(&self) -> &'s str {
        &self.src[self.pos.offset..self.pos.offset + self.length]
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token at line {} char {}: {} -- {:?}",
            self.pos.line,
            self.pos.column,
            self.get_lexeme(),
            self.kind
        )
    }
}
