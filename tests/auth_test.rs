use influxdb_rs::Client;
use url::Url;

#[tokio::test]
async fn client_token_auth() {

    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await.unwrap();

    let basic_auth_result = client.ping().await.await.unwrap();

    assert!(basic_auth_result, "PING DIDNT WORK: {}", basic_auth_result);
}