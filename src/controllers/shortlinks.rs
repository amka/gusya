use axum::body::Body;
use axum::debug_handler;
use axum::http::StatusCode;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize, Debug)]
pub struct AddShortLinkPayload {
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct AddShortLinkResponse {
    pub short_code: String,
}

#[debug_handler]
pub async fn redirect(
    Path(short_code): Path<String>,
    State(_ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    debug!("redirect to {}", short_code);
    
    // Set the location header and 302 status code
    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "https://example.com")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::empty())?;

    Ok(response)
}

#[debug_handler]
pub async fn add(
    State(_ctx): State<AppContext>,
    Json(payload): Json<AddShortLinkPayload>,
) -> Result<Response> {
    debug!("add short link from {:?}", payload);
    let short_code = "short_code".to_string();
    format::json(AddShortLinkResponse { short_code })
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/{short_code}", get(redirect))
        .add("/", post(add))
}
