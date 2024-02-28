use datadog_api_client::{
    client::{ClientBuilder, Config},
    models::metrics::{Point, Series},
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn submit_metric() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(mock_server.uri()),
    })
    .set_site(Some(mock_server.uri()));
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
        .submit_metrics(Series {
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
        })
        .await;

    response.unwrap();
    // assert!(response.is_ok());
}
