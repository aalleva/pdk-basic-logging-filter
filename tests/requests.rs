// Copyright 2023 Salesforce, Inc. All rights reserved.

use pdk_test::services::flex::{ApiConfig, FlexConfig, PolicyConfig, Flex};
use pdk_test::{pdk_test, TestComposite};
use pdk_test::port::Port;
use pdk_test::services::httpmock::{HttpMockConfig, HttpMock};

use common::*;

mod common;

// Flex port for the internal test network
const FLEX_PORT: Port = 8081;

// This integration test shows how to build a test to compose a local-flex instance
// with a MockServer backend
#[pdk_test]
async fn pdk_basic_logging_filter() -> anyhow::Result<()> {

    // Configure an HttpMock service
    let backend_config = HttpMockConfig::builder()
        .hostname("backend")
        .port(80)
        .build();

    // Configure a policy for the API
    let policy_config = PolicyConfig::builder()
        .name(POLICY_NAME)
        .configuration(serde_json::json!({}))
        .build();

    // Configure the API to deploy to the Flex
    let api_config = ApiConfig::builder()
        .name("ingress-http")
        .upstream(&backend_config)
        .path("/proxy/api/")
        .port(FLEX_PORT)
        .policies([policy_config])
        .build();

    // Configure the Flex service
    let flex_config = FlexConfig::builder()
        .version("1.7.0")
        .hostname("local-flex")
        .config_mounts([
            (POLICY_DIR, "policy"),
            (COMMON_CONFIG_DIR, "common")
        ])
        .with_api(api_config)
        .build();

    // Register HTTPMock service and start the docker network
    let composite = TestComposite::builder()
        .with_service(flex_config)
        .with_service(backend_config)
        .build()
        .await?;

    // Get the httpmock handle
    let httpmock: HttpMock = composite.service()?;

    // Get the Flex service handle
    let flex: Flex = composite.service()?; 

    // Get the URL for our configured port
    let flex_url = flex.external_url(FLEX_PORT).unwrap();

    // Connect the mock server
    let mock_server = httpmock::MockServer::connect_async(httpmock.socket()).await;

    // Configure the endpoint mocks
    mock_server.mock_async(|when, then| {
        when.path_contains("/ping");
        then.status(200).body("pong");
    }).await;

    // Hit the endpoint
    let response = reqwest::get(format!("{flex_url}/ping")).await?;

    // Assert the response
    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await?, "pong");

    Ok(())
}
