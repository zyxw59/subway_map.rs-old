use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use math;

#[derive(Clone, Debug)]
pub struct Variables {
    scalars: HashMap<SIdent, math::Scalar>,
    points: HashMap<PIdent, math::Point>,
    lines: HashMap<LIdent, math::Line>,
    scalar_macros: HashMap<SIdent, Macro<Scalar>>,
    point_macros: HashMap<PIdent, Macro<Point>>,
    line_macros: HashMap<LIdent, Macro<Line>>,
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            scalars: HashMap::new(),
            points: HashMap::new(),
            lines: HashMap::new(),
            scalar_macros: HashMap::new(),
            point_macros: HashMap::new(),
            line_macros: HashMap::new(),
        }
    }

    pub fn get(&self, id: &Ident) -> Option<math::Expr> {
        match *id {
            Ident::Scalar(ref id) => self.scalars.get(&id).map(|x| math::Expr::Scalar(*x)),
            Ident::Point(ref id) => self.points.get(&id).map(|x| math::Expr::Point(*x)),
            Ident::Line(ref id) => self.lines.get(&id).map(|x| math::Expr::Line(*x)),
        }
    }
}

trait Eval {
    type Output;

    fn eval(&self, vars: &Variables) -> Self::Output;
}

#[derive(Clone, Debug)]
pub enum Ident {
    Scalar(SIdent),
    Point(PIdent),
    Line(LIdent),
}

#[derive(Clone, Debug)]
pub enum Statement {
    Definition(Definition),
}

impl Statement {
    pub fn eval(self, vars: &mut Variables) {
        match self {
            Statement::Definition(d) => d.eval(vars),
        };
    }
}

#[derive(Clone, Debug)]
pub enum Definition {
    Scalar(SIdent, Scalar),
    Point(PIdent, Point),
    Line(LIdent, Line),
    ScalarMacro(SIdent, Vec<Ident>, Scalar),
    PointMacro(PIdent, Vec<Ident>, Point),
    LineMacro(LIdent, Vec<Ident>, Line),
}

