mod error;
mod tag;
pub use tag::Tag;
mod get_tag_history;
mod get_tag_info;
mod get_tags;
pub use error::Error;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::time::Duration;

const BASE_PATH: &str = "/api/v1";

pub struct Client {
    url: String,
    client: reqwest::Client,
    username: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTokenResponse {
    pub token: String,
    pub valid_until: String,
}

impl Client {
    pub async fn new(
        url: String,
        username: String,
        password: String,
        timeout: Option<Duration>,
    ) -> Result<Client, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let timeout = match timeout {
            Some(t) => t,
            None => Duration::new(60, 0),
        };

        let client = match reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(timeout)
            .build()
        {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::Unspecified(format!(
                    "Could not create reqwest client ({}).",
                    err.to_string()
                )))
            }
        };

        let c = Client {
            url,
            client,
            username,
            password,
        };
        Ok(c)
    }

    async fn get_token(&self) -> Result<GetTokenResponse, Error> {
        let url = format!(
            "{}{}/access/token?username={}&password={}",
            &self.url, BASE_PATH, &self.username, &self.password
        );
        let res = match self.client.get(&url).send().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::NetworkError(format!(
                    "Could not send message ({}).",
                    err.to_string()
                )));
            }
        };

        let status = res.status();

        let response_text: String = match res.text().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}).",
                    err.to_string()
                )));
            }
        };

        if status != 200 {
            return Err(Error::ApiError(format!(
                "Got {} / {} calling {}.",
                status, response_text, url
            )));
        }

        let res: GetTokenResponse = match serde_json::from_str(&response_text) {
            Ok(res) => res,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}) from {}.",
                    err.to_string(),
                    response_text
                )));
            }
        };

        Ok(res)
    }

    async fn post<'a, T: DeserializeOwned>(
        &self,
        url: &str,
        body: impl Serialize,
    ) -> Result<T, Error> {
        let token = self.get_token().await?;
        let g = format!("{}", token.token);
        let bearer = match HeaderValue::from_str(&g) {
            Ok(bearer) => bearer,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not parse auth token header value ({}).",
                    err.to_string()
                )));
            }
        };

        let res = match self
            .client
            .post(url)
            .json(&body)
            .header("Token", bearer)
            .send()
            .await
        {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::NetworkError(format!(
                    "Could not send message ({}).",
                    err.to_string()
                )));
            }
        };

        let status = res.status();

        let response_text: String = match res.text().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}).",
                    err.to_string()
                )));
            }
        };

        if status != 200 {
            return Err(Error::ApiError(format!(
                "Got {} / {} calling {} with token {}.",
                status, response_text, url, token.token
            )));
        }

        let body: T = match serde_json::from_str(&response_text) {
            Ok(res) => res,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}) from {}.",
                    err.to_string(),
                    response_text
                )));
            }
        };

        Ok(body)
    }

    async fn get<'a, T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let token = self.get_token().await?;
        let g = format!("{}", token.token);
        let bearer = match HeaderValue::from_str(&g) {
            Ok(bearer) => bearer,
            Err(err) => {
                return Err(Error::Unspecified(format!(
                    "Could not parse auth token header value ({}).",
                    err.to_string()
                )));
            }
        };

        let res = match self.client.get(url).header("Token", bearer).send().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::NetworkError(format!(
                    "Could not send message ({}).",
                    err.to_string()
                )));
            }
        };

        let status = res.status();

        let response_text: String = match res.text().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}).",
                    err.to_string()
                )));
            }
        };

        if status != 200 {
            return Err(Error::ApiError(format!(
                "Got {} / {} calling {} with token {}.",
                status, response_text, url, token.token
            )));
        }

        let body: T = match serde_json::from_str(&response_text) {
            Ok(res) => res,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}) from {}.",
                    err.to_string(),
                    response_text
                )));
            }
        };

        Ok(body)
    }

    async fn get_map<'a, T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<HashMap<String, T>, Error> {
        let token = self.get_token().await?;
        let g = format!("{}", token.token);
        let bearer = match HeaderValue::from_str(&g) {
            Ok(bearer) => bearer,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not parse auth token header value ({}).",
                    err.to_string()
                )));
            }
        };

        let res = match self.client.get(url).header("Token", bearer).send().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::NetworkError(format!(
                    "Could not send message ({}).",
                    err.to_string()
                )));
            }
        };

        let status = res.status();

        let response_text: String = match res.text().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}).",
                    err.to_string()
                )));
            }
        };

        if status != 200 {
            return Err(Error::ApiError(format!(
                "Got {} / {} calling {} with token {}.",
                status, response_text, url, token.token
            )));
        }

        let parsed: Value = match serde_json::from_str(&response_text) {
            Ok(parsed) => parsed,
            Err(err) => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response ({}).",
                    err.to_string()
                )));
            }
        };
        let map: Map<String, Value> = match parsed.as_object() {
            Some(obj) => obj.clone(),
            None => {
                return Err(Error::ParseError(format!(
                    "Could not deserialize response into map, as it was not an object."
                )));
            }
        };

        let mut list = HashMap::new();
        for (key, value) in map {
            let v = match serde_json::from_value(value) {
                Ok(parsed) => parsed,
                Err(err) => {
                    return Err(Error::ParseError(format!(
                        "Could not deserialize response into map of specified type ({}).",
                        err.to_string()
                    )));
                }
            };
            list.insert(key, v);
        }

        Ok(list)
    }
}
