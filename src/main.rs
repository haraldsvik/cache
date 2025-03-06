use axum::{
    extract::Json,
    http::{header, HeaderValue, Method},
    routing::post,
    Router,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::{BufRead, BufReader}, sync::Arc};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

mod grpc;
use grpc::CacheGrpcService;

// Types for our API
#[derive(Debug, Deserialize)]
struct LookupRequest {
    keys: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LookupResponse {
    found: std::collections::HashMap<String, String>,
    missing: Vec<String>,
}

// Cache type alias for better readability
type Cache = Arc<DashMap<String, String>>;

fn load_mock_data() -> Cache {
    let cache: Cache = Arc::new(DashMap::new());
    
    match File::open("mock_data.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some((key, value)) = line.split_once(':') {
                        // Validate that both key and value are numeric
                        if key.chars().all(|c| c.is_ascii_digit()) && 
                           value.chars().all(|c| c.is_ascii_digit()) {
                            cache.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
            info!("Loaded {} key-value pairs from mock_data.txt", cache.len());
        }
        Err(e) => {
            warn!("Failed to load mock_data.txt: {}", e);
        }
    }
    
    cache
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting cache service...");

    // Initialize the cache with mock data
    let cache = load_mock_data();

    // Create gRPC service
    let grpc_service = CacheGrpcService::new(cache.clone());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST])
        .allow_headers(vec![header::CONTENT_TYPE]);

    // Build our REST application with a route
    let app = Router::new()
        .route("/lookup", post(lookup_handler))
        .layer(cors)
        .with_state(cache);

    // Create gRPC server
    let grpc_addr = "[::1]:50051".parse().unwrap();
    let grpc_server = tonic::transport::Server::builder()
        .add_service(grpc::cache::cache_service_server::CacheServiceServer::new(grpc_service))
        .serve(grpc_addr);

    // Run both servers concurrently
    let rest_addr: std::net::SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let rest_server = axum::serve(tokio::net::TcpListener::bind(rest_addr).await.unwrap(), app);

    info!("REST server running on http://127.0.0.1:3000");
    info!("gRPC server running on http://[::1]:50051");

    tokio::select! {
        _ = rest_server => {}
        _ = grpc_server => {}
    }
}

async fn lookup_handler(
    axum::extract::State(cache): axum::extract::State<Cache>,
    Json(payload): Json<LookupRequest>,
) -> Json<LookupResponse> {
    let mut found = std::collections::HashMap::new();
    let mut missing = Vec::new();

    // Process each key
    for key in payload.keys {
        // Validate that the key is numeric
        if !key.chars().all(|c| c.is_ascii_digit()) {
            warn!("Invalid numeric key received: {}", key);
            missing.push(key);
            continue;
        }

        if let Some(value) = cache.get(&key) {
            found.insert(key.clone(), value.clone());
        } else {
            missing.push(key);
        }
    }

    // If we have missing keys, forward them to the upstream service
    if !missing.is_empty() {
        warn!("Found {} missing keys, forwarding to upstream service", missing.len());
        // TODO: Implement upstream service call here
        // This is where you would make an HTTP request to your upstream service
        // and potentially update the cache with new values
    }

    Json(LookupResponse { found, missing })
}

// Helper function to update the cache (can be called from the upstream service handler)
async fn update_cache(cache: &Cache, key: String, value: String) {
    // Validate that both key and value are numeric before inserting
    if key.chars().all(|c| c.is_ascii_digit()) && value.chars().all(|c| c.is_ascii_digit()) {
        cache.insert(key, value);
    } else {
        warn!("Attempted to insert non-numeric key-value pair");
    }
}
