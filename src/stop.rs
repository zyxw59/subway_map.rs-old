use std::cmp::Ordering;
use std::error::Error;
use std::io::prelude::*;

use errors;

use ast::{LabelPos, Variables};
use math::{Line, Point};
use route::Segment;

#[derive(Clone, Debug)]
pub enum Stop {
    Line(Segment, Line, LabelPos, String),
    Segment(Segment, Segment, LabelPos, LabelPos, String),
}

impl Stop {
    pub fn format_def<W: Write>(&self,
                                buf: &mut W,
                                vars: &Variables,
                                id: &String) -> Result<(), Box<Error>> {
        match *self {
            Stop::Line(seg, line, pos, ref label) => {
                unimplemented!();
            },
            Stop::Segment(a, b, pos_a, pos_b, ref label) => {
                let off_a = match pos_a {
                    LabelPos::Plus => {
                        vars.max_offset(a)
                            .ok_or(errors::Error::segment(a))? + 1.0
                    },
                    LabelPos::Minus => {
                        vars.min_offset(a)
                            .ok_or(errors::Error::segment(a))? - 1.0
                    }
                } * vars.r_sep;
                let off_b = match pos_b {
                    LabelPos::Plus => {
                        vars.max_offset(b)
                            .ok_or(errors::Error::segment(b))? + 1.0
                    },
                    LabelPos::Minus => {
                        vars.min_offset(b)
                            .ok_or(errors::Error::segment(b))? - 1.0
                    }
                } * vars.r_sep;
                let p0 = a.line().intersect(b.line());
                let p = a.line().offset(off_a).intersect(b.line().offset(off_b));
                let (dx, dy) = p.cmp(p0);
                Stop::label_at(buf, p, dx, dy, label, id)?;
            },
        }
        Ok(())
    }

    pub fn format_use<W: Write>(&self,
                                buf: &mut W,
                                vars: &Variables,
                                id: &String) -> Result<(), Box<Error>> {
        writeln!(buf, r##"<use xlink:href="#s{0}" class="stop s_{0}"/>"##, id)?;
        Ok(())
    }

    fn label_at<W: Write>(buf: &mut W,
                          p: Point,
                          x: Ordering,
                          y: Ordering,
                          label: &String,
                          id: &String) -> Result<(), Box<Error>> {
        write!(buf, r#"<text x="{}" y="{}" id="s{}" "#, p.0, p.1, id)?;
        match x {
            Ordering::Greater => write!(buf, r#"text-anchor="start" "#)?,
            Ordering::Equal => write!(buf, r#"text-anchor="middle" "#)?,
            Ordering::Less => write!(buf, r#"text-anchor="end" "#)?,
        };
        match y {
            Ordering::Greater => write!(buf, r#"dominant-baseline="hanging" "#)?,
            Ordering::Equal => write!(buf, r#"dominant-baseline="middle" "#)?,
            Ordering::Less => write!(buf, r#"dominant-baseline="alphabetic" "#)?,
        };
        writeln!(buf, ">{}</text>", label)?;
        Ok(())
    }
}
