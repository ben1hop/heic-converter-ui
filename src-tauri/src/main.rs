// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axum::{
    routing::post,
    Router,
    Json,
};
use serde::Deserialize;
use std::process::Command;
use tower_http::cors::{CorsLayer, Any};

#[derive(Deserialize)]
struct ConvertRequest {
    directory: String,
    format: String,
    delete_original: bool,
}

async fn convert_handler(Json(payload): Json<ConvertRequest>) -> String {
    let output = Command::new("./convert_heic.sh")
        .arg(&payload.directory)
        .arg(format!("--format={}", payload.format))
        .arg(if payload.delete_original { "--delete" } else { "" })
        .output();

    match output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).to_string()
        }
        Ok(o) => format!("Script failed: {}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => format!("Error running script: {}", e),
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
      .allow_origin(Any)
      .allow_methods(Any)
      .allow_headers(Any);
    
    let app = Router::new().route("/convert", post(convert_handler)).layer(cors);

    println!("Server running at http://127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}