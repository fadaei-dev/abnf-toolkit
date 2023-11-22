use crate::position::Position;

#[derive(Clone)]
pub enum TokenLiteral {
    StringLiteral(String),
    NumberLiteral(i32),
    IdentifierLiteral(TokenType),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    Equal,
    Dot,
    Star,
    Semicolon,
    Mod,
    Minus,

    // two character tokens
    EqualSlash,

    // Mods
    TerminalBinary,
    TerminalDecimal,
    TerminalHexadecimal,

    // Literal TokenTypes
    String,
    Number,
    Identifier,

    // Keywords *Appendix B*
    // todo
    EOF,
}

#[derive(Clone)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<TokenLiteral>,
    pos: Position,
}

impl Token {
    pub fn new(
        kind: TokenType,
        lexeme: String,
        literal: Option<TokenLiteral>,
        pos: Position,
    ) -> Self {
        Token {
            kind,
            lexeme,
            literal,
            pos,
        }
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn get_kind(&self) -> TokenType {
        self.kind.clone()
    }

    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }

    pub fn get_literal(&self) -> Option<TokenLiteral> {
        self.literal.clone()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token at line {} char {}: {} -- {:?}",
            self.pos.line, self.pos.char, self.lexeme, self.kind
        )
    }
}
