extern crate reqwest;
extern crate select;
use std::io::Read;
use select::document::Document;
use select::predicate::{Name, Class};

fn main() {
    // https://coinmarketcap.com/
    // http://crix.hu-berlin.de/
    let mut response = reqwest::get("https://coinmarketcap.com/").unwrap();
    assert!(response.status().is_success());

    let mut body = String::new();
    response.read_to_string(&mut body).expect("Read failed");

    let document = Document::from(body.as_str());
    for node in document.find(Name("tbody"))
        .next()
        .unwrap()
        .find(Name("tr"))
        .take(3)  
    {
        let coin_name = node.find(Class("currency-name")).next().unwrap().text();
        let coin_price = node.find(Class("price")).next().unwrap().text();
        println!("{}:   {}", coin_name.trim(), coin_price.trim());
    }

    
}