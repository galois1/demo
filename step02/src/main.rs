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
    let mut response = client.get("http://www.amazon.com.\
              br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS")
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
