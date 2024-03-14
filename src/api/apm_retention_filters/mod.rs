use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::models::apm_retention_filters::{
    ApmRetentionFilterCreateRequest, ApmRetentionFilterCreateResponse,
    ApmRetentionFilterGetResponse, ApmRetentionFilterReOrderRequest,
    ApmRetentionFilterUpdateRequest, ApmRetentionFilterUpdateResponse,
    ApmRetentionFiltersListResponse,
};
use crate::models::client::EmptyResponse;

/// Base route path for APM retention filters.
static BASE_PATH: &str = "api/v2/apm/config/retention-filters";

impl Client {
    /// Get the list of APM retention filters.
    pub async fn list_apm_retention_filters(
        &self,
    ) -> Result<ApmRetentionFiltersListResponse, Error> {
        let req = self.build_request(Method::GET, BASE_PATH)?;
        self.send_request::<ApmRetentionFiltersListResponse>(req)
            .await
    }

    /// Create a retention filter to index spans in your organization.
    pub async fn create_apm_retention_filter(
        &self,
        request: ApmRetentionFilterCreateRequest,
    ) -> Result<ApmRetentionFilterCreateResponse, Error> {
        let req = self.build_request(Method::POST, BASE_PATH)?;
        let req = req.json(&request);

        self.send_request::<ApmRetentionFilterCreateResponse>(req)
            .await
    }

    /// Get an APM retention filter.
    pub async fn get_apm_retention_filter(
        &self,
        id: &str,
    ) -> Result<ApmRetentionFilterGetResponse, Error> {
        let req = self.build_request(Method::GET, &format!("{}/{}", BASE_PATH, id))?;

        self.send_request::<ApmRetentionFilterGetResponse>(req)
            .await
    }

    /// Update a retention filter from your organization.
    pub async fn update_apm_retention_filter(
        &self,
        request: ApmRetentionFilterUpdateRequest,
    ) -> Result<ApmRetentionFilterUpdateResponse, Error> {
        let req = self.build_request(Method::PUT, &format!("{}/{}", BASE_PATH, request.data.id))?;
        let req = req.json(&request.data);

        self.send_request::<ApmRetentionFilterUpdateResponse>(req)
            .await
    }

    /// Delete a specific retention filter from your organization.
    pub async fn delete_apm_retention_filter(&self, id: &str) -> Result<EmptyResponse, Error> {
        let req = self.build_request(Method::DELETE, &format!("{}/{}", BASE_PATH, id))?;

        self.send_request::<EmptyResponse>(req).await
    }

    /// Re-order the execution order of retention filters.
    pub async fn re_order_retention_filters(
        &self,
        request: ApmRetentionFilterReOrderRequest,
    ) -> Result<(), Error> {
        let req = self.build_request(Method::PUT, &format!("{}-execution-order", BASE_PATH))?;
        let req = req.json(&request);

        self.send_request::<()>(req).await
    }
}
