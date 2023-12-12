use crate::position::Position;
use crate::report_kind::ReportKind;

#[derive(Debug)]
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

    pub fn get_kind(&self) -> ReportKind {
        self.kind.clone()
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.pos {
            Some(pos) => {
                let col = pos.column;

                write!(
                    f,
                    "ERROR -- on Line {} at char {}: {} -- {}\n\
                    {}\n\
                    {:>col$}",
                    pos.line, pos.column, self.kind, self.msg, self.line, "^", 
                )
            }
            None => write!(f, "ERROR -- {} -- {}\n", self.kind, self.msg),
        }
    }
}
