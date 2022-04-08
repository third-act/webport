use crate::{error::Error, Client, BASE_PATH};
use chrono::prelude::*;
use std::collections::HashMap;
use url::form_urlencoded;

impl Client {
    pub async fn get_tag_history(
        &self,
        tag_key: &str,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
    ) -> Result<Vec<(DateTime<Utc>, f64)>, Error> {
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("start", &start.to_rfc3339())
            .append_pair("end", &end.to_rfc3339())
            .finish();

        let url = format!(
            "{}{}/trend/history?tag={}&{}",
            &self.url, BASE_PATH, tag_key, encoded
        );
        let res: HashMap<String, HashMap<String, f64>> = self.get_map(&url).await?;

        // Get value map.
        let map = match res.get(tag_key) {
            Some(map) => map,
            None => {
                return Err(Error::ParseError(format!(
                    "Could not parse tag history for tag key \"{}\" (tag name not found).",
                    tag_key,
                )));
            }
        };

        let mut list = vec![];
        for (key, value) in map {
            // Parse time and convert to UTC.
            let key = match Local.datetime_from_str(key, "%+") {
                Ok(t) => t.with_timezone(&Utc),
                Err(err) => {
                    return Err(Error::ParseError(format!(
                        "Could not parse tag history for tag key \"{}\" with input {} ({}).",
                        tag_key,
                        key,
                        err.to_string(),
                    )));
                }
            };

            list.push((key, value.clone()));
        }

        Ok(list)
    }
}
