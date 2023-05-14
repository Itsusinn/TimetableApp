use serde::{Deserialize, Serialize};

pub mod util;

uniffi::include_scaffolding!("timetable");

pub fn add(left: i32, right: i32) -> i32 {
  left + right
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Course {
  credit: i32,
  kind: String,
  name: String,
  time: String,
  place: String,
}

#[derive(Debug, PartialEq)]
pub struct CourseDetail {
  credit: i32,
  kind: String,
  name: String,
  time: String,
  place: String,
}
impl From<Course> for CourseDetail {
  fn from(value: Course) -> Self {
    Self {
      credit: value.credit,
      kind: value.kind,
      name: value.name,
      time: value.time,
      place: value.place
    }
  }
}
