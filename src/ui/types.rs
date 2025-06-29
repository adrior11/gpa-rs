use std::{cell::RefCell, rc::Rc};

use crate::model::Gpa;

use super::component::Component;

pub type Model = Rc<RefCell<Gpa>>;
pub type Container = Box<dyn Component>;
