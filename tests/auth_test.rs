use influxdb_rs::Client;
use influxdb_rs::data_model::user::Status;
use influxdb_rs::data_model::authorization::AuthPermissions;
use influxdb_rs::data_model::authorization::AuthResource;
use influxdb_rs::data_model::authorization::AuthResourceType;
use url::Url;
use chrono::Utc;


#[tokio::test]
async fn client_token_auth() {

    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();

    let basic_auth_result = client.ping().await.await.unwrap();

    assert!(basic_auth_result, "PING DIDNT WORK: {}", basic_auth_result);
}


#[tokio::test]
async fn reduced_perms() {

    // Use Admin Client
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await;

    // Assert that it succeeded
    assert!(client.is_ok(), "CLIENT DIDNT WORK: {}", client.unwrap_err());

    // Get Client
    let client = client.unwrap();

    // Assert that Org ID is not empty
    assert_ne!(client.org_id, "");

    //Create Read Permissions
    let permissions = vec![AuthPermissions {
        action: "read".to_string(),
        resource: AuthResource {
            r#type: AuthResourceType::Bucket.to_string(),
            org_id: None,
            bucket_id: None,
        },
    }];

    // Create a new authorization
    let create_auth = client.create_authorization(None, &client.org_id, permissions, Status::Active, "read bucket").await;

    // Assert that it succeeded
    assert!(create_auth.is_ok(), "CREATE AUTH DIDNT WORK: {} ORG ID: {} AuthType: {}", create_auth.unwrap_err(), client.org_id, AuthResourceType::Bucket.to_string());

    //Get Auth Response
    let auth = create_auth.unwrap();

    // Create a new client with the user token

    let reduced_client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", &auth.token).await;

    // Assert that it fails
    assert!(reduced_client.is_err(), "REDUCED CLIENT DIDNT WORK: {}", reduced_client.unwrap_err());

    // Access Bucket with reduced client that doesnt have org ID access
    let reduced_client = Client::new_without_org_id(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", &auth.token).await.unwrap();

    // Need a date-timestamp for insert
    let now = Utc::now();

    // NOTE: convert time from timstamp_nanos() due to to_rfc3339() doesn't convert nicely with GOLANG
    let flux_query = format!("from(bucket: \"test_bucket\") 
    |> range(start: time(v: {:?}))
    |> yield()", now.timestamp_nanos());

    let query = influxdb_rs::data_model::query::ReadQuery{
        r#extern: None,
        query: flux_query,
        r#type: None,
        dialect: None,
        now: None,

    };

    let result = reduced_client.query(Some(query)).await;

    // Assert that it runs
    assert!(result.is_ok(), "REDUCED CLIENT QUERY DIDNT WORK: {}", result.unwrap_err());


     

}

