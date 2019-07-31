#![allow(non_snake_case)]
use std::io::Read;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result as Output};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use rustc_serialize::json::Json;

use serde_json::Value;

fn main() {
  let api_key = get_api_key();
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
  let platform = '2';
  let player_name = String::from("cortical_iv");
  let url = String::from("https://www.bungie.net/Platform/Destiny2/");

  let mut search_url = url.clone();
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
  println!("{}", buf);
  buf = fix_json(buf);
  let info : Value = serde_json::from_str(&buf).expect("Failed to parse response!");
  println!("{}",info["Response"]["membershipId"]);
  let membershipId = strip_quotes(info["Response"]["membershipId"].to_string());
  println!("{}",membershipId);
  let items = all_equipment(membershipId, platform, request_type);
}

fn all_equipment(membershipId:String, platform:char, request_type:String)-> Vec<String> {
  let mut results = Vec::new();
  let base_url = String::from("https://www.bungie.net/Platform/Destiny2/");
  let equipment = get_gear(base_url, platform, &membershipId, &request_type);
  let hold_items = unwrap_response(equipment, 5);
  if hold_items.is_empty() {
    println!("unwrap response failed somehow");
  }
  else {
    for item in hold_items {
      let item_json : Value = serde_json::from_str(&item).unwrap();
      let itemHash:String = item_json["itemHash"].to_string();
      println!("{}", itemHash);
      results.push(itemHash);
    }
  }
  results
}


fn get_api_key() -> String {
  let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
  let mut key = String::new();
  f.read_to_string(&mut key).expect("could not read api key");
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

fn get_gear(base_url:String, platform: char, membershipId:&String, request_type:&String) -> String {
  let api_key = get_api_key();
  let mut url = base_url;
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.

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

fn get_item(item_id2:String) -> String {
  let mut item_id = String::from("2147998056");
  let conn : Connection = Connection::open("world_sql_content_b6c7590005d9365b2723f8995f361e3f.content").unwrap();
  let mut item:String = conn.query_row(
      r#"SELECT quote(json) 
      FROM DestinyInventoryItemDefinition 
      WHERE quote(json) like '%"itemHash":2147998056%'"#,
      NO_PARAMS,
      |row| row.get(0),
      ).unwrap();
  println!("item: {}", item);
  item
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
    //if tempbuf.next().unwrap().1 == '{' {
    buf.remove(indexno);
    //}
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
    //if tempbuf.next().unwrap().1 == '}' {
    buf.remove(indexno);
    //}
    index = buf[indexno + 1..].find("]");
  }
  buf
}



