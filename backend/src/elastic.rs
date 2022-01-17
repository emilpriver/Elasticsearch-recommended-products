/**
 * Elasticsearch class used to get and put data into Elasticsearch.
 * Also used for calculating product recommendations.
 * This class is made as Elasticsearch official client is not finished yet and don't seem to work with Cloudflare WASM
 */

pub struct Elasticsearch {
  uri: String,
}

impl Elasticsearch {
  pub fn client(uri: String) -> Self {
    print!("{}", uri);

    Elasticsearch { uri }
  }

  async fn check_if_index_exists(self, index: String) -> bool {
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
      reqwest::StatusCode::NOT_FOUND => return false,
      _ => {
        panic!("Statuscode don't match 200 or 404")
      }
    }
  }

  pub async fn bulk_add() {

  }
}

impl Default for Elasticsearch {
  fn default() -> Self {
    Self {
      uri: "http://localhost:9200".to_string(),
    }
  }
}
