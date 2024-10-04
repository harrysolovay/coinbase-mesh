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

/// Operator : Operator is used by query-related endpoints to determine how to apply conditions.  If this field is not populated, the default `and` value will be used. 
/// Operator is used by query-related endpoints to determine how to apply conditions.  If this field is not populated, the default `and` value will be used. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "and")]
    And,

}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Or => write!(f, "or"),
            Self::And => write!(f, "and"),
        }
    }
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Or
    }
}

