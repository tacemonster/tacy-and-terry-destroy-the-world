use std::io::Read;

fn get_api_key() -> String {
    let mut f = std::fs::File::open("api-key.txt").expect("could not open api key");
    let mut key = String::new();
    f.read_to_string(&mut key).expect("could not read api key");
    key.trim().to_string()
}

fn main() {
    let api_key = get_api_key();
    let mut response = reqwest::Client::new()
        .get("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/2/cortical_iv/")
        .header("X-API-KEY", api_key)
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);
}
