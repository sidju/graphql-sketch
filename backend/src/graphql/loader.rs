use std::sync::Arc;
use std::collections::HashMap;
pub use async_graphql::dataloader::{
  Loader,
  DataLoader,
};
use super::async_trait::async_trait;

use super::{
  Spell,
  SpellName,
  SpellId,
};

pub struct DatabaseLoader{
  pub db_pool: &'static sqlx::postgres::PgPool,
}

#[async_trait]
impl Loader<SpellName> for DatabaseLoader {
  type Value = Spell;
  type Error = Arc<sqlx::Error>;

  async fn load(&self,
    keys: &[SpellName],
  ) -> Result<HashMap<SpellName, Self::Value>, Self::Error> {
    Ok(HashMap::new())
  }
}
