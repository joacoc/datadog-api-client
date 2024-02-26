use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::models::apm_retention_filters::{
    CreateApmRetentionFilterRequest, CreateApmRetentionFilterResponse,
    DeleteApmRetentionFilterRequest, GetApmRetentionFilterRequest, GetApmRetentionFilterResponse,
    ListApmRetentionFiltersResponse, ReOrderApmRetentionFiltersRequest,
    UpdateApmRetentionFilterRequest, UpdateApmRetentionFilterResponse,
};
use crate::models::client::EmptyResponse;

static BASE_PATH: &str = "api/v2/apm/config/retention-filters";

impl Client {
    /// Get the list of APM retention filters.
    ///
    /// [Datadog documentation](https://api.datadoghq.com/api/v2/apm/config/retention-filters)
    pub async fn list_apm_retention_filters(
        &self,
    ) -> Result<ListApmRetentionFiltersResponse, Error> {
        let req = self.build_request(Method::GET, BASE_PATH)?;
        Ok(self
            .send_request::<ListApmRetentionFiltersResponse>(req)
            .await?)
    }

    /// Create a retention filter to index spans in your organization.
    /// Returns the retention filter definition when the request is successful.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/apm-retention-filters/#create-a-retention-filter)
    pub async fn create_apm_retention_filter(
        &self,
        request: CreateApmRetentionFilterRequest,
    ) -> Result<CreateApmRetentionFilterResponse, Error> {
        let req = self.build_request(Method::POST, BASE_PATH)?;
        let req = req.json(&request);

        Ok(self
            .send_request::<CreateApmRetentionFilterResponse>(req)
            .await?)
    }

    /// Get an APM retention filter.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/apm-retention-filters/#get-a-given-apm-retention-filter)
    pub async fn get_apm_retention_filter(
        &self,
        request: GetApmRetentionFilterRequest,
    ) -> Result<GetApmRetentionFilterResponse, Error> {
        let req = self.build_request(Method::GET, &format!("{}/{}", BASE_PATH, request.id))?;

        Ok(self
            .send_request::<GetApmRetentionFilterResponse>(req)
            .await?)
    }

    /// Update a retention filter from your organization.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/apm-retention-filters/#update-a-retention-filter)
    pub async fn update_apm_retention_filter(
        &self,
        request: UpdateApmRetentionFilterRequest,
    ) -> Result<UpdateApmRetentionFilterResponse, Error> {
        let req = self.build_request(Method::PUT, &format!("{}/{}", BASE_PATH, request.data.id))?;
        let req = req.json(&request.data);

        Ok(self
            .send_request::<UpdateApmRetentionFilterResponse>(req)
            .await?)
    }

    /// Delete a specific retention filter from your organization.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/apm-retention-filters/#delete-a-retention-filter)
    pub async fn delete_apm_retention_filter(
        &self,
        request: DeleteApmRetentionFilterRequest,
    ) -> Result<EmptyResponse, Error> {
        let req = self.build_request(Method::DELETE, &format!("{}/{}", BASE_PATH, request.id))?;

        Ok(self.send_request(req).await?)
    }

    /// Re-order the execution order of retention filters.
    ///
    /// [Datadog documentation](https://docs.datadoghq.com/api/latest/apm-retention-filters/#re-order-retention-filters)
    pub async fn re_order_retention_filters(
        &self,
        request: ReOrderApmRetentionFiltersRequest,
    ) -> Result<(), Error> {
        let req = self.build_request(Method::PUT, &format!("{}-execution-order", BASE_PATH))?;
        let req = req.json(&request);

        Ok(self.send_request(req).await?)
    }
}
