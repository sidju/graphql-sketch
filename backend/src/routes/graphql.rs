use super::*;

use crate::graphql::*;

pub(super) fn graphql(
  state: &'static State,
  schema: &'static Schema,
  req: Request
) -> Result<Response, Error> {
  match req.method() {
    &Method::POST => {
      Ok(Response::new("Hello from graphql post handler".into()))
    },
    &Method::GET => {
      // Parse out the graphql query from query string in URL, then execute
      let query: async_graphql::Request = parse_query(&req)?;
      println!{"{:?}", query};
      Ok(Response::new("Hello from graphql get handler".into()))
    },
    _ => Err(Error::method_not_found(&req)),
  }
}
