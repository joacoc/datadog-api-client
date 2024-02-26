use serde_derive::{Deserialize, Serialize};

use super::client::{Request, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApmRetentionFilter {
    /// The attributes of the retention filter
    pub attributes: ApmRetentionFilterAttributes,
    /// The ID of the retention filter.
    /// This value is set by the API.
    pub id: String,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterType::ApmRetentionFilter]
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApmRetentionFilterType {
    /// Equals to `apm_retention_filter`.
    #[serde(rename = "apm_retention_filter")]
    ApmRetentionFilter,
}

impl From<&str> for ApmRetentionFilterType {
    fn from(s: &str) -> Self {
        match s {
            "apm_retention_filter" => ApmRetentionFilterType::ApmRetentionFilter,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct Filter {
    /// The search query - following the [span search syntax](https://docs.datadoghq.com/tracing/trace_explorer/query_syntax/).
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
            _ => unimplemented!(),
        }
    }
}

impl ToString for FilterType {
    fn to_string(&self) -> String {
        match self {
            FilterType::SpansSamplingProcessor => "spans-sampling-processor".to_string(),
            FilterType::SpansErrorsSamplingProcessor => todo!(),
            FilterType::SpansAppsecSamplingProcessor => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApmRetentionFilterRequestData {
    /// The attributes of the retention filter
    pub attributes: CreateApmRetentionFilterAttributes,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterType,
}

// TODO: Handle correct Response/Data type
// pub type CreateApmRetentionFilterType = ApmRetentionFilterType;
// pub type CreateApmRetentionFilterAttributesFilter = Filter;
// pub type CreateApmRetentionFilterAttributesFilterType = FilterType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApmRetentionFilterAttributes {
    /// The status of the retention filter (Enabled/Disabled).
    pub enabled: bool,
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

pub struct GetApmRetentionFilterRequest {
    pub id: String,
}

pub type ReOrderApmRetentionFiltersRequestData = Vec<ReOrderApmRetentionFilters>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReOrderApmRetentionFilters {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: ApmRetentionFilterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApmRetentionFilterAttributes {
    /// The status of the retention filter (Enabled/Disabled).
    pub enabled: bool,
    pub filter: Filter,
    /// The type of retention filter.
    pub filter_type: FilterType,
    /// The name of the retention filter.
    pub name: String,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    pub rate: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApmRetentionFilterRequestData {
    pub attributes: UpdateApmRetentionFilterAttributes,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: ApmRetentionFilterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteApmRetentionFilterRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApmRetentionFilterResponseData {
    /// The attributes of the retention filter
    pub attributes: CreateApmRetentionFilterAttributes,
    #[serde(rename = "type")]
    /// The type of the resource. Allowed enum values: [ApmRetentionFilterType::ApmRetentionFilter]
    ///
    /// Default: [ApmRetentionFilterType::ApmRetentionFilter]
    pub typ: ApmRetentionFilterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApmRetentionFilterResponseData {}

pub type ListApmRetentionFiltersResponse = Response<Vec<ApmRetentionFilter>>;
pub type ApmRetentionFilterResponse = Response<ApmRetentionFilter>;
pub type CreateApmRetentionFilterResponse = Response<CreateApmRetentionFilterResponseData>;
pub type GetApmRetentionFilterResponse = Response<ApmRetentionFilter>;
pub type UpdateApmRetentionFilterResponse = Response<UpdateApmRetentionFilterResponseData>;

pub type CreateApmRetentionFilterRequest = Request<CreateApmRetentionFilterRequestData>;
pub type ReOrderApmRetentionFiltersRequest = Request<ReOrderApmRetentionFiltersRequestData>;
pub type UpdateApmRetentionFilterRequest = Request<UpdateApmRetentionFilterRequestData>;
