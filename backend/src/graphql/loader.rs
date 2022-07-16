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
  type Error = Arc<crate::Error>;

  async fn load(&self,
    keys: &[SpellName],
  ) -> Result<HashMap<SpellName, Self::Value>, Self::Error> {
    let collection = self.mongodb.collection::<Spell>("spells");
    let mut spells = HashMap::new();
    let mut cursor = collection.find(
      bson::doc!{"name": { "$in": keys }},
      None,
    ).await.map_err(|e| crate::Error::from(e))?;
    while cursor.advance().await.map_err(|e| crate::Error::from(e))? {
      let spell = cursor.deserialize_current().map_err(|e| crate::Error::from(e))?;
      spells.insert(spell.name.clone(), spell);
    }
    Ok(spells)
  }
}
