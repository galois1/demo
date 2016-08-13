extern crate hyper;
extern crate select;

use hyper::Client;

use select::document::Document;
use select::predicate::{Class, Name};

use std::io::Read;

fn main() {
    let client = Client::new();
    let mut response = client.get("https://brson.github.io/demo/wishlist.html")
                             .send()
                             .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let items: Vec<(String, String)> =
        Document::from(body.as_str())
                  .find(Class("a-row"))
                  .iter()
                  .flat_map(|node| {
                     let title = node.find(Name("h5")).first();
                     let price = node.find(Class("a-color-price")).first();
                     match (title, price) {
                         (Some(title), Some(price)) => Some((title.text().trim().into(), price.text().trim().into())),
                         _ => None
                     }
                  })
                  .collect();

    for (title, price) in items {
        println!(" * Book \"{}\", with price {}", title, price);
    }
}
