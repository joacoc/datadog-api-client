use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use super::client::PagedResponse;

#[derive(Debug, Serialize, Deserialize)]
/// Type of the event. Allowed enum values: [AuditLogsResponseType::Audit]
///
/// default: [AuditLogsResponseType::Audit]
pub enum AuditLogsResponseType {
    #[serde(rename = "audit")]
    /// Audit type.
    Audit,
}

#[derive(Debug, Serialize, Deserialize)]
/// Attributes object containing all event attributes and their associated values.
pub struct AuditLogsResponseAttributes {
    /// JSON object of attributes from Audit Logs events.
    pub attributes: Value,
    /// Message of the event.
    pub message: String,
    /// Name of the application or service generating Audit Logs events. This name is used to correlate Audit Logs to APM, so make sure you specify the same value when you use both products.
    pub service: String,
    /// Array of tags associated with your event.
    pub tags: Vec<String>,
    // TODO: Use a datetime type.
    /// Timestamp of your event.
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Search and filter query settings.
pub struct AuditLogsSearchFilter {
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
/// Global query options that are used during the query.
///
/// Note: Specify either timezone or time offset, not both. Otherwise, the query fails.
pub struct AuditLogsSearchOptions {
    /// Time offset (in seconds) to apply to the query.
    pub time_offset: Option<i64>,
    /// The timezone can be specified as GMT, UTC, an offset from UTC (like UTC+1), or as a Timezone Database identifier (like America/New_York).
    ///
    /// default: `UTC`
    pub timezone: Option<String>,
}

/// Paging attributes for listing events.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogsSearchPage {
    /// List following results with a cursor provided in the previous query.
    pub cursor: Option<String>,
    /// Maximum number of events in the response.
    /// default: `10`
    pub limit: i32,
}

/// Sort parameters when querying events.
#[derive(Debug, Serialize, Deserialize)]
/// Sort parameters when querying events.
///
/// Allowed enum values: [AuditLogsSort::Timestamp],[AuditLogsSort::NegativeTimestamp]
pub enum AuditLogsSort {
    #[serde(rename = "timestamp")]
    /// Sorts in ordered timestamps.
    Timestamp,
    #[serde(rename = "-timestamp")]
    /// Sorts in reverse order timestamps.
    NegativeTimestamp,
}

#[derive(Debug, Serialize, Deserialize)]
/// Response object with all events matching the request and pagination information.
pub struct AuditLogsResponse {
    /// Attributes object containing all event attributes and their associated values.
    pub attributes: AuditLogsResponseAttributes,
    /// Unique ID of the event.
    pub id: String,
    #[serde(rename = "type")]
    /// Type of the event. Allowed enum values: [AuditLogsResponseType::Audit]
    ///
    /// default: [AuditLogsResponseType::Audit]
    pub typ: AuditLogsResponseType,
}

// Requests and responses

#[derive(Debug, Serialize, Deserialize)]
/// Request query object to search audit logs.
pub struct AuditLogsSearchRequest {
    /// Search and filter query settings.
    pub filter: AuditLogsSearchFilter,
    /// Global query options that are used during the query.
    ///
    /// Note: Specify either timezone or time offset, not both. Otherwise, the query fails.
    pub options: AuditLogsSearchOptions,
    /// Paging attributes for listing events.
    pub page: AuditLogsSearchPage,
    /// Sort parameters when querying events.
    pub sort: AuditLogsSearchSort,
}

#[derive(Debug, Serialize, Deserialize)]
/// Request query object to list audit logs.
pub struct AuditLogsListRequest {
    /// Search query following Audit Logs syntax.
    pub filter_query: Option<String>,
    /// Minimum timestamp for requested events.
    pub filter_from: Option<String>,
    /// Maximum timestamp for requested events.
    pub filter_to: Option<String>,
    /// Order of events in results.
    pub sort: Option<ListAuditLogsSort>,
    /// List following results with a cursor provided in the previous query.
    pub cursor: Option<String>,
    /// Maximum number of events in the response.
    pub page: Option<String>,
}

/// Sort parameters when querying events.
///
/// Allowed enum values: [AuditLogsSort::Timestamp],[AuditLogsSort::NegativeTimestamp]
pub type AuditLogsSearchSort = AuditLogsSort;
/// Sort parameters when querying events.
///
/// Allowed enum values: [AuditLogsSort::Timestamp],[AuditLogsSort::NegativeTimestamp]
pub type ListAuditLogsSort = AuditLogsSort;
/// Response object with all events matching the request and pagination information.
pub type AuditLogsListResponse = PagedResponse<AuditLogsResponse>;
/// Response object with all events matching the request and pagination information.
pub type AuditLogsSearchResponse = PagedResponse<AuditLogsResponse>;
