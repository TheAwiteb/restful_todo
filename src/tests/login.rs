use actix_web::{web, App};
use serial_test::serial;

use crate::{
    auth,
    schemas::{auth::LoginSchema, user::UserSchema},
};

use super::init_test_pool;

pub async fn login_req(username: String, password: String) -> super::TestResponseType {
    let user = LoginSchema { username, password };
    let pool = init_test_pool().await;
    let srv = actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(auth::login::login)
    });
    srv.post("/login").send_json(&user).await.unwrap()
}

#[rstest::rstest]
#[case::login_with_correct_credentials("testusername1", "testpassword", 200)]
#[case::login_with_uncorrect_credentials("testusername2", "testpassword", 400)]
#[actix_web::test]
#[serial(login_content_length)]
async fn login_endpoint(
    #[case] username: String,
    #[case] password: String,
    #[case] status_code: u16,
) {
    let res = login_req(username, password).await;
    assert_eq!(res.status(), status_code);
    super::check_content_type(&res);
    super::check_content_length(&res);
}

/// Deserialize the login response body, and check if the token is not empty
#[actix_web::test]
#[serial]
async fn login_deserialize_body() {
    const USERNAME: &str = "testusername1";
    const PASSWORD: &str = "testpassword";
    let mut res = login_req(USERNAME.to_owned(), PASSWORD.to_owned()).await;
    let login_body: UserSchema =
        serde_json::from_slice(res.body().await.unwrap().to_vec().as_slice())
            .expect("Failed to deserialize login response body");
    assert_eq!(login_body.name, USERNAME);
    assert!(!login_body.token.is_empty());
    super::check_content_type(&res);
    super::check_content_length(&res);
}
