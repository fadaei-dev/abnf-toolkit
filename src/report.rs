use crate::position::Position;
use strum_macros::Display;

#[derive(Display)]
pub enum ReportErrorKind {
    UnableToParseError,
    UnkownTokenError,
    InternalLexerError,
    MismatchedClosingHousingError,
    UnexpectedClosingHousingError,
}

pub struct Report {
    kind: ReportErrorKind,
    msg: String,

    pos: Option<Position>,
}

impl Report {
    pub fn new(kind: ReportErrorKind, msg: String, pos: Option<Position>) -> Self {
        Report { kind, msg, pos }
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.pos {
            Some(pos) => {
                write!(
                    f,
                    "On Line {} at char {}: {} -- {}",
                    pos.line, pos.column, self.kind, self.msg
                )
            }
            None => write!(f, "{} -- {}", self.kind, self.msg),
        }
    }
}
