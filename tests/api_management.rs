use datadog_api_client::{
    client::{ClientBuilder, Config},
    models::api_management::{OpenApiCreateRequest, OpenApiUpdateRequest},
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn create_openapi() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let client = client_builder.build().expect("Client");
    let body = r#"
        {
            "data": {
                "attributes": {
                    "failed_endpoints": [
                        {
                            "method": "string",
                            "path": "string"
                        }
                    ]
                },
                "id": "e467165b-4814-4c03-acd1-719cd4f22365"
            }
        }
    "#;

    let response = ResponseTemplate::new(201).set_body_raw(body, "application/json");
    Mock::given(method("POST"))
        .and(path("/api/v2/apicatalog/openapi"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    client
        .create_openapi(OpenApiCreateRequest {
            openapi_spec_file: vec![],
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn get_api_management() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let body = r#"{}"#;

    let response = ResponseTemplate::new(201).set_body_raw(body, "application/json");
    Mock::given(method("GET"))
        .and(path("api/v2/apicatalog/api/123/openapi"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let client = client_builder.build().expect("Client");
    client.get_openapi("123").await.unwrap();
}

#[tokio::test]
async fn update_api_management() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });

    let body = r#"
        {
            "data": {
                "attributes": {
                    "failed_endpoints": [
                    {
                        "method": "string",
                        "path": "string"
                    }
                    ]
                },
                "id": "90646597-5fdb-4a17-a240-647003f8c028"
            }
        }
    "#;

    let response = ResponseTemplate::new(200).set_body_raw(body, "application/json");
    Mock::given(method("PUT"))
        .and(path("api/v2/apicatalog/api/123/openapi"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let client = client_builder.build().expect("Client");
    client
        .update_openapi(
            "123",
            OpenApiUpdateRequest {
                openapi_spec_file: vec![],
            },
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_api_management() {
    let mock_server = MockServer::start().await;
    let response = ResponseTemplate::new(204);
    Mock::given(method("DELETE"))
        .and(path("api/v2/apicatalog/api/123"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let client = client_builder.build().expect("Client");
    client.delete_openapi("123").await.unwrap();
}
