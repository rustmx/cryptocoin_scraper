extern crate reqwest;
extern crate select;
use std::io::Read;
use select::document::Document;
use select::predicate::{Name, Class};

fn scrape_crix_local() {
    // This function by Abiel Parra (https://github.com/Heeled-Jim)
    
    // http://crix.hu-berlin.de/

    let document = Document::from(include_str!("../html/crix.html"));

    println!("Top 25 Cryptomonedas");
    for node in document.find(Class("currencies")).take(25) {
      let moneda = node.find(Class("currency-name")).next().unwrap();
      let precio = node.find(Class("price")).next().unwrap().text();
      println!(" Moneda: {} Precio: ${}", moneda.text(), precio);
    }
}

fn scrape_coinmarketcap_remote() {
    // https://coinmarketcap.com/  
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

fn main() {
    // Uncomment the function you want to use.

    // scrape_crix_local();
    // scrape_coinmarketcap_remote();
   
}