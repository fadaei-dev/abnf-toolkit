use strum_macros::Display;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum TokenKind {
    Equal,
    Dot,
    Star,
    Mod,
    Range,
    Slash,

    Paren,
    LeftParen,
    RightParen,
    Square,
    LeftSquare,
    RightSquare,
    Angle,
    LeftAngle,
    RightAngle,

    // two character tokens
    EqualSlash,

    // Mods
    TerminalBinary,
    TerminalDecimal,
    Decimal,
    TerminalHexadecimal,

    // Literal TokenTypes
    String,
    Number,
    Identifier,

    // Keywords *Appendix B*
    // todo
    Whitespace,
    EOF,
}
