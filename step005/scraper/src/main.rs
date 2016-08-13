extern crate hyper;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    let client = Client::new();
    let mut response =
        client.get("https://brson.github.io/demo/wishlist.html")
              .header(Connection::close())
              .send()
              .unwrap();

}
