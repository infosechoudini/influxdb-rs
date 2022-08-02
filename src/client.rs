use futures::prelude::*;
use reqwest::{Client as HttpClient, Url};
use std::{
    borrow::Borrow,
    iter::FromIterator,
    net::UdpSocket,
    net::{SocketAddr, ToSocketAddrs},
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
    pub fn new<T>(host: Url, bucket: T, org: T, jwt: T) -> impl Future<Output = Result<Self, error::Error>>
    where
        T: Into<String>,
    {
        let mut client = Client {
            host,
            bucket: bucket.into(),
            org: org.into(),
            org_id: "".to_string(),
            authentication: None,
            jwt_token: Some(jwt.into()),
            client: HttpClient::default(),
        };

        async {
            client.org_id = client.get_ord_id().await?;
            Ok(client)
        }


    }

    /// Retrieves Organization ID which is represented inside InfluxDB
    pub fn get_ord_id(&mut self) -> impl Future<Output = Result<String, error::Error>> {
        let param = vec![("org", self.org.as_str())];

        let url = self.build_url("api/v2/orgs", Some(param));
        let fut = self.client.get(url).bearer_auth(self.jwt_token.clone().unwrap()).send();

        async move {
            let res = fut.await?;
            let status = res.status().as_u16();

            match status {
                200 => {
                    let contents = res.json::<data_model::org::Orgs>().await;
                    if contents.is_ok(){
                        let org_id = contents.unwrap().orgs[0].id.clone();
                        Ok(org_id)
                    } else {
                        Err(error::Error::SyntaxError(serialization::conversion(&contents.err().unwrap().to_string())))
                    }

                }
                _ => {
                    let err = res.text().await?;

                    Err(error::Error::SyntaxError(serialization::conversion(&err)))
                }
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
    pub fn switch_database<T>(&mut self, database: T)
    where
        T: Into<String>,
    {
        self.bucket = database.into();
    }

    /// Change the client's user
    pub fn set_authentication<T>(mut self, user: T, passwd: T) -> Self
    where
        T: Into<String>,
    {
        self.authentication = Some((user.into(), passwd.into()));
        self
    }

    /// Set the client's jwt token
    pub fn set_jwt_token<T>(mut self, token: T) -> Self
    where
        T: Into<String>,
    {
        self.jwt_token = Some(token.into());
        self
    }

    /// View the current db name
    pub fn get_db(&self) -> &str {
        self.bucket.as_str()
    }

    /// Query whether the corresponding database exists, return bool
    pub  async fn ping(&self) -> impl Future<Output = Result<bool, error::Error>> {
        let url = self.build_url("ping", None);

        let resp_future = self.client.get(url).bearer_auth(self.jwt_token.clone().unwrap()).send().boxed();
        
        async move {
            let res = resp_future.await?;
            let status = res.status().as_u16();
            let err = res.text().await?;
            match status{
                204 => Ok(true),
                _ => Err(error::Error::SyntaxError(serialization::conversion(&err))),
            }
        }
        
        
    }

    /// Query the version of the database and return the version number
    pub fn get_version(&self) -> impl Future<Output = Result<String, error::Error>>{
        let url = self.build_url("ping", None);

        let resp_future = self.client.get(url).bearer_auth(self.jwt_token.clone().unwrap()).send().boxed();
        
        async move {
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

                    Err(error::Error::SyntaxError(serialization::conversion(&err)))
                }
            }
        }

    }

    /// Write a point to the database
    pub fn write_point<'a>(
        &self,
        point: Point<'a>,
        precision: Option<Precision>,
        rp: Option<&str>,
    ) -> impl Future<Output = Result<(), error::Error>> + 'a {
        let points = Points::new(point);
        self.write_points(points, precision, rp)
    }

    /// Write multiple points to the database
    pub fn write_points<'a, T: IntoIterator<Item = impl Borrow<Point<'a>>>>(
        &self,
        points: T,
        precision: Option<Precision>,
        rp: Option<&str>,
    ) -> impl Future<Output = Result<(), error::Error>> {
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
        let fut = self.client.post(url).bearer_auth(self.jwt_token.clone().unwrap()).body(line).send();

        async move {
            let res = fut.await?;
            let status = res.status().as_u16();
            let err = res.text().await?;

            match status {
                204 => Ok(()),
                400 => Err(error::Error::SyntaxError(serialization::conversion(&err))),
                401 | 403 => Err(error::Error::InvalidCredentials(
                    "Invalid authentication credentials.".to_string(),
                )),
                404 => Err(error::Error::DataBaseDoesNotExist(
                    serialization::conversion(&err),
                )),
                500 => Err(error::Error::RetentionPolicyDoesNotExist(err)),
                status => Err(error::Error::Unknow(format!(
                    "Received status code {}",
                    status
                ))),
            }
        }
    }


    /// Drop measurement
    pub fn drop_measurement(
        &self,
        measurement: &str, 
        start: &str,
        stop: &str,
    ) -> impl Future<Output = Result<(), error::Error>> {
        let param = vec![("bucket", self.bucket.as_str()),("org", self.org.as_str())];

        let url = self.build_url("api/v2/delete", Some(param));

        let measure = format!("_measurement=\"{}\"", measurement.clone()).as_str().to_owned();

        let body = data_model::query::DeleteQuery{
            predicate: measure,
            start: start.to_string(),
            stop: stop.to_string()
        };

        let builder = self.client.post(url).body(json!(body).to_string());

        let resp_future = builder.bearer_auth(self.jwt_token.clone().unwrap()).send().boxed();

        async move {
            let res = resp_future.await?;
            match res.status().as_u16() {
                204 => Ok(()),
                400 => {
                    let err = res.text().await?;

                    Err(error::Error::SyntaxError(serialization::conversion(
                        &err,
                    )))
                }
                401 | 403 => Err(error::Error::InvalidCredentials(
                    "Invalid authentication credentials.".to_string(),
                )),
                _ => Err(error::Error::Unknow("There is something wrong".to_string())),
            }
        }
    }

    /// Create a new database in InfluxDB.
    pub fn create_database(&self, dbname: &str) -> impl Future<Output = Result<(), error::Error>> {

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
        let fut = self.client.post(url).bearer_auth(self.jwt_token.clone().unwrap()).body(post_body).send();

        async move {
            let res = fut.await?;
            let status = res.status().as_u16();
            let err = res.text().await?;

            match status {
                201 => Ok(()),
                422 => Err(error::Error::SyntaxError(serialization::conversion(&err))),
                _ => Err(error::Error::SyntaxError(serialization::conversion(&err)))
            }
        }
    }



    /// Get bucket id from InfluxDB 
    /// Provide name and get internal ID
    pub fn get_bucket_id(&self, bucket_name: &str) -> impl Future<Output = Result<String, error::Error>> {
        let param = vec![("name", bucket_name)];

        let url = self.build_url("api/v2/buckets", Some(param));
        let fut = self.client.get(url).bearer_auth(self.jwt_token.clone().unwrap()).send();

        async move {
            let res = fut.await?;
            let status = res.status().as_u16();

            match status {
                200 => {
                    let contents = res.json::<data_model::bucket::Buckets>().await;
                    if contents.is_ok(){
                        let bucket_id = contents.unwrap().buckets[0].id.clone();
                        Ok(bucket_id)
                    } else {
                        Err(error::Error::SyntaxError(serialization::conversion(&contents.err().unwrap().to_string())))
                    }

                }
                _ => {
                    let err = res.text().await?;

                    Err(error::Error::SyntaxError(serialization::conversion(&err)))
                }
            }
        }




    }

    /// Drop a database from InfluxDB.
    pub fn drop_database(&self, dbname: &str) -> impl Future<Output = Result<(), error::Error>> {
        
        let client = self.clone();
        let name = dbname.to_string();

        async move {
            let id = client.get_bucket_id(&name.clone()).await?;
    
            let url = client.build_url(&format!("api/v2/buckets/{}", id), None);
            let fut = client.client.delete(url).bearer_auth(client.jwt_token.clone().unwrap()).send();

            let res = fut.await?;
            let status = res.status().as_u16();
            let err = res.text().await?;

            match status {
                204 => Ok(()),
                404 => Err(error::Error::SyntaxError(serialization::conversion(&err))),
                _ => Err(error::Error::SyntaxError(serialization::conversion(&err)))
            }
        }
    }

    /// Constructs the full URL for an API call.
    pub fn build_url(&self, key: &str, param: Option<Vec<(&str, &str)>>) -> Url {
        let url = self.host.join(key).unwrap();

        let mut authentication = Vec::new();

        if let Some(ref t) = self.authentication {
            authentication.push(("u", &t.0));
            authentication.push(("p", &t.1));
        }

        let url = Url::parse_with_params(url.as_str(), authentication).unwrap();

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
}


