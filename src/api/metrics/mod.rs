use reqwest::Method;

use crate::{
    client::Client,
    error::Error,
    models::{
        client::{EmptyErrorsResponse, EmptyResponse},
        metrics::{MetricMetadataResponse, MetricsSubmitRequest},
    },
};

const BASE_PATH: &str = "api/v2/series";

impl Client {
    /// The metrics end-point allows you to post time-series data that can be graphed on Datadog's dashboards.
    /// The maximum payload size is 500 kilobytes (512000 bytes).
    /// Compressed payloads must have a decompressed size of less than 5 megabytes (5242880 bytes).
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/metrics/#submit-metrics)
    pub async fn submit_metrics(
        &self,
        series: MetricsSubmitRequest,
    ) -> Result<EmptyResponse, Error> {
        // TODO: Support compression.
        let req: reqwest::RequestBuilder =
            self.build_request(Method::POST, &format!("{}", BASE_PATH))?;
        let req = req.json(&series);

        self.send_request::<EmptyErrorsResponse>(req).await?;
        Ok(EmptyResponse {})
    }

    /// Get metadata about a specific metric. This endpoint requires the metrics_read [authorization scope](https://docs.datadoghq.com/api/latest/scopes/#metrics).
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/metrics/#get-metric-metadata)
    pub async fn get_metric_metadata(
        &self,
        metric_name: String,
    ) -> Result<MetricMetadataResponse, Error> {
        let req: reqwest::RequestBuilder =
            self.build_request(Method::GET, &format!("api/v1/metrics/{metric_name}"))?;

        self.send_request::<MetricMetadataResponse>(req).await
    }
}
