use super::token::{Token, TokenLiteral, TokenType};
use crate::position::Position;
use crate::report::{self, Report};

use substring::Substring;

pub struct Lexer {
    source: String,
    pos: Position,

    start: usize,
    current: usize,

    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            pos: Position::new(),

            start: 0,
            current: 0,

            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<Report>> {
        let mut reports: Vec<Report> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            // append errors to error vector, ignore Ok value
            if let Err(report) = self.scan_token() {
                reports.push(report);
            }
        }

        // EOF token
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".into(),
            None,
            self.pos.clone(),
        ));

        // if we have errors, keep parsing and return all errors
        if reports.len() > 0 {
            return Err(reports);
        }

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Report> {
        if let Some(character) = self.advance() {
            match character {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '[' => self.add_token(TokenType::LeftSquare),
                ']' => self.add_token(TokenType::RightSquare),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '*' => self.add_token(TokenType::Star),

                ';' => match self.peek() {
                    Some(peeked) => {
                        while peeked != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    None => println!("FAILED"),
                },

                '=' => {
                    let token = if self.if_match_advance('/') {
                        TokenType::EqualSlash
                    } else {
                        TokenType::Equal
                    };

                    self.add_token(token)
                }

                // special characters
                ' ' | '\r' | '\t' => (),
                '\n' => {
                    self.pos.line += 1;
                    self.pos.char = 0
                }

                t => {
                    return Err(Report::new(
                        report::ReportErrorKind::UnkownTokenError,
                        format!("Unkown token {t}"),
                        Some(self.pos.clone()),
                    ))
                }
            };

            Ok(())
        } else {
            Err(Report::new(
                report::ReportErrorKind::UnableToParseError,
                String::from("Lexer was unable to parse file due to an unknown error!"),
                None,
            ))
        }
    }
}

// helper functions
impl Lexer {
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<TokenLiteral>) {
        let lexeme: String = self.source.substring(self.start, self.current).into();

        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.pos.clone()));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn if_match_advance(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(m) = self.source.chars().nth(self.current) {
            if m != ch {
                return false;
            }
        }

        self.current += 1;
        self.pos.char += 1;
        return true;
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }

        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return Some('\0');
        }

        self.source.chars().nth(self.current + 1)
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.pos.char += 1;

        self.source.chars().nth(self.current - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let source: String = "*\n ( [] ) \n = =/".into();

        let mut lexer = Lexer::new(source);

        if let Ok(tokens) = lexer.scan_tokens() {
            assert_eq!(tokens[0].get_kind(), TokenType::Star);
            assert_eq!(tokens[0].get_pos().line, 1);
            assert_eq!(tokens[6].get_kind(), TokenType::EqualSlash);
            assert_eq!(tokens[6].get_pos().line, 3);
        }
    }

    #[test]
    fn parse_tokens_with_comments() {
        let source: String = "= =/ / ;;IGNORE\n ;IGNORE / [] \n *".into();

        let mut lexer = Lexer::new(source);

        match lexer.scan_tokens() {
            Ok(tokens) => {
                for t in tokens {
                    println!("{t}")
                }
            }
            Err(err) => {
                for e in err {
                    println!("{e}")
                }
            }
        }
    }
}
