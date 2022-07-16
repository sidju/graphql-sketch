use super::*;

use crate::graphql::*;

pub(super) async fn graphql(
  state: &'static State,
  schema: &'static Schema,
  mut req: Request
) -> Result<Response, Error> {
  match req.method() {
    &Method::POST => {
      let body = get_body(&mut req, state.max_content_len).await?;
      let content_type = get_header(&req, "content_type")?;
      let opts = async_graphql::http::MultipartOptions::default();
      let query = async_graphql::http::receive_batch_body(
        content_type,
        &body[..],
        opts,
      ).await?;
     let res = schema.execute_batch(query).await;
     json(&res)
    },
    &Method::GET => {
      // Parse out the graphql query from query string in URL, then execute
      let query: async_graphql::Request = parse_query(&req)?;
      let res = schema.execute(query).await;
      json(&res)
    },
    _ => Err(Error::method_not_found(&req)),
  }
}
