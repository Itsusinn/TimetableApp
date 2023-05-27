use std::fmt::Debug;

use android_logger::FilterBuilder;
use color_eyre::{eyre, eyre::Result};
use log::{info, trace, LevelFilter};
use serde::{Deserialize, Serialize};

extern crate android_logger;
extern crate log;

pub mod util;

#[uniffi::export]
pub fn init_logger() {
  android_logger::init_once(
    android_logger::Config::default()
      .with_max_level(LevelFilter::Trace)
      .with_filter(
        // configure messages for specific crate
        FilterBuilder::new().parse("trace,timetable=trace").build(),
      ),
  );
  trace!("Init android logger")
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn get_data() -> Vec<CourseDetail> {
  util::test().await.unwrap()
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Course {
  credit: i32,
  kind: String,
  name: String,
  time: String,
  place: String,
}

#[derive(Debug, PartialEq, uniffi::Record)]
pub struct CourseDetail {
  credit: i32,
  kind: String,
  name: String,
  // week
  time_1: u32,
  // some day of week
  time_2: u32,
  // time start
  time_3: u32,
  // time end
  time_4: u32,
  place: String,
}

impl TryFrom<Course> for CourseDetail {
  type Error = color_eyre::eyre::Report;

  fn try_from(value: Course) -> Result<Self> {
    let mut time = value.time.split(" ");

    let time_1 = time
      .next()
      .and_then(|v| {
        let pos = v.find("周")?;
        let time: String = v.chars().skip(1).take(pos - 3).collect::<_>();
        time.parse::<u32>().ok()
      })
      .ok_or(eyre::eyre!("Parse Error: 第X周"))?;

    let time_2 = time
      .next()
      .and_then(|v| {
        let time: String = v.chars().skip(2).take(1).collect::<_>();

        match time.as_str() {
          "一" => Some(1),
          "二" => Some(2),
          "三" => Some(3),
          "四" => Some(4),
          "五" => Some(5),
          "六" => Some(6),
          "七" => Some(7),
          _ => None,
        }
      })
      .ok_or(eyre::eyre!("Parse Error: 星期X"))?;

    let time_3_4 = time
      .next()
      .and_then(|v| {
        let pos1 = v.find("-")?;
        let time1: String = v.chars().skip(1).take(pos1 - 1).collect::<_>();
        let pos2 = v.find("]")?;
        let time2: String = v
          .chars()
          .skip(pos1 + 1)
          .take(pos2 - pos1 - 1)
          .collect::<_>();
        Some((time1.parse::<u32>().ok()?, time2.parse::<u32>().ok()?))
      })
      .ok_or(eyre::eyre!("Parse Error: [X-Y]节"))?;

    Ok(Self {
      credit: value.credit,
      kind: value.kind,
      name: value.name,
      place: value.place,
      time_1,
      time_2,
      time_3: time_3_4.0,
      time_4: time_3_4.1,
    })
  }
}

uniffi::include_scaffolding!("timetable");
