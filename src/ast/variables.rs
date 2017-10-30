use std::collections::HashMap;

use math;
use route;

use super::{Definition, Ident, SIdent, PIdent, LIdent, RIdent, Eval, Scalar, Point, Line, Route, Macro};

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
        pub fn $f(&self, id: &$id_t) -> Option<&$out_t> {
            self.$m.get(&id).or_else(
                || self.globals.as_ref().and_then(|g| g.$f(id)))
        }
    }
}

macro_rules! insert_typed {
    ($f:ident, $m:ident, $id_t:ty, $expr_t:ty) => {
        pub fn $f(&mut self, id: $id_t, val: &$expr_t) {
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

    pub fn insert_route(&mut self, id: RIdent, route: Route) {
        let route = route.eval(self);
        self.routes.insert(id, route);
    }

    pub fn insert_segment(&mut self, seg: route::Segment, off: math::Scalar) {
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

#[derive(Clone, Debug)]
struct SegmentBase {
    start: math::Point,
    end: math::Point,
    min_offset: math::Scalar,
    max_offset: math::Scalar,
}
