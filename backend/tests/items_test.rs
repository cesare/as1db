use actix_web::{App, test, web::Data};
use serde_json::{Value, json};
use sqlx::PgPool;

mod common;

#[sqlx::test(fixtures("classes", "categories", "items", "item_categories", "materials"))]
async fn index_with_details(pool: PgPool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .app_data(Data::new(context))
            .configure(as1db::handlers::routes),
    )
    .await;

    let request = test::TestRequest::get().uri("/items/with_details").to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "items": [
            {
                "id": 1,
                "name": "item-01",
                "class": {
                    "id": 1,
                    "name": "class-01",
                },
                "categories": [
                    {
                        "id": 1,
                        "name": "category-01",
                    },
                ],
                "materialCategories": [
                    {
                        "id": 2,
                        "name": "category-02"
                    },
                ],
                "materialItems": [
                    {
                        "id": 2,
                        "name": "item-02",
                        "classId": 1,
                    },
                    {
                        "id": 3,
                        "name": "item-03",
                        "classId": 2,

                    }
                ],
            },
            {
                "id": 2,
                "name": "item-02",
                "class": {
                    "id": 1,
                    "name": "class-01",
                },
                "categories": [
                    {
                        "id": 1,
                        "name": "category-01",
                    },
                ],
                "materialCategories": [
                    {
                        "id": 1,
                        "name": "category-01"
                    },
                ],
                "materialItems": [
                    {
                        "id": 3,
                        "name": "item-03",
                        "classId": 2,

                    }
                ],
            },
            {
                "id": 3,
                "name": "item-03",
                "class": {
                    "id": 2,
                    "name": "class-02",
                },
                "categories": [
                    {
                        "id": 2,
                        "name": "category-02",
                    },
                ],
                "materialCategories": [
                    {
                        "id": 1,
                        "name": "category-01"
                    },
                    {
                        "id": 2,
                        "name": "category-02"
                    },
                ],
                "materialItems": [
                    {
                        "id": 1,
                        "name": "item-01",
                        "classId": 1,

                    }
                ],
            },
        ],
    });
    assert_eq!(response_json, expected_json);
}

#[sqlx::test(fixtures("classes", "categories", "items", "item_categories", "materials"))]
async fn show(pool: PgPool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .app_data(Data::new(context))
            .configure(as1db::handlers::routes),
    )
    .await;

    let request = test::TestRequest::get().uri("/items/1").to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "item": {
            "id": 1,
            "name": "item-01",
            "class": {
                "id": 1,
                "name": "class-01",
            },
            "categories": [
                {
                    "id": 1,
                    "name": "category-01",
                },
            ],
            "materialCategories": [
                {
                    "id": 2,
                    "name": "category-02"
                },
            ],
            "materialItems": [
                {
                    "id": 2,
                    "name": "item-02",
                    "classId": 1,
                },
                {
                    "id": 3,
                    "name": "item-03",
                    "classId": 2,

                }
            ],
        },
    });
    assert_eq!(response_json, expected_json);
}
