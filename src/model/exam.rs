use serde::{Deserialize, Serialize};
use time::{Date, Time};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExamKind {
    Written,
    Oral,
    Project,
    Final,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Exam {
    pub kind: ExamKind,
    pub date: Date,
    pub time: Option<Time>,
    pub weight: Option<f32>,
    pub locaation: Option<String>,
    pub grade: Option<f32>,
}
