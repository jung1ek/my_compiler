use std::env;
use std::fs::File;
use std::io::prelude::*;

mod scanner;
mod token_type;
mod byte_code;
mod vm;
mod op;
mod compiler;
mod value;

use byte_code::*;
use scanner::*;
use vm::*;
use compiler::*;
// use token_type::Token;

fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
	let mut text: String = "".to_string();
    if args.len() == 2 {
		let path = &args[1];
		let _text = read_file(path)?;
		text = _text;
		println!("{:?}",text);
		interpret(&text);


    }else {
		println!("Usage: run [file_path]");
    }
	// let mut scanner = ScannerSt::new(text);
	// let tokens = scanner.scan_tokens();
	// println!("{:?}",tokens);
    Ok(())
}

fn read_file(path: &String)-> Result<String, std::io::Error> {
	let mut file =  File::open(path)?;
	let mut text: String = String::new();
	file.read_to_string(&mut text)?;
	Ok(text)
}

