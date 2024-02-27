use datadog_api_client::{
    client::ClientBuilder,
    models::apm_retention_filters::{
        ApmRetentionFilterCreateRequest, ApmRetentionFilterCreateRequestData,
        ApmRetentionFilterCreateRequestDataAttributes, ApmRetentionFilterCreateRequestDataType,
        ApmRetentionFilterUpdateRequest, ApmRetentionFilterUpdateRequestData,
        ApmRetentionFilterUpdateRequestDataAttributes, ApmRetentionFilterUpdateRequestDataType,
        Filter, FilterType,
    },
};
use url::Url;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn list_retention_filters() {
    let mock_server = MockServer::start().await;
    let client_builder =
        ClientBuilder::new(&"&", &"").set_api_url(Url::parse(&mock_server.uri()).unwrap());
    let client = client_builder.build().expect("Client");

    // List
    let body = r#"
        {
            "data": [{
                "attributes": {
                    "created_at": 1,
                    "created_by": "string",
                    "editable": true,
                    "enabled": true,
                    "execution_order": 1,
                    "filter": {
                        "query": "@http.status_code:200 service:my-service"
                    },
                    "filter_type": "spans-errors-sampling-processor",
                    "modified_at": 1,
                    "modified_by": "string",
                    "name": "my retention filter",
                    "rate": 1
                },
                "id": "7RBOb7dLSYWI01yc3pIH8w",
                "type": "apm_retention_filter"
            }]
        }
    "#;

    let response = ResponseTemplate::new(200).set_body_raw(body, "application/json");
    Mock::given(method("GET"))
        .and(path("/api/v2/apm/config/retention-filters"))
        .respond_with(response)
        .mount(&mock_server)
        .await;
    client.list_apm_retention_filters().await.unwrap();
}

#[tokio::test]
async fn create_retention_filter() {
    let mock_server = MockServer::start().await;
    let client_builder =
        ClientBuilder::new(&"&", &"").set_api_url(Url::parse(&mock_server.uri()).unwrap());
    let client = client_builder.build().expect("Client");

    // Create
    let body = r#"
        {
            "data": {
            "attributes": {
                "created_at": 123,
                "created_by": "string",
                "editable": true,
                "enabled": true,
                "execution_order": 1,
                "filter": {
                    "query": "@http.status_code:200 service:my-service"
                },
                "filter_type": "spans-sampling-processor",
                "modified_at": 123,
                "modified_by": "string",
                "name": "my retention filter",
                "rate": 1
            },
            "id": "7RBOb7dLSYWI01yc3pIH8w",
            "type": "apm_retention_filter"
            }
        }
    "#;
    let response = ResponseTemplate::new(200).set_body_raw(body, "application/json");
    Mock::given(method("POST"))
        .and(path("/api/v2/apm/config/retention-filters"))
        .respond_with(response)
        .mount(&mock_server)
        .await;
    client
        .create_apm_retention_filter(ApmRetentionFilterCreateRequest {
            data: ApmRetentionFilterCreateRequestData {
                attributes: ApmRetentionFilterCreateRequestDataAttributes {
                    enabled: true,
                    filter: datadog_api_client::models::apm_retention_filters::Filter { query: "@http.status_code:200 service:my-service".to_string() },
                    filter_type: datadog_api_client::models::apm_retention_filters::FilterType::SpansSamplingProcessor,
                    name: "my retention filter".to_string(),
                    rate: 1.0
                },
                typ: ApmRetentionFilterCreateRequestDataType::ApmRetentionFilter,
            },
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn get_retention_filter() {
    let mock_server = MockServer::start().await;
    let client_builder =
        ClientBuilder::new(&"&", &"").set_api_url(Url::parse(&mock_server.uri()).unwrap());
    let client = client_builder.build().expect("Client");

    let body = r#"
    {
        "data": {
          "attributes": {
            "created_at": 1,
            "created_by": "string",
            "editable": true,
            "enabled": true,
            "execution_order": 1,
            "filter": {
              "query": "@http.status_code:200 service:my-service"
            },
            "filter_type": "spans-sampling-processor",
            "modified_at": 2,
            "modified_by": "string",
            "name": "my retention filter",
            "rate": 1
          },
          "id": "7RBOb7dLSYWI01yc3pIH8w",
          "type": "apm_retention_filter"
        }
      }
    "#;
    let response = ResponseTemplate::new(200).set_body_raw(body, "application/json");
    Mock::given(method("GET"))
        .and(path("/api/v2/apm/config/retention-filters/123"))
        .respond_with(response)
        .mount(&mock_server)
        .await;
    client.get_apm_retention_filter("123").await.unwrap();
}

#[tokio::test]
async fn update_retention_filter() {
    let mock_server = MockServer::start().await;
    let client_builder =
        ClientBuilder::new(&"&", &"").set_api_url(Url::parse(&mock_server.uri()).unwrap());
    let client = client_builder.build().expect("Client");

    let body = r#"
    {
        "data": {
        "attributes": {
            "created_at": "integer",
            "created_by": "string",
            "editable": true,
            "enabled": true,
            "execution_order": "integer",
            "filter": {
            "query": "@http.status_code:200 service:my-service"
            },
            "filter_type": "spans-sampling-processor",
            "modified_at": "integer",
            "modified_by": "string",
            "name": "my retention filter",
            "rate": 1
        },
        "id": "7RBOb7dLSYWI01yc3pIH8w",
        "type": "apm_retention_filter"
        }
    }
"#;
    let response = ResponseTemplate::new(200).set_body_raw(body, "application/json");
    Mock::given(method("PUT"))
        .and(path("/api/v2/apm/config/retention-filters/123"))
        .respond_with(response)
        .mount(&mock_server)
        .await;
    client
        .update_apm_retention_filter(ApmRetentionFilterUpdateRequest {
            data: ApmRetentionFilterUpdateRequestData {
                id: "123".to_string(),
                typ: ApmRetentionFilterUpdateRequestDataType::ApmRetentionFilter,
                attributes: ApmRetentionFilterUpdateRequestDataAttributes {
                    enabled: true,
                    filter: Filter {
                        query: "select 1".to_string(),
                    },
                    filter_type: FilterType::SpansAppsecSamplingProcessor,
                    name: "my retention filter".to_string(),
                    rate: 1.0,
                },
            },
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_retention_filters() {
    let mock_server = MockServer::start().await;
    let client_builder =
        ClientBuilder::new(&"&", &"").set_api_url(Url::parse(&mock_server.uri()).unwrap());
    let client = client_builder.build().expect("Client");

    let response = ResponseTemplate::new(200);
    Mock::given(method("DELETE"))
        .and(path("/api/v2/apm/config/retention-filters/123"))
        .respond_with(response)
        .mount(&mock_server)
        .await;
    client.delete_apm_retention_filter("123").await.unwrap();
}
