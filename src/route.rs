use std::error::Error;
use std::io::BufWriter;
use std::f64::consts::FRAC_PI_2;

use std::io::prelude::*;

use math;
use ast::Variables;
use math::{Point, Scalar};

#[derive(Clone, Debug)]
pub struct Route {
    segments: Vec<Segment>,
    offsets: Vec<Scalar>,
}

impl Route {
    pub fn new() -> Route {
        Route {
            segments: Vec::new(),
            offsets: Vec::new(),
        }
    }

    pub fn push(&mut self, s: Segment, o: Scalar) {
        self.segments.push(s);
        self.offsets.push(o);
    }

    pub fn format<W: Write>(&self,
                          buf: &mut BufWriter<W>,
                          vars: &Variables,
                          id: &String) -> Result<(), Box<Error>> {
        unimplemented!();
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl Segment {
    pub fn reverse(self) -> Segment {
        Segment {
            start: self.end,
            end: self.start,
        }
    }

    pub fn dir(self) -> Scalar {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        dy.atan2(dx)
    }

    pub fn arc_to<W: Write>(self,
                  buf: &mut BufWriter<W>,
                  vars: &Variables,
                  other: Segment,
                  off_in: Scalar,
                  off_out: Scalar) -> Result<(), Box<Error>> {
        let inr = off_in * vars.r_sep;
        let outr = off_out * vars.r_sep;
        if self.end != other.start {
            self.format_end(buf, vars, off_in)?;
            other.format_start(buf, vars, off_out)?;
            return Ok(());
        }
        let in_dir = self.dir();
        let out_dir = other.dir();
        if in_dir == out_dir {
            if off_in == off_out {
                // nothingn to do here
                return Ok(());
            }
            // otherwise, parallel shifts
            let delta = (off_out - off_in) * vars.r_sep;
            let p0 = self.end.basis(in_dir, -delta, inr);
            let p1 = self.end.basis(in_dir, 0.0, inr);
            let p2 = self.end.basis(in_dir, 0.0, outr);
            let p3 = self.end.basis(in_dir, delta, outr);
            writeln!(buf, "L {} C {} {} {}", p0, p1, p2, p3)?;
            return Ok(());
        }
        // rounded corner
        let sweep;
        let in_delta;
        let out_delta;
        let theta = math::mod_tau(in_dir - out_dir) / 2.0;
        if theta > FRAC_PI_2 {
            sweep = 1;
            in_delta = vars.max_offset(self).unwrap() - off_in;
            out_delta = vars.max_offset(other).unwrap() - off_out;
        } else {
            sweep = 0;
            in_delta = off_in - vars.min_offset(self).unwrap();
            out_delta = off_out - vars.min_offset(other).unwrap();
        }
        let r = vars.r_sep.mul_add(in_delta.min(out_delta), vars.r_base);
        let l = (r * theta.tan()).abs();
        let alpha = (theta*2.0).sin().recip();
        let p = self.end.basis(in_dir, alpha * outr, 0.0).basis(out_dir, -alpha *inr, 0.0);
        let start = p.basis(in_dir, -l, 0.0);
        let end = p.basis(in_dir, l, 0.0);
        writeln!(buf, "L {} A {},{1} 0 0 {} {}", start, r, sweep, end)?;
        Ok(())
    }

    fn format_end<W: Write>(self,
                            buf: &mut BufWriter<W>,
                            vars: &Variables,
                            off: Scalar) -> Result<(), Box<Error>> {
        let p = self.end.basis(self.dir(), 0.0, off * vars.r_sep);
        write!(buf, "L {}", p)?;
        Ok(())
    }

    fn format_start<W: Write>(self,
                            buf: &mut BufWriter<W>,
                            vars: &Variables,
                            off: Scalar) -> Result<(), Box<Error>> {
        let p = self.start.basis(self.dir(), 0.0, off * vars.r_sep);
        write!(buf, "L {}", p)?;
        Ok(())
    }
}
