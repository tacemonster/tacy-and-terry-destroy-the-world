use reqwest; // 0.9.17

fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.rust-lang.org")
        .header("X-My-Custom-Header", "foo")
        .send()?;

        println!("{}", res)
}
