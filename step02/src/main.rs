extern crate hyper;


use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
extern crate select;
use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;


fn main() {
    let client = Client::new();
    let mut response =
        client.get("https://raw.githubusercontent.com/brson/demo/master/wishlist.html")
              .header(Connection::close())
              .send()
              .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let titles: Vec<String> = Document::from(body.as_str())
        .find(Name("item"))
        .iter()
        .map(|node| node.find(Name("a")).first().unwrap().text())
        .collect();
    println!("{:?}", titles);

}
