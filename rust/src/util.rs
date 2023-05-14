use color_eyre::{eyre, eyre::Result};


pub fn encode_inp(input: &str) -> Result<String> {
  let mut output = String::new();
  let err = || eyre::eyre!("Encode Error");
  const KEY_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
  let mut num_val = 0;
  loop {
    let chr1 = input.chars().nth(num_val).ok_or(err())? as u16;
    num_val += 1;
    let chr2 = input.chars().nth(num_val);
    num_val += 1;
    let chr3 = input.chars().nth(num_val);
    num_val += 1;
    let enc1 = chr1 >> 2;
    let enc2: u16;
    let enc3: u16;
    let enc4: u16;
    if chr2.is_none() {
      enc2 = (chr1 & 3) << 4;
      enc3 = 64;
      enc4 = 64;
    } else if chr3.is_none() {
      let chr2 = chr2.unwrap() as u16;
      enc2 = (chr1 & 3) << 4 | chr2 >> 4;
      enc3 = (chr2 & 15) << 2;
      enc4 = 64;
    } else {
      let chr2 = chr2.unwrap() as u16;
      let chr3 = chr3.unwrap() as u16;
      enc2 = (chr1 & 3) << 4 | chr2 >> 4;
      enc3 = (chr2 & 15) << 2 | chr3 >> 6;
      enc4 = chr3 & 63;
    }
    output += &KEY_STR.chars().nth(enc1 as usize).ok_or(err())?.to_string();
    output += &KEY_STR.chars().nth(enc2 as usize).ok_or(err())?.to_string();
    output += &KEY_STR.chars().nth(enc3 as usize).ok_or(err())?.to_string();
    output += &KEY_STR.chars().nth(enc4 as usize).ok_or(err())?.to_string();
    if num_val >= input.len() {
      break;
    }
  }
  Ok(output)
}

#[tokio::test]
pub async fn test() -> Result<()> {
  use crate::{Course, CourseDetail};
  use std::collections::HashMap;

  std::env::set_var("HTTP_PROXY", "http://127.0.0.1:8080");
  let base_url = "http://jwxt.wit.edu.cn/jsxsd/";
  let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
  client.get(base_url).send().await?;

  let account = "2006******";
  let password = "*********";

  let encoded = format!("{}%%%{}", encode_inp(account)?, encode_inp(password)?);
  let mut params = HashMap::new();
  params.insert("userAccount", account);
  params.insert("encoded", &encoded);
  let _ = client
    .post(format!("{base_url}xk/LoginToXk"))
    .form(&params)
    .send()
    .await?;
  // TODO status check

  let mut params = HashMap::new();
  params.insert("rq", "2023-05-14");
  #[rustfmt::skip]
  let kb = client
    .post(format!("{base_url}framework/main_index_loadkb.jsp"))
    .form(&params)
    .send().await?
    .text().await?
    .replace(
      "style='text-align: left;font-size: 12px;font-weight: bold;null'",
      "class='kb.css'",
    );
  let dom = tl::parse(&kb, tl::ParserOptions::default()).unwrap();
  let parser = dom.parser();
  let courses: Vec<CourseDetail> = dom
    .get_elements_by_class_name("kb.css")
    .map(|node| {
      node
        .get(parser)
        .and_then(|node| node.as_tag())
        .and_then(|tag| tag.attributes().get("title"))
        .iter()
        .flatten()
        .next()
        .and_then(|raw| {
          let str = raw
            .as_utf8_str()
            .to_string()
            .replace("：", "=")
            .replace("<br/>", "&")
            .replacen("课程学分", "credit", 1)
            .replacen("课程属性", "kind", 1)
            .replacen("课程名称", "name", 1)
            .replacen("上课时间", "time", 1)
            .replacen("上课地点", "place", 1);
          serde_qs::from_str::<Course>(&str).ok()
        })
    })
    .flatten()
    .map(|course|course.into())
    .collect();
  println!("{:#?}", courses);
  Ok(())
}
