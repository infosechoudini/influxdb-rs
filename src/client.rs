use futures::prelude::*;
use reqwest::{Client as HttpClient, Url, header};
use std::{
    borrow::Borrow,
};

use crate::{error, serialization, Point, Points, Precision, data_model};
use serde_json::json;

/// The client to influxdb
#[derive(Debug, Clone)]
pub struct Client {
    /// URL of InfluxDB 
    /// http://192.168.0.1:8086
    pub host: Url,
    /// Bucket (database) that the Client will connect to
    pub bucket: String,
    /// Organization that the Bucket is associated with
    pub org: String,
    /// This is the internal Organization ID that InfluxDB refers to
    pub org_id: String,
    /// Basic Authentication for Legacy Endpoints i.e. http://url/query (Old Endpoint) vs http://url/api/v2/query (New Endpoint)
    pub authentication: Option<(String, String)>,
    /// Token authentication used for all interactions with the InfluxDB
    pub jwt_token: Option<String>,
    /// Used for a specifid HTTPClient
    pub client: HttpClient,
}

impl Client {
    /// Create a new influxdb client with http
    #[inline] 
    pub async fn new<T>(host: Url, bucket: T, org: T, jwt: T) -> Result<Self, error::Error>
    where
        T: Into<String>,
    {

        let token = jwt.into();
        let authorized = format!("Token {}", token.clone());

        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", header::HeaderValue::from_str(&authorized).unwrap());

        // get a client builder
        let httpclient = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let mut client = Client {
            host,
            bucket: bucket.into(),
            org: org.into(),
            org_id: "".to_string(),
            authentication: None,
            jwt_token: Some(token.clone()),
            client: httpclient,
        };

        client.org_id = client.get_org_id().await?;
        Ok(client)


    }

    /// Create a new influxdb client with http
    #[inline] 
    pub async fn new_without_org_id<T>(host: Url, bucket: T, org: T, jwt: T) -> Result<Self, error::Error>
    where
        T: Into<String>,
    {

        let token = jwt.into();
        let authorized = format!("Token {}", token.clone());

        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", header::HeaderValue::from_str(&authorized).unwrap());

        // get a client builder
        let httpclient = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let client = Client {
            host,
            bucket: bucket.into(),
            org: org.into(),
            org_id: "".to_string(),
            authentication: None,
            jwt_token: Some(token.clone()),
            client: httpclient,
        };

        Ok(client)


    }

