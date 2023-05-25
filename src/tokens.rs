use std::fmt;

#[derive(Debug)]
pub enum TokenTypes {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Colon,
    Slash,
    Star,
    Hash,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,      // Assign: =
    EqualEqual, // Equals: ==
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    ReturnType,
    AssignType,

    And,
    BitAnd,
    BitAndAssign,

    Or,
    BitOr,
    BitOrAssign,

    // Literals.
    Identifier,
    Quote,
    Number,

    // Keywords.
    Class,
    Else,
    False,
    Function,
    For,
    If,
    Null,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Var types
    Int,
    Double,
    Char,
    Bool,
    String,
    Float,

    EOF,
}

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "Nil"),
            Object::True => write!(f, "True"),
            Object::False => write!(f, "False"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenTypes,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenTypes,
        lexeme: String,
        literal: Option<Object>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            token_type: TokenTypes::EOF,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}
