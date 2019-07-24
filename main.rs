extern crate reqwest;


use std::io::Read;
//https://docs.rs/hyper/0.11.2/hyper/header/index.html

#[macro_use] extern crate hyper;
//use hyper::header::Headers;
//header! { (Key, "X-API-KEY") => [String]}


//Key: X-API-KEY
//Value: {paste your API key here}
//Description: Destiny


fn main() {
   // let mut headers = Headers::new();

 //   headers.set(Key("X-API-KEY".to_owned()));
 //   headers.set(Value("*****API-key here?************".to_owned()));
        //let mut response = reqwest::get("https://httpbin.org/status/418")
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
