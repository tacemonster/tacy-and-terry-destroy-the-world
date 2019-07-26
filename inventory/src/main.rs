#![allow(non_snake_case)]
use std::io::Read;
extern crate serde;
use serde::{Deserialize, Serialize};


//extern crate rustc_serialize;
//use rustc_serialize::json::Json;
//use serde_json::{Deserializer, Value};
//use serde_json::Result;

/*
#[derive(Serialize, Deserialize)]
struct Data {
Response: Response,
ErrorCode: usize,
ThrottleSeconds: usize,
ErrorStatus: String,
Message: String,
MessageData: String
}
 */

#[derive(Serialize, Deserialize)]
struct User {
            iconPath: String,
            membershipType: usize,
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


fn get_api_key() -> String {
  let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
  let mut key = String::new();
  f.read_to_string(&mut key).expect("could not read api key");
  key.trim().to_string()
}


fn main() {
  let api_key = get_api_key();
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
  let platform = '2';
  let player_name = String::from("cortical_iv");
  let base_url = String::from("https://www.bungie.net/Platform/Destiny2/");

  let mut search_url = base_url.clone();
  search_url.push_str("SearchDestinyPlayer/");
  search_url.push(platform);
  search_url.push('/');
  search_url.push_str(&player_name);
  search_url.push('/');

  let mut response = reqwest::Client::new()
    .get(&search_url)
    .header("X-API-KEY", api_key.clone())
    .send()
    .expect("Failed to send request");
  let mut buf = String::new();
  response.read_to_string(&mut buf).expect("Failed to read response");
  //let user = unwrap_initial_response(buf);
  let user = unwrap_response(buf, 1);
  let u: User= deserialize_user(user.first().expect("Failed to parse message correctly").to_string());
  let equipment = get_gear(&api_key, base_url, platform, u.membershipId, request_type);
  //println!("{}",equipment);
  let response = unwrap_response(equipment, 5);
  /*
  for val in response {
    println!("{:?}", val);
  }
  */
  let mut items = Vec::new();
  for val in response {
    items.push(deserialize_item(val));
  }
  for val in items {
    println!("{:?}", val);
  }
}

fn get_gear(api_key:&String, base_url:String, platform: char, membershipId:String, request_type:String) -> String {
  let api_key = get_api_key();
  let mut url = base_url;
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");

  url.push(platform);
  url.push_str("/Profile/");
  url.push_str(&membershipId);
  url.push_str("/?components=");
  url.push_str(&request_type);
  //let mut url = String::from("https://www.bungie.net/Platform/Destiny2/2/Profile/4611686018459314819/?components=205");

  let mut response = reqwest::Client::new()
    .get(&url)
    .header("X-API-KEY", api_key)
    .send()
    .expect("Failed to send request");
  let mut buf = String::new();
  response.read_to_string(&mut buf).expect("Failed to read response");
  //println!("{}", buf);
  buf
}

fn deserialize_user(ref mut buf:String) -> User {
  let u: User = serde_json::from_str(buf).expect("Failed to deserialize user info");
  u
}
fn deserialize_item(ref mut buf:String) -> Item {
  let i: Item = serde_json::from_str(buf).expect("Failed to deserialize item");
  i
}

fn unwrap_response(source:String, depth:usize) -> Vec<String> {
  let mut results = Vec::new();
  let mut starts = Vec::new();
  let mut ends = Vec::new();
  let mut open = 0;
  let mut close = 0;
  let chars = source.as_str().char_indices();
  for guy in chars {
    if guy.1 == '{' {
      open += 1;
      if open == close + depth + 1 {    
        starts.push(guy.0);
      }
    }
    else if guy.1 == '}' {
      close += 1;
      if open == close + depth {
        ends.push(guy.0);
        open -= 1;
        close -= 1;
      }
    }
  }
  let sections = starts.len();
  for index in 0..sections {
    results.push(source[starts[index]..ends[index]+ 1].to_string());
  }
  results
}

/*
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
fn unwrap_initial_response(source:String) -> String {
  let start_index = source.find("[").expect("failed to find open");
  let end_index = source.find("]").expect("failed to find close");
  let result = &source[(start_index + 1)..end_index];
  result.to_string()
}

 */
