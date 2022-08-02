use serde::{Serialize, Deserialize};

/// Rules to expire or retain data. No rules means data never expires.
#[derive(Serialize, Deserialize, Debug)]
pub struct RetentionRules{
    ///Duration in seconds for how long data will be kept in the database. 0 means infinite.
    #[serde(rename = "everySeconds")]
    pub every_seconds: i64,
    ///Shard duration measured in seconds.
    #[serde(rename = "shardGroupDurationSeconds")]
    pub shard_group_duration_seconds: i64,
    ///Default: "expire"
    /// Value: "expire"
    #[serde(rename = "type")]
    pub retention_type: String,
}