impl Definition {
    pub fn eval(self, vars: &mut Variables) {
        use self::Definition::*;
        match self {
            Scalar(id, val) => {
                let val = val.eval(vars);
                vars.scalars.insert(id, val);
            },
            Point(id, val) => {
                let val = val.eval(vars);
                vars.points.insert(id, val);
            },
            Line(id, val) => {
                let val = val.eval(vars);
                vars.lines.insert(id, val);
            },
            ScalarMacro(id, args, body) => {
                vars.scalar_macros.insert(id, Macro{args, body});
            },
            PointMacro(id, args, body) => {
                vars.point_macros.insert(id, Macro{args, body});
            },
            LineMacro(id, args, body) => {
                vars.line_macros.insert(id, Macro{args, body});
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Macro<T: Eval> {
    args: Vec<Ident>,
    body: T,
}

impl<T: Eval> Macro<T> {
    pub fn expand(&self, args: &Vec<Expr>, vars: &Variables) -> T::Output {
        let mut locals = Variables::new();
        if args.len() != self.args.len() {
            panic!("Incorrect number of arguments to macro (got {}, expected {})",
            args.len(), self.args.len());
        }
        for (id, val) in self.args.iter().zip(args.iter()) {
            match *id {
                Ident::Scalar(ref id) => {
                    if let Expr::Scalar(ref val) = *val {
                        locals.scalars.insert(id.clone(), val.eval(vars));
                    } else {
                        panic!("Argument {:?} is not a scalar", val);
                    }
                }
                Ident::Point(ref id) => {
                    if let Expr::Point(ref val) = *val {
                        locals.points.insert(id.clone(), val.eval(vars));
                    } else {
                        panic!("Argument {:?} is not a point", val);
                    }
                }
                Ident::Line(ref id) => {
                    if let Expr::Line(ref val) = *val {
                        locals.lines.insert(id.clone(), val.eval(vars));
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
pub enum Expr {
    Scalar(Scalar),
    Point(Point),
    Line(Line),
}

impl Eval for Expr {
    type Output = math::Expr;

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
pub enum IdentError {
    Scalar(String),
    Point(String),
    Line(String),
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
            Err(IdentError::Scalar(String::from(s))),
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
            Err(IdentError::Point(String::from(s))),
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
            Err(IdentError::Line(String::from(s))),
            |caps| Ok(LIdent(String::from(caps.get(1).unwrap().as_str()))))
    }
}


#[derive(Clone, Debug)]
pub enum Scalar {
    Add(Box<Scalar>, Box<Scalar>),
    Negative(Box<Scalar>),
    Multiply(Box<Scalar>, Box<Scalar>),
    Divide(Box<Scalar>, Box<Scalar>),
    Less(Box<Scalar>, Box<Scalar>),
    LessEq(Box<Scalar>, Box<Scalar>),
    Equal(Box<Scalar>, Box<Scalar>),
    Number(math::Scalar),
    Ident(SIdent),
    Macro(SIdent, Vec<Expr>),
}

impl Eval for Scalar {
    type Output = math::Scalar;

    fn eval(&self, vars: &Variables) -> math::Scalar {
        use self::Scalar::*;
        match *self {
            Add(ref a, ref b) => a.eval(vars) + b.eval(vars),
            Negative(ref a) => - a.eval(vars),
            Multiply(ref a, ref b) => a.eval(vars) * b.eval(vars),
            Divide(ref a, ref b) => a.eval(vars) / b.eval(vars),
            Less(ref a, ref b) => if a.eval(vars) < b.eval(vars) {1.0} else {0.0},
            LessEq(ref a, ref b) => if a.eval(vars) <= b.eval(vars) {1.0} else {0.0},
            Equal(ref a, ref b) => if a.eval(vars) == b.eval(vars) {1.0} else {0.0},
            Number(x) => x,
            Ident(ref id) => *vars.scalars.get(id).unwrap(),
            Macro(ref id, ref args) => vars.scalar_macros.get(id).unwrap().expand(args, vars),
        }
    }
}

impl Scalar {
    pub fn lt(a: Self, b: Self) -> Self {
        use self::Scalar::*;
        if let (&Number(n), &Number(m)) = (&a, &b) {
            Number(if n < m {1.0} else {0.0})
        } else {
            Less(Box::new(a), Box::new(b))
        }
    }

    pub fn le(a: Self, b: Self) -> Self {
        use self::Scalar::*;
        if let (&Number(n), &Number(m)) = (&a, &b) {
            Number(if n <= m {1.0} else {0.0})
        } else {
            LessEq(Box::new(a), Box::new(b))
        }
    }

    pub fn eq(a: Self, b: Self) -> Self {
        use self::Scalar::*;
        if let (&Number(n), &Number(m)) = (&a, &b) {
            Number(if n == m {1.0} else {0.0})
        } else {
            Equal(Box::new(a), Box::new(b))
        }
    }

    pub fn add(a: Self, b: Self) -> Self {
        use self::Scalar::*;
        if let (&Number(n), &Number(m)) = (&a, &b) {
            Number(n + m)
        } else {
            Add(Box::new(a), Box::new(b))
        }
    }

    pub fn subtract(a: Self, b: Self) -> Self {
        Scalar::add(a, Scalar::negative(b))
    }

    pub fn negative(a: Self) -> Self {
        use self::Scalar::*;
        match a {
            Negative(a) => *a,
            Number(a) => Number(-a),
            _ => Negative(Box::new(a)),
        }
    }

    pub fn multiply(a: Self, b: Self) -> Self {
        use self::Scalar::*;
        if let (&Number(n), &Number(m)) = (&a, &b) {
            Number(n * m)
        } else {
            Multiply(Box::new(a), Box::new(b))
        }
    }

    pub fn divide(a: Self, b: Self) -> Self {
        use self::Scalar::*;
        if let (&Number(n), &Number(m)) = (&a, &b) {
            Number(n / m)
        } else {
            Divide(Box::new(a), Box::new(b))
        }
    }
}

#[derive(Clone, Debug)]
pub enum Point {
    Add(Box<Point>, Box<Point>),
    Negative(Box<Point>),
    Multiply(Box<Point>, Scalar),
    Divide(Box<Point>, Scalar),
    Pair(Scalar, Scalar),
    Intersection(Box<Line>, Box<Line>),
    Ident(PIdent),
    Macro(PIdent, Vec<Expr>),
}

impl Eval for Point {
    type Output = math::Point;

    fn eval(&self, vars: &Variables) -> math::Point {
        use self::Point::*;
        match *self {
            Add(ref a, ref b) => a.eval(vars) + b.eval(vars),
            Negative(ref a) => -a.eval(vars),
            Multiply(ref a, ref n) => a.eval(vars) * n.eval(vars),
            Divide(ref a, ref n) => a.eval(vars) / n.eval(vars),
            Pair(ref x, ref y) => math::Point(x.eval(vars), y.eval(vars)),
            Intersection(ref a, ref b) => a.eval(vars).intersect(b.eval(vars)),
            Ident(ref id) => *vars.points.get(id).unwrap(),
            Macro(ref id, ref args) => vars.point_macros.get(id).unwrap().expand(args, vars),
        }
    }
}

impl Point {
    pub fn add(a: Self, b: Self) -> Self {
        use self::Point::*;
        let ab = (a, b);
        if let (Pair(xa, ya), Pair(xb, yb)) = ab {
            Pair(Scalar::add(xa, xb), Scalar::add(ya, yb))
        } else {
            Add(Box::new(ab.0), Box::new(ab.1))
        }
    }

    pub fn subtract(a: Self, b: Self) -> Self {
        Point::add(a, Point::negative(b))
    }

    pub fn negative(a: Self) -> Self {
        use self::Point::*;
        match a {
            Negative(a) => *a,
            Multiply(a, n) => Multiply(a, Scalar::negative(n)),
            Divide(a, n) => Divide(a, Scalar::negative(n)),
            Pair(x, y) => Pair(Scalar::negative(x), Scalar::negative(y)),
            _ => Negative(Box::new(a)),
        }
    }

    pub fn multiply(a: Self, b: Scalar) -> Self {
        Point::Multiply(Box::new(a), b)
    }

    pub fn divide(a: Self, b: Scalar) -> Self {
        Point::Divide(Box::new(a), b)
    }

    pub fn intersection(a: Line, b: Line) -> Self {
        Point::Intersection(Box::new(a), Box::new(b))
    }
}

#[derive(Clone, Debug)]
pub enum Line {
    Add(Box<Line>, Point),
    Parallel(Box<Line>, Point),
    Perpendicular(Box<Line>, Point),
    Vector(Point, Point),
    Ident(LIdent),
    Macro(LIdent, Vec<Expr>),
}

impl Eval for Line {
    type Output = math::Line;

    fn eval(&self, vars: &Variables) -> math::Line {
        use self::Line::*;
        match *self {
            Add(ref l, ref p) => l.eval(vars) + p.eval(vars),
            Parallel(ref l, ref p) => l.eval(vars).parallel(p.eval(vars)),
            Perpendicular(ref l, ref p) => l.eval(vars).perpendicular(p.eval(vars)),
            Vector(ref o, ref v) => math::Line {
                origin: o.eval(vars),
                vector: v.eval(vars),
            },
            Ident(ref id) => *vars.lines.get(id).unwrap(),
            Macro(ref id, ref args) => vars.line_macros.get(id).unwrap().expand(args, vars),
        }
    }
}

impl Line {
    pub fn add(a: Self, b: Point) -> Self {
        use self::Line::*;
        match a {
            Add(l, p) => Add(l, Point::add(p, b)),
            Parallel(l, p) => Parallel(l, Point::add(p, b)),
            Perpendicular(l, p) => Perpendicular(l, Point::add(p, b)),
            Vector(o, v) => Vector(Point::add(o, b), v),
            _ => Add(Box::new(a), b),
        }
    }

    pub fn subtract(a: Self, b: Point) -> Self {
        use self::Line::*;
        match a {
            Add(l, p) => Add(l, Point::subtract(p, b)),
            Parallel(l, p) => Parallel(l, Point::subtract(p, b)),
            Perpendicular(l, p) => Perpendicular(l, Point::subtract(p, b)),
            Vector(o, v) => Vector(Point::subtract(o, b), v),
            _ => Add(Box::new(a), Point::negative(b)),
        }
    }

    pub fn parallel(a: Self, b: Point) -> Self {
        use self::Line::*;
        match a {
            Add(l, _) => Parallel(l, b),
            Parallel(l, _) => Parallel(l, b),
            Perpendicular(l, _) => Perpendicular(l, b),
            Vector(_, v) => Vector(b, v),
            _ => Parallel(Box::new(a), b),
        }
    }

    pub fn perpendicular(a: Self, b: Point) -> Self {
        use self::Line::*;
        match a {
            Add(l, _) => Perpendicular(l, b),
            Parallel(l, _) => Perpendicular(l, b),
            Perpendicular(l, _) => Parallel(l, b),
            _ => Perpendicular(Box::new(a), b),
        }
    }

    pub fn vector(a: Point, b: Point) -> Self {
        Line::Vector(a, b)
    }

    pub fn between(a: Point, b: Point) -> Self {
        Line::Vector(a.clone(), Point::subtract(b, a))
    }
}
