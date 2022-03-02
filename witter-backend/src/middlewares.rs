pub struct JsonRespMiddleware;
use serde_json::json;
use tide::Request;
use tide::{Middleware, Next};

// 此 middleware 可以将错误转换成 json rsponse
#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for JsonRespMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        // self.log(req, next).await
        let mut response = next.run(req).await;
        if let Some(error) = response.take_error() {
            dbg!(&response);
            response.set_content_type("application/json");
            response.set_body(json!(
                    {
                        "error": {
                        "msg": error.to_string()
                    }
                }
            ));
        }
        Ok(response)
    }
}
