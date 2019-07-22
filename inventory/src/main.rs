use std::io::Read;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use serde_json::Result;

fn get_api_key() -> String {
	let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
	let mut key = String::new();
	f.read_to_string(&mut key).expect("could not read api key");
	key.trim().to_string()
}

#[derive(Serialize, Deserialize)]
struct Data {
Responce: Responce,
		  ErrorCode: usize,
		  ThrottleSeconds: usize,
		  ErrorStatus: String,
		  Message: String,
		  MessageData: String
}

#[derive(Serialize, Deserialize)]
struct Responce { 
		  iconPath: String,
		  membershipType: String,
		  membershipId: String,
		  displayName: String,
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
buf
}
