use crate::token_type::*;
use std::str::*;
use std::collections::HashMap;

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
        Self {
            start: 0,
            current: 0,
            line: 1,
            source: source,
            tokens: Vec::new(),
            keywords: HashMap::<String, TokenType>::new()
        }
    }
}

// interface
pub trait Scanner {
    // fn new()-> Self where Self: Sized;
    fn scan_tokens(&mut self)-> &Vec<Token>;
    fn scan_token(&mut self);
    fn is_at_end(&self)-> bool;
    fn advance(&mut self)-> char;
    fn peek(&self)-> char;
    fn peek_next(&self)-> char;
    fn identifier(&mut self);
    fn number(&mut self);
    fn string(&mut self);
    fn add_token(&mut self, ttype: TokenType,literal: Option<Literal>);
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

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        };
        let text: &str = &self.source[self.start..self.current];
        // println!("{}",text.to_string());
        match self.keywords.get(text) {
            Some(ttype) => self.add_token(ttype.clone(),None),
            None => self.add_token(TokenType::IDENTIFIER,None),
        }        
        
    }
    fn number(&mut self) {
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
        let num: f32 = num_text.parse().unwrap();
        self.add_token(TokenType::NUMBER,Some(Literal::Float(num)));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {self.line +=1 };
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated String.");
            return ()
        }

        self.advance();
        let value: &str = &self.source[self.start+1..self.current-1];
        self.add_token(TokenType::STRING, Some(Literal::String(value.to_string())));
    }

    fn scan_tokens(&mut self)-> &Vec<Token> {
        self.tokens = Vec::new();
        self.keywords.insert("or".to_string(),TokenType::OR);
        self.keywords.insert("and".to_string(),TokenType::AND);
        self.keywords.insert("class".to_string(),TokenType::CLASS);
        self.keywords.insert("else".to_string(),TokenType::ELSE);
        self.keywords.insert("false".to_string(),TokenType::FALSE);
        self.keywords.insert("for".to_string(),TokenType::FOR);
        self.keywords.insert("fun".to_string(),TokenType::FUN);
        self.keywords.insert("if".to_string(),TokenType::IF);
        self.keywords.insert("nil".to_string(),TokenType::NIL);
        self.keywords.insert("print".to_string(),TokenType::PRINT);
        self.keywords.insert("return".to_string(),TokenType::RETURN);
        self.keywords.insert("super".to_string(),TokenType::SUPER);
        self.keywords.insert("this".to_string(),TokenType::THIS);
        self.keywords.insert("true".to_string(),TokenType::TRUE);
        self.keywords.insert("var".to_string(),TokenType::VAR);
        self.keywords.insert("while".to_string(),TokenType::WHILE);
        println!("{:?}",self.keywords.get("hello"));
        println!("{:?}",self.tokens);
        // println!("{}",self.current == self.source.len());
        // println!("{:?}",self.advance());
        // println!("{:?}",self.source);
        // println!("{}",self.source.as_bytes()[0] as char);
        while !self.is_at_end() {
            self.start = self.current;
            println!("{}",self.current);
            self.scan_token();
        }
        println!("{:?}",self.tokens);
        &self.tokens
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Literal>){
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {token_type: ttype, literal:literal, lexeme:text.to_string(), line: self.line});
        println!(" {:?} start: {}, current: {}",text,self.start,self.current);

    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '('=> self.add_token(TokenType::LeftParen,None),
            ')'=> self.add_token(TokenType::RIGHT_PAREN,None),
            '{'=> self.add_token(TokenType::LEFT_BRACE,None),
            '}'=> self.add_token(TokenType::RIGHT_BRACE,None),
            ','=> self.add_token(TokenType::COMMA,None),
            '.'=> self.add_token(TokenType::DOT,None),
            '-'=> self.add_token(TokenType::MINUS,None),
            '+'=> self.add_token(TokenType::PLUS,None),
            ';'=> self.add_token(TokenType::SEMICOLON,None),
            '*'=> self.add_token(TokenType::STAR,None),
            '!'=> {
                if self.peek() == '=' {
                    self.current+=1;
                    self.add_token(TokenType::BANG_EQUAL,None);
                } else {
                    self.add_token(TokenType::BANG,None);
                };
            },
            '>'=> {
                if self.peek() == '=' {
                    self.current+=1;
                    self.add_token(TokenType::GREATER_EQUAL,None);
                } else {
                    self.add_token(TokenType::GREATER,None);
                };
            },
            '='=> {
                if self.peek() == '=' {
                    self.current+=1;
                    self.add_token(TokenType::EQUAL_EQUAL,None);
                } else {
                    self.add_token(TokenType::EQUAL,None);
                };
            },
            '<'=> {
                if self.peek() == '=' {
                    self.current+=1;
                    self.add_token(TokenType::LESS_EQUAL,None);
                } else {
                    self.add_token(TokenType::LESS,None);
                };
            },
            '/'=> {
                if self.peek() == '/' {
                    self.current += 1;
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    // let text: &str = &self.source[self.start..self.current];
                    // println!("{:?} {} start: {}, current: {}",self.source,text.to_string(),self.start,self.current);
                } else {
                    self.add_token(TokenType::SLASH,None)
                };
            },
            ' '=> {}
            '\r'=> {}
            '\t'=> {}
            '\n'=> {self.line+=1;}
            '\"'=> {println!("In String");self.string()}
            _ => {
                if c.is_numeric() {
                    self.number();
                } else if c.is_alphabetic() || c == '_'{
                    self.identifier();
                } else {
                    println!("Unexpected Character. {:?}",c);
                }
            }
        }
    }

}