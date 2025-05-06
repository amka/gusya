use crate::views;
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn render_home(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::home::home(v)
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(render_home))
}
