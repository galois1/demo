# Rust HTML Parsing Workshop

Suppose you have an Amazon.com wishlist of products and you want to find and
print a list the prices of each item using a command line program.

## Let's get started!

In this workshop we'll learn to use an external HTTP client library (`hyper`)
and an HTML DOM parser (`select`). We will take that data and process it using
`Iterators` into a product name and price which we will print to the screen.

## Step 0: Setup your local project

Open a command line terminal. This will be your primary way of building your
Rust code. Open a directory where you want to save your project and run.

```sh
cargo new --bin scraper
cd scraper
```

This command uses `cargo`, is the Rust Package Manager, to create a new project
structure that will build and run out of the box. Cargo simplifies the build and 
run steps of projects. The basic initial structure of a new cargo project has 3 
files.

```
scraper
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs

1 directory, 3 files
```

To build your project run the following. 

```sh
$ cargo build
   Compiling scraper v0.1.0 (file:///Users/foo/code/demo/scraper)
    Finished debug [unoptimized + debuginfo] target(s) in 0.98 secs
```

You can also execute the build as follows.

```sh
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/scraper`
Hello, world!
```

## Step 1: Using the Hyper HTTP Client



















### Executing the program

Now that we are inside our project, we can compile and run the program.

```sh
cargo run
   Compiling listr v0.1.0 (file:///home/example/listr)
     Running `target/debug/listr`
Hello, world!
```

## Retrieving the web page

That was useful, but we need more!
Let's use a library to help us retrieve the webpage content

# Retrieving the web page

We are going to use the [Hyper](https://github.com/hyperium/hyper) library.
Hyper is a fast, modern HTTP implementation written in and for Rust.

> Note: Introduce Crates and https://crates.io


## Including a dependency on our project

On the `Cargo.toml` file, we are adding a line to our dependency
The means we are taking any version of the library

```
[package]
name = "listr"
version = "0.1.0"
authors = ["Ferris The Crab <ferris@example.com"]

[dependencies]
hyper = "*"
```
=======
The `Cargo.toml` file hold the metadata necessary to build your project. 
>>>>>>> Stashed changes
