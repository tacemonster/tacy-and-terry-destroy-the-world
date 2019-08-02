#![allow(non_snake_case)]
use std::io::Read;

use std::collections::HashMap;
use rustc_serialize::json::Json;

use serde_json::Value;

fn main() {
  let api_key = get_api_key();
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
  let platform = '2';
  //let platform = '4';
  let player_name = String::from("cortical_iv");
  //let player_name = String::from("shark90%231673");
  let url = String::from("https://www.bungie.net/Platform/Destiny2/");

  let mut search_url = url.clone();
  search_url.push_str("SearchDestinyPlayer/");
  search_url.push(platform);
  search_url.push('/');
  search_url.push_str(&player_name);
  search_url.push('/');
  //println!("{}", search_url);

  let mut response = reqwest::Client::new()
    .get(&search_url)
    .header("X-API-KEY", api_key.clone())
    .send()
    .expect("Failed to send request");

  let mut buf = String::new();
  response.read_to_string(&mut buf).expect("Failed to read response");
  //println!("{}", buf);
  buf = fix_json(buf);
  let info : Value = serde_json::from_str(&buf).expect("Failed to parse response!");
  //println!("{}",info["Response"]["membershipId"]);
  let membershipId = strip_quotes(info["Response"]["membershipId"].to_string());
  //println!("{}",membershipId);
  let items = all_equipment(membershipId, platform, request_type);
  println!("You are equipped with the following items:");
  for item in items {
    let item_detail = get_item(item);
    if item_detail.is_empty() {
      continue;
    }
    else {
      let unwrapped_item : Value = serde_json::from_str(&item_detail).expect("Failure while parsing item details");
      let mut name = &unwrapped_item["Response"]["displayProperties"]["name"];
      
      let name = strip_quotes(name.to_string());
      let description = &unwrapped_item["Response"]["displayProperties"]["description"];
      let mut description = strip_quotes(description.to_string());
      description = description.replace("\\n", " \n\t");
      description = description.replace("\\", "");

      println!("{}: {}\n", name, description);
    }

  }
}

fn all_equipment(membershipId:String, platform:char, request_type:String)-> Vec<String> {
  let mut results = Vec::new();
  let base_url = String::from("https://www.bungie.net/Platform/Destiny2/");
  let equipment = get_gear(&get_api_key(), &base_url, platform, membershipId, request_type);
  let hold_items = unwrap_response(equipment, 5);
  if hold_items.is_empty() {
    println!("unwrap response failed somehow");
  }
  else {
    for item in hold_items {
      let item_json : Value = serde_json::from_str(&item).unwrap();
      let itemHash:String = item_json["itemHash"].to_string();
      //let itemHash:String = item_json["itemInstanceId"].to_string();
      //let itemId = strip_quotes(itemHash);
      //println!("{}", itemHash);
      //results.push(itemId);
      results.push(itemHash);
    }
  }
  results
}

fn get_api_key() -> String {
  let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
  let mut key = String::new();
  f.read_to_string(&mut key).expect("Could not read api key, check that api-key.txt has correct API key!");
  key.trim().to_string()
}

fn strip_quotes(source:String) -> String {
  let start_index = source.find("\"").expect("failed to find open");
  let end_index = source.rfind("\"").expect("failed to find close");
  if start_index == end_index {
    return source;
  }
  let result = &source[(start_index + 1)..end_index];
  result.to_string()
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
  buf
}

fn unwrap_initial_response(source:String) -> String {
  let start_index = source.find("[").expect("failed to find open");
  let end_index = source.find("]").expect("failed to find close");
  let result = &source[(start_index + 1)..end_index];
  result.to_string()
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
fn get_item(item_id:String) -> String {
//https://www.bungie.net/Platform/Destiny2/Manifest/DestinyInventoryItemDefinition/1345867571
	let api_key = get_api_key();
	let mut url = String::from("https://www.bungie.net/Platform/Destiny2/Manifest/DestinyInventoryItemDefinition/");
	url.push_str(&item_id);

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


fn fix_json(mut buf:String) -> String {
  let mut index = buf.find("[");
  while index != None {
    let indexno = index.unwrap() as usize;
    let mut tempbuf = buf.as_str().char_indices();
    let mut i = 0;
    while i <= indexno {
      tempbuf.next();
      i += 1;
    }
    buf.remove(indexno);
    index = buf[indexno + 1..].find("[");
  }
  index = buf.find("]");
  while index != None {
    let indexno = index.unwrap() as usize;
    let mut tempbuf = buf.as_str().char_indices();
    let mut i = 0;
    while i <= indexno {
      tempbuf.next();
      i += 1;
    }
    buf.remove(indexno);
    index = buf[indexno + 1..].find("]");
  }
  buf
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
#[test]
fn test_get_api_key() {
	assert_ne!(get_api_key(),"Could not read api key, check that api-key.txt has correct API key!");
}

/*
#[test]
fn get_gear_valid() {
	assert_eq!("
fn get_gear(api_key:&String, base_url:String, platform: char, membershipId:String, request_type:String) -> String {
  let api_key = get_api_key();
  let mut url = base_url;
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
>>>>>>> 72e06fe09bce5365a4084183f8382e49536f1a7a

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

