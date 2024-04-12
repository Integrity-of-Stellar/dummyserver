use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct House {
    id: String,
    owner: String,
    location: Vector3,
    rotation: Vector3,
    scale: Vector3,
}

async fn get_houses() -> impl Responder {
    // Read JSON file
    let contents = fs::read_to_string("houses.json").expect("Unable to read file");

    // Deserialize JSON to vector of House structs
    let houses: Vec<House> = serde_json::from_str(&contents).expect("Invalid JSON");

    // Convert vector of House structs to JSON string
    let json_response = serde_json::to_string(&houses).expect("Unable to serialize to JSON");

    // Return JSON response
    json_response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            // Define route for GET /api/public/land/houses
            .route("/api/public/land/houses", web::get().to(get_houses))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
