use std::{sync::Arc, time::Duration};

use apihules::{
    adapters::http::app_state::AppState,
    infra::{
        app::create_app,
        config::{AppConfig, MongoConfig},
    },
};
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use mongodb::Client;
use tower::ServiceExt;

#[tokio::test]
async fn health_endpoint_returns_ok_without_touching_mongo() {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("test MongoDB client URI should be valid");
    let config = Arc::new(AppConfig {
        server_address: "127.0.0.1:3001".to_owned(),
        cors_allowed_origin: "http://localhost:5173".to_owned(),
        mongo: MongoConfig {
            uri: "mongodb://localhost:27017".to_owned(),
            database_name: "apihules_test".to_owned(),
            app_name: "apihules_test".to_owned(),
            max_pool_size: None,
            min_pool_size: None,
            max_idle_time: Some(Duration::from_secs(60)),
        },
    });
    let state = AppState::new(config, Arc::new(client.database("apihules_test")));
    let app = create_app(state).expect("app should be created");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("request should complete");

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid JSON");

    assert_eq!(json["status"], "ok");
}
