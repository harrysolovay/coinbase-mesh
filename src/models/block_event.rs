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

/// BlockEvent : BlockEvent represents the addition or removal of a BlockIdentifier from storage. Streaming BlockEvents allows lightweight clients to update their own state without needing to implement their own syncing logic. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockEvent {
    /// sequence is the unique identifier of a BlockEvent within the context of a NetworkIdentifier. 
    #[serde(rename = "sequence")]
    pub sequence: i64,
    #[serde(rename = "block_identifier")]
    pub block_identifier: Box<models::BlockIdentifier>,
    #[serde(rename = "type")]
    pub r#type: models::BlockEventType,
}

impl BlockEvent {
    /// BlockEvent represents the addition or removal of a BlockIdentifier from storage. Streaming BlockEvents allows lightweight clients to update their own state without needing to implement their own syncing logic. 
    pub fn new(sequence: i64, block_identifier: models::BlockIdentifier, r#type: models::BlockEventType) -> BlockEvent {
        BlockEvent {
            sequence,
            block_identifier: Box::new(block_identifier),
            r#type,
        }
    }
}

