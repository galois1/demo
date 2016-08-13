extern crate hyper;


use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
extern crate select;
use select::document::Document;
use select::predicate::{Class,Name};
use select::node::Node;


// fn main() {
//     let client = Client::new();
//     let mut response = client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS").
//         header(Connection::close()).send().unwrap();
//     let mut body = String::new();
//     response.read_to_string(&mut body).unwrap();
//     println!("Body:   {}", body);

// }

fn main() {
    let books = Book::get_wishes();
    for book in books.iter().rev() {
        println!("Title:   {}", book.title);
    }
}

fn open_wishlist() -> String {
    let client = Client::new();
    let mut response = client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS").
        header(Connection::close()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    return body;
}

struct Book {
    title:   String,
}

impl Book {
    fn get_wishes() -> Vec<Book> {
        Document::from_str(&open_wishlist()).find(Name("item")).iter()
            .map(|node| Book::new(&node)).collect()
    }
    fn new(node: &Node) -> Book {
        let header = node.find(Name("a")).first().unwrap();
        // let mut link = String::from(header.attr("href").unwrap());
        Book { title: header.text() }
    }
}