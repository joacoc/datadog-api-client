use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::models::audit::{
    AuditLogsListRequest, AuditLogsListResponse, AuditLogsSearchRequest, AuditLogsSearchResponse,
};

static BASE_PATH: &str = "api/v2/audit/events";

impl Client {
    /// List endpoint returns Audit Logs events that match an Audit search query.
    /// [Results are paginated](https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination/?tab=v1api).
    /// Use this endpoint to build complex Audit Logs events filtering and search.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/audit/#search-audit-logs-events)
    pub async fn search_audit_logs(
        &self,
        request: AuditLogsSearchRequest,
    ) -> Result<AuditLogsSearchResponse, Error> {
        let req: reqwest::RequestBuilder =
            self.build_request(Method::GET, &format!("{}/search", BASE_PATH))?;
        let req = req.json(&request);

        self.send_request::<AuditLogsSearchResponse>(req).await
    }

    /// List endpoint returns events that match a Audit Logs search query.
    /// [Results are paginated](https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination/?tab=v1api).
    /// Use this endpoint to see your latest Audit Logs events.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/audit/#get-a-list-of-audit-logs-events)
    pub async fn list_audit_logs(
        &self,
        request: AuditLogsListRequest,
    ) -> Result<AuditLogsListResponse, Error> {
        let query = serde_qs::to_string(&request)?;
        let req = self.build_request(Method::GET, &format!("{}?{}", BASE_PATH, query))?;

        self.send_request::<AuditLogsListResponse>(req).await
    }
}
