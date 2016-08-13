extern crate hyper;


use std::env;
use std::sync::mpsc;
use std::time::Duration;
use std::io::Read;
use hyper::client::{Client, Request, Response};


fn main() {
    let client = Client::new();

    let mut res = client.get("http://www.amazon.com.\
              br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS")
        .send()
        .unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body);
    println!("Response: {}", body);
}
