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

        if config.jwt_token.is_some() {
            Ok(Client::new(Url::parse(&config.url).unwrap(), &config.db, &config.org).set_jwt_token(&config.jwt_token.unwrap()))
        } else {
            let (username, password) = &config.authentication.unwrap();
            Ok(Client::new(Url::parse(&config.url).unwrap(), &config.db, &config.org).set_authentication(username, password))
        }

    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.clone())
    }

}