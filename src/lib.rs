// Copyright © 2018 Tacy Bechtel and Terry Tower
// [This program is licensed under the "GNU License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/// In order to use this program, you will need to get an API key
/// from bungie.net and put it in a file called api-key.txt.

///The serde_json crate is useful for parsing json objects.
use serde_json::Value; 
///The std Read module is needed to pull the API key from an external file.
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct User { 
            icon_path: Option<String>,
            membership_type: usize,
            membership_id: String,
            display_name: String,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UserResponse {
    response: Vec<User>,
    error_code: i64,
    error_status: String,
    message_data: HashMap<(), ()>,
    throttle_seconds: u64,
    message: String,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Item {
   transfer_status : u64,
   dismantle_permission : u64,
   bucket_hash : u64,
   quantity : u64,
   item_hash : u64,
   state : u64,
   bind_status : u64,
   lockable : bool,
   is_wrapper : bool,
   item_instance_id : String,
   location : i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct EquipmentData {
    items: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug)]
struct Equipment {
    privacy: u64,
    data: HashMap<String, EquipmentData>,
}

#[derive(Serialize, Deserialize)]
struct CharacterEquipment {
    #[serde(rename = "characterEquipment")]
    character_equipment: Equipment,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct APIResponse {
    error_code: i64,
    response: CharacterEquipment,
    error_status: String,
    message_data: HashMap<(), ()>,
    throttle_seconds: u64,
    message: String,
}



pub fn print_equipped(items: Vec<u64>) {
    for item in items {
        // call get_item function
        let item_detail = get_item(item);
        // if the item is not found by the get_item call, we want to move on.
        if item_detail.is_empty() {
            continue;
        } else {
            // unwrap the item as a serde_json Value type
            let unwrapped_item: Value =
                serde_json::from_str(&item_detail).expect("Failure while parsing item details");
            // get the values we want (name and description)
            let jname = &unwrapped_item["Response"]["displayProperties"]["name"];
            let jdescription = &unwrapped_item["Response"]["displayProperties"]["description"];
            // strip the quote marks from the values (Rust reads them as string literals)
            let name = strip_quotes(jname.to_string());
            let mut description = strip_quotes(jdescription.to_string());
            // if the item has no description, provide the generic entry
            if description.is_empty() {
                description = "An item so awesome it cannot be described".to_string();
            } else {
                // remove/replace other literals we don't want!
                description = description.replace("\\n", " \n");
                description = description.replace("\\", "");
            }
            // print the results
            println!("{}: {}\n", name, description);
        }
    }
}


pub fn all_equipment(api_key: &str, membership_id: String, platform: char) -> Vec<u64> {
    let mut results = Vec::new();
    // call get_gear to connect with Destiny API
    let equipment = get_gear(api_key, platform, membership_id);
    let response: APIResponse =
        serde_json::from_str(&equipment).expect("failed in serde");
    let data_set = response.response.character_equipment.data;
    for (_string, map) in data_set {
      //let hashmap_items = data_set.map;
      for item in map.items {
        results.push(item.item_hash);
      }
    }
    results
}

pub fn get_api_key() -> String {
    //The API key is kept in a seperate file to allow for a simple line in .gitignore to prevent upload to git
    let mut f = std::fs::File::open("api-key.txt")
    	.expect("Could not open api-key.txt");
    //key will hold location where API will be stored.
    let mut key = String::new();
    //from file to string to be used for future functions
    f.read_to_string(&mut key)
        .expect("Could not read api key, check that api-key.txt has correct API key!");
    //remove whitespace
    key.trim().to_string()
}

pub fn strip_quotes(source: String) -> String {
    //strip_quotes() is used to remove quotation marks from string retrieved from json objects
    //removes the outermost pair of quotation marks in a string.
    //check for first quote
    let start_index = source.find('\"').expect("failed to find open");
    //check for last quote
    let end_index = source.rfind('\"').expect("failed to find close");
    //check that first is not same as last
    if start_index == end_index {
        return source;
    }
    //reove the quotes. will also remove any characters outside the quotes, so function must be used carefully
    //remove characters outside the quotes
    let result = &source[(start_index + 1)..end_index];
    result.to_string()
}

pub fn get_member_id(api_key: &str, platform: char, player_name: String) -> String {
    //get_member_id() is used to get the unique identifier that Bungie uses to identify the player
    //First the url to submit the query needs to be built
    let mut search_url = String::from("https://www.bungie.net/Platform/Destiny2/");
    search_url.push_str("SearchDestinyPlayer/");
    search_url.push(platform);
    search_url.push('/');
    search_url.push_str(&player_name);
    search_url.push('/');
    //The contructed url then needs to be sent with the API key attached the the header
    let mut response = reqwest::Client::new()
        .get(&search_url)
        .header("X-API-KEY", api_key.clone())
        .send()
        .expect("Failed to send request");
    //location for responce from Bungie
    let mut buf = String::new();
    response
        .read_to_string(&mut buf)
        .expect("Failed to read response");
    //Player did not have a match in player database
    if buf == "" {
        String::from("Player not found")
    //Player was found and membershipId is returned as part of a json object
    } else {
        let user_response : UserResponse = serde_json::from_str(&buf).expect("Failed to parse response!");
        let user = user_response.response.first().unwrap();
        user.membership_id.clone()
    }
}

pub fn get_gear(api_key: &str, platform: char, membership_id: String) -> String {
    //get_gear() takes the player's info and returns a json object that contains metadata about the character they are playing.
    //Query url needs to be built
    let mut url = String::from("https://www.bungie.net/Platform/Destiny2/");
    //100 is profiles, 200 is characters, 201 is non-equiped items, 205 currently equiped items (the option that we want).
    let request_type = String::from("205");
    url.push(platform);
    url.push_str("/Profile/");
    url.push_str(&membership_id);
    url.push_str("/?components=");
    url.push_str(&request_type);
    //The query is sent to Bungie
    let mut response = reqwest::Client::new()
        .get(&url)
        .header("X-API-KEY", api_key.clone())
        .send()
        .expect("Failed to send request");
    //responce is stored
    let mut buf = String::new();
    response
        .read_to_string(&mut buf)
        .expect("Failed to read response");
    buf
}

pub fn get_item(item_id: u64) -> String {
    //get_item takes the item identifier metadata and gets the information from Bungie
    let api_key = get_api_key();
    //url is built
    let mut url = String::from(
        "https://www.bungie.net/Platform/Destiny2/Manifest/DestinyInventoryItemDefinition/",
    );
    url.push_str(&item_id.to_string());
    //built url is sent to Bungie
    let mut response = reqwest::Client::new()
        .get(&url)
        .header("X-API-KEY", api_key)
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    //the responce is stored
    response
        .read_to_string(&mut buf)
        .expect("Failed to read response");
    //println!("{}", buf);
    buf
}

