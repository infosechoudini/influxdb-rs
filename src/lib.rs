//! # InfluxDB Client
//! InfluxDB is an open source time series database with no external dependencies.
//! InfluxDB can be used for metrics, performance tuning, and even machine learning
//!
//! ## Usage
//!
//!
//! ```Rust
//! use influxdb_rs::{Client, Point, point, points, Value, Precision};
//! use url::Url;
//! use std::borrow::Cow;
//! use chrono::prelude::*;
//! 
//! #[tokio::main]
//! async fn main() {
//!     // default with "http://127.0.0.1:8086", db with "test"
//!     let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();
//!
//!     let now = Utc::now();
//!     let mut point = point!("test1");
//!     let point = point
//!         .add_field("foo", Value::String(Cow::from("bar".to_string())))
//!         .add_field("integer", Value::Integer(11))
//!         .add_field("float", Value::Float(22.3))
//!         .add_field("'boolean'", Value::Boolean(false))
//!         .add_timestamp(now.timestamp());
//! 
//!     let point1 = Point::new("test1")
//!                     // We use Cow::From for memeory purposes
//!         .add_tag("tags", Value::String(Cow::from(String::from("\\\"fda"))))
//!         .add_tag("number", Value::Integer(12))
//!         .add_tag("float", Value::Float(12.6))
//!         .add_field("fd", Value::String(Cow::from("'3'".to_string())))
//!         .add_field("quto", Value::String(Cow::from("\\\"fda".to_string())))
//!         .add_field("quto1", Value::String(Cow::from("\"fda".to_string())));
//! 
//!     let points = points!(point1, point);
//! 
//!     // if Precision is None, the default is second
//!     // Multiple write
//!     let result = client.write_points(points, Some(Precision::Seconds), None).await;
//! 
//!     if result.is_err(){
//!         // DO SOMETHING
//!     }
//! 
//! 
//!     // NOTE: convert time from timstamp_nanos() due to to_rfc3339() doesn't convert nicely with GOLANG
//!     let flux_query = format!("from(bucket: \"test_bucket\") 
//!         |> range(start: time(v: {:?}))
//!         |> filter(fn: (r) => r._measurement == \"test4\")
//!         |> yield()", now.timestamp_nanos());
//! 
//! 
//!     let query = influxdb_rs::data_model::query::ReadQuery{
//!         r#extern: None,
//!         query: flux_query,
//!         r#type: None,
//!         dialect: None,
//!        now: None,
//! 
//!     };
//! 
//!     let result = client.query(Some(query)).await;
//! 
//!     if result.is_ok(){
//!         // Prints Response Results in String
//!         println!("{:?}", result.unwrap().text().await);
//!     }
//! }
//! ```

#![deny(warnings)]
#![deny(missing_docs)]

/// API Functions are implemented for influxdb_rs::client::Client
pub mod client;
/// Error module
pub mod error;

/// Serialization module
pub(crate) mod serialization;

/// Schema for influxdb api 
pub mod data_model;

/// InfluxDB Version 2 API endpoints
#[doc(hidden)]
pub mod api;

pub use client::Client;
pub use error::Error;
pub use data_model::data_points::{Point, Points, Precision, Value};

pub use reqwest;


/// Database driver for Rocket.rs
#[cfg(feature = "rocket_driver")]
pub mod rocket_driver;
