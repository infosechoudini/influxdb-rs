use serde::{Serialize, Deserialize};
use crate::data_model::{labels::Labels, links::BucketLinks, retention_rules::RetentionRules, links::Links};

/// Multiple Buckets Struct, used when retrieving list of all buckets from api/v2/buckets
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiBuckets{
    /// Created At
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Optional Description of Bucket
    pub description: Option<String>,
    /// Name of bucket
    pub name: String,

    /// Array of Labels
    pub labels: Vec<Labels>,

    /// Internal ID of bucket in InfluxDB
    pub id: String,

    /// Bucket API Links
    pub links: BucketLinks,

    /// Org it is owned by
    #[serde(rename = "orgID")]
    pub org_id: String,

    /// Retenetion Rules
    #[serde(rename = "retentionRules")]
    pub retention_rules: Vec<RetentionRules>,

    /// Precision
    pub rp: Option<String>,

    /// Optional Schema Type "Implicit" / "Explicit"
    #[serde(rename = "schemaType")]
    pub schema_type: Option<String>,

    /// Optional Schema Type "Implicit" / "Explicit"
    #[serde(rename = "type")]
    pub user_type: String,
}

/// Buckets Struct when retrieving list of Orgs from api/v2/buckets
#[derive(Serialize, Deserialize, Debug)]
pub struct Buckets {
    /// Links 
    pub links: Links,
    /// Org Structs
    pub buckets: Vec<MultiBuckets>,
}

/// Single Bucket Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Bucket{
    /// Optional Description of Bucket
    pub description: String,
    /// Name of bucket
    pub name: String,

    /// Org it is owned by
    #[serde(rename = "orgID")]
    pub org_id: String,

    /// Retenetion Rules
    #[serde(rename = "retentionRules")]
    pub retention_rules: Vec<RetentionRules>,

    /// Precision
    pub rp: Option<String>,

    /// Optional Schema Type "Implicit" / "Explicit"
    #[serde(rename = "schemaType")]
    pub schema_type: String,
}
