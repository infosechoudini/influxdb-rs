use serde::{Serialize, Deserialize};


/// Links for Singular Bucket
#[derive(Serialize, Deserialize, Debug)]
pub struct BucketLinks{
        /// Labels API Link
        pub labels: String,
        /// Members API Link
        pub members: String,
        /// Owners API Link
        pub owners: String,
        /// Org API Link
        pub org: String,
        #[serde(rename = "self")]
        /// Bucket API Link
        pub bucket_self: String,
        /// Write API Link
        pub write: String,
}

/// Page Links
#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    /// Next Links Page
    pub next: Option<String>,
    /// Previous Links Page
    pub prev: Option<String>,
    /// Current Links Page
    #[serde(rename = "self")]
    pub link_self: Option<String>,
}
