use std::{env, fs::File, io::BufReader};

use lexer::Lexer;

mod token;
mod predicate;
mod rule;
mod datalog_program;
mod lexer;
mod file_iterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: datalog-rs <input file>");
    }
    let filename = args.get(1).unwrap();
    let file = File::open(filename).expect(&format!("Failed to open {}", filename));
    let reader = BufReader::new(file);

    let tokens = Lexer::lex(reader);
}
