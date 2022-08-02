
use influxdb_rs::{Client, Point, point, points, Value, Precision};
use url::Url;
use std::borrow::Cow;
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    // default with "http://127.0.0.1:8086", db with "test"
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();

    let point = point!("test1");
    let point = point
        .add_field("foo", Value::String(Cow::from("bar".to_string())))
        .add_field("integer", Value::Integer(11))
        .add_field("float", Value::Float(22.3))
        .add_field("'boolean'", Value::Boolean(false));

    let point1 = Point::new("test1")
        .add_tag("tags", Value::String(Cow::from(String::from("\\\"fda"))))
        .add_tag("number", Value::Integer(12))
        .add_tag("float", Value::Float(12.6))
        .add_field("fd", Value::String(Cow::from("'3'".to_string())))
        .add_field("quto", Value::String(Cow::from("\\\"fda".to_string())))
        .add_field("quto1", Value::String(Cow::from("\"fda".to_string())));

    let points = points!(point1, point);

    // if Precision is None, the default is second
    // Multiple write
    let result = client.write_points(points, Some(Precision::Seconds), None).await;

    if result.is_err(){
        // DO SOMETHING
    }

    let now = Utc::now();

    // NOTE: convert time from timstamp_nanos() due to to_rfc3339() doesn't convert nicely with GOLANG
    let flux_query = format!("from(bucket: \"test_bucket\") 
    |> range(start: time(v: {:?}))
    |> filter(fn: (r) => r._measurement == \"test4\")
    |> yield()", now.timestamp_nanos());


    let query = influxdb_rs::data_model::query::ReadQuery{
        r#extern: None,
        query: flux_query,
        r#type: None,
        dialect: None,
        now: None,

    };

    let result = client.query(Some(query)).await;

    if result.is_ok(){
        // Prints Response Results in String
        println!("{:?}", result.unwrap().text().await);

    }

}