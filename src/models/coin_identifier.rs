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

/// CoinIdentifier : CoinIdentifier uniquely identifies a Coin. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoinIdentifier {
    /// Identifier should be populated with a globally unique identifier of a Coin. In Bitcoin, this identifier would be transaction_hash:index. 
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl CoinIdentifier {
    /// CoinIdentifier uniquely identifies a Coin. 
    pub fn new(identifier: String) -> CoinIdentifier {
        CoinIdentifier {
            identifier,
        }
    }
}

