use middlewares::JsonRespMiddleware;
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use tide::Server;

#[cfg(test)]
mod tests;

mod endpoints;
mod env;
mod middlewares;
mod responses;

#[async_std::main]
async fn main() {
    tide::log::start();
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    let app = make_app(pool).await;
    app.listen("127.0.0.1:8080").await.unwrap();
}

#[derive(Clone)]
pub struct State {
    db_pool: Pool<Postgres>,
}

pub async fn make_app(pool: Pool<Postgres>) -> Server<State> {
    let mut app = tide::with_state(State { db_pool: pool });
    app.with(JsonRespMiddleware);

    app.at("/users").post(endpoints::users::create);

    app.at("/me").get(endpoints::me::get);

    app.at("/users/:username/session")
        .post(endpoints::users::login);
    app
}
