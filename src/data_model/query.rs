use serde::{Serialize, Deserialize};
use crate::data_model::file::File;

/// Struct designed for Deleting Data
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteQuery{
    /// Predicate to run
    /// measurement="tempmeasurement"
    /// key="value"
    pub predicate: String,
    /// Start Time of when to find predicate
    pub start: String,
    /// End Time of when to find predicate
    pub stop: String,
}


/// Query influx using the Flux language
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReadQuery {
    /// Query Script
    #[serde(rename = "extern", skip_serializing_if = "Option::is_none")]
    pub r#extern: Option<File>,
    /// Query script to execute.
    pub query: String,
    /// The type of query. Must be \"flux\".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Dialect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    /// Specifies the time that should be reported as "now" in the query.
    /// Default is the server's now time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub now: Option<String>,
}

/// The type of query. Must be \"flux\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    /// Query Type
    Flux,
}


/// AnalyzeQueryResponse
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnalyzeQueryResponse {
    /// List of QueryResponseErrors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<AnalyzeQueryResponseErrors>,
}

impl AnalyzeQueryResponse {
    /// Return an instance of AnanlyzeQueryResponse
    pub fn new() -> Self {
        Self::default()
    }
}

/// AnalyzeQueryResponseErrors
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnalyzeQueryResponseErrors {
    /// Error line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    /// Error column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    /// Error char
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<i32>,
    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl AnalyzeQueryResponseErrors {
    /// Return an instance of AnalyzeQueryResponseErrors
    pub fn new() -> Self {
        Self::default()
    }
}