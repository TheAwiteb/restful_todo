use actix_web::{web, App};
use serial_test::serial;

use crate::{auth, schemas::user::UserSchema};

use super::init_test_pool;

/// Send revoke request, return the response
async fn revoke_request(token: String) -> super::TestResponseType {
    let pool = init_test_pool().await;
    let srv = actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(auth::revoke::revoke)
    });
    srv.patch("/revoke")
        .append_header(("Authorization", token))
        .send()
        .await
        .unwrap()
}

#[actix_web::test]
#[serial]
async fn revoke_with_currect_token() {
    let mut login_res =
        super::login::login_req("testusername1".to_owned(), "testpassword".to_owned()).await;
    let user: UserSchema =
        serde_json::from_slice(login_res.body().await.unwrap().to_vec().as_slice()).unwrap();
    let mut revoke_res = revoke_request(format!("Bearer {}", user.token)).await;
    println!("{:?}", revoke_res);
    println!("{:?}", revoke_res.body().await.unwrap());
    assert_eq!(revoke_res.status(), 200);
    super::check_content_type(&revoke_res);
    super::check_content_length(&revoke_res);
}

#[rstest::rstest]
#[case::revoke_with_uncorrect_token("Bearer testtoken", 401)]
#[case::revoke_without_barear("testtoken", 400)]
#[case::revoke_with_empty_token("", 400)]
#[actix_web::test]
#[serial]
async fn revoke_endpoin(#[case] token: String, #[case] status_code: u16) {
    let revoke_res = revoke_request(token).await;
    assert_eq!(revoke_res.status(), status_code);
    super::check_content_type(&revoke_res);
    super::check_content_length(&revoke_res);
}
