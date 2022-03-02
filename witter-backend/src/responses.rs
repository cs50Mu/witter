use tide::convert::Serialize;
use tide::Result;
use tide::{Body, Response, StatusCode};
use uuid::Uuid;

pub trait BodyJson {
    fn body_json(self, body: &impl Serialize) -> Result<Self>
    where
        Self: Sized;
}

impl BodyJson for Response {
    fn body_json(mut self, body: &impl Serialize) -> Result<Self> {
        self.set_body(Body::from_json(body)?);
        Ok(self)
    }
}

#[derive(Serialize)]
struct ApiResponse<T> {
    data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    fn new(data: T) -> ApiResponse<T> {
        Self { data }
    }

    #[allow(dead_code)]
    fn to_response(&self, status: StatusCode) -> Result<Response> {
        Response::new(status).body_json(&self)
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

impl TokenResponse {
    pub fn new(token: String) -> TokenResponse {
        Self { token }
    }
}

pub trait ToResponse: Serialize {
    fn to_response_with_status(&self, status: StatusCode) -> Result<Response> {
        ApiResponse::new(self).to_response(status)
    }

    // 默认 status 使用 200
    fn to_response(&self) -> Result<Response> {
        self.to_response_with_status(StatusCode::Ok)
    }
}

// 为任意实现了 Serialize 的类型实现 ToResponse trait
impl<T: Serialize> ToResponse for T {}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}
