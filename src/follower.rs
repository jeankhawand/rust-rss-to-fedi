use serde::{Serialize};
use serde_json::Value;

use crate::mailer::*;

use reqwest::Request;
use reqwest_middleware::{ClientBuilder, RequestBuilder};
use reqwest::header::{HeaderValue, HeaderMap, HeaderName};

use anyhow::Error as AnyError;

use reqwest::header::{
  ACCEPT
};


#[derive(Debug, Serialize)]
pub struct Follower {
  pub id: i64,
  pub feed_id: i64,
  pub actor: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime
}

impl PartialEq for Follower {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Follower {
  pub async fn find_inbox(&self) -> Result<String, AnyError> {
    Ok(format!("{}/inbox", &self.actor))
    // // get the AP url for the user
    // let webfinger = find_actor_url(&self.actor).await;

    // match webfinger {
    //   Ok(webfinger) => {
    //     let profile_url = webfinger;

    //     let mut host = profile_url.domain().expect("Domain is valid").to_string();
    //     if let Some(port) = profile_url.port() {
    //       host = format!("{}:{}", host, port);
    //     }
      
    //     let mut headers = HeaderMap::new();

    //     headers.insert(
    //       reqwest::header::ACCEPT,
    //       HeaderValue::from_str("application/ld+json").unwrap(),
    //     );

    //     // query that
    //     let client = reqwest::Client::new();
    //     let res = client
    //       .get(profile_url)
    //       .headers(headers)
    //       .send()
    //       .await?;


    //     let body = res.text().await?;

    //     let v: Value = serde_json::from_str(&body).unwrap();
    //     Ok(v["inbox"].as_str().unwrap().to_string())
    //   },
    //   Err(_why) => panic!("oops!")
    // }
  }

}
