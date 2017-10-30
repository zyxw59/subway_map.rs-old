use std::str::FromStr;
use regex::Regex;
use math;
use route;

mod errors;
mod variables;

pub use self::errors::IdentError;
pub use self::variables::Variables;

#[derive(Clone, Debug)]
pub enum Ident {
    Scalar(SIdent),
    Point(PIdent),
    Line(LIdent),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SIdent(String);

impl FromStr for SIdent {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\$(\w+)").unwrap();
        }
        RE.captures(s).map_or(
            Err(IdentError::new(s, "Scalar")),
            |caps| Ok(SIdent(String::from(caps.get(1).unwrap().as_str()))))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PIdent(String);

impl FromStr for PIdent {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"@(\w+)").unwrap();
        }
        RE.captures(s).map_or(
            Err(IdentError::new(s, "Point")),
            |caps| Ok(PIdent(String::from(caps.get(1).unwrap().as_str()))))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LIdent(String);

impl FromStr for LIdent {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"!(\w+)").unwrap();
        }
        RE.captures(s).map_or(
            Err(IdentError::new(s, "Line")),
            |caps| Ok(LIdent(String::from(caps.get(1).unwrap().as_str()))))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RIdent(String);

impl FromStr for RIdent {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"~(\w+)").unwrap();
        }
        RE.captures(s).map_or(
            Err(IdentError::new(s, "Route")),
            |caps| Ok(RIdent(String::from(caps.get(1).unwrap().as_str()))))
    }
}

pub trait Eval {
    type Output;

    type Ident;

    fn eval(&self, vars: &Variables) -> Self::Output;
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

    fn eval(&self, vars: &Variables) -> math::Expr {
        use self::Expr::*;
        match *self {
            Scalar(ref x) => math::Expr::Scalar(x.eval(vars)),
            Point(ref x) => math::Expr::Point(x.eval(vars)),
            Line(ref x) => math::Expr::Line(x.eval(vars)),
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

    fn eval(&self, vars: &Variables) -> math::Scalar {
        use self::Scalar::*;
        match *self {
            Add(ref a, ref b) => a.eval(vars) + b.eval(vars),
            Neg(ref a) => - a.eval(vars),
            Mul(ref a, ref b) => a.eval(vars) * b.eval(vars),
            Div(ref a, ref b) => a.eval(vars) / b.eval(vars),
            Less(ref a, ref b) => if a.eval(vars) < b.eval(vars) {1.0} else {0.0},
            LessEq(ref a, ref b) => if a.eval(vars) <= b.eval(vars) {1.0} else {0.0},
            Equal(ref a, ref b) => if a.eval(vars) == b.eval(vars) {1.0} else {0.0},
            Num(x) => x,
            Ident(ref id) => *vars.get_scalar(id).unwrap(),
            Macro(ref id, ref args) => vars.get_scalar_macro(id).unwrap().expand(args, vars),
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

    fn eval(&self, vars: &Variables) -> math::Point {
        use self::Point::*;
        match *self {
            Add(ref a, ref b) => a.eval(vars) + b.eval(vars),
            Neg(ref a) => -a.eval(vars),
            Mul(ref a, ref n) => a.eval(vars) * n.eval(vars),
            Div(ref a, ref n) => a.eval(vars) / n.eval(vars),
            Pair(ref x, ref y) => math::Point(x.eval(vars), y.eval(vars)),
            Intersection(ref a, ref b) => a.eval(vars).intersect(b.eval(vars)),
            Ident(ref id) => *vars.get_point(id).unwrap(),
            Macro(ref id, ref args) => vars.get_point_macro(id).unwrap().expand(args, vars),
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

    fn eval(&self, vars: &Variables) -> math::Line {
        use self::Line::*;
        match *self {
            Add(ref l, ref p) => l.eval(vars) + p.eval(vars),
            Parallel(ref l, ref p) => l.eval(vars).parallel(p.eval(vars)),
            Perpendicular(ref l, ref p) => l.eval(vars).perpendicular(p.eval(vars)),
            Offset(ref l, ref s) => l.eval(vars).offset(s.eval(vars)),
            Vector(ref o, ref v) => math::Line {
                origin: o.eval(vars),
                vector: v.eval(vars),
            },
            Ident(ref id) => *vars.get_line(id).unwrap(),
            Macro(ref id, ref args) => vars.get_line_macro(id).unwrap().expand(args, vars),
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

    fn eval(self, vars: &mut Variables) -> route::Route {
        let mut r = route::Route::new();
        for (seg, off) in self.segments.iter().zip(self.offsets.iter()) {
            let seg = seg.eval(vars);
            let off = off.eval(vars);
            vars.insert_segment(seg, off);
            r.push(seg, off);
        }
        r
    }
}

#[derive(Clone, Debug)]
pub struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn eval(&self, vars: &Variables) -> route::Segment {
        route::Segment {
            start: self.start.eval(vars),
            end: self.end.eval(vars),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Macro<T: Eval> {
    id: T::Ident,
    args: Vec<Ident>,
    body: T,
}

impl<T: Eval> Macro<T> {
    pub fn expand(&self, args: &Vec<Expr>, vars: &Variables) -> T::Output {
        let mut locals = Variables::with_globals(vars);
        if args.len() != self.args.len() {
            panic!("Incorrect number of arguments to macro (got {}, expected {})",
            args.len(), self.args.len());
        }
        for (id, val) in self.args.iter().zip(args.iter()) {
            match *id {
                Ident::Scalar(ref id) => {
                    if let Expr::Scalar(ref val) = *val {
                        locals.insert_scalar(id.clone(), val);
                    } else {
                        panic!("Argument {:?} is not a scalar", val);
                    }
                }
                Ident::Point(ref id) => {
                    if let Expr::Point(ref val) = *val {
                        locals.insert_point(id.clone(), val);
                    } else {
                        panic!("Argument {:?} is not a point", val);
                    }
                }
                Ident::Line(ref id) => {
                    if let Expr::Line(ref val) = *val {
                        locals.insert_line(id.clone(), val);
                    } else {
                        panic!("Argument {:?} is not a line", val);
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
    pub fn eval(self, vars: &mut Variables) {
        vars.eval_def(self);
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Definition(Definition),
    Command(Command),
    None,
}

impl Statement {
    pub fn eval(self, vars: &mut Variables) {
        use self::Statement::*;
        match self {
            Definition(d) => d.eval(vars),
            Command(_) => {},
            None => {},
        };
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    Routes(Vec<RIdent>, String),
}
