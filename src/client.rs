use reqwest::Error;

use super::resp;

pub struct CalciferApi {
  server_url: String,
  brand_code: String,
}

impl CalciferApi {
  pub fn new(server_url: String, brand_code: String) -> Self {
    Self {
      server_url,
      brand_code,
    }
  }

  pub async fn get_skus(&self) -> Result<Vec<resp::Sku>, Error> {
    let url = format!("{}/ecfit/{}/skus", self.server_url, self.brand_code);
    reqwest::get(&url).await?.json().await
  }
}
