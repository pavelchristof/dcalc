//! Derivative calculator.

#[crate_id = "dcalc#0.1"];
#[comment = "Derivative calculator"];
#[feature(struct_variant)];

use std::io::buffered::BufferedReader;
use std::io::stdin;
use std::io::stdio::flush;
use std::io::mem::MemReader;

use func::DiffFunc;
use tokenizer::{invalid_token, Ignore};
use parser::Parser;
use simplify::Simplify;

mod func;
mod monad;
mod parser;
mod tokenizer;
mod simplify;

/// Parse a string into a function.
fn parse(s: &str) -> Result<~DiffFunc, ~str> {
    let stream = MemReader::new(s.as_bytes().to_owned());
    Parser::parse(stream)
}

/// Parses a string and computes its derivative.
fn interpret(s: &str) {
    match parse(s) {
        Ok(f)  => println!("{}", f.simplify().derivative().simplify().to_str("x")),
        Err(s) => println!("Error: {}", s)
    }
}

/// Runs the read_line-parse loop.
fn run() {
    let mut stdin = BufferedReader::new(stdin());

    loop {
        print!("> ");
        flush();

        let line = stdin.read_line();
        match line {
            Some(s) => interpret(s),
            None    => break
        }
    }
}

fn main() {
    invalid_token::cond.trap(|s| {
        println!("Invalid token: {}", s);
        Ignore
    }).inside(|| run());
}
