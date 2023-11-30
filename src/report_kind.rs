use strum_macros::Display;

#[derive(Display)]
pub enum ReportKind {
    UnableToParseError,
    UnclosedBracketError,
    InternalLexerError,
    MismatchedClosingBracketError,
    UnexpectedClosingBracketError,
}

impl ReportKind {
    pub fn msg(&self) -> &'static str {
        use ReportKind::*;

        match &self {
            UnableToParseError => "lexer was unable to tokenize file",
            InternalLexerError => "you should never see this error",
            MismatchedClosingBracketError => "one or more brackets are never closed",
            UnexpectedClosingBracketError => "unexpected closing bracket",
            UnclosedBracketError => "one or more brackets are never closed",
        }
    }
}