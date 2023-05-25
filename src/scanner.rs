use crate::error::*;
use crate::tokens::*;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0, // NOTE: start is called at the start of a character of a token, not the start of the file
            current: 0, // NOTE: current is the end character of a token. the oppisite of start. added to when advance is called
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;

        result
    }

    fn add_token_object(&mut self, token_type: TokenTypes, literal: Option<Object>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line))
    }

    fn add_token(&mut self, token_type: TokenTypes) {
        self.add_token_object(token_type, None);
    }

    fn is_match(&mut self, match_char: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == match_char => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn string(&mut self) -> Result<(), SunError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    break;
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }

            self.advance();
        }
        if self.is_at_end() {
            return Err(SunError::error(
                self.line,
                "untrerminated string".to_string(),
            ));
        }

        self.advance();
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenTypes::Quote, Some(Object::Str(value)));

        Ok(())
    }

    fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    fn is_alpha(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphabetic() || ch == '_'
        }  else {
            false
        }
    }

    fn is_alphanumeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphanumeric() || ch == '_'
        } else {
            false
        }
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();

        self.add_token_object(TokenTypes::Number, Some(Object::Num(num)));
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
 
        if let Some(token_type) = Scanner::keyword(text.as_str()) {
            self.add_token(token_type);
        } else {
            self.add_token(TokenTypes::Identifier);
        }
    }

    fn keyword(mat: &str) -> Option<TokenTypes> {
        match mat {
            "class" => Some(TokenTypes::Class),
            "else" => Some(TokenTypes::Else),
            "false" => Some(TokenTypes::False),
            "for" => Some(TokenTypes::For),
            "if" => Some(TokenTypes::If),
            "Null" => Some(TokenTypes::Null),
            "print" => Some(TokenTypes::Print),
            "return" => Some(TokenTypes::Return),
            "super" => Some(TokenTypes::Super),
            "this" => Some(TokenTypes::This),
            "true" => Some(TokenTypes::True),
            "int" => Some(TokenTypes::Int),
            "double" => Some(TokenTypes::Double),
            "char" => Some(TokenTypes::Char),
            "bool" => Some(TokenTypes::Bool),
            "while" => Some(TokenTypes::While),
            "String" => Some(TokenTypes::String),
            "float" => Some(TokenTypes::Float),
            _ => None,
        }
    }

    fn scan_comment(&mut self) -> Result<(), SunError> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();

                    if self.is_match('/') {
                        return Ok(());
                    }
                }

                Some('/') => {
                    self.advance();

                    if self.is_match('*') {
                        self.scan_comment()?;
                    }
                }

                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }

                None => {
                    return Err(SunError::error(
                        self.line,
                        "Untemninated Comment".to_string(),
                    ));
                }

                _ => {
                    self.advance();
                }
            }
        }
    }

    fn scan_token(&mut self) -> Result<(), SunError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenTypes::LeftParen),
            ')' => self.add_token(TokenTypes::RightParen),
            '{' => self.add_token(TokenTypes::LeftBrace),
            '}' => self.add_token(TokenTypes::RightBrace),
            '[' => self.add_token(TokenTypes::LeftBracket),
            ']' => self.add_token(TokenTypes::RightBracket),
            ',' => self.add_token(TokenTypes::Comma),
            '.' => self.add_token(TokenTypes::Dot),
            '+' => self.add_token(TokenTypes::Plus),
            ';' => self.add_token(TokenTypes::SemiColon),
            '*' => self.add_token(TokenTypes::LeftParen),
            '#' => self.add_token(TokenTypes::Hash),
            ':' => {
                let tok = if self.is_match(':') {
                    TokenTypes::AssignType
                } else {
                    TokenTypes::Colon
                };
                self.add_token(tok);
            }
            '-' => {
                let tok = if self.is_match('>') {
                    TokenTypes::ReturnType
                } else {
                    TokenTypes::Minus
                };
                self.add_token(tok);
            }
            '!' => {
                let tok = if self.is_match('=') {
                    TokenTypes::BangEqual
                } else {
                    TokenTypes::Bang
                };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.is_match('=') {
                    TokenTypes::EqualEqual
                } else {
                    TokenTypes::Equal
                };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.is_match('=') {
                    TokenTypes::LessEqual
                } else {
                    TokenTypes::Less
                };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.is_match('=') {
                    TokenTypes::GreaterEqual
                } else {
                    TokenTypes::Greater
                };
                self.add_token(tok);
            }
            '&' => {
                let tok = if self.is_match('=') {
                    TokenTypes::BitAndAssign
                } else if self.is_match('&') {
                    TokenTypes::And
                } else {
                    TokenTypes::BitAnd
                };
                self.add_token(tok);
            }
            '|' => {
                let tok = if self.is_match('=') {
                    TokenTypes::BitOrAssign
                } else if self.is_match('|') {
                    TokenTypes::Or
                } else {
                    TokenTypes::BitOr
                };
                self.add_token(tok);
            }
            '/' => {
                if self.is_match('/') {
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else if self.is_match('*') {
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenTypes::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            '0'..='9' => {
                self.number();
            }
            _ if c.is_alphabetic() || c == '_' => {
                self.identifier();
            }
            _ => {
                return Err(SunError::error(
                    self.line,
                    "Unexpected Character".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, SunError> {
        let mut had_error: Option<SunError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("".to_string());
                    had_error = Some(e);
                }
            }
        }
        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }
}
