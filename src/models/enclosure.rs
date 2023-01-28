
use sqlx::postgres::PgPool;
use serde::{Serialize};

use chrono::Utc;

use crate::models::item::Item;

#[derive(Debug, Serialize)]
pub struct Enclosure {
  pub id: i32,
  pub item_id: i32,
  pub url: String,
  pub content_type: Option<String>,
  pub size: Option<i32>,

  pub created_at: chrono::DateTime::<Utc>,
  pub updated_at: chrono::DateTime::<Utc>
}

impl PartialEq for Enclosure {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id || (self.item_id == other.item_id && self.url == other.url)
  }
}

impl Enclosure {
  pub async fn for_item(item: &Item, pool: &PgPool) -> Result<Vec<Enclosure>, sqlx::Error> {
    sqlx::query_as!(Enclosure, "SELECT * FROM enclosures WHERE item_id = $1 ORDER by id", item.id)
    .fetch_all(pool)
    .await
  }
}

