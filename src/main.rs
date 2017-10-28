#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;

pub mod grammar;
pub mod ast;
pub mod math;
pub mod route;


fn main() {
    let stdin = io::stdin();
    let mut vars = ast::Variables::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let s: &str = &*line.trim();
                let id = grammar::parse_Ident(s);
                if let Err(err) = grammar::parse_Statement(s).map(|x| x.eval(&mut vars)) {
                    println!("{:?}", err);
                }
                if let Ok(id) = id {
                    println!("{:?}", vars.get(&id));
                }
            },
            Err(err) => {println!("Error: {}", err); break;},
        }
    }
}
