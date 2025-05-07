use gusya::app::App;
use insta::assert_debug_snapshot;
use loco_rs::testing::prelude::*;
use serial_test::serial;
use gusya::views::shortlinks::AddShortLinkResponse;

#[tokio::test]
#[serial]
async fn can_get_redirect() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/short_link").await;
        assert_eq!(res.status_code(), 302);
        assert_eq!(res.header("location"), "https://example.com");
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
        assert_eq!(res.json::<AddShortLinkResponse>().short_code.is_empty(), false);
        assert_eq!(res.json::<AddShortLinkResponse>().short_url.is_empty(), false);
    })
    .await;
}
