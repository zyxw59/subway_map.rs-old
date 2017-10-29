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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}
