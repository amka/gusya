use gusya::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_homes() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/homes/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_add() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/homes/add").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_get() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/homes/get").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

