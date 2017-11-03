use std::fmt;
use std::cmp::{Ordering};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::hash::{Hash, Hasher};
pub use std::f64::consts::PI;
use ordered_float::OrderedFloat;

pub const TAU: f64 = PI * 2.0;

#[derive(Clone, Copy, Debug)]
pub enum Expr {
    Scalar(Scalar),
    Point(Point),
    Line(Line),
}

pub type Scalar = f64;

#[derive(Clone, Copy, Debug)]
pub struct Point(pub Scalar, pub Scalar);

impl Point {
    pub fn abs(self) -> Scalar {
        self.0.hypot(self.1)
    }

    pub fn unit(self) -> Point {
        self / self.abs()
    }

    pub fn perp(self) -> Point {
        Point(self.1, -self.0)
    }

    pub fn basis(self, vector: Point, u: Scalar, v: Scalar) -> Point {
        let Point(c, s) = vector.unit();
        self + Point(u * c + v * s, u * s - v * c)
    }

    pub fn angle(self, other: Point) -> Scalar {
        let dot = self.0 * other.0 + self.1 * other.1;
        let cross = self.0 * other.1 - self.1 * other.0;
        mod_tau(-cross.atan2(dot))
    }

    pub fn cmp(self, other: Point) -> (Ordering, Ordering) {
        let x = OrderedFloat(self.0).cmp(&OrderedFloat(other.0));
        let y = OrderedFloat(self.1).cmp(&OrderedFloat(other.1));
        (x, y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        OrderedFloat(self.0) == OrderedFloat(rhs.0) && OrderedFloat(self.1) == OrderedFloat(rhs.1)
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        OrderedFloat(self.0).hash(state);
        OrderedFloat(self.1).hash(state);
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, rhs: Scalar) -> Point {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<Scalar> for Point {
    type Output = Point;

    fn div(self, rhs: Scalar) -> Point {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point(-self.0, -self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub origin: Point,
    pub vector: Point,
}

impl Add<Point> for Line {
    type Output = Line;

    fn add(self, rhs: Point) -> Line {
        Line {
            origin: self.origin + rhs,
            vector: self.vector,
        }
    }
}

impl Line {
    pub fn intersect(self, other: Line) -> Point {
        let Point(x2, y2) = self.origin - self.vector;
        let Point(x4, y4) = other.origin - other.vector;
        let Point(dx1, dy1) = self.vector;
        let Point(dx2, dy2) = other.vector;
        let det = dx1*dy2 - dx2*dy1;
        let det1 = self.origin.0*y2 - self.origin.1*x2;
        let det2 = other.origin.0*y4 - other.origin.1*x4;
        Point(det1*dx2 - dx1*det2, det1*dy2 - dy1*det2) / det
    }

    pub fn offset(self, offset: Scalar) -> Line {
        Line {
            origin: self.origin.basis(self.vector, 0.0, offset),
            vector: self.vector,
        }
    }

    pub fn parallel(self, p: Point) -> Line {
        Line {
            origin: p,
            vector: self.vector,
        }
    }

    pub fn perpendicular(self, p: Point) -> Line {
        Line {
            origin: p,
            vector: self.vector.perp(),
        }
    }

    pub fn between(a: Point, b: Point) -> Line {
        Line {
            origin: a,
            vector: b - a,
        }
    }
}

pub fn mod_tau(theta: Scalar) -> Scalar {
    let theta = theta % TAU;
    theta + if theta < 0.0 { TAU } else { 0.0 }
}

