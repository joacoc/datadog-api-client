use serde::{Deserialize, Serialize};

/// Represents the type of metric.
#[derive(Debug, Serialize, Deserialize)]
pub enum MetricType {
    /// Unspecified type.
    Unspecified,
    /// Count type.
    Count,
    /// Rate type.
    Rate,
    /// Gauge type.
    Gauge,
}

/// Metric origin information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Origin {
    /// The origin metric type code.
    pub metric_type: i32,
    /// The origin product code.
    pub product: i32,
    /// The origin service code.
    pub service: i32,
}

/// Metadata for the metric.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Metric origin information.
    pub origin: Option<Origin>,
}

/// Points relating to a metric.
#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    /// The timestamp should be in seconds and current.
    pub timestamp: i64,
    /// The numeric value format should be a 64bit float gauge-type value.
    pub value: f64,
}

/// A list of resources to associate with this metric.
#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    /// The name of the resource.
    pub name: Option<String>,
    /// The type of the resource.
    #[serde(rename = "type")]
    pub typ: Option<String>,
}

/// Represents the timeseries data to submit.
#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    /// If the type of the metric is rate or count, define the corresponding interval.
    pub interval: Option<i64>,
    /// Metadata for the metric.
    pub metadata: Option<Metadata>,
    /// The name of the timeseries.
    pub metric: String,
    /// Points relating to a metric.
    pub points: Vec<Point>,
    /// A list of resources to associate with this metric.
    pub resources: Option<Vec<Resource>>,
    /// The source type name.
    pub source_type_name: Option<String>,
    /// A list of tags associated with the metric.
    pub tags: Option<Vec<String>>,
    /// The type of metric.
    #[serde(rename = "type")]
    pub typ: Option<MetricType>,
    /// The unit of point value.
    pub unit: Option<String>,
}

/// Metric type, such as `gauge` or `rate`.

/// Unlike [MetricType],
/// [MetricMetadataType] serializes as a string,
/// whereas [MetricType] serializes as an integer.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetricMetadataType {
    /// Gauge metric type.
    #[serde(rename = "gauge")]
    Gauge,
    /// Rate metric type.
    #[serde(rename = "rate")]
    Rate,
    /// Count metric type.
    #[serde(rename = "count")]
    Count,
    /// Unspecified metric type.
    #[serde(rename = "unspecified")]
    Unspecified,
}

impl Default for MetricType {
    fn default() -> Self {
        MetricType::Unspecified
    }
}

/// Represents a metric with detailed information.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetricMetadata {
    /// Metric description.
    pub description: String,
    /// Name of the integration that sent the metric if applicable.
    pub integration: Option<String>,
    /// Per unit of the metric such as `second` in `bytes per second`.
    pub per_unit: Option<String>,
    /// A more human-readable and abbreviated version of the metric name.
    pub short_name: Option<String>,
    /// StatsD flush interval of the metric in seconds if applicable.
    pub statsd_interval: Option<i64>,
    /// Metric type such as `gauge` or `rate`.
    #[serde(rename = "type")]
    pub typ: Option<MetricMetadataType>,
    /// Primary unit of the metric such as `byte` or `operation`.
    pub unit: Option<String>,
}

/// Request to submit a metric.
#[derive(Debug, Deserialize, Serialize)]
pub struct MetricsSubmitRequest {
    /// Represents the timeseries data to submit.
    pub series: Vec<Series>,
}

/// Response containing the metric metadata object.
pub type MetricMetadataResponse = MetricMetadata;
