use crate::config::LexerConfig;
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

    config: LexerConfig,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str, config: LexerConfig) -> Self {
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
            config,
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
            '<' => self.lex_bracket(TokenKind::LeftAngle)?,
            '>' => self.lex_bracket(TokenKind::RightAngle)?,

            '.' => self.lex_single(TokenKind::Dot)?,
            '-' => self.lex_single(TokenKind::Range)?,
            '*' => self.lex_single(TokenKind::Star)?,
            '/' => self.lex_single(TokenKind::Slash)?,

            '=' => self.lex_assignment()?,
            ';' => self.lex_comment()?,
            '%' => self.lex_terminal()?,

            ' ' | '\t' => self.lex_whitespace()?,
            '\n' | '\r' => self.lex_eol()?,

            '"' => self.lex_string_literal()?,

            _ if start.is_ascii_digit() => self.lex_number_literal(TokenKind::Number)?,
            _ if start.is_ascii_alphabetic() => self.lex_identifier()?,
            _ => {
                self.advance()?; // continue lexing

                if self.is_at_end() {
                    return Err(Report::new(
                        ReportKind::UnableToParseError,
                        Some(self.token_end.clone()),
                        self.current_line.into(),
                    ));
                }
            }
        };

        Ok(())
    }

    fn lex_string_literal(&mut self) -> LexResult<()> {
        self.advance()?; // get first "
        while let Some(peeked) = self.next {
            if peeked != '"' && !self.is_at_end() {
                self.advance()?;
            } else {
                break;
            }
        }

        if self.is_at_end() {
            return Err(Report::new(
                ReportKind::UnterminatedStringError,
                Some(self.token_start.clone()),
                self.current_line.into(),
            ));
        }

        self.advance()?; // grab string terminator
        self.add_token(TokenKind::String);
        Ok(())
    }

    fn lex_number_literal(&mut self, kind: TokenKind) -> LexResult<()> {
        while let Some(peeked) = self.next {
            if peeked.is_ascii_digit() && !self.is_at_end() {
                self.advance()?;
            } else {
                break;
            }
        }

        self.add_token(kind);
        Ok(())
    }

    fn lex_identifier(&mut self) -> LexResult<()> {
        self.advance()?;
        while let Some(c) = self.next {
            if !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'| '-') {
                break;
            }
            self.advance()?;
        }

        self.add_token(TokenKind::Identifier);
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
                ' ' | '\t' => {
                    return Err(Report::new(
                        ReportKind::NoTerminalFoundError,
                        Some(self.token_end.clone()),
                        self.current_line.into(),
                    ))
                }
                _ => {
                    return Err(Report::new(
                        ReportKind::IncorrectTerminalFoundError,
                        Some(self.token_end.clone()),
                        self.current_line.into(),
                    ))
                }
            },
        };

        self.lex_single(terminal.clone())?;

        match terminal {
            TokenKind::TerminalBinary => self.lex_terminal_binary()?,
            TokenKind::TerminalDecimal => self.lex_terminal_decimal()?,
            TokenKind::TerminalHexadecimal => self.lex_terminal_hexadecimal()?,
            _ => unreachable!(),
        };

        Ok(())
    }

    fn lex_terminal_binary(&mut self) -> LexResult<()> {
        self.lex_number_literal(TokenKind::Binary)?;

        if self.advance_if_next_is('-')? || self.advance_if_next_is('.')? {
            self.lex_terminal_binary()?;
        }

        // check for binary
        if let Some(last) = self.tokens.last() {
            if last.length != 7 {
                return Err(Report::new(
                    ReportKind::SevenBitsError,
                    Some(self.token_end.clone()),
                    self.current_line.into(),
                ));
            }
            for c in last.get_lexeme().chars() {
                if c != '0' && c != '1' {
                    return Err(Report::new(
                        ReportKind::BinaryTerminalError,
                        Some(self.token_end.clone()),
                        self.current_line.into(),
                    ));
                }
            }
        } else {
            return Err(Report::new(
                ReportKind::InternalLexerError,
                None,
                self.current_line.into(),
            ));
        }

        Ok(())
    }

    fn lex_terminal_decimal(&mut self) -> LexResult<()> {
        self.lex_number_literal(TokenKind::Decimal)?;

        if self.advance_if_next_is('-')? || self.advance_if_next_is('.')? {
            self.lex_terminal_decimal()?;
        }

        if let Some(last) = self.tokens.last() {
            let n = last.get_lexeme().parse::<i32>();

            match n {
                Ok(num) => {
                    if num < 0 || num > 126 && !self.config.extended {
                        return Err(Report::new(
                            ReportKind::DecimalTerminalError,
                            Some(self.token_end.clone()),
                            self.current_line.into(),
                        ));
                    }
                }
                Err(_err) => {
                    return Err(Report::new(
                        ReportKind::NaNError,
                        Some(self.token_end.clone()),
                        self.current_line.into(),
                    ))
                }
            }
        } else {
            return Err(Report::new(
                ReportKind::InternalLexerError,
                None,
                self.current_line.into(),
            ));
        }

        Ok(())
    }

    fn lex_terminal_hexadecimal(&mut self) -> LexResult<()> {
        let look_ahead = self.rest();

        if look_ahead.len() < 2 {
            return Err(Report::new(
                ReportKind::EofError,
                Some(self.token_end.clone()),
                self.current_line.into(),
            ));
        }
        match u32::from_str_radix(&look_ahead[..2], 16) {
            Ok(hex) => {
                self.advance()?;
                self.advance()?;

                if self.config.extended {
                    loop {
                        if let Some(n) = self.next {
                            match u32::from_str_radix(&n.to_string(), 16) {
                                Ok(_) => self.advance()?,
                                Err(_) => break,
                            }
                        } else {
                            break;
                        }
                    }
                }
                self.add_token(TokenKind::Hexadecimal);

                if hex > 126 && !self.config.extended {
                    return Err(Report::new(
                        ReportKind::HexadecimalTerminalError,
                        Some(self.token_end.clone()),
                        self.current_line.into(),
                    ));
                }
            }
            Err(_err) => {
                return Err(Report::new(
                    ReportKind::NaHexNError,
                    Some(self.token_end.clone()),
                    self.current_line.into(),
                ))
            }
        };

        Ok(())
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

        self.add_token(TokenKind::Whitespace);

        Ok(())
    }

    fn lex_bracket(&mut self, kind: TokenKind) -> LexResult<()> {
        match kind {
            TokenKind::LeftParen => self.open_bracket(TokenKind::Paren),
            TokenKind::RightParen => self.close_bracket(TokenKind::Paren)?,
            TokenKind::LeftSquare => self.open_bracket(TokenKind::Square),
            TokenKind::RightSquare => self.close_bracket(TokenKind::Square)?,
            TokenKind::LeftAngle => self.open_bracket(TokenKind::Angle),
            TokenKind::RightAngle => self.close_bracket(TokenKind::Angle)?,
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
                ReportKind::UnableToAdvanceError,
                None,
                self.current_line.into(),
            )),
        }
    }

    fn advance_if_next_is(&mut self, c: char) -> LexResult<bool> {
        self.advance_if_next_is_in_range(c..=c)
    }

    fn advance_if_next_is_in_range(
        &mut self,
        range: std::ops::RangeInclusive<char>,
    ) -> LexResult<bool> {
        if let Some(c) = self.next {
            if range.contains(&c) {
                self.advance()?;
                Ok(true)
            } else {
                Ok(false)
            }
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

    macro_rules! test {
        {
            name:   $name:ident,
            text:   $text:expr,
            tokens: ($($kind:expr),*)
        } => {
            #[test]
            fn $name() {
                let kinds: &[TokenKind] = &[$($kind,)* TokenKind::EOF];

                test($text, Some(kinds), None, false);
            }
        };
    }

    macro_rules! error {
        {
            name:   $name:ident,
            text:   $text:expr,
            errors:  ($($kind:expr),*)
        } => {
            #[test]
            fn $name() {
                let kinds: &[ReportKind] = &[$($kind,)*];

                test($text, None, Some(kinds), true);
            }
        }
    }

    fn test(
        text: &str,
        want_kinds: Option<&[TokenKind]>,
        want_report_kinds: Option<&[ReportKind]>,
        expect_error: bool,
    ) {
        let mut lexer = Lexer::new(text);

        match lexer.tokenize() {
            Ok(tokens) => {
                let have_kinds = tokens
                    .iter()
                    .map(|t| t.kind.clone())
                    .collect::<Vec<TokenKind>>();

                // safe unwrap as it is a guarantee
                assert_eq!(have_kinds, want_kinds.unwrap(), "{have_kinds:?}");
            }
            Err(err) => {
                if !expect_error {
                    for e in err {
                        println!("{e}");
                    }
                    panic!("== test produces errors, run test command with --nocapture to see output ==")
                }

                let have_report_kinds = err
                    .iter()
                    .map(|r| r.get_kind())
                    .collect::<Vec<ReportKind>>();

                // safe unwrap
                assert_eq!(
                    have_report_kinds,
                    want_report_kinds.unwrap(),
                    "{have_report_kinds:?}"
                )
            }
        }
    }

    test! {
        name: char_delim_paren,
        text: "()",
        tokens: (TokenKind::LeftParen, TokenKind::RightParen)
    }

    test! {
        name: char_delim_square,
        text: "[]",
        tokens: (TokenKind::LeftSquare, TokenKind::RightSquare)
    }

    test! {
        name: char_delim_angle,
        text: "<>",
        tokens: (TokenKind::LeftAngle, TokenKind::RightAngle)
    }

    test! {
        name: char_dot,
        text: ".",
        tokens: (TokenKind::Dot)
    }

    test! {
        name: char_range,
        text: "-",
        tokens: (TokenKind::Range)
    }

    test! {
        name: char_star,
        text: "*",
        tokens: (TokenKind::Star)
    }

    test! {
        name: char_slash,
        text: "/",
        tokens: (TokenKind::Slash)
    }

    test! {
        name: char_equal,
        text: "=",
        tokens: (TokenKind::Equal)
    }

    test! {
        name: char_equal_slash,
        text: "=/",
        tokens: (TokenKind::EqualSlash)
    }

    test! {
        name: char_whitespace,
        text: " ",
        tokens: (TokenKind::Whitespace)
    }

    test! {
        name: newline_safe,
        text: "*\n-\n=/\n=",
        tokens: (TokenKind::Star, TokenKind::Range, TokenKind::EqualSlash, TokenKind::Equal)
    }

    test! {
        name: comment_ignored,
        text: "=; = = * *\n*",
        tokens: (TokenKind::Equal, TokenKind::Star)
    }

    test! {
        name: terminal_decimal,
        text: "%d50",
        tokens: (TokenKind::Mod, TokenKind::TerminalDecimal, TokenKind::Decimal)
    }

    error! {
        name: decimal_terminal_error,
        text: "%d128",
        errors: (ReportKind::DecimalTerminalError)
    }

    error! {
        name: decimal_nan_error,
        text: "%dk",
        errors: (ReportKind::NaNError)
    }

    test! {
        name: terminal_binary,
        text: "%b1100110",
        tokens: (TokenKind::Mod, TokenKind::TerminalBinary, TokenKind::Binary)
    }

    error! {
        name: binary_terminal_error,
        text: "%b1001302",
        errors: (ReportKind::BinaryTerminalError)
    }

    error! {
        name: binary_seven_bits_error,
        text: "%b110",
        errors: (ReportKind::SevenBitsError)
    }

    test! {
        name: hex_binary,
        text: "%x50",
        tokens: (TokenKind::Mod, TokenKind::TerminalHexadecimal, TokenKind::Hexadecimal)
    }

    error! {
        name: hex_terminal_error,
        text: "%x80",
        errors: (ReportKind::HexadecimalTerminalError)
    }

    error! {
        name: nahex_error,
        text: "%xlo\n%x \n",
        errors: (ReportKind::NaHexNError, ReportKind::NaHexNError)
    }

    test! {
        name: string_literal,
        text: "\"string\"",
        tokens: (TokenKind::String)
    }

    error! {
        name: unterminated_string_error,
        text: "\"unterminated string",
        errors: (ReportKind::UnterminatedStringError)
    }
}
