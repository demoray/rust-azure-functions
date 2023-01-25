// Copyright (C) Microsoft Corporation. All rights reserved.

use anyhow::Result;
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router, Server};
use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    value: String,
}

async fn echo_get(search: Query<Request>) -> Result<String, StatusCode> {
    Ok(format!("echo from get: {search:?}\n"))
}

async fn echo_post(Json(search): Json<Request>) -> Result<String, StatusCode> {
    Ok(format!("echo from post: {search:?}\n"))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let port: u16 = match std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let api = Router::new().route("/api/endpoint", get(echo_get).post(echo_post));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("starting service on {}", addr);

    Server::bind(&addr).serve(api.into_make_service()).await?;

    Ok(())
}
