use influxdb_rs::Client;
use influxdb_rs::data_model::user::Status;
use influxdb_rs::data_model::authorization::AuthPermissions;
use influxdb_rs::data_model::authorization::AuthResource;
use influxdb_rs::data_model::authorization::AuthResourceType;
use url::Url;

#[tokio::test]
async fn users() {

        // Use Admin Client
        let client = Client::new(Url::parse("http://localhost:8086").unwrap(), "test_bucket", "test_org", "0123456789").await;

        // Assert that it succeeded
        assert!(client.is_ok(), "CLIENT DIDNT WORK: {}", client.unwrap_err());
    
        // Get Client
        let client = client.unwrap();
    
        // Assert that Org ID is not empty
        assert_ne!(client.org_id, "");
    
        // List Users
        let list_users = client.list_users().await;
        // Assert that it succeeded
        assert!(list_users.is_ok(), "LIST USERS DIDNT WORK: {}", list_users.unwrap_err());
    
        // Get List Users Response
        let users = list_users.unwrap();
    
        // Search users for reduced_perms, if exists, delete
        for user in users {
            if user.name == "reduced_perms" {
                let delete_user = client.delete_user(&user.id).await;
                // Assert that it succeeded
                assert!(delete_user.is_ok(), "DELETE USER DIDNT WORK: {}", delete_user.unwrap_err());
            }
        }
    
        // Test Create User
        let create_user = client.create_new_user("reduced_perms", Status::Active).await;
    
        // Assert that it succeeded
        assert!(create_user.is_ok(), "CREATE USER DIDNT WORK: {}", create_user.unwrap_err());
    
        // Get Create User Response
        let user = create_user.unwrap();
    
        //Create Read Permissions
        let permissions = vec![AuthPermissions {
            action: "read".to_string(),
            resource: AuthResource {
                r#type: AuthResourceType::Bucket.to_string(),
                org_id: None,
                bucket_id: None,
            },
        }];

}

