mod routes;
use routes::create_routes;

pub async fn run() {
    // Build our application with a single route.
    // The `get` handler returns a "Hello, World!" string with a 200 OK status.
    let app = create_routes();
    // Run our app with hyper, listening globally on port 8080.
    // `0.0.0.0` allows connections from any network interface.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}