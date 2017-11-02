use std::error::Error;
use std::f64::consts::FRAC_PI_2;

use std::io::prelude::*;

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

    pub fn format_def<W: Write>(&self,
                                w: &mut W,
                                vars: &Variables,
                                id: &String) -> Result<(), Box<Error>> {
        writeln!(w, r#"<path id="{}" d=""#, id)?;
        self.segments[0].format_start(w, vars, self.offsets[0])?;
        for (i, seg) in self.segments[1..].iter().enumerate() {
            self.segments[i].arc_to(w, vars, *seg, self.offsets[i], self.offsets[i+1])?;
        }
        self.segments.last().unwrap().format_end(w, vars, *self.offsets.last().unwrap())?;
        writeln!(w, r#"" />"#)?;
        Ok(())
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

    pub fn dir(self) -> Point {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        Point(dx, dy)
    }

    pub fn arc_to<W: Write>(self,
                            w: &mut W,
                            vars: &Variables,
                            other: Segment,
                            off_in: Scalar,
                            off_out: Scalar) -> Result<(), Box<Error>> {
        let inr = off_in * vars.r_sep;
        let outr = off_out * vars.r_sep;
        if self.end != other.start {
            self.format_end(w, vars, off_in)?;
            other.format_start(w, vars, off_out)?;
            return Ok(());
        }
        let in_dir = self.dir();
        let out_dir = other.dir();
        if in_dir.angle(out_dir).abs() < 1e-3 {
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
            writeln!(w, "L {} C {} {} {}", p0, p1, p2, p3)?;
            return Ok(());
        }
        // rounded corner
        let sweep;
        let in_delta;
        let out_delta;
        let theta = in_dir.angle(out_dir) / 2.0;
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
        let end = p.basis(out_dir, l, 0.0);
        writeln!(w, "L {start} A {r},{r} 0 0 {sweep} {end}",
                 start=start,
                 r=r,
                 sweep=sweep,
                 end=end)?;
        Ok(())
    }

    fn format_end<W: Write>(self,
                            w: &mut W,
                            vars: &Variables,
                            off: Scalar) -> Result<(), Box<Error>> {
        let p = self.end.basis(-self.dir(), 0.0, off * vars.r_sep);
        writeln!(w, "L {}", p)?;
        Ok(())
    }

    fn format_start<W: Write>(self,
                              w: &mut W,
                              vars: &Variables,
                              off: Scalar) -> Result<(), Box<Error>> {
        let p = self.start.basis(-self.dir(), 0.0, off * vars.r_sep);
        writeln!(w, "M {}", p)?;
        Ok(())
    }
}
