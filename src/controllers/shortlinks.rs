use crate::models::shortlinks::{AddParams, Model};
use crate::views;
use crate::views::shortlinks::AddShortLinkResponse;
use axum::body::Body;
use axum::debug_handler;
use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde::Deserialize;
use tracing::debug;

#[derive(Deserialize, Debug, Validate)]
pub struct AddShortLinkPayload {
    #[validate(url)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Redirect to original URL by short code
///
/// # Errors
///
/// * `Error::NotFound` - if the `Shortlink` is not found in database
/// * `Error::CustomError(StatusCode::GONE)` - if the `Shortlink` is not active
///
/// # Response
///
/// The response will be an HTTP 302 redirect with the location header set to the original URL.
#[debug_handler]
pub async fn redirect(
    Path(code): Path<String>,
    State(_ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    debug!("redirect from {}", code);

    // Поиск ссылки в базе с кешированием
    let link = Model::find_by_code(&_ctx.db, &code).await?;

    // Если ссылка не найдена
    if link.is_none() {
        return views::shortlinks::not_found(v);
    }
    let link = link.unwrap();

    // Check if link is active
    if !link.is_active.unwrap() {
        return Err(Error::CustomError(
            StatusCode::GONE,
            ErrorDetail::with_reason("Ссылка деактивирована"),
        ));
    }

    if let Some(expires_at) = link.expires_at {
        if chrono::Utc::now() > expires_at.and_utc() {
            return Err(Error::CustomError(
                StatusCode::GONE,
                ErrorDetail::with_reason("Срок действия ссылки истек"),
            ));
        }
    }

    // TODO: Check for password

    // Set the location header and 302 status code
    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", link.original_url.unwrap())
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::empty())?;

    Ok(response)
}

/// Add new short link
///
/// # Errors
///
/// If there is an error in database interaction, an error variant will be returned.
///
/// # Response
///
/// The response will be a JSON object in the following format:
///
///
#[debug_handler]
pub async fn add(
    State(_ctx): State<AppContext>,
    Json(payload): Json<AddShortLinkPayload>,
) -> Result<Response> {
    debug!("add short link from {:?}", payload);

    // Covert AddShortLinkPayload to AddParams
    let params = AddParams {
        original_url: payload.url,
        custom_alias: payload.custom_alias,
        domain: payload.domain,
        password: payload.password,
    };

    let link = Model::create_link(&_ctx.db, &params).await?;

    // Формирование ответа
    let full_url = format!(
        "{}/{}",
        link.domain
            .unwrap_or_else(|| _ctx.config.server.full_url().clone()),
        link.short_code
    );

    format::json(AddShortLinkResponse {
        short_code: link.short_code,
        short_url: full_url,
        // qr_code: generate_qr_code(&full_url), // Реализовать отдельно
        qr_code: None,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/{code}", get(redirect))
        .add("/", post(add))
}
