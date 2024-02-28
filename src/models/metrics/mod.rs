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