    /// Retrieves Organization ID which is represented inside InfluxDB
    #[inline] 
    pub async fn get_org_id(&mut self) -> Result<String, error::Error> {
        let param = vec![("org", self.org.as_str())];

        let url = self.build_url("api/v2/orgs", Some(param));
        let fut = self.client.get(url.await).send();

        let res = fut.await?;
        let status = res.status().as_u16();

        match status {
            200 => {
                let contents = res.json::<data_model::org::Orgs>().await?;

                if contents.orgs.len() == 0 {
                    return Err(error::Error{
                        inner: error::ErrorKind::SyntaxError("No organization found".to_string())
                    });
                }

                let org_id = contents.orgs[0].id.clone();
                Ok(org_id)
            }
            _ => {
                let err = res.text().await?;

                Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))
                })
            }
        }
        
    }


    /// Create a new influxdb client with custom reqwest's client.
    pub fn new_with_client<T>(host: Url, bucket: T, org: T, client: HttpClient) -> Self
    where
        T: Into<String>,
    {
        Client {
            host,
            bucket: bucket.into(),
            org: org.into(),
            org_id: "".to_string(),
            authentication: None,
            jwt_token: None,
            client,
        }
    }

    /// Change the client's database
    #[inline] 
    pub fn switch_database<T>(&mut self, database: T)
    where
        T: Into<String>,
    {
        self.bucket = database.into();
    }

    /// Change the client's user
    #[inline] 
    pub fn set_authentication<T>(mut self, user: T, passwd: T) -> Self
    where
        T: Into<String>,
    {
        self.authentication = Some((user.into(), passwd.into()));
        self
    }

    /// Set the client's jwt token
    #[inline] 
    pub fn set_jwt_token<T>(mut self, token: T) -> Self
    where
        T: Into<String>,
    {
        self.jwt_token = Some(token.into());
        self
    }

    /// View the current db name
    #[inline] 
    pub fn get_db(&self) -> &str {
        self.bucket.as_str()
    }

    /// Query whether the corresponding database exists, return bool
    #[inline] 
    pub  async fn ping(&self) -> impl Future<Output = Result<bool, error::Error>> {
        let url = self.build_url("ping", None);

        let resp_future = self.client.get(url.await).send().boxed();
        
        async move {
            let res = resp_future.await?;
            let status = res.status().as_u16();
            let err = res.text().await?;
            match status{
                204 => Ok(true),
                _ => Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))
            }),
            }
        }
        
        
    }

    /// Query the version of the database and return the version number
    pub async fn get_version(&self) ->  Result<String, error::Error>{
        let url = self.build_url("ping", None);

    let resp_future = self.client.get(url.await).send().boxed();
    
        let res = resp_future.await?;
        let status = res.status().as_u16();
        let headers = res.headers().get("X-Influxdb-Version");
        
        match status{
            204 => {            

                match headers {
                    Some(header) => {
                        let version = header.to_owned().to_str().unwrap().to_string();
                        Ok(version)
                    }
                    None => {
                        Ok(String::from("Don't know"))
                    }
                }
            },
            _ => {
                let err = res.text().await?;

                Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))
            })
            }
        }

    }

    /// Write a point to the database
    pub async fn write_point<'a>(
        &self,
        point: Point<'a>,
        precision: Option<Precision>,
        rp: Option<&str>,
    ) -> Result<(), error::Error>{
        let points = Points::new(point);
        self.write_points(points, precision, rp).await
    }

    /// Write multiple points to the database
    pub async fn write_points<'a, T: IntoIterator<Item = impl Borrow<Point<'a>>>>(
        &self,
        points: T,
        precision: Option<Precision>,
        rp: Option<&str>,
    ) -> Result<(), error::Error> {
        let line = serialization::line_serialization(points);

        let mut param = vec![("bucket", self.bucket.as_str()), ("org", self.org.as_str())];

        match precision {
            Some(ref t) => param.push(("precision", t.to_str())),
            None => param.push(("precision", "s")),
        };

        if let Some(t) = rp {
            param.push(("rp", t))
        }

        let url = self.build_url("api/v2/write", Some(param));
        let fut = self.client.post(url.await).body(line).send();

        let res = fut.await?;
        let status = res.status().as_u16();
        let err = res.text().await?;

        match status {
            204 => Ok(()),
            400 => Err(error::Error {
                inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))
            }),
            401 | 403 => Err(error::Error {
                inner: error::ErrorKind::InvalidCredentials(
                "Invalid authentication credentials.".to_string(),
            )}),
            404 => Err(error::Error{
                inner: error::ErrorKind::DataBaseDoesNotExist(
                serialization::conversion(&err),
            )}),
            500 => Err(error::Error{
                inner: error::ErrorKind::RetentionPolicyDoesNotExist(err)}),
            status => Err(error::Error{
                inner: error::ErrorKind::Unknown(format!(
                "Received status code {}",
                status
            ))}),
        }
    }


    /// Drop measurement
    pub async fn drop_measurement(
        &self,
        measurement: &str, 
        start: &str,
        stop: &str,
    ) -> Result<(), error::Error> {
        let param = vec![("bucket", self.bucket.as_str()),("org", self.org.as_str())];

        let url = self.build_url("api/v2/delete", Some(param));

        let measure = format!("_measurement=\"{}\"", measurement.clone()).as_str().to_owned();

        let body = data_model::query::DeleteQuery{
            predicate: measure,
            start: start.to_string(),
            stop: stop.to_string()
        };

        let builder = self.client.post(url.await).body(json!(body).to_string());

        let resp_future = builder.send();

        let res = resp_future.await?;
        match res.status().as_u16() {
            204 => Ok(()),
            400 => {
                let err = res.text().await?;

                Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(
                    &err,
                ))})
            }
            401 | 403 => Err(error::Error{
                inner: error::ErrorKind::InvalidCredentials(
                "Invalid authentication credentials.".to_string(),
            )}),
            _ => Err(error::Error{
                inner: error::ErrorKind::Unknown("There is something wrong".to_string())
            }),
        }
    }

    /// Create a new database in InfluxDB.
    pub async fn create_database(&self, dbname: &str) -> Result<(), error::Error> {

        let retention_rules = data_model::retention_rules::RetentionRules{
            every_seconds: 0,
            shard_group_duration_seconds: 0,
            retention_type: "expire".to_string(),
        };

        let body = data_model::bucket::Bucket{
            name: dbname.to_string(),
            org_id: self.org_id.clone(),
            retention_rules: vec![retention_rules],
            description: "".to_string(),
            rp: None,
            schema_type: "Implicit".to_string(),

        };

        let post_body = json!(body).to_string();

        let url = self.build_url("api/v2/buckets", None);
        let fut = self.client.post(url.await).body(post_body).send();

        let res = fut.await?;
        let status = res.status().as_u16();
        let err = res.text().await?;

        match status {
            201 => Ok(()),
            422 => Err(error::Error{
                inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))}),
            _ => Err(error::Error{
                inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))
            })
        }
    }



    /// Get bucket id from InfluxDB 
    /// Provide name and get internal ID
    pub async fn get_bucket_id(&self, bucket_name: &str) -> Result<String, error::Error> {
        let param = vec![("name", bucket_name)];

        let url = self.build_url("api/v2/buckets", Some(param));
        let fut = self.client.get(url.await).send();

        let res = fut.await?;
        let status = res.status().as_u16();

        match status {
            200 => {
                let contents = res.json::<data_model::bucket::Buckets>().await?;
                let bucket_id = contents.buckets[0].id.clone();
                Ok(bucket_id)
            }
            _ => {
                let err = res.text().await?;

                Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
        }

    }

    /// Drop a database from InfluxDB.
    pub async fn drop_database(&self, dbname: &str) -> Result<(), error::Error> {
        
        let client = self.clone();
        let name = dbname.to_string();

        let id = client.get_bucket_id(&name.clone()).await?;

        let id = format!("api/v2/buckets/{}", id);

        let url = client.build_url(&id, None);
        let fut = client.client.delete(url.await).send();

        let res = fut.await?;
        let status = res.status().as_u16();
        let err = res.text().await?;

        match status {
            204 => Ok(()),
            404 => Err(error::Error{
                inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))}),
            _ => Err(error::Error{
                inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
        }
    }

    /// Constructs the full URL for an API call.
    /// No Basic Authentication
    #[inline] 
    pub async fn build_url(&self, key: &str, param: Option<Vec<(&str, &str)>>) -> Url {
        let url = self.host.join(key).unwrap();

        if let Some(param) = param {
            Url::parse_with_params(url.as_str(), param).unwrap()
        } else {
            url
        }
    }

    /// connecting for default database `test` and host `http://localhost:8086`
    pub fn default() -> impl Future<Output = Result<Self, error::Error>> {
        async {
            Client::new(Url::parse("http://localhost:8086").unwrap(), "test", "test", "00000000").await
        }
    }

    ///Create a new user in InfluxDB.
    pub async fn create_new_user(&self, name: &str, status: data_model::user::Status ) -> Result<data_model::user::UserResponse, error::Error> {
        let url = self.build_url("api/v2/users", None);

        let json_status = match status {
            data_model::user::Status::Active => "active",
            data_model::user::Status::Inactive => "inactive",
        };
        
        let body = data_model::user::CreateUser{
            name: name.to_string(),
            status: json_status.to_string(),
        };

        let post_body = json!(body).to_string();

        let fut = self.client.post(url.await).body(post_body).send();

        let res = fut.await?;
        let res_status = res.status().as_u16();

        match res_status {
            201 =>  {
                let contents = res.json::<data_model::user::UserResponse>().await?;
                Ok(contents)
            },
            400 => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
            _ => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
        }
    }

    /// Create Authorization for user
    pub async fn create_authorization(&self, user_id: Option<String>, org_id: &str, permissions: Vec<data_model::authorization::AuthPermissions>, status: data_model::user::Status, description: &str) -> Result<data_model::authorization::AuthorizationResponse, error::Error> {
        let url = self.build_url("api/v2/authorizations", None);

        let json_status = match status {
            data_model::user::Status::Active => "active",
            data_model::user::Status::Inactive => "inactive",
        };

        let body = data_model::authorization::CreateAuthorization{
            org_id: org_id.to_string(),
            user_id,
            permissions,
            status: json_status.to_string(),
            description: description.to_string(),
        };

        let post_body = json!(body).to_string();

        let fut = self.client.post(url.await).body(post_body).send();

        let res = fut.await?;
        let res_status = res.status().as_u16();

        match res_status {
            201 =>  {
                let contents = res.json::<data_model::authorization::AuthorizationResponse>().await?;
                Ok(contents)
            },
            400 => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
            _ => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
        }
    }

    /// List Users
    pub async fn list_users(&self) -> Result<Vec<data_model::user::UserResponse>, error::Error> {
        let url = self.build_url("api/v2/users", None);

        let fut = self.client.get(url.await).send();

        let res = fut.await?;
        let res_status = res.status().as_u16();

        match res_status {
            200 =>  {
                let contents = res.json::<data_model::user::ListUserResponse>().await?;
                Ok(contents.users)
            },
            400 => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
            _ => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
        }
    }

    /// Delete A User
    pub async fn delete_user(&self, user_id: &str) -> Result<(), error::Error> {
        let url_format = format!("api/v2/users/{}", user_id);
        let url = self.build_url(&url_format, None);

        let fut = self.client.delete(url.await).send();

        let res = fut.await?;
        let res_status = res.status().as_u16();

        match res_status {
            204 =>  {
                Ok(())
            },
            400 => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
            _ => {
                let err = res.text().await?;

                return Err(error::Error{
                    inner: error::ErrorKind::SyntaxError(serialization::conversion(&err))})
            }
        }
    }


    
}
