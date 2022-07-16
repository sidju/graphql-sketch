use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::*;

mod types;
pub use types::*;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
  async fn spell(&self,
    ctx: &Context<'_>,
    name: SpellName
  ) -> Result<Option<Spell>, Arc<crate::Error>> {
    let loader = ctx.data_unchecked::<DataLoader<DatabaseLoader>>();
    loader.load_one(name).await
  }
  async fn spell_search(&self,
    ctx: &Context<'_>,
    filter: SpellFilter,
  ) -> Result<Vec<Spell>, &'static str> {
    let _mongodb = &ctx.data_unchecked::<&'static crate::State>().mongodb;
    Ok(Vec::new())
  }
}
