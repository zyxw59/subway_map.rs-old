use std::error::Error;
use std::io::BufWriter;

use std::io::prelude::*;

use route;
use ast::Variables;

#[derive(Clone, Debug)]
pub enum Command {
    Group(Vec<Command>, String),
    Route(route::Route, String),
}

impl Command {
    pub fn format<W: Write>(&self,
                          buf: &mut BufWriter<W>,
                          vars: &Variables) -> Result<(), Box<Error>> {
        use self::Command::*;
        match *self {
            Group(ref v, ref s) => {
                writeln!(buf, r#"<g id="{}">"#, s)?;
                writeln!(buf, r#"<g class="bg">"#)?;
                for ref c in v {
                    c.format(buf, vars)?;
                }
                writeln!(buf, "</g>")?;
                for ref c in v {
                    c.format(buf, vars)?;
                }
                writeln!(buf, "</g>")?;
            },
            Route(ref r, ref s) => {
                r.format(buf, vars, s)?;
            }
        }
        Ok(())
    }
}
