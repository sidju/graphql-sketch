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
};

pub struct DatabaseLoader{
  pub mongodb: &'static mongodb::Database,
  pub mongodb_client: &'static mongodb::Client,
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
