use std::collections::HashMap;
use std::sync::Arc;
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
  ) -> Result<Option<Spell>, Arc<sqlx::Error>> {
    let loader = ctx.data_unchecked::<DataLoader<DatabaseLoader>>();
    loader.load_one(name).await
  }
}
