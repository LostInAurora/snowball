use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use snowball_server::client::Clients;
use snowball_server::handler;

// GET /health: Indicates if the service is up
// POST /register: Registers clients in the application
// DELETE /register/{client_id}: Unregisters the client with an ID
// POST /publish: Broadcasts an event to clients
// GET /ws: The WebSocket endpoint
#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));


    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}


// 将clients传递
fn with_clients(clients: Clients) -> impl Filter<Extract=(Clients, ), Error=Infallible> + Clone {
    // Arc并发场景中的引用计数，clone
    warp::any().map(move || clients.clone())
}