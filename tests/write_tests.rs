use influxdb_rs::{point, points, Client, Point, Points, Precision};
use std::thread::sleep;
use std::time::Duration;
use url::Url;
use chrono::prelude::*;

#[tokio::test]
async fn create_and_delete_database() {
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();


    // Do a check to see if the bucket is already created
    if client.create_database("temporary").await.is_ok(){
        client.drop_database("temporary").await.unwrap();
    } else {
        // Delete bucket after verifying that one already exists
        // Next test should run the first part of the if statement
        client.drop_database("temporary").await.unwrap();
    }

}

#[tokio::test]
async fn create_and_delete_measurement() {
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();
    
    let now = Utc::now();

    let point = Point::new("temporary")
        .add_field("foo", "bar")
        .add_field("integer", 11)
        .add_field("float", 22.3)
        .add_field("'boolean'", false)
        .add_timestamp(now.timestamp());

    let result = client.write_point(point, Some(Precision::Seconds), None).await;
    if result.is_err(){
        // Error!
    }

    let later = Utc::now().to_rfc3339().to_string();

    client.drop_measurement("temporary", &now.to_rfc3339(), &later).await.unwrap();
}

#[tokio::test]
async fn use_points() {

    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();
    let now = Utc::now();



    let point = Point::new("test1")
        .add_field("foo", "bar")
        .add_field("integer", 11)
        .add_field("float", 22.3)
        .add_field("'boolean'", false);

    let point1 = Point::new("test2")
        .add_tag("tags", "'=213w")
        .add_tag("number", 12)
        .add_tag("float", 12.6)
        .add_field("fd", "'3'".to_string());

    let points = Points::create_new(vec![point1, point]);

    client.write_points(points, Some(Precision::Seconds), None).await.unwrap();

    sleep(Duration::from_secs(3));


    let later = Utc::now().to_rfc3339().to_string();


    client.drop_measurement("test1", &now.to_rfc3339(), &later).await.unwrap();
    client.drop_measurement("test2", &now.to_rfc3339(), &later).await.unwrap();
}

#[tokio::test]
async fn query_with_macros() {


    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();
    let now = Utc::now();


    let point = point!("test4").add_field("foo", "bar");
    let point1 = point.clone();
    let point = point.add_timestamp(now.timestamp());
    let point1 = point1.add_timestamp(now.timestamp());

    let points = points![point, point1];
    client.write_points(points, Some(Precision::Nanoseconds), None).await.unwrap();


    let later = Utc::now();

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


    assert!(result.is_ok(), "{:?}", result.err());



    client.drop_measurement("test4", &now.to_rfc3339(), &later.to_rfc3339()).await.unwrap();
}