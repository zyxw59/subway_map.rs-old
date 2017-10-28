use math::{Point, Scalar};

#[derive(Clone, Debug)]
pub struct Route<'a> {
    segments: Vec<&'a Segment>,
    offsets: Vec<&'a Scalar>,
}

impl<'a> Route<'a> {
}

#[derive(Clone, Debug)]
pub struct Segment {
    start: Point,
    end: Point,
    min_offset: Scalar,
    max_offset: Scalar,
}
