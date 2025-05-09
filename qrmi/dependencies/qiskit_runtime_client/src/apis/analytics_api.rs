/*
 * Qiskit Runtime API
 *
 * The Qiskit Runtime API description
 *
 * The version of the OpenAPI document: 0.21.2
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`analytics_filters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsFiltersError {
    Status401(models::FindInstanceWorkloads401Response),
    Status403(models::FindInstanceWorkloads401Response),
    Status404(models::FindInstanceWorkloads401Response),
    Status500(models::FindInstanceWorkloads401Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsUsageError {
    Status401(models::FindInstanceWorkloads401Response),
    Status403(models::FindInstanceWorkloads401Response),
    Status404(models::FindInstanceWorkloads401Response),
    Status500(models::FindInstanceWorkloads401Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_usage_analytics_grouped`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAnalyticsGroupedError {
    Status401(models::FindInstanceWorkloads401Response),
    Status403(models::FindInstanceWorkloads401Response),
    Status404(models::FindInstanceWorkloads401Response),
    Status500(models::FindInstanceWorkloads401Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_usage_analytics_grouped_by_date`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAnalyticsGroupedByDateError {
    Status401(models::FindInstanceWorkloads401Response),
    Status403(models::FindInstanceWorkloads401Response),
    Status404(models::FindInstanceWorkloads401Response),
    Status500(models::FindInstanceWorkloads401Response),
    UnknownValue(serde_json::Value),
}

/// Get analytics filters
pub async fn analytics_filters(
    configuration: &configuration::Configuration,
    ibm_api_version: Option<&str>,
    instance: Option<Vec<String>>,
) -> Result<models::AnalyticsFilters200Response, Error<AnalyticsFiltersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ibm_api_version = ibm_api_version;
    let p_instance = instance;

    let uri_str = format!("{}/analytics/filters", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_instance {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("instance".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "instance",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_ibm_api_version {
        req_builder = req_builder.header("IBM-API-Version", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref crn) = configuration.crn {
        req_builder = req_builder.header("Service-CRN", crn.clone());
    }
    req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AnalyticsFilters200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AnalyticsFilters200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsFiltersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get usage analytics
pub async fn analytics_usage(
    configuration: &configuration::Configuration,
    ibm_api_version: Option<&str>,
    instance: Option<Vec<String>>,
    interval_start: Option<String>,
    interval_end: Option<String>,
    backend: Option<Vec<String>>,
    user_id: Option<Vec<String>>,
    simulators: Option<bool>,
) -> Result<models::AnalyticsUsage200Response, Error<AnalyticsUsageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ibm_api_version = ibm_api_version;
    let p_instance = instance;
    let p_interval_start = interval_start;
    let p_interval_end = interval_end;
    let p_backend = backend;
    let p_user_id = user_id;
    let p_simulators = simulators;

    let uri_str = format!("{}/analytics/usage", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_instance {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("instance".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "instance",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_interval_start {
        req_builder = req_builder.query(&[("interval_start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_interval_end {
        req_builder = req_builder.query(&[("interval_end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_backend {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("backend".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "backend",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_user_id {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("user_id".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "user_id",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_simulators {
        req_builder = req_builder.query(&[("simulators", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_ibm_api_version {
        req_builder = req_builder.header("IBM-API-Version", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref crn) = configuration.crn {
        req_builder = req_builder.header("Service-CRN", crn.clone());
    }
    req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AnalyticsUsage200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AnalyticsUsage200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsUsageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get usage analytics grouped
pub async fn get_usage_analytics_grouped(
    configuration: &configuration::Configuration,
    group_by: &str,
    ibm_api_version: Option<&str>,
    instance: Option<Vec<String>>,
    interval_start: Option<String>,
    interval_end: Option<String>,
    backend: Option<Vec<String>>,
    user_id: Option<Vec<String>>,
    simulators: Option<bool>,
) -> Result<models::GetUsageAnalyticsGrouped200Response, Error<GetUsageAnalyticsGroupedError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_group_by = group_by;
    let p_ibm_api_version = ibm_api_version;
    let p_instance = instance;
    let p_interval_start = interval_start;
    let p_interval_end = interval_end;
    let p_backend = backend;
    let p_user_id = user_id;
    let p_simulators = simulators;

    let uri_str = format!("{}/analytics/usage_grouped", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("group_by", &p_group_by.to_string())]);
    if let Some(ref param_value) = p_instance {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("instance".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "instance",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_interval_start {
        req_builder = req_builder.query(&[("interval_start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_interval_end {
        req_builder = req_builder.query(&[("interval_end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_backend {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("backend".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "backend",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_user_id {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("user_id".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "user_id",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_simulators {
        req_builder = req_builder.query(&[("simulators", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_ibm_api_version {
        req_builder = req_builder.header("IBM-API-Version", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref crn) = configuration.crn {
        req_builder = req_builder.header("Service-CRN", crn.clone());
    }
    req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetUsageAnalyticsGrouped200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetUsageAnalyticsGrouped200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUsageAnalyticsGroupedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get usage analytics grouped by date
pub async fn get_usage_analytics_grouped_by_date(
    configuration: &configuration::Configuration,
    group_by: &str,
    ibm_api_version: Option<&str>,
    instance: Option<Vec<String>>,
    interval_start: Option<String>,
    interval_end: Option<String>,
    backend: Option<Vec<String>>,
    user_id: Option<Vec<String>>,
    simulators: Option<bool>,
) -> Result<
    models::GetUsageAnalyticsGroupedByDate200Response,
    Error<GetUsageAnalyticsGroupedByDateError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_group_by = group_by;
    let p_ibm_api_version = ibm_api_version;
    let p_instance = instance;
    let p_interval_start = interval_start;
    let p_interval_end = interval_end;
    let p_backend = backend;
    let p_user_id = user_id;
    let p_simulators = simulators;

    let uri_str = format!(
        "{}/analytics/usage_grouped_by_date",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("group_by", &p_group_by.to_string())]);
    if let Some(ref param_value) = p_instance {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("instance".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "instance",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_interval_start {
        req_builder = req_builder.query(&[("interval_start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_interval_end {
        req_builder = req_builder.query(&[("interval_end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_backend {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("backend".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "backend",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_user_id {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("user_id".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "user_id",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_simulators {
        req_builder = req_builder.query(&[("simulators", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_ibm_api_version {
        req_builder = req_builder.header("IBM-API-Version", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref crn) = configuration.crn {
        req_builder = req_builder.header("Service-CRN", crn.clone());
    }
    req_builder = req_builder.header(reqwest::header::ACCEPT, "application/json");

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetUsageAnalyticsGroupedByDate200Response`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetUsageAnalyticsGroupedByDate200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUsageAnalyticsGroupedByDateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
