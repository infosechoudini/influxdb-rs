use serde::{Serialize, Deserialize};

/// Label Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Labels {
    /// Internal ID of label in InfluxDB
    pub id: String,
    /// Name of Label
    pub name: String,
    /// Org it is owned by
    #[serde(rename = "orgID")]
    pub org_id: String,
    /// Properties of Label
    pub properties: Properties
}


/// Properties for Labels
#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {

    /// Color of Label
    pub color: String,
    /// Description,
    pub description: String,
}