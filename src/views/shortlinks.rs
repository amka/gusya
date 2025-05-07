use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AddShortLinkPayload {
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct AddShortLinkResponse {
    pub short_code: String,
}