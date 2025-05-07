use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddShortLinkResponse {
    pub short_code: String,
    pub short_url: String,
    pub qr_code: Option<String>,
}
