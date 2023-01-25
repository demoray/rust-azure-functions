// Copyright (C) Microsoft Corporation. All rights reserved.

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router, Server};
use log::info;
use std::net::SocketAddr;

async fn hello() -> Result<String, StatusCode> {
    Ok(String::from("hello from rust!"))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let port: u16 = match std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let api = Router::new().route("/api/hello", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("starting service on {}", addr);

    Server::bind(&addr).serve(api.into_make_service()).await?;

    Ok(())
}
