use datadog_api_client::{
    client::ClientBuilder,
    models::audit::{
        ListAuditLogsEventsRequest, SearchAuditLogsEventsFilter, SearchAuditLogsEventsOptions,
        SearchAuditLogsEventsPage, SearchAuditLogsEventsRequest, SearchAuditLogsEventsSort,
    },
};
use url::Url;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn search_audit_logs() {
    let mock_server = MockServer::start().await;
    let client_builder =
        ClientBuilder::new(&"&", &"").set_api_url(Url::parse(&mock_server.uri()).unwrap());
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
        .search_audit_logs(SearchAuditLogsEventsRequest {
            filter: SearchAuditLogsEventsFilter {
                from: "now-15m".to_string(),
                query: "*".to_string(),
                to: "now".to_string(),
            },
            options: SearchAuditLogsEventsOptions {
                time_offset: 0,
                timezone: "GMT".to_string(),
            },
            page: SearchAuditLogsEventsPage {
                cursor: None,
                limit: 25,
            },
            sort: SearchAuditLogsEventsSort::Timestamp,
        })
        .await
        .unwrap();

    assert!(audit_logs.links.is_some());
    assert!(audit_logs.meta.is_some());
}

#[tokio::test]
async fn list_audit_logs() {
    let mock_server = MockServer::start().await;
    let client_builder =
        ClientBuilder::new(&"&", &"").set_api_url(Url::parse(&mock_server.uri()).unwrap());
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
        .list_audit_logs(ListAuditLogsEventsRequest {
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
