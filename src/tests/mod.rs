use migration::{Migrator, MigratorTrait};

mod login;
mod register;
mod revoke;

pub type TestResponseType = awc::ClientResponse<
    actix_web::dev::Decompress<
        actix_http::Payload<
            std::pin::Pin<
                std::boxed::Box<
                    dyn futures_core::Stream<
                        Item = std::result::Result<
                            actix_web::web::Bytes,
                            actix_web::error::PayloadError,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;

pub async fn init_test_pool() -> sea_orm::DatabaseConnection {
    let pool = crate::enishalize_poll().await;
    Migrator::up(&pool, None)
        .await
        .expect("Failed to run migrations");
    pool
}

/// Check if the response content type is application/json
pub fn check_content_type(res: &TestResponseType) {
    assert_eq!(
        res.headers()
            .get("Content-Type")
            .expect("No Content-Type header"),
        "application/json"
    );
}

/// Check if the response content length is not 0
pub fn check_content_length(res: &TestResponseType) {
    assert!(
        res.headers()
            .get("Content-Length")
            .expect("No Content-Length header")
            != "0"
    );
}
