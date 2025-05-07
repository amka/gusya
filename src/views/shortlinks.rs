use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AddShortLinkResponse {
    pub short_code: String,
}