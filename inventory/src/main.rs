use std::io::Read;
//extern crate rustc_serialize;
//use rustc_serialize::json::Json;
extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Value};
use serde_json::Result;

fn get_api_key() -> String {
  let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
  let mut key = String::new();
  f.read_to_string(&mut key).expect("could not read api key");
  key.trim().to_string()
}

#[derive(Serialize, Deserialize)]
struct Data {
            Response: Response,
            ErrorCode: usize,
            ThrottleSeconds: usize,
            ErrorStatus: String,
            Message: String,
            MessageData: String
}

#[derive(Serialize, Deserialize)]
struct Response { 
            iconPath: String,
            membershipType: String,
            membershipId: String,
            displayName: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
  itemHash: usize,
  itemInstanceId: String,
  quantity: usize,
  bindStatus: usize,
  location: usize,
  bucketHash: usize,
  transferStatus: usize,
  lockable: bool,
  state: usize,
  dismantlePermission: usize,
  isWrapper: bool
}


fn deserialize_json(ref mut buf:std::string::String) -> Item {
    let i: Item = serde_json::from_str(buf).expect("Failed to deserialize item");
    i
}

//fn get_json(source:String, start:str, end:str) -> String {
fn get_json(source:String) -> String {
  let start_index = source.find("2305843009260353573").expect("failed to find open");
  let end_index = source.find("2305843009260353575").expect("failed to find close");
  let mut result = &source[(start_index + 31)..(end_index - 4)];
  result.to_string()
}

fn split_inventory(mut source:String) -> Vec<String> {
  let mut results = Vec::new();
    while !source.is_empty() {
      let start = source.find("{").expect("Failed to find start of item");
      let end = source.find("}").expect("Failed to find end of item");
      let item = &source[start..(end+1)];
      if !item.is_empty() {
        results.push(item.to_string());
      }
      source = source[(end+1)..].to_string();
    }
  results
}

fn main() {
  let api_key = get_api_key();
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
  let platform = '2';
  let player_name = String::from("cortical_iv");
  let mut url = String::from("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/");


  url.push(platform);
  url.push('/');
  url.push_str(&player_name);
  url.push('/');


  let mut response = reqwest::Client::new()
    .get(&url)
    .header("X-API-KEY", api_key)
    .send()
    .expect("Failed to send request");
  let mut buf = String::new();
  /*example responce {
    "Response":[
    {
    "iconPath":"/img/theme/destiny/icons/icon_psn.png",
    "membershipType":2,
    "membershipId":"4611686018459314819",
    "displayName":"cortical_iv"
    }
    ],
    "ErrorCode":1,
    "ThrottleSeconds":0,
    "ErrorStatus":"Success",
    "Message":"Ok",
    "MessageData":{}
    }
   */
  response.read_to_string(&mut buf).expect("Failed to read response");
  //let info: Data = serde_json::from_str(&buf);
  //println!("{}",info["Message"]);

  let membershipId = String::from("4611686018459314819");

  let equipment = get_gear(membershipId);
}

fn get_gear(membershipId:String) -> String {
  let api_key = get_api_key();
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
  let platform = '2';
  let player_name = String::from("cortical_iv");
  let mut url = String::from("https://www.bungie.net/Platform/Destiny2/2/Profile/4611686018459314819/?components=205");

  /*
     url.push(platform);
     url.push('/');
     url.push_str(&player_name);
     url.push('/');
   */

  let mut response = reqwest::Client::new()
    .get(&url)
    .header("X-API-KEY", api_key)
    .send()
    .expect("Failed to send request");
  let mut buf = String::new();
  response.read_to_string(&mut buf).expect("Failed to read response");
  println!("{}", buf);
  let contents = get_json(buf); //, "2305843009260353573", "2305843009260353575");
//  let stream = Deserializer::from_str(&contents).into_iter::<Value>();
  let stream = split_inventory(contents);
  for value in stream {
    println!("{:?}", deserialize_json(value));
  }
  //println!("{}", buf);
  //buf
  "success".to_string()
}
