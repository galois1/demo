Fix OpenSSL problem on MacOS

    $ brew install openssl
    $ C_INCLUDE_PATH=$(brew --prefix openssl)/include cargo build
* Rust HTML Parsing Workshop

Suppose you have an Amazon.com wishlist of products and you want to find and print 
a list the prices of each item using a command line program. 

** Let's get started!

In this workshop we'll learn to use an external HTTP client library (`hyper`) and 
an HTML DOM parser (`select`). We will take that data and process it using 
`Iterators` into a product name and price which we will print to the screen. 

*** Step 1: Setup your local project

Open a command line terminal. This will be your primary way of building your Rust 
code. Open a directory where you want to save your project and run 

#+BEGIN_SRC sh
cargo new --bin scraper
#+END_SRC

This command uses cargo, is the Rust Package Manager, to create a new project 
structure that will build and run out of the box. Cargo simplifies the build and 
run steps of projects.  
 
#+BEGIN_SRC sh
cd listr
cargo build
#+END_SRC

** Overview of the project

#+BEGIN_EXAMPLE
scraper
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
#+END_EXAMPLE

Note:
Explain Cargo.toml is where we include some information about our project
and =src/lib.rs= is where the application will live.

** Executing the program

Now that we are inside our project, we can compile and run the program.

#+BEGIN_SRC sh
cargo run
#+END_SRC

#+BEGIN_EXAMPLE
   Compiling listr v0.1.0 (file:///home/example/listr)
     Running `target/debug/listr`
Hello, world!
#+END_EXAMPLE

** Retrieving the web page

That was useful, but we need more!
Let's use a library to help us retrieve the webpage content

** Retrieving the web page

We are going to use the [[https://github.com/hyperium/hyper][Hyper]] library.

#+BEGIN_QUOTE
hyper is a fast, modern HTTP implementation written in and for Rust. 
#+END_QUOTE

Note:
Introduce Crates and https://crates.io


** Including a dependency on our project
   
On the =Cargo.toml= file, we are adding a line to our dependency
The =*= means we are taking any version of the library

#+BEGIN_SRC toml :source listr/Cargo.toml
[package]
name = "listr"
version = "0.1.0"
authors = ["Ferris The Crab <ferris@example.com"]

[dependencies]
hyper = "*"
#+END_SRC

Note:
If possible, introduce SemVer schema that crates uses
Trivia: Ferris The Crab is the Rust mascot

** Let's verify it still builds

Let's execute our program to be sure we have our dependency installed.
 
#+BEGIN_SRC sh
cargo run
#+END_SRC

This will download a list of packages from crates.io
And install the new dependency we've added.

#+BEGIN_EXAMPLE
    Updating registry `https://github.com/rust-lang/crates.io-index`
    [...snip...]
   Compiling hyper v0.9.10
   Compiling listr v0.1.0 (file:///home/example/listr)
     Running `target/debug/listr`
Hello, world!
#+END_EXAMPLE

Note:
This helps ensure we have everyone on the same step, with the dependency installed
A good test to the internet link

** Downloading the page 
*** Including hyper library
    
On the =src/main.rs= file we will include that we want to link the
code from the crate into our application.

#+BEGIN_SRC rust :file listr/src/main.rs
extern crate hyper;
#+END_SRC

Note:
This is going to be a bigger step
Ensure everybody is on the same page, give space to have questions
about syntax and semantics

** Downloading the page
*** Importing the client

Still on =src/main.rs= file, we will expose the =Client= code to the
file, so we don't have to reference the full path to the structure.

#+BEGIN_SRC rust :file listr/src/main.rs
extern crate hyper;
#+END_SRC

** Downloading the page
*** Making a request with the Client

Now that our =src/main.rs= file has a Client, we can make a request using the code.

#+BEGIN_SRC rust :file listr/src/main.rs
fn main() {
    let client = Client::new();
    let request = client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS");
    let request_result = request.send();
    let response = request_result.expect("The HTTP request failed to be made");

    println!("Response status: {}", response.status);
}
#+END_SRC

Note:
This is a lot of lines.
Each of the variables have a different type.
=.expect()= was used instead of =.unwrap()= to introduce a good practice and help out debugging possible.
The print of the =status= is to help out debugging,
Maybe the person typed an url that does not exist,
and a setup for the next step introducing documentation

** Downloading the page
*** Printing the content of the page

That =status= of the HTTP request is useful, but we still need to
figure out how to ready the body of the request.

Cargo and Rust allow you to generate documentation to every library on
your project.

#+BEGIN_SRC sh
cargp doc --open
#+END_SRC

#+BEGIN_EXAMPLE
 Documenting bitflags v0.7.0
 [...snip...]
 Documenting hyper v0.9.10
 Documenting listr v0.1.0 (file:///home/example/listr)
#+END_EXAMPLE

Note:
Give a quick overview on how to navigate around the docuemntation
Search for =hyper::client::Response=

** Downloading the page
*** Getting the content of the request body

We are going to use =Read= trait to put the content into a =String=

#+BEGIN_SRC rust :file listr/src/main.rs
  use std::io::Read;

  let mut body = String::new();
  response.read_to_string(&mut body).expect("Could not put the body content into the string");
  println!("Response body: {}", body);
#+END_SRC

Note:
Explain that in Rust, values are immutable by default.
When we want to be able to change the values of the variables itself, we have to mark it with =mut=
Teach how to read the references:
- When there is a &mut body, we read it as "a mutable reference to the body"
This will lead to a compilation error, because =response= is immutable.

** Downloading the page
*** Fixing the compilation error

Let's try it out

#+BEGIN_SRC sh
cargo run
#+END_SRC
    
At this point, you might have gotten a little compiler error.

#+BEGIN_EXAMPLE
src/main.rs:16:5: 16:13 error: cannot borrow immutable local variable `response` as mutable
src/main.rs:16     response.read_to_string(&mut body).expect("Could not put the body content into the string");
                   ^~~~~~~~
src/main.rs:9:9: 9:17 note: use `mut response` here to make mutable
src/main.rs:9     let response = request_result.expect("The HTTP request failed to be made");
                      ^~~~~~~~
error: aborting due to previous error
error: Could not compile `listr`.
#+END_EXAMPLE

Note:
Explain that the compiler is trying to be helpful
Show how to read the error message
If possible, share a bit your experience getting started with the compiler errors in Rust

** Downloading the page
*** Fixing the compilation error

The compiler is telling we are trying to change the content of a variable, but it is marked as immutable.
Lets add a =mut= marker on the =response= variable.

#+BEGIN_SRC rust :file listr/src/main.rs
  let mut response = request_result.expect("The HTTP request failed to be made");
#+END_SRC

** Downloading the page
*** Page download completed.
    
At this point, you sould be seeing the content of the page.
Our file is looking like this:

#+BEGIN_SRC rust :file listr/src/main.rs
extern crate hyper;

use hyper::client::Client;
use std::io::Read;

fn main() {
    let client = Client::new();
    let request = client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS");
    let request_result = request.send();
    let mut response = request_result.expect("The HTTP request failed to be made");

    println!("Response status: {}", response.status);

    let mut body = String::new();
    response.read_to_string(&mut body).expect("Could not put the body content into the string");
    println!("Response body: {}", body);
}
#+END_SRC