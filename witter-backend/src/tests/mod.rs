#[allow(unused_imports)]
pub mod test_helper;
use test_helper::*;

mod test_db;
use test_db::*;

use serde::Deserialize;

#[async_std::test]
async fn user_create_and_login() {
    let app = test_setup().await;

    // RequestBuilder 实现了 Future trait，await 后会得到 Response
    let mut resp = app
        .post("/users", json!({"username":"bob", "password":"foobar"}))
        .make()
        .await;
    assert_eq!(resp.status(), 201);
    let resp = resp.body_json::<Data<Token>>().await.unwrap();
    let token = dbg!(resp.data.token);

    let mut resp = app
        .get("/me")
        .header("Authentication", format!("Bearer {}", token))
        .make()
        .await;
    assert_eq!(resp.status(), 200);
    let json: Value = resp.body_json().await.unwrap();
    assert_json_include!(
        actual: json,
        expected: json!({
                "data": {"username":"bob"}
            }
        )
    );

    let mut resp = app
        .post("/users/bob/session", json!({"password": "foobar"}))
        .make()
        .await;
    assert_eq!(resp.status(), 200);
    let json: Value = resp.body_json().await.unwrap();
    assert_json_include!(
        actual: json,
        expected: json!({
                "data": {"token":token}
            }
        )
    );
}

#[async_std::test]
async fn logging_in_with_unknown_user() {
    let app = test_setup().await;

    let resp = app
        .post("/users/unknown_user/session", json!({"password": "foobar"}))
        .make()
        .await;
    assert_eq!(resp.status(), 404);
}

#[async_std::test]
async fn logging_in_without_auth_header() {
    let app = test_setup().await;
    let resp = app
        .post("/users", json!({"username":"bob", "password":"foobar"}))
        .make()
        .await;
    assert_eq!(resp.status(), 201);

    let mut resp = app.get("/me").make().await;
    assert_eq!(resp.status(), 400);
    let header = resp.header("content-type").unwrap();
    assert_eq!(header, "application/json");
    let json: Value = resp.body_json().await.unwrap();
    assert_json_include!(
        actual: json,
        expected: json!({
                "error": {"msg": "missing value for `Authentication` header"}
            }
        )
    );
}

#[async_std::test]
async fn logging_in_with_invalid_auth_header() {
    let app = test_setup().await;
    let resp = app
        .post("/users", json!({"username":"bob", "password":"foobar"}))
        .make()
        .await;
    assert_eq!(resp.status(), 201);

    let mut resp = app
        .get("/me")
        .header("Authentication", format!("foo {}", "xxxx"))
        .make()
        .await;
    assert_eq!(resp.status(), 400);
    let header = resp.header("content-type").unwrap();
    assert_eq!(header, "application/json");
    let json: Value = resp.body_json().await.unwrap();
    assert_json_include!(
        actual: json,
        expected: json!({
                "error": {"msg": "unable to parse `Authentication` header value"}
            }
        )
    );
}

#[async_std::test]
async fn logging_in_with_invalid_password() {
    let app = test_setup().await;

    let resp = app
        .post("/users", json!({"username":"bob", "password":"foobar"}))
        .make()
        .await;
    assert_eq!(resp.status(), 201);

    let resp = app
        .post("/users/bob/session", json!({"password": "invalid"}))
        .make()
        .await;
    assert_eq!(resp.status(), 403);
}

#[derive(Debug, Deserialize)]
struct Data<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct Token {
    token: String,
}
