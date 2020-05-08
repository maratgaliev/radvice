#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Advice {
  id: i64,
  text: String,
  sound: String
}

fn main() -> Result<(), Error> {
    let request_url = String::from("http://fucking-great-advice.ru/api/random");
    let mut response = reqwest::get(&request_url)?;
    let quote: Advice = response.json()?;
    println!("{:?}", quote.text);
    Ok(())
}