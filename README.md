# smallserve

This is inspired by Python SimpleHTTPServer, but written in Rust. It is a simple HTTP server that serves files from a
directory.

## What is this for?

Sometimes you may want to quickly test some static files locally, or share files at your local network.

In Python, you have SimpleHTTPServer, which is a simple HTTP server that serves files from a directory. In Rust, you
have `smallserve`, which is a simple HTTP server that serves files from a directory.

smallserve is a rust implementation of a simple HTTP server that serves files from a directory. It is designed to be
fast, lightweight, and easy to use.

## How to use


## Development

- clone this repository
- `cargo run -- --port 7777 -d ./`