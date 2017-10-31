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
    pub fn format_def<W: Write>(&self,
                          buf: &mut BufWriter<W>,
                          vars: &Variables) -> Result<(), Box<Error>> {
        use self::Command::*;
        match *self {
            Group(ref v, _) => {
                for ref c in v {
                    c.format_def(buf, vars)?;
                }
            },
            Route(ref r, ref s) => {
                r.format_def(buf, vars, s)?;
            }
        }
        Ok(())
    }
    pub fn format_use<W: Write>(&self,
                          buf: &mut BufWriter<W>,
                          vars: &Variables) -> Result<(), Box<Error>> {
        use self::Command::*;
        match *self {
            Group(ref v, ref s) => {
                writeln!(buf, r#"<g class="{}">"#, s)?;
                writeln!(buf, r#"<g class="bg">"#)?;
                for ref c in v {
                    c.format_use(buf, vars)?;
                }
                writeln!(buf, "</g>")?;
                writeln!(buf, r#"<g class="fg">"#)?;
                for ref c in v {
                    c.format_use(buf, vars)?;
                }
                writeln!(buf, "</g>")?;
            },
            Route(_, ref s) => {
                writeln!(buf, r#"<use xlink:href"{0}" class="{0}"/>"#, s)?;
            }
        }
        Ok(())
    }
}
