use crate::{error::Error, Client, BASE_PATH};
use std::collections::HashMap;

impl Client {
    pub async fn get_tags(&self) -> Result<Vec<(String, String)>, Error> {
        let url = format!("{}{}/tag/list?type=list", &self.url, BASE_PATH);
        let res: HashMap<String, String> = self.get_map(&url).await?;

        let mut list = vec![];
        for (key, description) in res {
            list.push((key, description));
        }

        Ok(list)
    }
}
