use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{http::Method, ws::Message, Filter, Rejection};

pub mod battle;
pub mod fixtures;
pub mod handler;
pub mod ws;

type Result<T> = std::result::Result<T, Rejection>;
pub type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
pub async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

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

    let teams = warp::path("api")
        .and(warp::path("team"))
        .and(warp::get())
        .and_then(battle::get_team);

    let oppo = warp::path("api")
        .and(warp::path("opposition"))
        .and(warp::get())
        .and_then(battle::get_opposition);

    let battle = warp::path("api")
        .and(warp::path("battle"))
        .and(warp::get())
        .and_then(battle::get_battle_result);

    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .or(teams)
        .or(oppo)
        .or(battle)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec![
                    "Access-Control-Allow-Headers",
                    "Access-Control-Request-Method",
                    "Access-Control-Request-Headers",
                    "Origin",
                    "Accept",
                    "X-Requested-With",
                    "Content-Type",
                ])
                .allow_methods(&[
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                    Method::HEAD,
                ]),
        );

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub fn with_clients(
    clients: Clients,
) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

// use warp::test::request;

#[tokio::test]
async fn test_main() {
    use serde_json::json;
    use serde_json::Value;
    use warp::test::request;

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let health = warp::path!("health").and_then(handler::health_handler);

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

    let routes = health
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    let register_data = json!({
        "user_id": 1,
    });

    let publish_data = json!({
        "user_id": 1,
        "topic": "rust",
        "message": "hello world"
    });

    // Use `warp::test::request` to create test requests for each route
    let health_request = request().method("GET").path("/health");
    let register_post_request = request()
        .method("POST")
        .path("/register")
        .json(&register_data);
    let publish_request = request()
        .method("POST")
        .path("/publish")
        .json(&publish_data);

    // Test each route separately
    let health_response = health_request.reply(&routes).await;
    assert_eq!(health_response.status(), 200);

    let register_post_response = register_post_request.reply(&routes).await;
    assert_eq!(register_post_response.status(), 200);

    let publish_response = publish_request.reply(&routes).await;
    assert_eq!(publish_response.status(), 200);

    let json_value: Value = serde_json::from_slice(&register_post_response.body())
        .expect("Failed to deserialize response body as JSON");

    if let Some(url_value) = json_value.get("url") {
        let url = url_value.as_str().expect("URL is not a string");

        let ws_request = request()
            .method("GET")
            .path(url)
            .header("upgrade", "websocket")
            .header("Connection", "Upgrade")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==");

        let ws_response = ws_request.reply(&routes).await;
        assert_eq!(ws_response.status(), 101);
    }
}
