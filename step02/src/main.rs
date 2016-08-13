extern crate hyper;


use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
extern crate select;
use select::document::Document;
use select::predicate::{Class,Attr, Name};
use select::node::Node;


fn main() {
    let client = Client::new();
    let mut response =
        client.get("https://brson.github.io/demo/wishlist.html")
              .header(Connection::close())
              .send()
              .unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let titles: Vec<String> = Document::from(body.as_str())
        .find(Attr("title",()))
        .iter()
        .map(|node| node.attr("title").unwrap().to_string())
        .collect();
    println!("{:?}", titles);

}
