use surf::{Client, RequestBuilder, Response};

pub use crate::{make_app, State};
pub use assert_json_diff::{assert_json_eq, assert_json_include};
use async_trait::async_trait;
pub use serde_json::json;
pub use serde_json::Value;
use std::ops::Deref;
use tide::Server;

use crate::tests::TestDb;

pub async fn test_setup() -> TestServer {
    dotenv::dotenv().ok();
    // tide::log 只能 start 一次，跑多个测试会有问题
    // tide::log::start();
    pretty_env_logger::try_init().ok();
    std::env::set_var("APP_ENV", "test");

    let test_db = TestDb::new().await;
    let db_pool = test_db.db();
    let app = make_app(db_pool).await;
    TestServer { app, test_db }
}

// bind test_db into TestServer
// 否则测试数据库会被提前 drop
#[allow(dead_code)]
pub struct TestServer {
    app: Server<State>,
    test_db: TestDb,
}

// 有了这个以后，TestServer 就可以当成 app 用啦
// 这真 TM 酷
// 参考：https://stackoverflow.com/a/32552688/1543462
impl Deref for TestServer {
    type Target = Server<State>;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

// 为啥 app 在扩展了这个 trait 后，既可以当 client 又可以当 server
// 玄机在于 `tide::Server` 实现了 HttpClient trait，并且对这个 trait 的
// `send` 方法做了如下定义：
// #[crate::utils::async_trait]
// impl<State: Clone + Send + Sync + Unpin + 'static> http_client::HttpClient for Server<State> {
//     async fn send(&self, req: crate::http::Request) -> crate::http::Result<crate::http::Response> {
//         self.respond(req).await
//     }
// }
/// Trait that adds test request capabilities to tide [`Server`]s
#[async_trait]
pub trait ServerTestingExt {
    /// Construct a new surf Client
    fn client(&self) -> Client;

    /// Builds a `CONNECT` request.
    async fn connect(&self, uri: &str) -> Response {
        self.client().connect(uri).await.unwrap()
    }

    /// Builds a `DELETE` request.
    async fn delete(&self, uri: &str) -> Response {
        self.client().delete(uri).await.unwrap()
    }

    /// Builds a `GET` request.
    fn get(&self, uri: &str) -> RequestBuilder {
        self.client().get(uri)
    }

    /// Builds a `HEAD` request.
    async fn head(&self, uri: &str) -> Response {
        self.client().head(uri).await.unwrap()
    }

    /// Builds an `OPTIONS` request.
    async fn options(&self, uri: &str) -> Response {
        self.client().options(uri).await.unwrap()
    }

    /// Builds a `PATCH` request.
    async fn patch(&self, uri: &str) -> Response {
        self.client().patch(uri).await.unwrap()
    }

    /// Builds a `POST` request.
    fn post(&self, uri: &str, body: Value) -> RequestBuilder {
        self.client().post(uri).body_json(&body).unwrap()
    }

    /// Builds a `PUT` request.
    async fn put(&self, uri: &str) -> Response {
        self.client().put(uri).await.unwrap()
    }

    /// Builds a `TRACE` request.
    async fn trace(&self, uri: &str) -> Response {
        self.client().trace(uri).await.unwrap()
    }
}

#[allow(deprecated)]
impl<State: Clone + Send + Sync + Unpin + 'static> ServerTestingExt for tide::Server<State> {
    fn client(&self) -> Client {
        let mut client = Client::with_http_client(self.clone());
        client.set_base_url(tide::http::Url::parse("http://example.com").unwrap());
        client
    }
}

// 这里为 RequestBuilder 新增一个方法
// 主要目的是，可以在写测试的时候更爽一点
// 不用 unwrap 了
#[async_trait]
pub trait SendRequest {
    async fn make(self) -> Response;
}

#[async_trait]
impl SendRequest for RequestBuilder {
    async fn make(self) -> Response {
        self.send().await.unwrap()
    }
}
