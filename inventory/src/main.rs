use std::io::Read;
extern crate rustc_serialize;
extern crate serde;
use serde::{Deserialize, Serialize};

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
		.header("X-API-KEY", api_key)
		.send()
		.expect("Failed to send request");
	let mut buf = String::new();
	response.read_to_string(&mut buf).expect("Failed to read response");
	let user = get_json(buf);
	let u: User = serde_json::from_str(&user).expect("failed to parse response");
	let api_key_2 = get_api_key();//TODO figure out how to fix this!!
	let equipment = get_gear(&api_key_2, base_url, platform, u.membershipId, request_type);
	println!("{}",equipment)
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

fn get_json(source:String) -> String {
  let start_index = source.find("[").expect("failed to find open");
  let end_index = source.find("]").expect("failed to find close");
  let mut result = &source[(start_index + 1)..end_index];
  result.to_string()
}
