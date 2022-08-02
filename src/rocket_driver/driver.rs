use crate::client::Client;
use rocket::figment::Figment;
use rocket_db_pools::Pool;
use url::Url;
use crate::rocket_driver::influx_config::Config;
use rocket_db_pools::Error;

#[rocket::async_trait]
impl Pool for Client {
    type Connection = Client;
    type Error = Error<String>;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {


        let config = figment.extract::<Config>()?;
        let client = Client::new(Url::parse(&config.url).unwrap(), &config.bucket, &config.org, &config.jwt_token.unwrap()).await;

        if client.is_err(){
            return Err(Error::Init(client.err().unwrap().to_string()));
        }

        Ok(client.unwrap())


    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.clone())
    }
}
