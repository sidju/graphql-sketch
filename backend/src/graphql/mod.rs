use async_graphql::*;
use async_graphql::Schema as OGSchema;

mod schema;
use schema::*;
mod loader;
use loader::*;

type Q = Query;
type M = EmptyMutation;
type S = EmptySubscription;
pub type Schema = OGSchema<Q,M,S>;

pub fn create_schema(state: &'static crate::State) -> &'static Schema {
  Box::leak(Box::new(
    OGSchema::build(Q::default(), M::default(), S::default())
      .data(DataLoader::new(DatabaseLoader{
        mongodb: &state.mongodb,
        mongodb_client: &state.mongodb_client,
      }, tokio::task::spawn))
      .data(state)
      .finish()
  ))
}
