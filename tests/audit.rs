use datadog_api_client::{
    client::{ClientBuilder, Config},
    models::audit::{
        AuditLogsListRequest, AuditLogsSearchFilter, AuditLogsSearchOptions, AuditLogsSearchPage,
        AuditLogsSearchRequest, AuditLogsSearchSort,
    },
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn search_audit_logs() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let client = client_builder.build().expect("Client");
    let body = r#"
        {
            "data": [
                {
                    "attributes": {
                        "attributes": {
                            "customAttribute": 123,
                            "duration": 2345
                        },
                        "message": "string",
                        "service": "web-app",
                        "tags": [
                            "team:A"
                        ],
                        "timestamp": "2019-01-02T09:42:36.320Z"
                    },
                    "id": "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA",
                    "type": "audit"
                }
            ],
            "links": {
                "next": "https://app.datadoghq.com/api/v2/audit/event?filter[query]=foo\u0026page[cursor]=eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ=="
            },
            "meta": {
                "elapsed": 132,
                "page": {
                    "after": "eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ=="
                },
                "request_id": "MWlFUjVaWGZTTTZPYzM0VXp1OXU2d3xLSVpEMjZKQ0VKUTI0dEYtM3RSOFVR",
                "status": "done",
                "warnings": [
                    {
                    "code": "unknown_index",
                    "detail": "indexes: foo, bar",
                    "title": "One or several indexes are missing or invalid, results hold data from the other indexes"
                    }
                ]
            }
        }
    "#;

    let response = ResponseTemplate::new(201).set_body_raw(body, "application/json");
    Mock::given(method("GET"))
        .and(path("api/v2/audit/events/search"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let audit_logs = client
        .search_audit_logs(AuditLogsSearchRequest {
            filter: AuditLogsSearchFilter {
                from: "now-15m".to_string(),
                query: "*".to_string(),
                to: "now".to_string(),
            },
            options: AuditLogsSearchOptions {
                time_offset: None,
                timezone: Some("GMT".to_string()),
            },
            page: AuditLogsSearchPage {
                cursor: None,
                limit: 25,
            },
            sort: AuditLogsSearchSort::Timestamp,
        })
        .await
        .unwrap();

    assert!(audit_logs.links.is_some());
    assert!(audit_logs.meta.is_some());
}

#[tokio::test]
async fn list_audit_logs() {
    let mock_server = MockServer::start().await;
    let client_builder = ClientBuilder::new(Config {
        api_key: None,
        application_key: None,
        site: Some(datadog_api_client::client::Site::Custom(mock_server.uri())),
    });
    let body = r#"
        {
            "data": [{
                "attributes": {
                    "attributes": {
                        "customAttribute": 123,
                        "duration": 2345
                    },
                    "message": "string",
                    "service": "web-app",
                    "tags": [
                        "team:A"
                    ],
                    "timestamp": "2019-01-02T09:42:36.320Z"
                    },
                    "id": "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA",
                    "type": "audit"
            }],
            "links": {
                "next": "https://app.datadoghq.com/api/v2/audit/event?filter[query]=foo\u0026page[cursor]=eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ=="
            },
            "meta": {
                "elapsed": 132,
                "page": {
                    "after": "eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ=="
                },
                "request_id": "MWlFUjVaWGZTTTZPYzM0VXp1OXU2d3xLSVpEMjZKQ0VKUTI0dEYtM3RSOFVR",
                "status": "done",
                "warnings": [
                    {
                    "code": "unknown_index",
                    "detail": "indexes: foo, bar",
                    "title": "One or several indexes are missing or invalid, results hold data from the other indexes"
                    }
                ]
            }
        }
    "#;

    let response = ResponseTemplate::new(200).set_body_raw(body, "application/json");
    Mock::given(method("GET"))
        .and(path("api/v2/audit/events"))
        .respond_with(response)
        .mount(&mock_server)
        .await;

    let client = client_builder.build().expect("Client");
    client
        .list_audit_logs(AuditLogsListRequest {
            cursor: None,
            filter_query: None,
            filter_from: None,
            filter_to: None,
            sort: None,
            page: None,
        })
        .await
        .unwrap();
}
