#![allow(non_snake_case)]
use std::io::Read;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result as Output};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use rustc_serialize::json::Json;

use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Value};
use serde_json::Result;

fn main() {
  let api_key = get_api_key();
  //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items.
  let request_type = String::from("205");
  let platform = '2';
  let player_name = String::from("cortical_iv");
  let url = String::from("https://www.bungie.net/Platform/Destiny2/");
  let item_id = String::from("6917529095138904228");//TODO variable for testing (delete when not needed)

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
  //println!("{}",response);
  let mut buf = String::new();
  response.read_to_string(&mut buf).expect("Failed to read response");
  println!("{}", buf);
  buf = fix_json(buf);
  let info : Value = serde_json::from_str(&buf).expect("Failed to parse response!");
  //let user = response["Response"].to_string();
  println!("{}",info["Response"]["membershipId"]);
  let membershipId = strip_quotes(info["Response"]["membershipId"].to_string());
  println!("{}",membershipId);
  /*
     let hold_items = unwrap_response(buf, 3);
     if hold_items.is_empty() {
     println!("unwrap response failed somehow");
     }
     else {
     let mut items = Deserializer::from_str(&hold_items[0]).into_iter::<Value>();
     for item in items {
     println!("{}", item.unwrap()["itemHash"]);
     }
     }
   */
  //let info : Value = serde_json::from_str(&buf).expect("Failed to parse response!");
  //let user = response["Response"].to_string();
  //println!("{}",info["Response"]["membershipId"]);
  //let reparse = json::parse(&user).expect("failed parsing user");
  //let name = reparse["displayName"].clone();
  //println!("{}",name);
  /*
     let mut buf = String::new();
     response.read_to_string(&mut buf).expect("Failed to read response");
     let user = unwrap_initial_response(buf);
     let u: User= deserialize_user(user);
   */
     let base_url=url.clone();	
  let equipment = get_gear(&api_key, base_url, platform, &membershipId, &request_type);
  //println!("{}", equipment);
  let hold_items = unwrap_response(equipment, 5);
  if hold_items.is_empty() {
    println!("unwrap response failed somehow");
  }
  else {
    for item in hold_items {
      let item_json:serde_json::Value = serde_json::from_str(&item).unwrap();
      //items.push(serde_json::from_str(&hold_items[0]));
      let itemHash:String = item_json["itemHash"].to_string();
      println!("{}", itemHash);
    }
    //for item in items {
    //}
  }

  //let mut item = get_item(item_id);
  //let cutoff = item.find("stats").expect("did not find stats section!");
  //item.truncate(cutoff - 2);
  //item.push('}');
  //println!("{}", item);
  //let i: ItemDetails = serde_json::from_str(&item[1..]).expect("Failed to deserialize user info");
  //println!("{}",i.itemName)
  //https://www.bungie.net/Platform/2/Profile/4611686018459314819/item/6917529095138904228
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
    return String::new();
  }
  let mut result = &source[(start_index + 1)..end_index];
  result.to_string()
}

fn unwrap_initial_response(source:String) -> String {
  let start_index = source.find("[").expect("failed to find open");
  let end_index = source.find("]").expect("failed to find close");
  let mut result = &source[(start_index + 1)..end_index];
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

fn get_gear(api_key:&String, base_url:String, platform: char, membershipId:&String, request_type:&String) -> String {
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

/*
#[derive(Debug)]
struct item_name {
name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageData {
}
#[derive(Serialize, Deserialize)]
struct ItemDetails {
itemHash: usize,
itemName: String,
itemDescription: String,
icon: String,
hasIcon: bool,
secondaryIcon: String,
actionName: String,
hasAction: bool,
deleteOnAction: bool,
tierTypeName: String,
tierType: usize,
itemTypeName: String,
bucketTypeHash: usize,
primaryBaseStatHash: usize
}

#[derive(Serialize, Deserialize)]
struct User { 
iconPath: String,
membershipType: usize,
membershipId: String,
displayName: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemResponse {
Response: CharacterEquipment,
ErrorCode: u32,
ThrottleSeconds: u32,
ErrorStatus: String,
Message: String,
MessageData: MessageData,
}

#[derive(Serialize, Deserialize, Debug)]
struct CharacterEquipment {
data: Data,
}
#[derive(Serialize, Deserialize, Debug)]
struct Data {
/*
String,
Items: vec<Item>
String,
Items: vec<Item>
String,
Items: vec<Item>
 */
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
*/


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

/*
   fn deserialize_user(ref mut buf:std::string::String) -> User {
   let u: User = serde_json::from_str(buf).expect("Failed to deserialize user info");
   u
   }

   fn deserialize_item(ref mut buf:std::string::String) -> Item {
   let i: Item = serde_json::from_str(buf).expect("Failed to deserialize item");
   i
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

 */






