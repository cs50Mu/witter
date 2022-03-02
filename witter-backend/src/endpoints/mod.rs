use lazy_static::lazy_static;
// use rand::distributions::Alphanumeric;
// use rand::rngs::OsRng;
// use rand::Rng;
use regex::Regex;
// use tide::convert::Deserialize;
// use tide::Result;
use crate::responses::User;
use crate::State;
use tide::{Request, Result, StatusCode};
// use uuid::Uuid;

pub mod me;
pub mod users;

lazy_static! {
    static ref BEARER_TOKEN_REGEX: Regex = Regex::new("^Bearer (.*)$").unwrap();
}

fn get_header_token<'a>(req: &'a Request<State>, header_key: &str) -> Result<&'a str> {
    let header_value = match req.header(header_key) {
        Some(value) => value,
        None => {
            return Err(tide::Error::from_str(
                StatusCode::BadRequest,
                format!("missing value for `{}` header", header_key),
            ))
        }
    };

    dbg!(header_value);

    // 小技巧：通过闭包来链式处理多个 option，适用于需要返回统一错误的情况
    let token = (|| BEARER_TOKEN_REGEX.captures(header_value.as_str())?.get(1))();
    match token {
        Some(token) => Ok(token.as_str()),
        None => Err(tide::Error::from_str(
            StatusCode::BadRequest,
            format!("unable to parse `{}` header value", header_key),
        )),
    }
}

async fn authenticate(req: Request<State>) -> Result<User> {
    let db_pool = &req.state().db_pool;

    let token = get_header_token(&req, "Authentication")?;

    // sqlxError --> anyhowError --> tideError
    let user = sqlx::query_as!(
        User,
        r#"
        select users.id, users.username
        from users
        join auth_tokens
        on auth_tokens.user_id = users.id
        where auth_tokens.token = $1
        "#,
        token
    )
    .fetch_one(db_pool)
    .await?;

    Ok(user)
}
