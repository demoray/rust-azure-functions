// Copyright (C) Microsoft Corporation. All rights reserved.

use anyhow::Result;
use axum::extract::State;
use axum::{http::StatusCode, routing::get, Router, Server};
use azure_identity::{AutoRefreshingTokenCredential, DefaultAzureCredential};
use azure_mgmt_compute::Client as ComputeClient;
use futures::StreamExt;
use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;

async fn search(State(compute_client): State<ComputeClient>) -> Result<String, StatusCode> {
    let subscription_id = std::env::var("SUBSCRIPTION_ID").map_err(|e| {
        error!("missing subscription id: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut stream = compute_client
        .virtual_machines_client()
        .list_all(subscription_id)
        .into_stream();

    let mut results = vec![];
    while let Some(entry) = stream.next().await {
        let entry = entry.map_err(|e| {
            error!("error: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        for value in entry.value {
            results.push(value.resource);
        }
    }

    Ok(format!("{results:#?}\n"))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let port: u16 = match std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };


    let default_creds = Arc::new(DefaultAzureCredential::default());
    let credential = Arc::new(AutoRefreshingTokenCredential::new(default_creds));
    let compute_client = azure_mgmt_compute::Client::builder(credential).build();

    info!("compute client created");

    let api = Router::new()
        .route("/api/endpoint", get(search))
        .with_state(compute_client);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("starting service on {}", addr);

    Server::bind(&addr).serve(api.into_make_service()).await?;

    Ok(())
}
