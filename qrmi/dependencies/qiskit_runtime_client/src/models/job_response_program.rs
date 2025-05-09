/*
 * Qiskit Runtime API
 *
 * The Qiskit Runtime API description
 *
 * The version of the OpenAPI document: 0.21.2
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JobResponseProgram : Program associated with the job
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponseProgram {
    /// Identifier from the executed program
    #[serde(rename = "id")]
    pub id: String,
}

impl JobResponseProgram {
    /// Program associated with the job
    pub fn new(id: String) -> JobResponseProgram {
        JobResponseProgram { id }
    }
}
