use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

use super::client::{Request, Response};

#[derive(Debug, Serialize, Deserialize)]
/// The type of the resource.
pub enum ApmRetentionFilterRequestDataType {
    /// Equals to `apm_retention_filter`.
    #[serde(rename = "apm_retention_filter")]
    ApmRetentionFilter,
}

impl From<&str> for ApmRetentionFilterRequestDataType {
    fn from(s: &str) -> Self {
        match s {
            "apm_retention_filter" => ApmRetentionFilterRequestDataType::ApmRetentionFilter,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// The attributes of the retention filter.
pub struct ApmRetentionFilterAttributes {
    /// The creation timestamp of the retention filter.
    pub created_at: i64,
    /// The creator of the retention filter.
    pub created_by: String,
    /// Shows whether the filter can be edited.
    pub editable: bool,
    /// The status of the retention filter (Enabled/Disabled).
    pub enabled: bool,
    /// The execution order of the retention filter.
    pub execution_order: i64,
    /// The spans filter. Spans matching this filter will be indexed and stored.
    pub filter: Filter,
    /// The type of retention filter.
    pub filter_type: FilterType,
    /// The modification timestamp of the retention filter.
    pub modified_at: i64,
    /// The modifier of the retention filter.
    pub modified_by: String,
    /// The name of the retention filter.
    pub name: String,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    pub rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
/// The spans filter used to index spans.
pub struct Filter {
    /// The search query - following the [span search syntax](https://docs.datadoghq.com/tracing/trace_explorer/query_syntax/).
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Enum for the different filter types in an APM retention filter.
pub enum FilterType {
    /// Equals to `spans-sampling-processor`.
    #[serde(rename = "spans-sampling-processor")]
    SpansSamplingProcessor,
    /// Equals to `spans-errors-sampling-processor`.
    #[serde(rename = "spans-errors-sampling-processor")]
    SpansErrorsSamplingProcessor,
    /// Equals to `spans-appsec-sampling-processor`.
    #[serde(rename = "spans-appsec-sampling-processor")]
    SpansAppsecSamplingProcessor,
}

impl From<&str> for FilterType {
    fn from(s: &str) -> Self {
        match s {
            "spans-sampling-processor" => FilterType::SpansSamplingProcessor,
            "spans-errors-sampling-processor" => FilterType::SpansErrorsSamplingProcessor,
            "spans-appsec-sampling-processor" => FilterType::SpansAppsecSamplingProcessor,
            _ => unimplemented!(),
        }
    }
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterType::SpansSamplingProcessor => f.write_str("spans-sampling-processor"),
            FilterType::SpansErrorsSamplingProcessor => {
                f.write_str("spans-errors-sampling-processor")
            }

            FilterType::SpansAppsecSamplingProcessor => {
                f.write_str("spans-appsec-sampling-processor")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// The definition of the new retention filter.
pub struct ApmRetentionFilterCreateRequestData {
    /// The object describing the configuration of the retention filter to create/update.
    pub attributes: ApmRetentionFilterCreateRequestDataAttributes,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterCreateRequestDataType,
}

#[derive(Debug, Serialize, Deserialize)]
/// /// The object describing the configuration of the retention filter to create/update.
pub struct ApmRetentionFilterCreateRequestDataAttributes {
    /// The status of the retention filter (Enabled/Disabled).
    pub enabled: bool,
    /// The spans filter. Spans matching this filter will be indexed and stored.
    pub filter: Filter,
    /// The type of retention filter. The value should always be `spans-sampling-processor`.
    /// Allowed enum value: [FilterType::SpansSamplingProcessor].
    ///
    /// Default: [FilterType::SpansSamplingProcessor].
    pub filter_type: FilterType,
    /// The name of the retention filter.
    pub name: String,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    pub rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
/// A filter object.
pub struct ApmRetentionFilterReOrder {
    /// The ID of the retention filter.
    pub id: String,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterRequestDataType,
}

#[derive(Debug, Serialize, Deserialize)]
/// The body of the retention filter to be updated.
pub struct ApmRetentionFilterUpdateRequestData {
    /// The object describing the configuration of the retention filter to create/update.
    pub attributes: ApmRetentionFilterUpdateRequestDataAttributes,
    /// The ID of the retention filter.
    pub id: String,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterUpdateRequestDataType,
}

#[derive(Debug, Serialize, Deserialize)]
/// The body of the retention filter to be created.
pub struct ApmRetentionFilterCreateResponseData {
    /// The attributes of the retention filter
    pub attributes: ApmRetetionFilterCreateResponseDataAttributes,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterRequestDataType,
}

#[derive(Debug, Serialize, Deserialize)]
/// The body of the retention filter listed.
pub struct ApmRetentionFilterListResponseData {
    /// The attributes of the retention filter
    pub attributes: ApmRetentionFilterAttributes,
    /// The ID of the retention filter.
    pub id: String,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterRequestDataType,
}

#[derive(Debug, Serialize, Deserialize)]
/// The retention filters definition.
pub struct ApmRetentionFilterGetResponseData {
    /// The attributes of the retention filter
    pub attributes: ApmRetentionFilterAttributes,
    /// The ID of the retention filter.
    pub id: String,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterRequestDataType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterRequestDataType,
}

// Auxiliary types
/// The attributes of the retention filter.
pub type ApmRetetionFilterCreateResponseDataAttributes = ApmRetentionFilterAttributes;
/// The definition of the new retention filter.
pub type ApmRetentionFilterUpdateResponseData = ApmRetentionFilterCreateRequestData;
/// A list of retention filters objects.
pub type ApmRetentionFiltersReOrderRequestData = Vec<ApmRetentionFilterReOrder>;
/// The object describing the configuration of the retention filter to create/update.
pub type ApmRetentionFilterUpdateRequestDataAttributes =
    ApmRetentionFilterCreateRequestDataAttributes;
/// The type of the resource.
pub type ApmRetentionFilterCreateRequestDataType = ApmRetentionFilterRequestDataType;
/// The type of the resource.
pub type ApmRetentionFilterUpdateRequestDataType = ApmRetentionFilterRequestDataType;

// Requests and responses
/// An ordered list of retention filters.
pub type ApmRetentionFiltersListResponse = Response<Vec<ApmRetentionFilterListResponseData>>;

/// The retention filters definition.
pub type ApmRetentionFilterGetResponse = Response<ApmRetentionFilterGetResponseData>;

/// The retention filters definition.
pub type ApmRetentionFilterCreateResponse = Response<ApmRetentionFilterCreateResponseData>;
/// The definition of the new retention filter.
pub type ApmRetentionFilterCreateRequest = Request<ApmRetentionFilterCreateRequestData>;

/// The retention filters definition.
pub type ApmRetentionFilterUpdateResponse = Response<ApmRetentionFilterUpdateResponseData>;
/// The updated definition of the retention filter.
pub type ApmRetentionFilterUpdateRequest = Request<ApmRetentionFilterUpdateRequestData>;

/// The list of retention filters in the new order.
pub type ApmRetentionFilterReOrderRequest = Request<ApmRetentionFiltersReOrderRequestData>;
