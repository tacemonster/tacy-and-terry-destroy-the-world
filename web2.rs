use reqwest::header;
use std::io::Read;
//https://docs.rs/hyper/0.11.2/hyper/header/index.html
//fn build_client() -> Result<(), Box<std::error::Error>> {
//#[macro_use] extern crate hyper;
//use hyper::header::Headers;
//header! { (Key, "X-API-KEY") => [String]}


//Key: X-API-KEY
//Value: {paste your API key here}
//Description: Destiny


fn main() {
   // let mut headers = Headers::new();

 //   headers.set(Key("X-API-KEY".to_owned()));
 //   headers.set(Value("".to_owned()));
        //let mut response = reqwest::get("https://httpbin.org/status/418")
        //let mut headers = header::HeaderMap::new();
        //headers.insert(header::Key, header::HeaderValue::from_static("X-API-KEY"));
        //headers.insert(header::Value, header::HeaderValue::from_static(""));
request.header(XApiKey("X-API-KEY"))
request.header(Value("****API KEY****"))
        //let client = reqwest::Client::builder()
                .default_headers(headers)
                .build();
        let mut response = reqwest::get("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/2/cortical_iv/")
            .expect("Failed to send request");
        println!("{}", response.status());
        for header in response.headers().iter() {
        //    println!("{}: {}", header.name(), header.value_string());
        }
        let mut buf = String::new();
        response.read_to_string(&mut buf).expect("Failed to read response");
        println!("=====================================");
        println!("{}", buf);
}
