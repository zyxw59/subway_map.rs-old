use std::ops::{Add, Sub, Mul, Div, Neg};
use std::hash::{Hash, Hasher};
use ordered_float::OrderedFloat;

#[derive(Clone, Copy, Debug)]
pub enum Expr {
    Scalar(Scalar),
    Point(Point),
    Line(Line),
}

pub type Scalar = f64;

#[derive(Clone, Copy, Debug)]
pub struct Point(pub Scalar, pub Scalar);

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

    pub fn parallel(self, p: Point) -> Line {
        Line {
            origin: p,
            vector: self.vector,
        }
    }

    pub fn perpendicular(self, p: Point) -> Line {
        Line {
            origin: p,
            vector: Point(self.vector.1, -self.vector.0),
        }
    }
}
