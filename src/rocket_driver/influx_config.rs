use rocket::serde::{Deserialize, Serialize};


///Config struct for Influx DB Client
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    /// Database-specific connection and configuration URL.
    ///
    /// The format of the URL is database specific; consult your database's
    /// documentation.
    pub url: String,
    /// Minimum number of connections to maintain in the pool.
    ///
    ///
    /// _Default:_ `None`.
    pub min_connections: Option<u32>,
    /// Maximum number of connections to maintain in the pool.
    ///
    /// _Default:_ `workers * 4`.
    pub max_connections: usize,
    /// Number of seconds to wait for a connection before timing out.
    ///
    /// If the timeout elapses before a connection can be made or retrieved from
    /// a pool, an error is returned.
    ///
    /// _Default:_ `5`.
    pub connect_timeout: u64,
    /// Maximum number of seconds to keep a connection alive for.
    ///
    /// After a connection is established, it is maintained in a pool for
    /// efficient connection retrieval. When an `idle_timeout` is set, that
    /// connection will be closed after the timeout elapses. If an
    /// `idle_timeout` is not specified, the behavior is driver specific but
    /// typically defaults to keeping a connection active indefinitely.
    ///
    /// _Default:_ `None`.
    pub idle_timeout: Option<u64>,

    /// Database for the Influx DB Client to connect to
    pub bucket: String,
    /// Organization for Influx DB
    pub org: String,
    /// Basic Authentication (Optional) for Influx DB
    /// Basic Authentication or JWT must be set
    pub authentication: Option<(String, String)>,
    /// JWT Token (Optional) for Influx DB 
    /// This is the token created within Influx DB
    /// JWT or Basic Authentication must be set
    pub jwt_token: Option<String>,
}