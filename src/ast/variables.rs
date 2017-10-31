use std::collections::HashMap;
use command::Command;

use math;
use route;

use super::{Definition, Ident, SIdent, PIdent, LIdent, RIdent, Eval, Scalar, Point, Line, Route, Macro};
use super::errors::Error;

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
    commands: Vec<Command>,
    globals: Option<&'a Variables<'a>>,
    pub r_sep: f64,
    pub r_base: f64,
}

macro_rules! get_typed {
    ($f:ident, $m:ident, $id_t:ty, $out_t:ty) => {
        pub fn $f(&self, id: &$id_t) -> Result<&$out_t, Box<Error>> {
            self.$m.get(&id).map_or_else(
                || self.globals.as_ref().map_or(
                    Err(Box::new(Error::undefined(format!("{}", id).as_ref()))),
                    |g| g.$f(id)),
                |val| Ok(val))
        }
    }
}

macro_rules! insert_typed {
    ($f:ident, $m:ident, $id_t:ty, $expr_t:ty) => {
        pub fn $f(&mut self, id: $id_t, val: &$expr_t) -> Result<(), Box<Error>> {
                let val = val.eval(self)?;
                self.$m.insert(id, val);
                Ok(())
        }
    }
}

impl<'a> Variables<'a> {
    pub fn new(r_sep: f64, r_base: f64) -> Variables<'static> {
        Variables {
            scalars: HashMap::new(),
            points: HashMap::new(),
            lines: HashMap::new(),
            routes: HashMap::new(),
            segments: HashMap::new(),
            scalar_macros: HashMap::new(),
            point_macros: HashMap::new(),
            line_macros: HashMap::new(),
            commands: Vec::new(),
            globals: None,
            r_sep,
            r_base,
        }
    }

    pub fn with_globals(globals: &'a Variables) -> Variables<'a> {
        Variables {
            globals: Some(globals),
            ..Variables::new(0.0, 0.0)
        }
    }

    pub fn get(&self, id: &Ident) -> Result<math::Expr, Box<Error>> {
        match *id {
            Ident::Scalar(ref id) => self.get_scalar(&id).map(|x| math::Expr::Scalar(*x)),
            Ident::Point(ref id) => self.get_point(&id).map(|x| math::Expr::Point(*x)),
            Ident::Line(ref id) => self.get_line(&id).map(|x| math::Expr::Line(*x)),
        }
    }

    get_typed!(get_scalar, scalars, SIdent, math::Scalar);
    get_typed!(get_point, points, PIdent, math::Point);
    get_typed!(get_line, lines, LIdent, math::Line);
    get_typed!(get_route, routes, RIdent, route::Route);
    get_typed!(get_scalar_macro, scalar_macros, SIdent, Macro<Scalar>);
    get_typed!(get_point_macro, point_macros, PIdent, Macro<Point>);
    get_typed!(get_line_macro, line_macros, LIdent, Macro<Line>);

    insert_typed!(insert_scalar, scalars, SIdent, Scalar);
    insert_typed!(insert_point, points, PIdent, Point);
    insert_typed!(insert_line, lines, LIdent, Line);

    pub fn insert_route(&mut self, id: RIdent, route: Route) -> Result<(), Box<Error>> {
        let route = route.eval(self)?;
        self.routes.insert(id, route);
        Ok(())
    }

    fn get_segment(&self, seg: &route::Segment) -> Option<&SegmentBase> {
        self.segments.get(&seg).or(self.segments.get(&seg.reverse()))
    }

    fn get_segment_mut(&mut self, seg: &route::Segment) -> Option<&mut SegmentBase> {
        if self.segments.contains_key(seg) {
            self.segments.get_mut(seg)
        } else {
            self.segments.get_mut(&seg.reverse())
        }
    }

    pub fn insert_segment(&mut self, seg: route::Segment, off: math::Scalar) {
        if let Some(ref mut base) = self.get_segment_mut(&seg) {
            base.min_offset = base.min_offset.min(off);
            base.max_offset = base.max_offset.max(off);
            return;
        }
        self.segments.insert(seg, SegmentBase {
            start: seg.start,
            end: seg.end,
            min_offset: off,
            max_offset: off,
        });
    }

    pub fn min_offset(&self, seg: route::Segment) -> Option<math::Scalar> {
        self.get_segment(&seg).map(|base| base.min_offset)
    }

    pub fn max_offset(&self, seg: route::Segment) -> Option<math::Scalar> {
        self.get_segment(&seg).map(|base| base.max_offset)
    }

    pub fn push_command(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    pub fn eval_def(&mut self, def: Definition) -> Result<(), Box<Error>> {
        use self::Definition::*;
        match def {
            Scalar(id, val) => {
                self.insert_scalar(id, &val)
            },
            Point(id, val) => {
                self.insert_point(id, &val)
            },
            Line(id, val) => {
                self.insert_line(id, &val)
            },
            Route(id, val) => {
                self.insert_route(id, val)
            },
            ScalarMacro(id, args, body) => {
                self.scalar_macros.insert(id.clone(), Macro{id, args, body});
                Ok(())
            },
            PointMacro(id, args, body) => {
                self.point_macros.insert(id.clone(), Macro{id, args, body});
                Ok(())
            },
            LineMacro(id, args, body) => {
                self.line_macros.insert(id.clone(), Macro{id, args, body});
                Ok(())
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
