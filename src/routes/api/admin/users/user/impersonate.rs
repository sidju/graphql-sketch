use super::*;

#[derive(Deserialize)]
struct Impersonate {
  admin_password: String,
}

// Note that this password validation is does allow an attacker to know if the
// admin whose session they have stolen has a password or not via timing.
// But there shouldn't be a session otherwise, so not really a risk.
pub async fn route(
  state: &'static State,
  mut req: Request,
  path_vec: Vec<String>,
  permissions: Permissions,
  userid: i32,
) -> Result<Response, Error> {
  verify_method_path_end(&path_vec, &req, &Method::POST)?;
  let query: Impersonate = from_json(&mut req).await?;

  // Verify the admin_password, so it takes more than a session key to
  // create unlimited session keys
  let admin_user = sqlx::query!(
    "SELECT pass, locked FROM users WHERE id = $1",
    permissions.userid
  )
    .fetch_one(&state.db_pool)
    .await?
  ;
  let admin_hash = match admin_user.pass {
    Some(hash) => {
      hash
    },
    None => {
      // Normally impossible, since setting passhash to None
      // also deletes all sessions (but maybe race condition).
      // However, impersonate makes it possible again.
      return Err(Error::BadLogin);
    },
  };
  let correct_pass = crate::auth::hash::verify(
    &state.cpu_semaphore,
    &state.hasher,
    admin_hash,
    query.admin_password,
  )
    .await?
  ;

  if !correct_pass { return Err(Error::BadLogin); }
  if admin_user.locked { return Err(Error::AccountLocked); }

  // With all verification done we create the session
  let key = nanoid::nanoid!(32);
  // Allow only un-extended sessions for impersonation
  let until = chrono::offset::Utc::now().naive_utc() + chrono::Duration::days(1);

  let ret = sqlx::query_as!( crate::auth::Session,
    "INSERT INTO sessions(userid, key, until) VALUES($1, $2, $3) RETURNING id, key, until",
    userid,
    &key,
    &until,
  )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| -> Error { match e {
      sqlx::Error::Database(ref err) => {
        if err.constraint() == Some("key") { Error::SessionKeyCollision }
        else { e.into() }
      },
      _ => e.into(),
    }})
    ?
  ;

  // Return, should be the exact same as login handlers return format
  json(&ret)
}