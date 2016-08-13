extern crate hyper;


use hyper::Client;
use hyper::header::Connection;
use std::io::Read;


// fn main() {
//     let client = Client::new();

//     let res = client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS").send().unwrap();

//     println!("Response: {}", res.body);
// }

fn main() {
    let client = Client::new();
    let mut response =
        client.get("https://raw.githubusercontent.com/brson/demo/master/wishlist.html")
              .header(Connection::close()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    println!("Body:   {}", body);
}
