use gusya::app::App;
use gusya::views::shortlinks::AddShortLinkResponse;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn redirect_return_not_found() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/not-found-link").await;
        assert_eq!(res.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_create_short_link() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .post("/")
            .json(&serde_json::json!({
                "url": "https://example.com"
            }))
            .await;

        assert_eq!(res.status_code(), 200);
        assert_eq!(
            res.json::<AddShortLinkResponse>().short_code.is_empty(),
            false
        );
        assert_eq!(
            res.json::<AddShortLinkResponse>().short_url.is_empty(),
            false
        );
    })
    .await;
}
