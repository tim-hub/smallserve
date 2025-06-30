use axum::{Router, routing::get_service}; // Axum framework imports
use clap::Parser; // Command line argument parsing
use std::{net::SocketAddr, path::PathBuf}; // Standard library imports
use tower_http::services::ServeDir ; // HTTP service for serving files

#[derive(Parser)] // Define command line arguments structure
struct Args {
    /// The directory to serve files from
    #[clap(short, long, default_value = "./")]
    dir: PathBuf,

    /// The address to bind the server to
    #[clap(short, long, default_value = "7777")]
    port: u16,
}

#[tokio::main] // Main function for asynchronous execution
async fn main() {
    let args = Args::parse(); // Parse command line arguments
    let app = Router::new().route("/", get_service(ServeDir::new(args.dir)))
        .fallback(get_service(ServeDir::new("./")));

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap(); // Bind TcpListener
    axum::serve(listener, app.into_make_service()) // Pass listener to axum::serve
        .await
        .unwrap(); // Handle any errors that occur during server execution
}