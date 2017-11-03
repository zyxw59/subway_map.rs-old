use std::fmt;
use std::str::FromStr;
use std::error::Error;
use regex::Regex;
use command::Command as Cmd;
use math;
use route;
use stop;

mod variables;

use errors;
pub use self::variables::Variables;

#[derive(Clone, Debug)]
pub enum Ident {
    Scalar(SIdent),
    Point(PIdent),
    Line(LIdent),
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Ident::*;
        match *self {
            Scalar(ref id) => id.fmt(f),
            Point(ref id) => id.fmt(f),
            Line(ref id) => id.fmt(f),
        }
    }
}

macro_rules! ident {
    ($type: ident, $name: expr, $prefix: expr) => {
        ident!{$type, $name, $prefix, $prefix}
    };
    ($type: ident, $name: expr, $prefix: expr, $prefix_re: expr) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $type(String);

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!($prefix, "{}"), self.0)
            }
        }

        impl FromStr for $type {
            type Err = errors::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                lazy_static! {
                    static ref RE: Regex = Regex::new(concat!($prefix_re, r"(\w+)")).unwrap();
                }
                RE.captures(s).map_or(
                    Err(errors::Error::ident_type(s, $name)),
                    |caps| Ok($type(String::from(caps.get(1).unwrap().as_str()))))
            }
        }
    };
}

ident!{SIdent, "Scalar", "$", r"\$"}
ident!{PIdent, "Scalar", "@"}
ident!{LIdent, "Scalar", "!"}
ident!{RIdent, "Scalar", "~"}

pub trait Eval {
    type Output;

    type Ident: fmt::Display;

    fn eval(&self, vars: &Variables) -> Result<Self::Output, Box<Error>>;
}

#[derive(Clone, Debug)]
pub enum Expr {
    Scalar(Scalar),
    Point(Point),
    Line(Line),
}

impl Eval for Expr {
    type Output = math::Expr;

    type Ident = Ident;

