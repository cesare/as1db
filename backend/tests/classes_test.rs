use actix_web::{App, test, web::Data};
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
}
