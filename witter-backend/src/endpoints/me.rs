use super::authenticate;
use crate::responses::ToResponse;
use crate::State;
use tide::{Request, Result};

// get user info
pub async fn get(req: Request<State>) -> Result {
    let user = authenticate(req).await?;

    user.to_response()
}
