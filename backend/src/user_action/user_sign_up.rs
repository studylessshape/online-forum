use actix_web::{web::Json, HttpResponse, Responder};
use mysql::serde_json::json;

use crate::dao::user::{UserSignUpInfo, UserSummary};

/// bind path `/user/sign-up` with method `post`
pub async fn sign_up(sign_info: Json<UserSignUpInfo>) -> impl Responder {
    match sign_info.sign_up() {
        Ok(mut user) => {
            if let Err(err) = user.update_nickname(&format!("user{}", user.user_id)) {
                log::error!("Sign up Update Nickname Error | {:?}", err);
                return HttpResponse::Ok().json(json!({"error":["unknow"]}));
            }
            let mut user_summary = UserSummary::from(user);
            if user_summary.update_token() {
                HttpResponse::Ok().json(json!({ "user": user_summary }))
            } else {
                HttpResponse::Ok().json(json!({"error":["unknow"]}))
            }
        }
        Err(response_json) => HttpResponse::Ok().json(response_json),
    }
}