/// Udp client
pub struct UdpClient {
    hosts: Vec<SocketAddr>,
}

impl UdpClient {
    /// Create a new udp client.
    pub fn new(address: SocketAddr) -> Self {
        UdpClient {
            hosts: vec![address],
        }
    }

    /// Crates a new UDP client from anything that `ToSocketAddrs` can handle: e.g. a DNS name.
    pub fn with_host<TSA: ToSocketAddrs>(tsa: TSA) -> Result<Self, error::Error> {
        let result = Self {
            hosts: tsa.to_socket_addrs()?.collect(),
        };
        Ok(result)
    }

    /// add udp host.
    pub fn add_host(&mut self, address: SocketAddr) {
        self.hosts.push(address)
    }

    /// View current hosts
    pub fn get_host(&self) -> &[SocketAddr] {
        self.hosts.as_ref()
    }

    /// Send Points to influxdb.
    pub fn write_points(&self, points: Points) -> Result<(), error::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        let line = serialization::line_serialization(points);
        let line = line.as_bytes();
        socket.send_to(line, self.hosts.as_slice())?;

        Ok(())
    }

    /// Send Point to influxdb.
    pub fn write_point(&self, point: Point) -> Result<(), error::Error> {
        let points = Points { point: vec![point] };
        self.write_points(points)
    }
}

impl FromIterator<SocketAddr> for UdpClient {
    /// Create udp client from iterator.
    fn from_iter<I: IntoIterator<Item = SocketAddr>>(iter: I) -> Self {
        let mut hosts = Vec::new();

        for i in iter {
            hosts.push(i);
        }

        UdpClient { hosts }
    }
}
