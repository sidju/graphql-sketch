use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

mod state;
pub use state::*;
mod graphql;
pub use graphql::*;
mod traits;
pub use traits::*;
mod error;
pub use error::*;

mod auth;
mod db;
mod routes;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() {
  let state = init_state().await;
  let graphql_schema = create_schema(state);

  // Define the socket to bind to
  let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

  run_server(state, graphql_schema, addr).await
}

pub(crate) async fn run_server(
  state: &'static State,
  schema: &'static graphql::Schema,
  addr: SocketAddr,
) {
  // Define what to do with requests
  // - A Service is a stateful worker that responds to one request at a time.
  //   service_fn creates a Service from a FnMut that accepts Request and returns a Response Future.
  // - A "MakeService" is a Service that creates more Services.
  //   make_service_fn is essentially the same as service_fn, but requiring that Fn::Return is a Service
  //   Since we can create that from a closure, we can bind in variables to all created Services
  let make_service = make_service_fn(|_conn| async move {
    Ok::<_, Infallible>(service_fn(move |req| routes::handle_request(state, schema, req)))
  });

  // Create and configure the server
  let server = Server::bind(&addr).serve(make_service);

  // Create a parallel task to clean up old sessions from database
  let _cleaner = tokio::task::spawn(async move {
    auth::session::prune_sessions(state).await;
  });

  // Finally run it all (forever)
  match server.await {
    Ok(_) => println!("Server ran successfully"),
    Err(e) => eprintln!("Error occured: {}", e),
  }
}
