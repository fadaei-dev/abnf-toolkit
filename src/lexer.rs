use crate::position::Position;
use crate::report::{Report, ReportKind};
use crate::token::Token;
use crate::token_kind::TokenKind;

pub struct Lexer<'s> {
    src: &'s str,
    chars: std::str::Chars<'s>,
    next: Option<char>,

    tokens: Vec<Token<'s>>,
    token_start: Position,
    token_end: Position,

    current_line: &'s str,

    open_brackets: Vec<TokenKind>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        let mut chars = source.chars();
        let next = chars.next();

        let index = if let Some(index) = source.find('\n') {
            index
        } else {
            source.len()
        };

        Lexer {
            src: source,
            token_start: Position::new(),
            token_end: Position::new(),
            tokens: Vec::new(),
            open_brackets: Vec::new(),
            current_line: &source[..index],
            chars,
            next,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'s>>, Vec<Report>> {
        let mut reports: Vec<Report> = Vec::new();

        while !self.is_at_end() {
            let start = match self.next {
                Some(start) => start,
                None => break,
            };
            // append errors to error vector, ignore Ok value
            if let Err(report) = self.lex(start) {
                reports.push(report);
            }
        }

        self.add_token(TokenKind::EOF);

        if !self.open_brackets.is_empty() {
            reports.push(Report::new(
                ReportKind::MismatchedClosingBracketError,
                None,
                self.current_line.into(),
            ))
        }

        // if we have errors, keep parsing and return all errors
        if !reports.is_empty() {
            return Err(reports);
        }

        Ok(self.tokens.clone())
    }

    fn lex(&mut self, start: char) -> Result<(), Report> {
        match start {
            '(' => self.lex_bracket(TokenKind::LeftParen)?,
            ')' => self.lex_bracket(TokenKind::RightParen)?,
            '[' => self.lex_bracket(TokenKind::LeftSquare)?,
            ']' => self.lex_bracket(TokenKind::RightSquare)?,
            '.' => self.lex_single(TokenKind::Dot)?,
            '-' => self.lex_single(TokenKind::Minus)?,
            '*' => self.lex_single(TokenKind::Star)?,

            // ';' => {
            //     while let Some(peeked) = self.next {
            //         if peeked != '\n' && !self.is_at_end() {
            //             self.advance();
            //         } else {
            //             break;
            //         }
            //     }
            // }

            // '=' => {
            //     let token = if self.if_match_advance('/') {
            //         TokenKind::EqualSlash
            //     } else {
            //         TokenKind::Equal
            //     };
            //
            //     self.add_token(token)
            // }

            // special characters
            // ' ' | '\r' | '\t' => (),
            // '\n' => {
            //     self.token_end.line += 1;
            //     self.token_end.column = 0
            // }
            _ => {
                self.advance()?;
                return Err(Report::new(
                    ReportKind::UnableToParseError,
                    None,
                    self.current_line.into(),
                ));
            }
        };

        Ok(())
    }

    fn lex_single(&mut self, kind: TokenKind) -> Result<(), Report> {
        self.advance()?;
        self.add_token(kind);
        Ok(())
    }

    fn lex_bracket(&mut self, kind: TokenKind) -> Result<(), Report> {
        match kind {
            TokenKind::LeftParen => self.open_bracket(TokenKind::Paren),
            TokenKind::RightParen => self.close_bracket(TokenKind::Paren)?,
            TokenKind::LeftSquare => self.open_bracket(TokenKind::Square),
            TokenKind::RightSquare => self.close_bracket(TokenKind::Square)?,
            _ => {
                return Err(Report::new(
                    ReportKind::InternalLexerError,
                    Some(self.token_end.clone()),
                    self.current_line.into(),
                ))
            }
        }

        self.lex_single(kind)
    }

    fn open_bracket(&mut self, kind: TokenKind) {
        self.open_brackets.push(kind)
    }

    fn close_bracket(&mut self, kind: TokenKind) -> Result<(), Report> {
        match self.open_brackets.pop() {
            Some(open) if open == kind => Ok(()),
            Some(_) => {
                let before = self.token_end.clone();
                self.lex_single(kind)?;
                return Err(Report::new(
                    ReportKind::MismatchedClosingBracketError,
                    Some(before),
                    self.current_line.into(),
                ));
            }
            None => {
                let before = self.token_end.clone();
                self.lex_single(kind)?;
                Err(Report::new(
                    ReportKind::UnexpectedClosingBracketError,
                    Some(before),
                    self.current_line.into(),
                ))
            }
        }
    }

    fn advance(&mut self) -> Result<(), Report> {
        match self.next {
            Some(c) => {
                let len_utf8 = c.len_utf8();

                self.token_end.offset += len_utf8;
                self.token_end.column += len_utf8; // test to see if this should be 1

                self.next = self.chars.next();

                Ok(())
            }
            None => Err(Report::new(
                ReportKind::UnableToParseError,
                None,
                self.current_line.into(),
            )),
        }
    }

    fn next_is(&self, c: char) -> bool {
        self.next == Some(c)
    }

    fn rest(&self) -> &'s str {
        &self.src[self.token_end.offset..]
    }

    fn peek_rest(&self, prefix: &str) -> bool {
        self.rest().starts_with(prefix)
    }

    fn is_at_end(&self) -> bool {
        self.rest().is_empty()
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            pos: self.token_start.clone(),
            src: self.src,
            length: self.token_end.offset - self.token_start.offset,
            kind,
        });

        self.token_start = self.token_end.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let source = "[]]";

        let mut lexer = Lexer::new(source);

        match lexer.tokenize() {
            Ok(tokens) => {
                assert_eq!(tokens[0].kind, TokenKind::Star);
                assert_eq!(tokens[0].pos.line, 1);

                for t in tokens {
                    println!("{t}");
                }
            }
            Err(reports) => {
                for e in reports {
                    println!("{e}")
                }
            }
        }
    }

    // #[test]
    // fn parse_tokens_with_comments() {
    //     let source: String = "= =/ ;;IGNORE\n ;IGNORE / [] \n *".into();
    //
    //     let mut lexer = Lexer::new(source);
    //
    //     match lexer.scan_tokens() {
    //         Ok(tokens) => {
    //             for t in tokens {
    //                 println!("{t}")
    //             }
    //         }
    //         Err(reports) => {
    //             for report in reports {
    //                 println!("{report}")
    //             }
    //         }
    //     }
    // }
}
