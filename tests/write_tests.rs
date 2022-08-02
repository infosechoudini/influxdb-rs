use influxdb_rs::{point, points, Client, Point, Points, Precision};
use url::Url;
use chrono::prelude::*;

#[tokio::test]
async fn create_and_delete_database() {
    // Create client with a parsed url, bucket, org, and jwt token
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();

    let create = client.create_database("temporary").await;

    // Do a check to see if the bucket is already created
    if create.is_ok() {
        let drop = client.drop_database("temporary").await;

        assert_eq!(drop.is_ok(), true);

    } else {
        // Delete bucket after verifying that one already exists
        let drop = client.drop_database("temporary").await;

        assert_eq!(drop.is_ok(), true);
    }

}

#[tokio::test]
async fn create_and_delete_measurement() {
    // Create client with a parsed url, bucket, org, and jwt token
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();
    
    let now = Utc::now();

    let point = Point::new("temporary")
        .add_field("foo", "bar")
        .add_field("integer", 11)
        .add_field("float", 22.3)
        .add_field("'boolean'", false)
        .add_timestamp(now.timestamp());

    let result = client.write_point(point, Some(Precision::Seconds), None).await;

    // No need to check to see whether the fieds and timestamp are available 
    // variable drop verifies that for us
    assert_eq!(result.is_ok(), true);

    let later = Utc::now().to_rfc3339().to_string();


    // No Error means the value was present
    let drop = client.drop_measurement("temporary", &now.to_rfc3339(), &later).await;

    assert_eq!(drop.is_ok(), true);

}

#[tokio::test]
async fn use_points() {
    // Create client with a parsed url, bucket, org, and jwt token
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

    let write_points = client.write_points(points, Some(Precision::Seconds), None).await;

    assert_eq!(write_points.is_ok(), true);

    let later = Utc::now().to_rfc3339().to_string();

    let drop1 = client.drop_measurement("test1", &now.to_rfc3339(), &later).await;

    assert_eq!(drop1.is_ok(), true);

    let drop2 = client.drop_measurement("test2", &now.to_rfc3339(), &later).await;

    assert_eq!(drop2.is_ok(), true);
}

#[tokio::test]
async fn query_with_macros() {
    // Create client with a parsed url, bucket, org, and jwt token
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();
    // Need a date-timestamp for insert
    let now = Utc::now();

    // user macro to create points
    let point = point!("test4").add_field("foo", "bar");
    let point1 = point.clone();
    let point = point.add_timestamp(now.timestamp());
    let point1 = point1.add_timestamp(now.timestamp());

    let points = points![point, point1];
    client.write_points(points, Some(Precision::Nanoseconds), None).await.unwrap();


    let later = Utc::now();

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


    assert_eq!(result.is_ok(), true);

    let drop = client.drop_measurement("test4", &now.to_rfc3339(), &later.to_rfc3339()).await;

    assert_eq!(drop.is_ok(), true);
}