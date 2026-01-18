//! A basic Hello World web server using the Axum framework.
//!
//! This example demonstrates:
//! 1. Setting up an asynchronous main function using the `tokio` runtime.
//! 2. Defining a `Router` with a simple `GET` route.
//! 3. Binding a `TcpListener` to a specific address and port.
//! 4. Serving the application to handle incoming HTTP requests.
//! 
//! Run with `cargo run` or `bacon run-long`

use axum::{Router, routing::get};

/// The entry point for the application.
///
/// It initializes the Tokio runtime, builds the Axum router,
/// and starts the server on port 8080.
#[tokio::main]
async fn main() {
    // Build our application with a single route.
    // The `get` handler returns a "Hello, World!" string with a 200 OK status.
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Run our app with hyper, listening globally on port 8080.
    // `0.0.0.0` allows connections from any network interface.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
