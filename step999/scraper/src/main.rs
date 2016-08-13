extern crate hyper;


use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
extern crate select;
use select::document::Document;
use select::predicate::{Class, Attr, Name};
use select::node::Node;


fn main() {
    let client = Client::new();
    let mut response = client.get("https://brson.github.io/demo/wishlist.html")
                             .header(Connection::close())
                             .send()
                             .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let items: Vec<(String, String)> = Document::from(body.as_str())
                   .find(Class("a-row"))
                   .iter()
                   .map(|node| {
                      let title = node.find(Name("h5")).first().unwrap().text();
                      let price = node.find(Class("a-color-price")).first().unwrap().text();
                      (title, price)
                   })
                   .collect();
    println!("{:?}", items);
}

