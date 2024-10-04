/*
 * Rosetta
 *
 * Build Once. Integrate Your Blockchain Everywhere. 
 *
 * The version of the OpenAPI document: 1.4.13
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// NetworkRequest : A NetworkRequest is utilized to retrieve some data specific exclusively to a NetworkIdentifier. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRequest {
    #[serde(rename = "network_identifier")]
    pub network_identifier: Box<models::NetworkIdentifier>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl NetworkRequest {
    /// A NetworkRequest is utilized to retrieve some data specific exclusively to a NetworkIdentifier. 
    pub fn new(network_identifier: models::NetworkIdentifier) -> NetworkRequest {
        NetworkRequest {
            network_identifier: Box::new(network_identifier),
            metadata: None,
        }
    }
}

