use std::io::Read;

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
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);
}
