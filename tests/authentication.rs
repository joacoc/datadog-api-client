use datadog_api_client::client::{ClientBuilder, Config};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn authenticate_api_key() {
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
            "valid": true
        }
    "#;

    let response = ResponseTemplate::new(201).set_body_raw(body, "application/json");
    Mock::given(method("GET"))
        .and(path("api/v1/validate"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let validation = client.validate_api_key().await.unwrap();

    assert!(validation.valid);
}
