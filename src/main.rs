extern crate reqwest;
extern crate select;
use std::io::Read;
use select::document::Document;
use select::predicate::{Predicate, Class, Attr, Name};

fn main() {
    // https://coinmarketcap.com/
    // http://crix.hu-berlin.de/
    let mut response = reqwest::get("https://coinmarketcap.com/").unwrap();
    assert!(response.status().is_success());

    // let mut content = String::new();
    // resp.read_to_string(&mut content);

    let mut body = String::new();
    response.read_to_string(&mut body).expect("Read failed");
    //println!("{:?}", body);

    let document = Document::from(body.as_str());
    for node in document.find(Name("tbody"))
        .next()
        .unwrap()
        .parent()
        .unwrap()
        .find(Name("tr"))
        //.take(10)  */  
    {  
        println!("{}", node.text());
    }

    
}