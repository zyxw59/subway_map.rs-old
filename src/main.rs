#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate ordered_float;

use std::io;
use std::io::BufRead;

pub mod grammar;
pub mod ast;
pub mod errors;
pub mod command;
pub mod math;
pub mod route;


fn main() {
    let stdin = io::stdin();
    let mut vars = ast::Variables::new(10.0, 10.0);
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let s: &str = &*line.trim();
                let id = grammar::parse_Ident(s);
                match grammar::parse_Statement(s).map(|x| x.eval(&mut vars)) {
                    Err(err) => println!("{:?}", err),
                    Ok(Err(err)) => println!("{}", err),
                    _ => {
                        if let Ok(id) = id {
                            println!("{:?}", vars.get(&id));
                        }
                    },
                };
            },
            Err(err) => {println!("Error: {}", err); break;},
        }
    }
}
