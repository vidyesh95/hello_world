//! A basic Hello World web server using the Axum framework.
//!
//! This example demonstrates:
//! 1. Setting up an asynchronous main function using the `tokio` runtime.
//! 2. Defining a `Router` with a simple `GET` route.
//! 3. Binding a `TcpListener` to a specific address and port.
//! 4. Serving the application to handle incoming HTTP requests.
//! 
//! Run with `cargo run` or `bacon run-long`

use hello_world::run;

/// The entry point for the application.
///
/// It initializes the Tokio runtime, builds the Axum router,
/// and starts the server on port 8080.
#[tokio::main]
async fn main() {
    run().await;
}
