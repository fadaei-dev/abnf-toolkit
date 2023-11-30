use crate::position::Position;
use crate::report::Report;
use crate::report_kind::ReportKind;
use crate::token::Token;
use crate::token_kind::TokenKind;

type LexResult<T> = Result<T, Report>;

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

        // grab current line, if no \n grab whole file
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

        // check for unclosed brackets
        if !self.open_brackets.is_empty() {
            reports.push(Report::new(
                ReportKind::UnclosedBracketError,
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

    fn lex(&mut self, start: char) -> LexResult<()> {
        match start {
            '(' => self.lex_bracket(TokenKind::LeftParen)?,
            ')' => self.lex_bracket(TokenKind::RightParen)?,
            '[' => self.lex_bracket(TokenKind::LeftSquare)?,
            ']' => self.lex_bracket(TokenKind::RightSquare)?,

            '.' => self.lex_single(TokenKind::Dot)?,
            '-' => self.lex_single(TokenKind::Range)?,
            '*' => self.lex_single(TokenKind::Star)?,
            '/' => self.lex_single(TokenKind::Slash)?,

            '=' => self.lex_assignment()?,
            ';' => self.lex_comment()?,
            '%' => self.lex_terminal()?,

            ' ' | '\t' => self.lex_whitespace()?,
            '\n' | '\r' => self.lex_eol()?,

            _ => {
                self.advance()?; // continue lexing
                return Err(Report::new(
                    ReportKind::UnableToParseError,
                    None,
                    self.current_line.into(),
                ));
            }
        };

        Ok(())
    }

    fn lex_single(&mut self, kind: TokenKind) -> LexResult<()> {
        self.advance()?;
        self.add_token(kind);
        Ok(())
    }

    fn lex_terminal(&mut self) -> LexResult<()> {
        self.lex_single(TokenKind::Mod)?;

        let terminal = match self.next {
            None => {
                return Err(Report::new(
                    ReportKind::EofError,
                    Some(self.token_end.clone()),
                    self.current_line.into(),
                ))
            }
            Some(terminal) => match terminal {
                'b' => TokenKind::TerminalBinary,
                'd' => TokenKind::TerminalDecimal,
                'x' => TokenKind::TerminalHexadecimal,
                _ => {
                    return Err(Report::new(
                        ReportKind::NoTerminalFoundError,
                        None,
                        self.current_line.into(),
                    ))
                }
            },
        };

        self.lex_single(terminal)
    }

    fn lex_assignment(&mut self) -> LexResult<()> {
        self.advance()?;
        let token = if self.advance_if_next_is('/')? {
            TokenKind::EqualSlash
        } else {
            TokenKind::Equal
        };

        self.add_token(token);

        Ok(())
    }

    fn lex_comment(&mut self) -> LexResult<()> {
        while let Some(peeked) = self.next {
            if peeked != '\n' && !self.is_at_end() {
                self.advance()?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn lex_eol(&mut self) -> LexResult<()> {
        // check for \r\n on windows
        if self.advance_if_next_is('\r')? {
            if !self.advance_if_next_is('\n')? {
                return Err(Report::new(
                    ReportKind::InternalLexerError,
                    None,
                    self.current_line.into(),
                ));
            }
        } else {
            self.advance()?;
        }

        // update current line for nicely reporting errors
        if let Some(index) = self.src[self.token_end.offset..].find('\n') {
            self.current_line = &self.src[self.token_end.offset..index + self.token_end.offset]
        } else {
            self.current_line = &self.src[self.token_end.offset..]
        };

        // update line and column pointers on \n
        self.token_end.column = 1;
        self.token_end.line += 1;

        // flush token start
        self.token_start = self.token_end.clone();

        Ok(())
    }

    fn lex_whitespace(&mut self) -> LexResult<()> {
        while self.next_is_whitespace() {
            self.advance()?;
        }

        // flush token start
        self.token_start = self.token_end.clone();

        Ok(())
    }

    fn lex_bracket(&mut self, kind: TokenKind) -> LexResult<()> {
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

    fn close_bracket(&mut self, kind: TokenKind) -> LexResult<()> {
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

    fn advance(&mut self) -> LexResult<()> {
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

    fn advance_if_next_is(&mut self, c: char) -> LexResult<bool> {
        if self.next_is(c) {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn next_is(&self, c: char) -> bool {
        self.next == Some(c)
    }

    fn next_is_whitespace(&self) -> bool {
        self.next_is(' ') || self.next_is('\t')
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
        let source = "* %";

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
}
