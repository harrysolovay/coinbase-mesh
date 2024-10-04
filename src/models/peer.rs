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

/// Peer : A Peer is a representation of a node's peer. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    #[serde(rename = "peer_id")]
    pub peer_id: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Peer {
    /// A Peer is a representation of a node's peer. 
    pub fn new(peer_id: String) -> Peer {
        Peer {
            peer_id,
            metadata: None,
        }
    }
}

