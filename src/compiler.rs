use crate::scanner::*;
use crate::byte_code::*;
use crate::token_type::*;
use crate::vm::*;
use crate::value::*;
//TODO use impl for compiler.


static mut HAD_ERROR:bool = false;
static mut PANIC_MODE:bool = false;

// max slot for constants.
static UINT8_MAX: i32 = 256;

#[derive(Debug)]
pub struct Parser {
    current: Option<Token>,
    previous:  Option<Token>
}

pub fn compile(source: &str, chunk: &mut ChunkSt)-> bool {
	let mut scanner = ScannerSt::new(source.to_string());
    let mut parser = Parser {current: None, previous: None};
    let mut chunk = chunk;

	// advance
	advance(&mut parser, &mut scanner);
    // match parser.previous {
    //     Some(ref token)=> println!("Prev: {:?}",token.token_type),
    //     None=> {println!("Previous: None")}
    // };

    // expression, emit based on operation's precedence.
    expression(&mut chunk,&mut parser,&mut scanner);
    // match parser.previous {
    //     Some(ref token)=> println!("Prev: {:?}",token.token_type),
    //     None=> {println!("Previous: None")}
    // };

    // consume
    consume(&mut parser, &mut scanner,TokenType::SEMICOLON,"Expected end of expression.");
    // println!("Current: {:?}",parser.current.clone().unwrap().token_type);

    // end compiler; at last return operation
    end_compiler(chunk,&mut parser);

    // println!("{:?}",chunk);

	return unsafe {!HAD_ERROR};
}

fn end_compiler(chunk: &mut ChunkSt, parser: &mut Parser) {
    emit_byte(chunk,parser,OpCode::OpReturn as u8);
}

// step one token forward.
fn advance(parser: &mut Parser, scanner: &mut ScannerSt) {
    parser.previous = parser.current.clone();
    loop {
        let token = scanner.scan_token();

        // println!("{:?}",token);
        parser.current = Some(token);


        // if no error then break
        if parser.current.clone().unwrap().token_type != TokenType::Error {
            break;
            println!("No same prev and Cur");
        };
        error_at_current(parser,&parser.current.clone().unwrap().lexeme);
    }
}

// take token type and check.
fn consume(parser: &mut Parser,scanner: &mut ScannerSt,ttype: TokenType,message: &str) {
    if parser.current.clone().unwrap().token_type == ttype {
        return advance(parser,scanner);
    }
    return error_at_current(parser,message);
}

// Parse based on precedence
fn expression(chunk: &mut ChunkSt,parser: &mut Parser,scanner: &mut ScannerSt){
    // start from lowest precedence.
    parse_precedence(chunk,parser,scanner,Precedence::PrecAssignment);
}

fn parse_precedence(chunk: &mut ChunkSt,parser: &mut Parser,scanner: &mut ScannerSt,precedence: Precedence){
    advance(parser,scanner);
    let prefix = get_rule(
        &parser.previous.clone().unwrap().token_type
    ).prefix;

    if !prefix.is_none() {
        prefix.clone().unwrap()(chunk,parser,scanner);
    };

    loop {
        let current_type = parser.current.clone().unwrap().token_type;
        let rule = get_rule(&current_type);
        if precedence_value(&precedence) > precedence_value(&rule.precedence) {
            break;
        }

        advance(parser,scanner);

        if let Some(infix) = rule.infix {
            infix(chunk, parser, scanner);
        };
    }

}

//======Functions=====
// to emit constant, act as the leaf node for recursion.
fn number(chunk: &mut ChunkSt,parser: &mut Parser,scanner: &mut ScannerSt) {

    let value: Value = Value::new(ValueType::Float(parser.previous
        .clone()
        .unwrap()
        .lexeme
        .parse()
        .unwrap()));

    emit_constant(chunk, parser, value);
}

fn string(){
    
}

// to emit operation byte. (neg,not,..)
fn unary(chunk: &mut ChunkSt,parser: &mut Parser,scanner: &mut ScannerSt){
    let operator_type: &TokenType = &parser.previous.clone().unwrap().token_type;

    // compile the right operand
    parse_precedence(chunk,parser,scanner,Precedence::PrecUnary);

    // emit the operator instruction.
    match operator_type {
        TokenType::MINUS=> emit_byte(chunk,parser,OpCode::OpNegate as u8),
        _=>{}
    }
}

