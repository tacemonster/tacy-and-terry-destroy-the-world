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
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufRead;

pub fn print_equipped(items: Vec<String>) {
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


pub fn all_equipment(api_key: &str, membership_id: String, platform: char) -> Vec<String> {
    let mut results = Vec::new();
    // call get_gear to connect with Destiny API
    let equipment = get_gear(api_key, platform, membership_id);
    // call unwrap_response to remove outer portions of code we don't need (see function for details)
    let hold_items = unwrap_response(equipment, 5);
    if hold_items.is_empty() {
        println!("unwrap response failed somehow");
    } else {
        //create the vector of item hash strings that will be given to print_equipment later
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
    // this one was... fun. due to the varying formats of the jsons returned from
    // our API calls, this function uses a depth integer to indicate
    // how many opening curly brackets we want to ignore when pulling our
    // data out of the source
    let mut results = Vec::new();
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let mut open = 0;
    let mut close = 0;
    let chars = source.as_str().char_indices();
    for guy in chars {
        if guy.1 == '{' {
            open += 1;
            // once we reach the correct number of brackets to ignore,
            // we then want to start grabbing the indices where we find open brackets
            if open == close + depth + 1 {
                starts.push(guy.0);
            }
        } else if guy.1 == '}' {
            close += 1;
            // if we have found the end of one of the pieces we want,
            // get its closing index and reduce the open and close counts
            if open == close + depth {
                ends.push(guy.0);
                open -= 1;
                close -= 1;
            }
        }
    }
    // we should now have vector "starts" with some number of opening indices
    // and vector "ends" with the corresponding closing indices
    // not we populate "results" with the corresponing substrings
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
    // this function exists because I could not get serde_json to play nicely with
    // jsons that had structs as the values of key-value pairs. By removing the
    // square brackets around these structs, I was able to get everything running

    // first remove opening brackets
    let mut index = buf.find('[');
    while index != None {
        let indexno = index.unwrap() as usize;
        buf.remove(indexno);
        index = buf[indexno + 1..].find('[');
    }
    // then remove closing brackets!
    index = buf.find(']');
    while index != None {
        let indexno = index.unwrap() as usize;
        buf.remove(indexno);
        index = buf[indexno + 1..].find(']');
    }
    buf
}

fn save_to_file(character_info:String) -> Result<()> {
    let mut buffer = String::new();//storage for raw responce from user
    io::stdin().read_to_string(&mut buffer)?;
    let mut handle = stdin.lock();

    println!("What is the filename you want to save the charater info to?");
    let file_info = handle.read_to_string(&mut buffer).unwrap();//unwraped input from user
    let path = Path::new(file_info);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the character information to file
    match file.write_all(character_info.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
