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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct User { 
iconPath: String,
            membershipType: usize,
            membershipId: String,
            displayName: String,
}

#[allow(non_snake_case)]
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



fn main() {
  let api_key = get_api_key();
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
  let platform = '2';
  let player_name = String::from("cortical_iv");
  let base_url = String::from("https://www.bungie.net/Platform/Destiny2/");

 let member_info = get_member_id(&api_key, &base_url, platform, player_name, &request_type);
 println!("{}", member_info);
  let user = unwrap_initial_response(member_info);
  let u: User= deserialize_user(user);
  let equipment = get_gear(&api_key, &base_url, platform, u.membershipId, request_type);
  //println!("{}",equipment);
  let response = unwrap_response_sections(equipment);
  /*
  for val in response{
    println!("{}", val);
  }
  */
  let mut sections = Vec::new();
  for val in response {
    let holder = unwrap_response_sections(val);
    for item in holder {
      sections.push(item);
    }
  }
  /*
  println!("{}", sections.len());
  for val in sections {
    println!("{:?}", val);
  }
  */
  let mut categories = Vec::new();
  for val in sections {
    let holder = unwrap_response_sections(val);
    for item in holder {
      categories.push(item);
    }
  }
  /*
  for val in categories {
    println!("{:?}", val);
  }
  */
  let mut items = Vec::new();
  for val in categories {
    let holder = unwrap_response_sections(val);
    for item in holder {
      items.push(item);
    }
  }
  /*
  for val in items {
    println!("{:?}", val);
  }
  */
  //TODO if we need to keep them separated by the groups they came in, this is where that needs to happen.
  let mut individuals = Vec::new();
  for val in items {
    let holder = unwrap_response_sections(val);
    for item in holder {
      individuals.push(item);
    }
  }
  /*
  for val in individuals {
    println!("{:?}", val);
  }
  */
  let mut all_items = Vec::new();
  for val in individuals {
    all_items.push(deserialize_item(val));
  }
  for val in all_items {
    println!("{:?}", val);
  }

}

fn get_api_key() -> String {
  let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
  let mut key = String::new();
  f.read_to_string(&mut key).expect("Could not read api key, check that api-key.txt has correct API key!");
  key.trim().to_string()
}

fn get_member_id(api_key: &String, base_url: &String, platform: char, player_name: String, request_type: &String) -> String {
  let mut search_url = base_url.clone();
  search_url.push_str("SearchDestinyPlayer/");
  search_url.push(platform);
  search_url.push('/');
  search_url.push_str(&player_name);
  search_url.push('/');

  let mut response = reqwest::Client::new()
    .get(base_url)
    .header("X-API-KEY", api_key.clone())
    .send()
    .expect("Failed to send request");
  let mut buf = String::new();
  response.read_to_string(&mut buf).expect("Failed to read response");
  if buf == "" {
	  	String::from("Characters not found")
	  } else {
		unwrap_initial_response(buf)
	  }
}
fn get_gear(api_key:&String, base_url: &String, platform: char, membershipId: String, request_type:String) -> String {
  let api_key = get_api_key();
  let mut url = base_url.clone();
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

fn unwrap_initial_response(source:String) -> String {
  let start_index = source.find("[").expect("failed to find open");
  let end_index = source.find("]").expect("failed to find close");
  let result = &source[(start_index + 1)..end_index];
  result.to_string()
}

fn deserialize_user(ref mut buf:String) -> User {
  let u: User = serde_json::from_str(buf).expect("Failed to deserialize user info");
  u
}
fn deserialize_item(ref mut buf:String) -> Item {
  let i: Item = serde_json::from_str(buf).expect("Failed to deserialize item");
  i
}

fn unwrap_response_sections(source:String) -> Vec<String> {
  let mut results = Vec::new();
  let mut starts = Vec::new();
  let mut ends = Vec::new();
  let mut open = 0;
  let mut close = 0;
  let chars = source.as_str().char_indices();
  for guy in chars {
    if guy.1 == '{' {
      open += 1;
      if open == close + 2 {    //ignore first one to unwrap!
        starts.push(guy.0);
      }
    }
    else if guy.1 == '}' {
      close += 1;
      if open == close + 1 {
        ends.push(guy.0);
        open = 1;
        close = 0;
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
 */
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


//Test section
pub fn hello() -> String {
  let platform = String::from("2");
  platform
}

#[test]
fn first_test() {
	assert_eq!(hello(),"2")
}

/*
#[test]
fn test_get_api_key() {
	assert_ne!(get_api_key(),"Could not read api key, check that api-key.txt has correct API key!");
}

#[test]
fn get_gear_valid() {
	assert_eq!("
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
*/
