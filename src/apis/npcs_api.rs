/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 4.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_all_npcs_npcs_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllNpcsNpcsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_npc_items_npcs_code_items_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNpcItemsNpcsCodeItemsGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_npc_npcs_code_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNpcNpcsCodeGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Fetch NPCs details.
pub fn get_all_npcs_npcs_get(configuration: &configuration::Configuration, r#type: Option<models::NpcType>, page: Option<i32>, size: Option<i32>) -> Result<models::DataPageNpcSchema, Error<GetAllNpcsNpcsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_type = r#type;
    let p_page = page;
    let p_size = size;

    let uri_str = format!("{}/npcs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_size {
        req_builder = req_builder.query(&[("size", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DataPageNpcSchema`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DataPageNpcSchema`")))),
        }
    } else {
        let content = resp.text()?;
        let entity: Option<GetAllNpcsNpcsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve the items list of a NPC. If the NPC has items to buy or sell, they will be displayed.
pub fn get_npc_items_npcs_code_items_get(configuration: &configuration::Configuration, code: &str, page: Option<i32>, size: Option<i32>) -> Result<models::DataPageNpcItem, Error<GetNpcItemsNpcsCodeItemsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_code = code;
    let p_page = page;
    let p_size = size;

    let uri_str = format!("{}/npcs/{code}/items", configuration.base_path, code=crate::apis::urlencode(p_code));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_size {
        req_builder = req_builder.query(&[("size", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DataPageNpcItem`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DataPageNpcItem`")))),
        }
    } else {
        let content = resp.text()?;
        let entity: Option<GetNpcItemsNpcsCodeItemsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve the details of a NPC.
pub fn get_npc_npcs_code_get(configuration: &configuration::Configuration, code: &str) -> Result<models::NpcResponseSchema, Error<GetNpcNpcsCodeGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_code = code;

    let uri_str = format!("{}/npcs/{code}", configuration.base_path, code=crate::apis::urlencode(p_code));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::NpcResponseSchema`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::NpcResponseSchema`")))),
        }
    } else {
        let content = resp.text()?;
        let entity: Option<GetNpcNpcsCodeGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

