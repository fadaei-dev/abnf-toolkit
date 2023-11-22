use crate::position::Position;

pub enum ReportErrorKind {
    UnableToParseError,
    UnkownTokenError,
}

impl std::fmt::Display for ReportErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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
                    pos.line, pos.char, self.kind, self.msg
                )
            }
            None => write!(f, "{} -- {}", self.kind, self.msg),
        }
    }
}
