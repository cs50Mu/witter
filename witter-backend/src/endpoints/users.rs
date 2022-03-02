use crate::env;
use crate::responses::{ToResponse, TokenResponse};
use crate::State;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::Rng;
use tide::convert::Deserialize;
use tide::{Request, Response, Result, StatusCode};
use uuid::Uuid;

pub async fn login(mut req: Request<State>) -> Result {
    let db_pool = &req.state().db_pool.clone();
    let username = req.param("username").map(String::from)?;
    // get password
    let password = req.body_json::<Password>().await?.password;

    // fetch user
    let user = sqlx::query!(
        r#"
            select *
            from users
            where username = $1
                "#,
        username
    )
    .fetch_optional(db_pool)
    .await?;
    let user = match user {
        Some(user) => user,
        None => {
            return Ok(Response::new(StatusCode::NotFound));
        }
    };
    // verify password
    let is_valid = verify(password, &user.hashed_password)?;
    if is_valid {
        let token = sqlx::query!(
            r#"
                select token
                from auth_tokens
                where user_id = $1
                "#,
            user.id
        )
        .fetch_one(db_pool)
        .await?;

        TokenResponse::new(token.token).to_response()
    } else {
        Ok(Response::new(StatusCode::Forbidden))
    }
}

pub async fn create(mut req: Request<State>) -> Result {
    let db_pool = &req.state().db_pool.clone();
    let user: CreateUser = req.body_json().await?;
    // https://github.com/Keats/rust-bcrypt
    let hash_cost = match env::current() {
        env::Env::Test => 4,
        _ => DEFAULT_COST,
    };
    let hashed_password = hash(user.password, hash_cost)?;
    dbg!(&hashed_password);
    let now = Utc::now();
    let row = sqlx::query!(
        r#"
            insert into users (id, username, hashed_password, created_at, updated_at)
            values ($1, $2, $3, $4, $5) returning id
                "#,
        Uuid::new_v4(),
        user.username,
        hashed_password,
        now,
        now,
    )
    .fetch_one(db_pool)
    .await?;
    let user_id = row.id;

    let token: String = OsRng
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let row = sqlx::query!(
        r#"
            insert into auth_tokens (id, user_id, token, created_at, updated_at)
            values ($1, $2, $3, $4, $5) returning token
            "#,
        Uuid::new_v4(),
        user_id,
        token,
        now,
        now,
    )
    .fetch_one(db_pool)
    .await?;

    TokenResponse::new(row.token).to_response_with_status(StatusCode::Created)
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Password {
    password: String,
}
