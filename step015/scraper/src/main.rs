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
    Document::from(body.as_str())
                  .find(Class("a-row"))
                  .iter()
                  .map(|node| {
                    println!(" * Row {}", node.text());
                  })
                  .collect::<Vec<_>>();
}
