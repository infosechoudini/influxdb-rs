use serde::{Serialize, Deserialize};
use crate::data_model::{links::Links};

/// Org Struct when retrieving Orgs api/v2/orgs
#[derive(Serialize, Deserialize, Debug)]
pub struct OrgStruct{
    /// Created At
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Description of Org
    pub description: Option<String>,
    /// Org ID 
    pub id: String,
    /// OrgLinks Struct
    pub links: OrgLinks,
    /// Name of Org
    pub name: String,
    /// Status of Org
    pub status: Option<String>,
    /// Last Updated 
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,

}

/// Embedded Links in Org Struct when retrieving Orgs api/v2/orgs
#[derive(Serialize, Deserialize, Debug)]
pub struct OrgLinks{
    /// Buckets API Link
    pub buckets: String,
    /// Dashboard API Link
    pub dashboards: String,
    /// Labels API Link
    pub labels: String,
    /// Members API Link
    pub members: String,
    /// Owners API Link
    pub owners: String,
    /// Secrets API Link
    pub secrets: String,
    #[serde(rename = "self")]
    /// Org API Link
    pub org_self: String,
    /// Tasks API Link
    pub tasks: String,
}

/// Orgs Struct when retrieving list of Orgs from api/v2/orgs
#[derive(Serialize, Deserialize, Debug)]
pub struct Orgs {
    /// Links 
    pub links: Links,
    /// Org Structs
    pub orgs: Vec<OrgStruct>,
}

