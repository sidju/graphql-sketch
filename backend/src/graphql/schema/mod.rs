use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::*;

mod types;
pub use types::*;

#[derive(Default)]
pub struct Query;

#[Object(rename_args = "snake_case", rename_fields = "snake_case")]
impl Query {

  /// Get a spell by name
  async fn spell(&self,
    ctx: &Context<'_>,
    name: SpellName
  ) -> Result<Option<Spell>, Arc<crate::Error>> {
    let loader = ctx.data_unchecked::<DataLoader<DatabaseLoader>>();
    loader.load_one(name).await
  }

  /// Get spells matching given criteria
  /// Uses mongodb's regex engine
  async fn spell_search(&self,
    ctx: &Context<'_>,
    #[graphql(default)] name_regex: String,
    #[graphql(default)] school_regex: String,
  ) -> Result<Vec<Spell>, Arc<crate::Error>> {
    let mongodb = &ctx.data_unchecked::<&'static crate::State>().mongodb;
    let mut cursor = mongodb.collection::<Spell>("spells").find(
      bson::doc!{
        "name": { "$regex": name_regex},
        "school.school": { "$regex": school_regex }
      },
      None,
    ).await.map_err(crate::Error::from)?;
    let mut spells = Vec::new();
    while cursor.advance().await.map_err(crate::Error::from)? {
      let spell = cursor.deserialize_current().map_err(crate::Error::from)?;
      spells.push(spell);
    }
    Ok(spells)
  }
}
