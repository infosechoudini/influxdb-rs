use crate::data_model::query::ReadQuery;
use crate::error;
use crate::client::Client;
use serde_json::json;
use futures::prelude::*;
use reqwest::Response;
use crate::serialization;


impl Client {


    /// Query function used to access api/v2/query
    /// Utilizes Flux Formatting
    pub async fn query(
        &self,
        query: Option<ReadQuery>,
    ) -> Result<Response, error::Error> {
        let param = vec![("org", self.org.as_str())];


        let url = self.build_url("api/v2/query", Some(param));

        if query.is_some() {
            let builder = self.client.post(url).body(json!(query.unwrap()).to_string());
            let resp_future = builder.bearer_auth(self.jwt_token.clone().unwrap()).send().boxed();

            let res = resp_future.await?;
            match res.status().as_u16() {
                200 => {
                    return Ok(res);
                }
                400 => {
                    let json_data = res.text().await?;
    
                    return Err(error::Error::SyntaxError(serialization::conversion(
                        &json_data,
                    )));
                }
                401 | 403 => {
                        return Err(error::Error::InvalidCredentials(
                            "Invalid authentication credentials.".to_string()
                        ));
                }
                _ => {
                    let err = res.text().await?;
                    return Err(error::Error::Unknow(err));
                }
            }

        } else {
            return Err(error::Error::Unknow("No flux query to serialize".to_string()));
        };

        
    }
}