// Copyright © 2018 Tacy Bechtel and Terry Tower
// [This program is licensed under the "GNU License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///The serde_json crate is useful for parsing json objects.
use serde_json::Value; 
///The std Read module is needed to pull the API key from an external file.
use std::io::Read;

pub fn print_equipped(items: Vec<String>) {
    for item in items {
        let item_detail = get_item(item);
        if item_detail.is_empty() {
            continue;
        } else {
            let unwrapped_item: Value =
                serde_json::from_str(&item_detail).expect("Failure while parsing item details");
            let jname = &unwrapped_item["Response"]["displayProperties"]["name"];
            let name = strip_quotes(jname.to_string());
            let jdescription = &unwrapped_item["Response"]["displayProperties"]["description"];
            let mut description = strip_quotes(jdescription.to_string());
            if description.is_empty() {
                description = "An item so awesome it cannot be described".to_string();
            }
            description = description.replace("\\n", " \n");
            description = description.replace("\\", "");

            println!("{}: {}\n", name, description);
        }
    }
}

pub fn all_equipment(api_key: &str, membership_id: String, platform: char) -> Vec<String> {
    let mut results = Vec::new();
    let equipment = get_gear(api_key, platform, membership_id);
    let hold_items = unwrap_response(equipment, 5);
    if hold_items.is_empty() {
        println!("unwrap response failed somehow");
    } else {
        for item in hold_items {
            let item_json: Value = serde_json::from_str(&item).unwrap();
            let item_hash = item_json["itemHash"].to_string();
            results.push(item_hash);
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
    //check for first quote
    let start_index = source.find('\"').expect("failed to find open");
    //check for last quote
    let end_index = source.rfind('\"').expect("failed to find close");
    //check that first is not same as last
    if start_index == end_index {
        return source;
    }
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
        buf = fix_json(buf);
        let info: Value = serde_json::from_str(&buf).expect("Failed to parse response!");
        info["Response"]["membershipId"].to_string()
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

pub fn unwrap_response(source: String, depth: usize) -> Vec<String> {
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
        } else if guy.1 == '}' {
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
        results.push(source[starts[index]..=ends[index]].to_string());
    }
    results
}

pub fn get_item(item_id: String) -> String {
    //get_item takes the item identifier metadata and gets the information from Bungie
    let api_key = get_api_key();
    //url is built
    let mut url = String::from(
        "https://www.bungie.net/Platform/Destiny2/Manifest/DestinyInventoryItemDefinition/",
    );
    url.push_str(&item_id);
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

pub fn fix_json(mut buf: String) -> String {
    let mut index = buf.find('[');
    while index != None {
        let indexno = index.unwrap() as usize;
        let mut tempbuf = buf.as_str().char_indices();
        let mut i = 0;
        while i <= indexno {
            tempbuf.next();
            i += 1;
        }
        buf.remove(indexno);
        index = buf[indexno + 1..].find('[');
    }
    index = buf.find(']');
    while index != None {
        let indexno = index.unwrap() as usize;
        let mut tempbuf = buf.as_str().char_indices();
        let mut i = 0;
        while i <= indexno {
            tempbuf.next();
            i += 1;
        }
        buf.remove(indexno);
        index = buf[indexno + 1..].find(']');
    }
    buf
}
