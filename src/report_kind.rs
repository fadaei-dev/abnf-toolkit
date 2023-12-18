use strum_macros::Display;

#[derive(Display, Debug, PartialEq, Clone)]
pub enum ReportKind {
    UnableToParseError,
    UnableToAdvanceError,
    InternalLexerError,
    EofError,

    // bracket errors
    UnclosedBracketError,
    MismatchedClosingBracketError,
    UnexpectedClosingBracketError,

    // terminal errors
    NoTerminalFoundError,
    IncorrectTerminalFoundError,
    BinaryTerminalError,
    SevenBitsError,
    DecimalTerminalError,
    HexadecimalTerminalError,
    NaHexNError,

    // literal errors
    UnterminatedStringError,
    NaNError,
}

impl ReportKind {
    pub fn msg(&self) -> &'static str {
        use ReportKind::*;

        match &self {
            UnableToParseError => "lexer was unable to tokenize file",
            UnableToAdvanceError => "Unable To advance tokenstream",
            InternalLexerError => "you should never see this error",
            EofError => "reached end of file before complete expression",
            MismatchedClosingBracketError => "one or more brackets are never closed",
            UnexpectedClosingBracketError => "unexpected closing bracket",
            UnclosedBracketError => "one or more brackets are never closed",
            NoTerminalFoundError => "expected terminal (b, d, x) after %, found none",
            IncorrectTerminalFoundError => "expected terminal (b, d, x) after %",
            BinaryTerminalError => "expected binary representation (0, 1)",
            SevenBitsError => "expected 7 bits after terminal received more or less",
            DecimalTerminalError => "decimal terminal should be in range 0..=127",
            HexadecimalTerminalError => "hexadecimal terminal should be in range 00..=7E",
            UnterminatedStringError => "string was never closed",
            NaNError => "expected a number",
            NaHexNError => "expected a 2 character hexadecimal",
        }
    }
}
