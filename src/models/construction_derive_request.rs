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

/// ConstructionDeriveRequest : ConstructionDeriveRequest is passed to the `/construction/derive` endpoint. Network is provided in the request because some blockchains have different address formats for different networks. Metadata is provided in the request because some blockchains allow for multiple address types (i.e. different address for validators vs normal accounts). 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstructionDeriveRequest {
    #[serde(rename = "network_identifier")]
    pub network_identifier: Box<models::NetworkIdentifier>,
    #[serde(rename = "public_key")]
    pub public_key: Box<models::PublicKey>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl ConstructionDeriveRequest {
    /// ConstructionDeriveRequest is passed to the `/construction/derive` endpoint. Network is provided in the request because some blockchains have different address formats for different networks. Metadata is provided in the request because some blockchains allow for multiple address types (i.e. different address for validators vs normal accounts). 
    pub fn new(network_identifier: models::NetworkIdentifier, public_key: models::PublicKey) -> ConstructionDeriveRequest {
        ConstructionDeriveRequest {
            network_identifier: Box::new(network_identifier),
            public_key: Box::new(public_key),
            metadata: None,
        }
    }
}

