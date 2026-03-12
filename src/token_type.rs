use std::fmt;

#[derive(Debug)] // to print explictly
#[derive(Clone)] // to clone this enum
#[derive(PartialEq)] // to apply binary eq
pub enum TokenType {
    // single-char tokens.
    LeftParen, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STRING, NUMBER,
    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    EOF, Error
}

impl fmt::Display for TokenType{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f,"Left '('"),
            TokenType::OR => write!(f,"OR"),
            _ => write!(f,"Umatched Token"),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Literal {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    String(String)
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType, // type of token; identifier, value, keywords ...
    // pub start: &char,
    pub lexeme: String, // string value of that token
    pub literal: Option<Literal>, // literal value of that token
    pub line: usize, // in which line it exist.
}

