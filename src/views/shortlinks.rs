use loco_rs::controller::format;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddShortLinkResponse {
    pub short_code: String,
    pub short_url: String,
    pub qr_code: Option<String>,
}

pub fn not_found(v: impl ViewRenderer) -> loco_rs::Result<Response> {
    format::render().view(&v, "not-found.html", data!({}))
}
