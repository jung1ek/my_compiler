use crate::token_type::*;
use std::str::*;
use std::collections::HashMap;
use crate::value::*;

// traits is like interface
pub struct ScannerSt {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source: String,
    pub tokens: Vec<Token>,
    pub keywords: HashMap::<String,TokenType>
}

impl ScannerSt {
    pub fn new(source: String)-> Self {
        let mut scanner = Self {
            start: 0,
            current: 0,
            line: 1,
            source: source,
            tokens: Vec::new(),
            keywords: HashMap::<String, TokenType>::new()
        };
        scanner.keywords.insert("or".to_string(),TokenType::OR);
        scanner.keywords.insert("and".to_string(),TokenType::AND);
        scanner.keywords.insert("class".to_string(),TokenType::CLASS);
        scanner.keywords.insert("else".to_string(),TokenType::ELSE);
        scanner.keywords.insert("false".to_string(),TokenType::FALSE);
        scanner.keywords.insert("for".to_string(),TokenType::FOR);
        scanner.keywords.insert("fun".to_string(),TokenType::FUN);
        scanner.keywords.insert("if".to_string(),TokenType::IF);
        scanner.keywords.insert("nil".to_string(),TokenType::NIL);
        scanner.keywords.insert("print".to_string(),TokenType::PRINT);
        scanner.keywords.insert("return".to_string(),TokenType::RETURN);
        scanner.keywords.insert("super".to_string(),TokenType::SUPER);
        scanner.keywords.insert("this".to_string(),TokenType::THIS);
        scanner.keywords.insert("true".to_string(),TokenType::TRUE);
        scanner.keywords.insert("var".to_string(),TokenType::VAR);
        scanner.keywords.insert("while".to_string(),TokenType::WHILE);
        scanner
    }
}

// interface
pub trait Scanner {
    // fn new()-> Self where Self: Sized;
    fn scan_tokens(&mut self)-> &Vec<Token>;
    fn scan_token(&mut self)-> Token;
    fn is_at_end(&self)-> bool;
    fn advance(&mut self)-> char;
    fn peek(&self)-> char;
    fn peek_next(&self)-> char;
    fn identifier(&mut self)-> Token;
    fn number(&mut self)-> Token;
    fn string(&mut self)-> Token;
    fn add_token(&mut self, ttype: TokenType,literal: Option<Literal>)-> Token;
    fn error_token(&mut self,c: char)-> Token;
    fn skip_whitespace(&mut self);
}

impl Scanner for ScannerSt {
    fn is_at_end(&self)-> bool{
        return self.current == self.source.len();
    }

    //TODO working for non-asci, more than one byte.
    fn advance(&mut self)-> char {
        self.current += 1;
        // as_bytes is borrow concept.
        self.source.as_bytes()[self.current-1] as char
    }

    fn peek(&self)-> char {
        if self.is_at_end(){return '\0'};
        return self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self)-> char {
        if self.current+1>=self.source.len(){return '\0'};
        return self.source.as_bytes()[self.current+1] as char
    }

    fn identifier(&mut self)-> Token {
        while self.peek().is_alphanumeric() {
            self.advance();
        };
        let text: &str = &self.source[self.start..self.current];
        // println!("{}",text);
        match self.keywords.get(text) {
            Some(ttype) => return self.add_token(ttype.clone(),None),
            None =>return self.add_token(TokenType::IDENTIFIER,None),
        }        
        
    }
    fn number(&mut self)-> Token {
        while self.peek().is_numeric() {
            self.advance();
        }
        // look for fraction
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }
        let num_text: &str = &self.source[self.start..self.current];
        // println!("{:?}",num_text);
        let num: f32 = num_text.parse().unwrap();
        return self.add_token(TokenType::NUMBER,Some(Literal::Float(num)));
    }

    fn string(&mut self)-> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {self.line +=1 };
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated String.");
            return self.error_token('U');
        }

        self.advance();
        let value: &str = &self.source[self.start+1..self.current-1];
        return self.add_token(TokenType::STRING, Some(Literal::String(value.to_string())));
    }

    fn scan_tokens(&mut self)-> &Vec<Token> {
        self.tokens = Vec::new();
        while !self.is_at_end() {
            let token = self.scan_token();
            self.tokens.push(token);
        }
        &self.tokens
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Literal>)-> Token{
        let text = &self.source[self.start..self.current];
        // println!("{:?}",ttype);
        return Token {token_type: ttype, literal:literal, lexeme:text.to_string(), line: self.line};
        // println!(" {:?} start: {}, current: {}",text,self.start,self.current);

    }

    // should return Token with add it to list.
    fn scan_token(&mut self)->Token {

        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end(){
            return self.add_token(TokenType::EOF,Some(Literal::String("".to_string())))
        }

        let c: char = self.advance();
        match c {
            '('=> return self.add_token(TokenType::LeftParen,None),
            ')'=> return self.add_token(TokenType::RIGHT_PAREN,None),
            '{'=> return self.add_token(TokenType::LEFT_BRACE,None),
            '}'=> return self.add_token(TokenType::RIGHT_BRACE,None),
            ','=> return self.add_token(TokenType::COMMA,None),
            '.'=> return self.add_token(TokenType::DOT,None),
            '-'=> return self.add_token(TokenType::MINUS,None),
            '+'=> return self.add_token(TokenType::PLUS,None),
            ';'=> return self.add_token(TokenType::SEMICOLON,None),
            '*'=> return self.add_token(TokenType::STAR,None),
            '!'=> {
                if self.peek() == '=' {
                    self.current+=1;
                    return self.add_token(TokenType::BANG_EQUAL,None);
                } else {
                    return self.add_token(TokenType::BANG,None);
                };
            },
            '>'=> {
                if self.peek() == '=' {
                    self.current+=1;
                    return self.add_token(TokenType::GREATER_EQUAL,None);
                } else {
                    return self.add_token(TokenType::GREATER,None);
                };
            },
            '='=> {
                if self.peek() == '=' {
                    self.current+=1;
                    return self.add_token(TokenType::EQUAL_EQUAL,None);
                } else {
                    return self.add_token(TokenType::EQUAL,None);
                };
            },
            '<'=> {
                if self.peek() == '=' {
                    self.current+=1;
                    return self.add_token(TokenType::LESS_EQUAL,None);
                } else {
                    return self.add_token(TokenType::LESS,None);
                };
            },
            
            _ => {
                if c.is_numeric() {
                    return self.number();
                } else if c.is_alphabetic() || c == '_'{
                    return self.identifier();
                } else {
                    println!("Unexpected Character. {:?}",c);
                    return self.error_token(c);
                }
            }
        }
    }

    // skip if there is white spaces and comment.
    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // skip comment
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn error_token(&mut self,c: char)-> Token{
        return Token {token_type: TokenType::Error, literal:None, lexeme:c.to_string(), line: self.line};
    }

}

