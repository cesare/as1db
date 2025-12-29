use actix_web::{App, test, web::Data};
use serde_json::{Value, json};
use sqlx::PgPool;

mod common;

#[sqlx::test(fixtures("classes"))]
async fn index(pool: PgPool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .app_data(Data::new(context))
            .configure(as1db::handlers::routes),
    )
    .await;

    let request = test::TestRequest::get().uri("/classes").to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "classes": [
            {
                "id": 1,
                "name": "category-01",
            },
            {
                "id": 2,
                "name": "category-02",
            },
        ],
    });
    assert_eq!(response_json, expected_json);
}

#[sqlx::test(fixtures("classes", "items"))]
async fn show(pool: PgPool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .app_data(Data::new(context))
            .configure(as1db::handlers::routes),
    )
    .await;

    let request = test::TestRequest::get().uri("/classes/1").to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "class": {
            "id": 1,
            "name": "category-01",
        },
        "items": [
            {
                "id": 1,
                "classId": 1,
                "name": "item-01",
            },
            {
                "id": 2,
                "classId": 1,
                "name": "item-02",
            },
        ],
    });
    assert_eq!(response_json, expected_json);
}
