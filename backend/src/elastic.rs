/**
 * Elasticsearch class used to get and put data into Elasticsearch.
 * Also used for calculating product recommendations.
 * This class is made as Elasticsearch official client is not finished yet and don't seem to work with Cloudflare WASM
 */

use std::collections::HashMap;

pub struct Elasticsearch {
  uri: String,
  order_index: String,
  products_index: String
}

impl Elasticsearch {
  pub fn client(uri: String, products_index: String, order_index: String) -> Self {
    print!("{}", uri);

    Elasticsearch {
      uri,
      products_index,
      order_index,
    }
  }

  async fn create_index(self, index: String) -> Result<String, String> {
    let http_client = reqwest::Client::new();
    let request_url = format!("{base}/{index}", base = self.uri, index = index);

    let data = http_client
    .put(request_url)
    .header("content-type", "application/json")
    .header("accept", "application/json")
    .send()
    .await
    .unwrap();

    match data.status() {
      reqwest::StatusCode::OK => Ok("Index created".to_string()),
      reqwest::StatusCode::NOT_FOUND => Err("Index not created".to_string()),
      _ => {
        panic!("Unexepcted error")
      }
    }
  }

  async fn create_index_if_not_exists(self, index: String) -> bool {
    let http_client = reqwest::Client::new();
    let request_url = format!("{base}/{index}", base = self.uri, index = index);

    let data = http_client
      .head(request_url)
      .header("content-type", "application/json")
      .header("accept", "application/json")
      .send()
      .await
      .unwrap();

    match data.status() {
      reqwest::StatusCode::OK => return true,
      reqwest::StatusCode::NOT_FOUND => {
        let status = self.create_index(index).await.unwrap();

        return assert_eq!(status, "Index created")
      },
      _ => {
        panic!("Statuscode don't match 200 or 404")
      }
    }
  }

  pub async fn conver_and_add_order_to_elastic(self, order: char, products: Vec<char>) -> bool {
    // This functions fetches the products and create recommendations to all given products
    // based on both new info and the information inside of elastic.

    return true
  }
}

impl Default for Elasticsearch {
  fn default() -> Self {
    Self {
      uri: "http://localhost:9200".to_string(),
      products_index: "products".to_string(),
      order_index: "orders".to_string()
    }
  }
}
 