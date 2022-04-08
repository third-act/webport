use crate::{error::Error, Client, Tag, BASE_PATH};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Client {
    pub async fn get_tag_info(&self, tag_key: &str) -> Result<Tag, Error> {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(rename_all = "PascalCase")]
        pub struct PartialTag {
            pub description: String,
            pub value: String,
            pub unit: String,
            pub datatype: String,
            pub status: String,
        }

        let url = format!("{}{}/tag/info?tag={}", &self.url, BASE_PATH, tag_key);
        let res: HashMap<String, PartialTag> = self.get_map(&url).await?;

        // Get value map.
        let partial_tag = match res.get(tag_key) {
            Some(map) => map,
            None => {
                return Err(Error::ParseError(format!(
                    "Could not parse tag info (tag name not found)."
                )));
            }
        };

        let tag = match partial_tag.datatype.as_str() {
            "REAL" => {
                let value = match partial_tag.value.parse::<f64>() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(Error::ParseError(format!(
                            "Could not parse tag value ({}).",
                            err.to_string()
                        )));
                    }
                };

                Tag::Real {
                    key: tag_key.to_string(),
                    description: partial_tag.description.clone(),
                    unit: partial_tag.unit.clone(),
                    value: value,
                    status: partial_tag.status.clone(),
                }
            }
            _ => {
                return Err(Error::ParseError(format!(
                    "Unrecognized tag datatype ({}).",
                    partial_tag.datatype
                )));
            }
        };

        Ok(tag)
    }
}
