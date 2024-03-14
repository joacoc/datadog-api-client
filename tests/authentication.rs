use datadog_api_client::{
    client::{ClientBuilder, Config},
    error::Error,
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn authenticate_api_key() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new().set_config(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
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

#[tokio::test]
async fn authenticate_invalid_api_key() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new().set_config(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let client = client_builder.build().expect("Client");

    let body = r#"{"errors":["Forbidden"]}"#;
    let response = ResponseTemplate::new(403).set_body_raw(body, "application/json");
    Mock::given(method("GET"))
        .and(path("api/v1/validate"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let validation = client.validate_api_key().await;
    match validation {
        Ok(_) => panic!(),
        Err(e) => match e {
            Error::RequestTransport(_) => panic!(),
            Error::InvalidUrl(_) => panic!(),
            Error::InvalidRequest(e) => {
                let err_vec = e.0;
                assert!(err_vec.get(0).unwrap() == "Forbidden");
            }
            Error::InvalidRequestQueryHeaders(_) => panic!(),
        },
    }
}
