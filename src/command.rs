use std::error::Error;

use std::io::prelude::*;

use route::Route;
use stop::Stop;
use ast::Variables;

#[derive(Clone, Debug)]
pub enum Command {
    Group(Vec<Command>, String),
    Route(Route, String),
    Stop(Stop, String),
}

impl Command {
    pub fn format_def<W: Write>(&self,
                                buf: &mut W,
                                vars: &Variables) -> Result<(), Box<Error>> {
        use self::Command::*;
        match *self {
            Group(ref v, _) => {
                for ref c in v {
                    c.format_def(buf, vars)?;
                }
            },
            Route(ref r, ref id) => {
                r.format_def(buf, vars, id)?;
            }
            Stop(ref s, ref id) => {
                s.format_def(buf, vars, id)?;
            }
        }
        Ok(())
    }
    pub fn format_use<W: Write>(&self,
                                buf: &mut W,
                                vars: &Variables) -> Result<(), Box<Error>> {
        use self::Command::*;
        match *self {
            Group(ref v, ref s) => {
                writeln!(buf, r#"<g class="g_{}">"#, s)?;
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
                writeln!(buf, "</g>")?;
            },
            Route(_, ref id) => {
                writeln!(buf, r##"<use xlink:href="#{0}" class="route r_{0}"/>"##, id)?;
            },
            Stop(ref s, ref id) => {
                s.format_use(buf, vars, id)?;
            },
        }
        Ok(())
    }
}