//to emit operation byte. (mul,sub,...)
fn binary(chunk: &mut ChunkSt,parser: &mut Parser, scanner: &mut ScannerSt) {
    let operator_type: &TokenType = &parser.previous.clone().unwrap().token_type;
    
    // compile the right operand.
    let rule: ParseRule = get_rule(operator_type);
    parse_precedence(chunk,parser,scanner,rule.precedence);

    // emit operatior instruction.
    match operator_type {

    TokenType::PLUS =>
        emit_byte(chunk, parser, OpCode::OpAdd as u8),

    TokenType::MINUS =>
        emit_byte(chunk, parser, OpCode::OpSub as u8),

    TokenType::STAR =>
        emit_byte(chunk, parser, OpCode::OpMul as u8),

    TokenType::SLASH =>
        emit_byte(chunk, parser, OpCode::OpDiv as u8),

    _ => {}
    }
}

// emitting bytes.
fn emit_bytes(chunk: &mut ChunkSt,parser: &Parser,byte_1: u8, byte_2: u8) {
    emit_byte(chunk,parser,byte_1);
    emit_byte(chunk,parser,byte_2);
}

fn emit_byte(chunk: &mut ChunkSt,parser: &Parser,byte: u8) {
    // we assume the token has been consumed and in prev.
    let line = match &parser.previous {
        Some(token)=> token.line as u8,
        None=> {println!("None rn"); return ();}
    };
    chunk.write_chunk(byte,line);
}

// Constant
fn emit_constant(chunk: &mut ChunkSt,parser: &Parser,value: Value) {
    let const_idx = make_constant(chunk,value);
    emit_bytes(chunk,parser,OpCode::OpConstant as u8, const_idx);
}

// get constant index and store constant value in constant array
fn make_constant(chunk: &mut ChunkSt,value: Value)->u8 {
    let const_idx: u8 = chunk.add_constant(value);
    if const_idx as i32 > UINT8_MAX {
        println!("Error too many constant in one chunk.");
        return 0;
    }
    return const_idx;
}

// grouping expression (); precedence
fn grouping(chunk: &mut ChunkSt,parser: &mut Parser,scanner: &mut ScannerSt){
    expression(chunk,parser,scanner);
    consume(parser,scanner,TokenType::RIGHT_PAREN, "Expect ')' after expression.");
}

// parse rule based on the token type.
fn get_rule(op_type: &TokenType) -> ParseRule {

    match op_type {

        TokenType::LeftParen => ParseRule {
            prefix: Some(grouping),
            infix: None,
            precedence: Precedence::PrecNone
        },

        TokenType::MINUS => ParseRule {
            prefix: Some(unary),
            infix: Some(binary),
            precedence: Precedence::PrecTerm
        },

        TokenType::PLUS => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::PrecTerm
        },

        TokenType::STAR => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::PrecFactor
        },

        TokenType::SLASH => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::PrecFactor
        },
        TokenType::NUMBER => ParseRule {
            prefix: Some(number),
            infix: None,
            precedence: Precedence::PrecFactor
        },
        _ => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        }
    }
}

// parse rule structure.
struct ParseRule {
    prefix: Option<fn(&mut ChunkSt,&mut Parser,&mut ScannerSt)>,
    infix: Option<fn(&mut ChunkSt,&mut Parser,&mut ScannerSt)>,
    precedence: Precedence
}

// order from low to high.
enum Precedence {
    PrecNone,PrecAssignment, // =
    PrecOr, PrecAnd, PrecEq, // or ,and , == !=
    PrecComparison, PrecTerm, // < =>, + -
    PrecFactor,PrecUnary, // * / , - !
    PrecCall,PrecPrimary // . (),
}

// precedence values.
fn precedence_value(p: &Precedence) -> u8 {
    match p {
        Precedence::PrecNone => 0,
        Precedence::PrecAssignment => 1,
        Precedence::PrecOr => 2,
        Precedence::PrecAnd => 3,
        Precedence::PrecEq => 4,
        Precedence::PrecComparison => 5,
        Precedence::PrecTerm => 6,
        Precedence::PrecFactor => 7,
        Precedence::PrecUnary => 8,
        Precedence::PrecCall => 9,
        Precedence::PrecPrimary => 10,
    }
}

fn error_at_current(parser: &Parser,message: &str) {
    error_at(&parser.current,message);
}

fn error(parser: &Parser,message: &str) {
    error_at(&parser.previous,message);
}

// TODO formatting
fn error_at(token: &Option<Token>,message: &str) {
    if unsafe {PANIC_MODE} {return ()};
    unsafe {PANIC_MODE=true};

    let token = match token {
        Some(v)=> v,
        None=> return,
    };

    eprint!("[line {}] Error",token.line);

    if token.token_type == TokenType::EOF {
        eprint!(" at end");
    } else if token.token_type == TokenType::Error {
        {};
    } else {
        eprint!(" at {}",token.lexeme.len());
    }
    eprint!(": {}\n",message);
    unsafe {HAD_ERROR = true};
}