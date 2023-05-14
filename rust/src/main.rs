use std::collections::HashMap;
use color_eyre::eyre::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
  // std::env::set_var("HTTP_PROXY", "http://127.0.0.1:8080");
  let base_url = "http://jwxt.wit.edu.cn/";
  let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
  client.get(base_url).send().await?;
  let sess = client.post(format!("{base_url}Logon.do?method=logon&flag=sess")).send().await?.text().await?;
  let mut sess = sess.split("#");
  let mut scode = sess.next().unwrap();
  let sxh = sess.next().unwrap();

  let account = "2006*****";
  let password = "*******";
  let code = format!("{account}%%%{password}");
  let mut encoded = String::new();
  let mut index = 0;
  for char in code.chars() {
    if index >= code.len() {
      break;
    }
    if index < 20 {
      encoded += char.to_string().as_str();
      let scode_num = sxh.chars().nth(index).unwrap().to_string().parse::<usize>()?;
      encoded += &scode[..scode_num];
      scode = &scode[scode_num..];
    } else {
      encoded += &code[index..];
      index = code.len();
    }
    index += 1;
  }

  let mut params = HashMap::new();
    params.insert("userAccount", "");
    params.insert("userPassword", "");
    params.insert("encoded", encoded.as_str());

  client.post(format!("{base_url}Logon.do?method=logon")).form(&params).send().await?.text().await?;
  client.get(format!("{base_url}framework/main.jsp")).send().await?.text().await?;
  Ok(())
}