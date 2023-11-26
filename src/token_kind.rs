use strum_macros::Display;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum TokenKind {
    Equal,
    Dot,
    Star,
    Semicolon,
    Mod,
    Minus,

    Paren,
    LeftParen,
    RightParen,
    Square,
    LeftSquare,
    RightSquare,

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
