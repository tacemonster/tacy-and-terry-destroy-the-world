use std::io::Read;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct User {
  iconPath: String,
  membershipType: u32,
  membershipId: String,
  displayName: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserResponse {
  Response: User,
  ErrorCode: u32,
  ThrottleSeconds: u32,
  ErrorStatus: String,
  Message: String,
  MessageData: Vec<String>,
}
/*
{
  "Response":[{
    "iconPath":"/img/theme/destiny/icons/icon_psn.png",
    "membershipType":2,
    "membershipId":"4611686018459314819",
    "displayName":"cortical_iv"
    }],
  "ErrorCode":1,
  "ThrottleSeconds":0,
  "ErrorStatus":"Success",
  "Message":"Ok",
  "MessageData":{}
}
*/

fn get_api_key() -> String {
    let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
    let mut key = String::new();
    f.read_to_string(&mut key).expect("could not read api key");
    key.trim().to_string()
}
/*
fn make_literal(mut buf:String) -> String {
  let mut result = String::new();
  let mut chars = buf.as_str().char_indices();
  for val in chars {
    if val.1 == '\\' {
      continue;
    } else {
      result.push(val.1);
    }
  }
  result
}
*/
fn parse(ref mut buf:std::string::String) -> Result <()> {
    let u: UserResponse = serde_json::from_str(buf)?;
    //response
    println!("{:?}", u);
    Ok(())
}

fn get_json(source:String) -> String {
  let start_index = source.find("[").expect("failed to find open");
  let end_index = source.find("]").expect("failed to find close");
  let mut result = &source[(start_index + 1)..end_index];
  result.to_string()
}

  //let json: UserResponse = reqwest::get("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/2/cortical_iv/").header("X-API-KEY", api_key)?.json()?;
  //json

fn main() {
    let api_key = get_api_key();
  //  let json = get_json(api_key);
//}

    let mut response = reqwest::Client::new()
        .get("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/2/cortical_iv/")
        .header("X-API-KEY", api_key)
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);
    let user = get_json(buf);
    println!("{}", user);
    let u: User = serde_json::from_str(&user).expect("failed to parse response");
    println!("gamertag : {}", u.membershipId);
    //let parsed = buf.parse::<String>().expect("shit why");
    //println!("{}", parsed);
    //let literal = make_literal(buf);
    //parse(buf).expect("Failed to parse!");;
    //let u: UserResponse = serde_json::from_str(&parsed).expect("failed to parse response");
    //println!("{:?}", u);
    //println!("{}", literal);
}
