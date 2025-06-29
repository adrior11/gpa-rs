use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Semester {
    pub id: u8,
    pub start: Date,
    pub end: Date,
}
