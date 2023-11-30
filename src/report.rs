use crate::position::Position;
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

pub struct Report {
    kind: ReportKind,
    msg: &'static str,

    pos: Option<Position>,
    line: String,
}

impl Report {
    pub fn new(kind: ReportKind, pos: Option<Position>, line: String) -> Self {
        let msg = kind.msg();

        Report {
            kind,
            msg,
            pos,
            line,
        }
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.pos {
            Some(pos) => {
                let col = pos.column;

                write!(
                    f,
                    "ERROR -- on Line {} at char {}: \n\
                    {}\n\
                    {:>col$}\n\
                    {} -- {}\n",
                    pos.line, pos.column, self.line, "^", self.kind, self.msg
                )
            }
            None => write!(f, "ERROR -- {} -- {}\n", self.kind, self.msg),
        }
    }
}