    fn eval(&self, vars: &Variables) -> Result<math::Expr, Box<Error>> {
        use self::Expr::*;
        match *self {
            Scalar(ref x) => Ok(math::Expr::Scalar(x.eval(vars)?)),
            Point(ref x) => Ok(math::Expr::Point(x.eval(vars)?)),
            Line(ref x) => Ok(math::Expr::Line(x.eval(vars)?)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Scalar {
    Add(Box<Scalar>, Box<Scalar>),
    Neg(Box<Scalar>),
    Mul(Box<Scalar>, Box<Scalar>),
    Div(Box<Scalar>, Box<Scalar>),
    Less(Box<Scalar>, Box<Scalar>),
    LessEq(Box<Scalar>, Box<Scalar>),
    Equal(Box<Scalar>, Box<Scalar>),
    Num(math::Scalar),
    Ident(SIdent),
    Macro(SIdent, Vec<Expr>),
}

impl Eval for Scalar {
    type Output = math::Scalar;

    type Ident = SIdent;

    fn eval(&self, vars: &Variables) -> Result<math::Scalar, Box<Error>> {
        use self::Scalar::*;
        match *self {
            Add(ref a, ref b) => Ok(a.eval(vars)? + b.eval(vars)?),
            Neg(ref a) => Ok(-a.eval(vars)?),
            Mul(ref a, ref b) => Ok(a.eval(vars)? * b.eval(vars)?),
            Div(ref a, ref b) => Ok(a.eval(vars)? / b.eval(vars)?),
            Less(ref a, ref b) => if a.eval(vars)? < b.eval(vars)? {Ok(1.0)} else {Ok(0.0)},
            LessEq(ref a, ref b) => if a.eval(vars)? <= b.eval(vars)? {Ok(1.0)} else {Ok(0.0)},
            Equal(ref a, ref b) => if a.eval(vars)? == b.eval(vars)? {Ok(1.0)} else {Ok(0.0)},
            Num(x) => Ok(x),
            Ident(ref id) => Ok(*vars.get_scalar(id)?),
            Macro(ref id, ref args) => vars.get_scalar_macro(id)?.expand(args, vars),
        }
    }
}

impl Scalar {
    pub fn lt(self, b: Scalar) -> Scalar {
        use self::Scalar::*;
        if let (&Num(n), &Num(m)) = (&self, &b) {
            Num(if n < m {1.0} else {0.0})
        } else {
            Less(Box::new(self), Box::new(b))
        }
    }

    pub fn le(self, b: Scalar) -> Scalar {
        use self::Scalar::*;
        if let (&Num(n), &Num(m)) = (&self, &b) {
            Num(if n <= m {1.0} else {0.0})
        } else {
            LessEq(Box::new(self), Box::new(b))
        }
    }

    pub fn eq(self, b: Scalar) -> Scalar {
        use self::Scalar::*;
        if let (&Num(n), &Num(m)) = (&self, &b) {
            Num(if n == m {1.0} else {0.0})
        } else {
            Equal(Box::new(self), Box::new(b))
        }
    }

    pub fn add(self, b: Scalar) -> Scalar {
        use self::Scalar::*;
        if let (&Num(n), &Num(m)) = (&self, &b) {
            Num(n + m)
        } else {
            Add(Box::new(self), Box::new(b))
        }
    }

    pub fn sub(self, b: Scalar) -> Scalar {
        self.add(b.neg())
    }

    pub fn neg(self) -> Scalar {
        use self::Scalar::*;
        match self {
            Neg(a) => *a,
            Num(a) => Num(-a),
            _ => Neg(Box::new(self)),
        }
    }

    pub fn mul(self, b: Scalar) -> Scalar {
        use self::Scalar::*;
        if let (&Num(n), &Num(m)) = (&self, &b) {
            Num(n * m)
        } else {
            Mul(Box::new(self), Box::new(b))
        }
    }

    pub fn div(self, b: Scalar) -> Scalar {
        use self::Scalar::*;
        if let (&Num(n), &Num(m)) = (&self, &b) {
            Num(n / m)
        } else {
            Div(Box::new(self), Box::new(b))
        }
    }
}

#[derive(Clone, Debug)]
pub enum Point {
    Add(Box<Point>, Box<Point>),
    Neg(Box<Point>),
    Mul(Box<Point>, Scalar),
    Div(Box<Point>, Scalar),
    Pair(Scalar, Scalar),
    Intersection(Box<Line>, Box<Line>),
    Ident(PIdent),
    Macro(PIdent, Vec<Expr>),
}

impl Eval for Point {
    type Output = math::Point;

    type Ident = PIdent;

    fn eval(&self, vars: &Variables) -> Result<math::Point, Box<Error>> {
        use self::Point::*;
        match *self {
            Add(ref a, ref b) => Ok(a.eval(vars)? + b.eval(vars)?),
            Neg(ref a) => Ok(-a.eval(vars)?),
            Mul(ref a, ref n) => Ok(a.eval(vars)? * n.eval(vars)?),
            Div(ref a, ref n) => Ok(a.eval(vars)? / n.eval(vars)?),
            Pair(ref x, ref y) => Ok(math::Point(x.eval(vars)?, y.eval(vars)?)),
            Intersection(ref a, ref b) => Ok(a.eval(vars)?.intersect(b.eval(vars)?)),
            Ident(ref id) => Ok(*vars.get_point(id)?),
            Macro(ref id, ref args) => vars.get_point_macro(id)?.expand(args, vars),
        }
    }
}

impl Point {
    pub fn add(self, b: Point) -> Point {
        use self::Point::*;
        let ab = (self, b);
        if let (Pair(xa, ya), Pair(xb, yb)) = ab {
            Pair(Scalar::add(xa, xb), Scalar::add(ya, yb))
        } else {
            Add(Box::new(ab.0), Box::new(ab.1))
        }
    }

    pub fn sub(self, b: Point) -> Point {
        Point::add(self, b.neg())
    }

    pub fn neg(self) -> Point {
        use self::Point::*;
        match self {
            Neg(a) => *a,
            Mul(a, n) => Mul(a, n.neg()),
            Div(a, n) => Div(a, n.neg()),
            Pair(x, y) => Pair(x.neg(), y.neg()),
            _ => Neg(Box::new(self)),
        }
    }

    pub fn mul(self, b: Scalar) -> Point {
        Point::Mul(Box::new(self), b)
    }

    pub fn div(self, b: Scalar) -> Point {
        Point::Div(Box::new(self), b)
    }

    pub fn intersection(a: Line, b: Line) -> Point {
        Point::Intersection(Box::new(a), Box::new(b))
    }
}

#[derive(Clone, Debug)]
pub enum Line {
    Add(Box<Line>, Point),
    Parallel(Box<Line>, Point),
    Perpendicular(Box<Line>, Point),
    Offset(Box<Line>, Scalar),
    Vector(Point, Point),
    Ident(LIdent),
    Macro(LIdent, Vec<Expr>),
}

impl Eval for Line {
    type Output = math::Line;

    type Ident = LIdent;

    fn eval(&self, vars: &Variables) -> Result<math::Line, Box<Error>> {
        use self::Line::*;
        match *self {
            Add(ref l, ref p) => Ok(l.eval(vars)? + p.eval(vars)?),
            Parallel(ref l, ref p) => Ok(l.eval(vars)?.parallel(p.eval(vars)?)),
            Perpendicular(ref l, ref p) => Ok(l.eval(vars)?.perpendicular(p.eval(vars)?)),
            Offset(ref l, ref s) => Ok(l.eval(vars)?.offset(s.eval(vars)?)),
            Vector(ref o, ref v) => Ok(math::Line {
                origin: o.eval(vars)?,
                vector: v.eval(vars)?,
            }),
            Ident(ref id) => Ok(*vars.get_line(id)?),
            Macro(ref id, ref args) => vars.get_line_macro(id)?.expand(args, vars),
        }
    }
}

impl Line {
    pub fn add(self, b: Point) -> Line {
        use self::Line::*;
        match self {
            Add(l, p) => Add(l, p.add(b)),
            Parallel(l, p) => Parallel(l, p.add(b)),
            Perpendicular(l, p) => Perpendicular(l, p.add(b)),
            Vector(o, v) => Vector(o.add(b), v),
            _ => Add(Box::new(self), b),
        }
    }

    pub fn sub(self, b: Point) -> Line {
        use self::Line::*;
        match self {
            Add(l, p) => Add(l, p.sub(b)),
            Parallel(l, p) => Parallel(l, p.sub(b)),
            Perpendicular(l, p) => Perpendicular(l, p.sub(b)),
            Vector(o, v) => Vector(o.sub(b), v),
            _ => Add(Box::new(self), b.neg()),
        }
    }

    pub fn parallel(self, b: Point) -> Line {
        use self::Line::*;
        match self {
            Add(l, _) => Parallel(l, b),
            Parallel(l, _) => Parallel(l, b),
            Perpendicular(l, _) => Perpendicular(l, b),
            Vector(_, v) => Vector(b, v),
            _ => Parallel(Box::new(self), b),
        }
    }

    pub fn perpendicular(self, b: Point) -> Line {
        use self::Line::*;
        match self {
            Add(l, _) => Perpendicular(l, b),
            Parallel(l, _) => Perpendicular(l, b),
            Perpendicular(l, _) => Parallel(l, b),
            _ => Perpendicular(Box::new(self), b),
        }
    }

    pub fn offset(self, b: Scalar) -> Line {
        Line::Offset(Box::new(self), b)
    }

    pub fn vector(a: Point, b: Point) -> Line {
        Line::Vector(a, b)
    }

    pub fn between(a: Point, b: Point) -> Line {
        Line::Vector(a.clone(), b.sub(a))
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    segments: Vec<Segment>,
    offsets: Vec<Scalar>,
}

impl Route {
    pub fn start(start: Point, offset: Option<Scalar>, end: Point) -> Route {
        let seg = Segment { start, end };
        Route {
            segments: vec![seg],
            offsets: vec![offset.unwrap_or(Scalar::Num(0.0))]
        }
    }

    pub fn concat(mut self, b: Route) -> Route {
        let mut b = b;
        self.segments.append(&mut b.segments);
        self.offsets.append(&mut b.offsets);
        self
    }

    pub fn extend(mut self, offset: Option<Scalar>, end: Point) -> Route {
        let offset = offset.unwrap_or(Scalar::Num(0.0));
        let seg = Segment {
            start: self.segments.last().unwrap().clone().end,
            end,
        };
        self.segments.push(seg);
        self.offsets.push(offset);
        self
    }

    fn eval(self, vars: &mut Variables) -> Result<route::Route, Box<Error>> {
        let mut r = route::Route::new();
        for (seg, off) in self.segments.iter().zip(self.offsets.iter()) {
            let seg = seg.eval(vars)?;
            let off = off.eval(vars)?;
            vars.insert_segment(seg, off);
            r.push(seg, off);
        }
        Ok(r)
    }
}

#[derive(Clone, Debug)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl Segment {
    fn eval(&self, vars: &Variables) -> Result<route::Segment, Box<Error>> {
        Ok(route::Segment {
            start: self.start.eval(vars)?,
            end: self.end.eval(vars)?,
        })
    }

    fn line(self) -> Line {
        Line::between(self.start, self.end)
    }
}

#[derive(Clone, Debug)]
pub enum Stop {
    Line(Segment, Line, LabelPos, String),
    Segment(Segment, Segment, LabelPos, LabelPos, String),
}

impl Stop {
    pub fn eval(&self, vars: &Variables) -> Result<stop::Stop, Box<Error>> {
        use self::Stop::*;
        match *self {
            Line(ref seg, ref line, lp, ref label) => {
                Ok(stop::Stop::Line(
                        seg.eval(vars)?,
                        line.eval(vars)?,
                        lp,
                        label.clone()))
            },
            Segment(ref a, ref b, pos_a, pos_b, ref label) => {
                Ok(stop::Stop::Segment(
                        a.eval(vars)?,
                        b.eval(vars)?,
                        pos_a,
                        pos_b,
                        label.clone()))
            },
        }
    }

    pub fn perpendicular(
        seg: Segment,
        p: Point,
        pos: LabelPos,
        label: String) -> Stop {
        Stop::Line(seg.clone(), seg.line().perpendicular(p), pos, label)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LabelPos {
    Plus,
    Minus,
}

#[derive(Clone, Debug)]
pub struct Macro<T: Eval> {
    id: T::Ident,
    args: Vec<Ident>,
    body: T,
}

impl<T: Eval> Macro<T> {
    pub fn expand(&self,
                  args: &Vec<Expr>,
                  vars: &Variables) -> Result<T::Output, Box<Error>> {
        let mut locals = Variables::with_globals(vars);
        if args.len() != self.args.len() {
            Err(errors::Error::macro_args(
                    format!("{}", self.id).as_ref(),
                    args.len(),
                    self.args.len()))?;
        }
        for (id, val) in self.args.iter().zip(args.iter()) {
            match *id {
                Ident::Scalar(ref id) => {
                    if let Expr::Scalar(ref val) = *val {
                        locals.insert_scalar(id.clone(), val)?;
                    } else {
                        Err(errors::Error::macro_arg_type(
                                format!("{}", self.id).as_ref(),
                                format!("{}", id).as_ref(),
                                "Scalar",
                                format!("{:?}", val).as_ref()))?;
                    }
                }
                Ident::Point(ref id) => {
                    if let Expr::Point(ref val) = *val {
                        locals.insert_point(id.clone(), val)?;
                    } else {
                        Err(errors::Error::macro_arg_type(
                                format!("{}", self.id).as_ref(),
                                format!("{}", id).as_ref(),
                                "Point",
                                format!("{:?}", val).as_ref()))?;
                    }
                }
                Ident::Line(ref id) => {
                    if let Expr::Line(ref val) = *val {
                        locals.insert_line(id.clone(), val)?;
                    } else {
                        Err(errors::Error::macro_arg_type(
                                format!("{}", self.id).as_ref(),
                                format!("{}", id).as_ref(),
                                "Line",
                                format!("{:?}", val).as_ref()))?;
                    }
                }
            }
        }
        self.body.eval(&locals)
    }
}

#[derive(Clone, Debug)]
pub enum Definition {
    Scalar(SIdent, Scalar),
    Point(PIdent, Point),
    Line(LIdent, Line),
    Route(RIdent, Route),
    ScalarMacro(SIdent, Vec<Ident>, Scalar),
    PointMacro(PIdent, Vec<Ident>, Point),
    LineMacro(LIdent, Vec<Ident>, Line),
}

impl Definition {
    pub fn eval(self, vars: &mut Variables) -> Result<(), Box<Error>> {
        vars.eval_def(self)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Definition(Definition),
    Setup(Setup),
    Command(Command),
    None,
}

impl Statement {
    pub fn eval(self, vars: &mut Variables) -> Result<(), Box<Error>> {
        use self::Statement::*;
        match self {
            Definition(d) => d.eval(vars)?,
            Setup(s) => s.eval(vars)?,
            Command(c) => c.eval_push(vars)?,
            None => {},
        };
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Setup {
    RSep(Scalar),
    RBase(Scalar),
    Bounds(Scalar, Scalar, Scalar, Scalar),
    BoundsPoints(Point, Point),
    Style(String),
}

impl Setup {
    pub fn eval(self, vars: &mut Variables) -> Result<(), Box<Error>> {
        use self::Setup::*;
        match self {
            RSep(r_sep) => {
                let r_sep = r_sep.eval(vars)?;
                vars.r_sep = r_sep;
            },
            RBase(r_base) => {
                let r_base = r_base.eval(vars)?;
                vars.r_base = r_base;
            },
            Bounds(x0, y0, x1, y1) => {
                let p0 = math::Point(x0.eval(vars)?, y0.eval(vars)?);
                let p1 = math::Point(x1.eval(vars)?, y1.eval(vars)?);
                vars.bounds.0 = p0;
                vars.bounds.1 = p1;
            },
            BoundsPoints(p0, p1) => {
                let p0 = p0.eval(vars)?;
                let p1 = p1.eval(vars)?;
                vars.bounds.0 = p0;
                vars.bounds.1 = p1;
            },
            Style(s) => {
                vars.style.push(s);
            },
        };
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    Group(Vec<Command>, String),
    Routes(Vec<RIdent>, String),
    Stop(Stop, String),
}

impl Command {
    pub fn eval_push(self, vars: &mut Variables) -> Result<(), Box<Error>> {
        let cmd = self.eval(vars)?;
        vars.push_command(cmd);
        Ok(())
    }

    fn eval(self, vars: &mut Variables) -> Result<Cmd, Box<Error>> {
        use self::Command::*;
        match self {
            Group(v, s) => {
                let v = v.into_iter()
                    .map(|c| c.eval(vars))
                    .collect::<Result<Vec<_>, Box<Error>>>()?;
                Ok(Cmd::Group(v, s))
            }
            Routes(v, s) => {
                let v = v.into_iter()
                    .map(|r| {
                        let route = vars.get_route(&r)?.clone();
                        let id = r.0;
                        Ok(Cmd::Route(route, id))
                    }).collect::<Result<Vec<_>, Box<Error>>>()?;
                Ok(Cmd::Group(v, s))
            },
            Stop(st, id) => {
                Ok(Cmd::Stop(st.eval(vars)?, id))
            },
        }
    }
}
