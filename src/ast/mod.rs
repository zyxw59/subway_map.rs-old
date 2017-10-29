use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use math;
use route;

mod errors;

pub use self::errors::IdentError;

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

trait Eval {
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

    type Ident = SIdent;

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
            Ident(ref id) => *vars.get_scalar(id).unwrap(),
            Macro(ref id, ref args) => vars.get_scalar_macro(id).unwrap().expand(args, vars),
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

    type Ident = PIdent;

    fn eval(&self, vars: &Variables) -> math::Point {
        use self::Point::*;
        match *self {
            Add(ref a, ref b) => a.eval(vars) + b.eval(vars),
            Negative(ref a) => -a.eval(vars),
            Multiply(ref a, ref n) => a.eval(vars) * n.eval(vars),
            Divide(ref a, ref n) => a.eval(vars) / n.eval(vars),
            Pair(ref x, ref y) => math::Point(x.eval(vars), y.eval(vars)),
            Intersection(ref a, ref b) => a.eval(vars).intersect(b.eval(vars)),
            Ident(ref id) => *vars.get_point(id).unwrap(),
            Macro(ref id, ref args) => vars.get_point_macro(id).unwrap().expand(args, vars),
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

    type Ident = LIdent;

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
            Ident(ref id) => *vars.get_line(id).unwrap(),
            Macro(ref id, ref args) => vars.get_line_macro(id).unwrap().expand(args, vars),
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
            offsets: vec![offset.unwrap_or(Scalar::Number(0.0))]
        }
    }

    pub fn concat(a: Route, b: Route) -> Route {
        let mut a = a;
        let mut b = b;
        a.segments.append(&mut b.segments);
        a.offsets.append(&mut b.offsets);
        a
    }

    pub fn extend(r: Route, offset: Option<Scalar>, end: Point) -> Route {
        let offset = offset.unwrap_or(Scalar::Number(0.0));
        let mut segments = r.segments;
        let mut offsets = r.offsets;
        let seg = Segment {
            start: segments.last().unwrap().clone().end,
            end,
        };
        segments.push(seg);
        offsets.push(offset);
        Route { segments, offsets }
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
struct Macro<T: Eval> {
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
    None,
}

impl Statement {
    pub fn eval(self, vars: &mut Variables) {
        use self::Statement::*;
        match self {
            Definition(d) => d.eval(vars),
            None => {},
        };
    }
}

#[derive(Clone, Debug)]
struct SegmentBase {
    start: math::Point,
    end: math::Point,
    min_offset: math::Scalar,
    max_offset: math::Scalar,
}

#[derive(Clone, Debug)]
pub struct Variables<'a> {
    scalars: HashMap<SIdent, math::Scalar>,
    points: HashMap<PIdent, math::Point>,
    lines: HashMap<LIdent, math::Line>,
    routes: HashMap<RIdent, route::Route>,
    segments: HashMap<route::Segment, SegmentBase>,
    scalar_macros: HashMap<SIdent, Macro<Scalar>>,
    point_macros: HashMap<PIdent, Macro<Point>>,
    line_macros: HashMap<LIdent, Macro<Line>>,
    globals: Option<&'a Variables<'a>>,
}

macro_rules! get_typed {
    ($f:ident, $m:ident, $id_t:ty, $out_t:ty) => {
        fn $f(&self, id: &$id_t) -> Option<&$out_t> {
            self.$m.get(&id).or_else(
                || self.globals.as_ref().and_then(|g| g.$f(id)))
        }
    }
}

macro_rules! insert_typed {
    ($f:ident, $m:ident, $id_t:ty, $expr_t:ty) => {
        fn $f(&mut self, id: $id_t, val: &$expr_t) {
                let val = val.eval(self);
                self.$m.insert(id, val);
        }
    }
}

impl<'a> Variables<'a> {
    pub fn new() -> Variables<'static> {
        Variables {
            scalars: HashMap::new(),
            points: HashMap::new(),
            lines: HashMap::new(),
            routes: HashMap::new(),
            segments: HashMap::new(),
            scalar_macros: HashMap::new(),
            point_macros: HashMap::new(),
            line_macros: HashMap::new(),
            globals: None,
        }
    }

    pub fn with_globals(globals: &'a Variables) -> Variables<'a> {
        Variables {
            globals: Some(globals),
            ..Variables::new()
        }
    }

    pub fn get(&self, id: &Ident) -> Option<math::Expr> {
        match *id {
            Ident::Scalar(ref id) => self.get_scalar(&id).map(|x| math::Expr::Scalar(*x)),
            Ident::Point(ref id) => self.get_point(&id).map(|x| math::Expr::Point(*x)),
            Ident::Line(ref id) => self.get_line(&id).map(|x| math::Expr::Line(*x)),
        }
    }

    get_typed!(get_scalar, scalars, SIdent, math::Scalar);
    get_typed!(get_point, points, PIdent, math::Point);
    get_typed!(get_line, lines, LIdent, math::Line);
    get_typed!(get_scalar_macro, scalar_macros, SIdent, Macro<Scalar>);
    get_typed!(get_point_macro, point_macros, PIdent, Macro<Point>);
    get_typed!(get_line_macro, line_macros, LIdent, Macro<Line>);

    insert_typed!(insert_scalar, scalars, SIdent, Scalar);
    insert_typed!(insert_point, points, PIdent, Point);
    insert_typed!(insert_line, lines, LIdent, Line);

    fn insert_route(&mut self, id: RIdent, route: Route) {
        let route = route.eval(self);
        self.routes.insert(id, route);
    }

    fn insert_segment(&mut self, seg: route::Segment, off: math::Scalar) {
        if let Some(ref mut base) = self.segments.get_mut(&seg) {
            if base.min_offset > off {
                base.min_offset = off;
            }
            if base.max_offset < off {
                base.max_offset = off;
            }
            return;
        }
        self.segments.insert(seg, SegmentBase {
            start: seg.start,
            end: seg.end,
            min_offset: off,
            max_offset: off,
        });
    }

    pub fn eval_def(&mut self, def: Definition) {
        use self::Definition::*;
        match def {
            Scalar(id, val) => {
                self.insert_scalar(id, &val);
            },
            Point(id, val) => {
                self.insert_point(id, &val);
            },
            Line(id, val) => {
                self.insert_line(id, &val);
            },
            Route(id, val) => {
                self.insert_route(id, val);
            },
            ScalarMacro(id, args, body) => {
                self.scalar_macros.insert(id.clone(), Macro{id, args, body});
            },
            PointMacro(id, args, body) => {
                self.point_macros.insert(id.clone(), Macro{id, args, body});
            },
            LineMacro(id, args, body) => {
                self.line_macros.insert(id.clone(), Macro{id, args, body});
            },
        }
    }
}
