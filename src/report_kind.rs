use strum_macros::Display;

#[derive(Display, Debug, PartialEq, Clone)]
pub enum ReportKind {
    UnableToParseError,
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
    DecimalTerminalError,

    // literal errors
    UnterminatedStringError,
    NaNError,
}

impl ReportKind {
    pub fn msg(&self) -> &'static str {
        use ReportKind::*;

        match &self {
            UnableToParseError => "lexer was unable to tokenize file",
            InternalLexerError => "you should never see this error",
            EofError => "reached end of file before complete expression",
            MismatchedClosingBracketError => "one or more brackets are never closed",
            UnexpectedClosingBracketError => "unexpected closing bracket",
            UnclosedBracketError => "one or more brackets are never closed",
            NoTerminalFoundError => "expected terminal (b, d, x) after %, found none",
            IncorrectTerminalFoundError => "expected terminal (b, d, x) after %",
            BinaryTerminalError => "expected binary representation (0, 1)",
            DecimalTerminalError => "decimal terminal should be in range 0..=127",
            UnterminatedStringError => "string was never closed",
            NaNError => "expected a number",
        }
    }
}
