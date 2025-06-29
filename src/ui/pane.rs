use super::{component::Component, types::Container};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneId {
    Upcoming,
    Calendar,
    Insights,
    Courses,
}

pub struct Pane {
    pub id: PaneId,
    pub container: Container,
}

impl Pane {
    pub fn new(id: PaneId, component: impl Component + 'static) -> Self {
        Self {
            id,
            container: Box::new(component),
        }
    }
}
