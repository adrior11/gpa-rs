use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Mark {
    Empty,
    Done,
    Skipped,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProgressRow {
    pub label: String,
    pub marks: Vec<Mark>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Progress {
    pub start: Date,
    pub weeks: u16,
    // NOTE: needs indicator on which weekday the week starts
    pub rows: Vec<ProgressRow>,
}
