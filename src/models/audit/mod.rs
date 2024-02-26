use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use super::client::PagedResponse;

#[derive(Debug, Serialize, Deserialize)]
pub enum AuditTypes {
    #[serde(rename = "audit")]
    Audit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogsEvents {
    pub attributes: Value,
    pub message: String,
    pub service: String,
    pub tags: Vec<String>,
    // TODO: Use a datetime type.
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAuditLogsEvents {
    pub attributes: AuditLogsEvents,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: AuditTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAuditLogsEventsFilter {
    /// Minimum time for the requested events. Supports date, math, and regular timestamps (in milliseconds).
    ///
    /// default: `now-15m`
    pub from: String,
    /// Search query following the Audit Logs search syntax.
    ///
    /// default: `*`
    pub query: String,
    /// Maximum time for the requested events. Supports date, math, and regular timestamps (in milliseconds).
    ///
    /// default: `now`
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAuditLogsEventsOptions {
    /// Time offset (in seconds) to apply to the query.
    pub time_offset: i64,
    /// The timezone can be specified as GMT, UTC, an offset from UTC (like UTC+1), or as a Timezone Database identifier (like America/New_York).
    ///
    /// default: `UTC`
    pub timezone: String,
}

/// Paging attributes for listing events.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAuditLogsEventsPage {
    /// List following results with a cursor provided in the previous query.
    pub cursor: Option<String>,
    /// Maximum number of events in the response.
    /// default: `10`
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAuditLogsEventsRequest {
    /// Search and filter query settings.
    pub filter: SearchAuditLogsEventsFilter,
    /// Global query options that are used during the query.
    ///
    /// Note: Specify either timezone or time offset, not both. Otherwise, the query fails.
    pub options: SearchAuditLogsEventsOptions,
    /// Paging attributes for listing events.
    pub page: SearchAuditLogsEventsPage,
    /// Sort parameters when querying events.
    pub sort: SearchAuditLogsEventsSort,
}

/// Sort parameters when querying events.
#[derive(Debug, Serialize, Deserialize)]
pub enum AuditLogsEventsSort {
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "-timestamp")]
    NegativeTimestamp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAuditLogsEventsRequest {
    /// Search query following Audit Logs syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<String>,
    /// Order of events in results.
    pub sort: Option<ListAuditLogsEventsSort>,
    /// List following results with a cursor provided in the previous query.
    pub cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct ListAuditLogsEvents {
    pub attributes: AuditLogsEvents,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: AuditTypes,
}

pub type SearchAuditLogsEventsSort = AuditLogsEventsSort;
pub type ListAuditLogsEventsSort = AuditLogsEventsSort;
pub type ListAuditLogsEventsResponse = PagedResponse<ListAuditLogsEvents>;
pub type SearchAuditLogsEventsResponse = PagedResponse<SearchAuditLogsEvents>;
