#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate ordered_float;

use std::env;
use std::io;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub mod grammar;
pub mod ast;
pub mod errors;
pub mod command;
pub mod math;
pub mod route;

macro_rules! raise {
    ($e: expr) => (Err(::std::convert::From::from($e)));
}

macro_rules! raise_str {
    ($e: expr) => (raise!(errors::Error::from_str($e)));
}

macro_rules! raise_val {
    ($e: expr) => (raise!(errors::Error::from_val($e)));
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let res = match args.len() {
        // no args
        1 => {
            let mut infile = io::stdin();
            let mut outfile = io::stdout();
            parse(&mut infile, &mut outfile)
        },
        2 => {
            let mut infile = File::open(&args[1]).unwrap();
            let mut outfile = io::stdout();
            parse(&mut infile, &mut outfile)
        },
        3 => {
            let mut infile = File::open(&args[1]).unwrap();
            let mut outfile = File::create(&args[2]).unwrap();
            parse(&mut infile, &mut outfile)
        },
        _ => raise_str!("Too many arguments"),
    };
    if let Err(err) = res {
        println!("{}", err);
    }
}

fn parse<R: Read, W: Write>(r: &mut R, w: &mut W) -> Result<(), Box<Error>> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    match grammar::parse_Program(s.as_str()) {
        Err(err) => raise_val!(err),
        Ok(prog) => {
            let mut vars = ast::Variables::new();
            for st in prog {
                st.eval(&mut vars)?;
            }
            vars.format(w)?;
            Ok(())
        }
    }
}
