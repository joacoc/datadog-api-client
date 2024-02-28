use datadog_api_client::{
    client::{ClientBuilder, Config},
    models::metrics::{MetricsSubmitRequest, Point, Series},
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn submit_metrics() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let client = client_builder.build().expect("Client");
    let body = r#"
        {
            "errors": []
        }
    "#;

    let response = ResponseTemplate::new(202).set_body_raw(body, "application/json");
    Mock::given(method("POST"))
        .and(path("api/v2/series"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let response = client
        .submit_metrics(MetricsSubmitRequest {
            series: vec![Series {
                interval: None,
                metadata: None,
                metric: "rust_client_new_metric".to_string(),
                points: vec![Point {
                    timestamp: 1709127093,
                    value: 10.0,
                }],
                resources: None,
                source_type_name: None,
                tags: None,
                typ: None,
                unit: None,
            }],
        })
        .await;

    response.unwrap();
}

#[tokio::test]
async fn get_metric_metadata() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let client = client_builder.build().expect("Client");

    // 1. Response body
    let body = r#"
        {
            "description": "string",
            "integration": "string",
            "per_unit": "second",
            "short_name": "string",
            "statsd_interval": 1,
            "type": "count",
            "unit": "byte"
        }
    "#;

    // 2. Status
    let response = ResponseTemplate::new(202).set_body_raw(body, "application/json");

    // 3. Method and path
    Mock::given(method("GET"))
        .and(path("api/v1/metrics/rust_client_new_metric"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    // 4. Content
    let response = client
        .get_metric_metadata("rust_client_new_metric".to_string())
        .await
        .unwrap();

    assert!(response.per_unit.is_some())
}
