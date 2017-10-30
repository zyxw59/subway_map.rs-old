use route;

#[derive(Clone, Debug)]
pub enum Command {
    Group(Vec<Command>, String),
    Route(route::Route, String),
}
