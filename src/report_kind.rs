use strum_macros::Display;

#[derive(Display)]
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

    // literal errors
    UnterminatedStringError
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
            UnterminatedStringError => "string was never closed"
        }
    }
}

