use actix_web::{web, HttpResponse, Responder};
use mysql::serde_json::json;

use crate::dao::{
    user::{UserIdentity, UserNameAndPwd, UserSummary},
    ConditionFor,
};

/// bind path `/user/sign-in` with method `post`
pub async fn sign_in(user: web::Json<UserNameAndPwd>) -> impl Responder {
    let box_user: Box<dyn ConditionFor<UserSummary>> = Box::new(user.into_inner());
    let user_summary_res = UserSummary::try_from(box_user);

    match user_summary_res {
        Ok(mut user_summary) => {
            if user_summary.update_token() {
                HttpResponse::Ok().json(json!({ "user": user_summary }))
            } else {
                HttpResponse::Ok().json(json!({ "responseText": "登录失败" }))
            }
        }
        Err(err) => {
            log::error!("Sign In Error | {:?}", err);
            HttpResponse::Ok().json(json!({ "responseText": "登录失败" }))
        }
    }
}

/// bind path `/user/online-check` with method `post`
pub async fn sign_check(user: web::Json<UserIdentity>) -> impl Responder {
    let box_user: Box<dyn ConditionFor<UserSummary>> = Box::new(user.into_inner());
    let user_summary_res = UserSummary::try_from(box_user);
    match user_summary_res {
        Ok(mut user_summary) => {
            if user_summary.sign_span().num_days() >= 30 {
                return HttpResponse::Ok()
                    .json(json!({ "check_online": false, "responseText": "Token 超时" }));
            }

            if user_summary.update_token() {
                HttpResponse::Ok().json(json!({ "check_online": true }))
            } else {
                HttpResponse::Ok()
                    .json(json!({ "check_online": false, "responseText": "更新登录状态失败" }))
            }
        }
        Err(err) => {
            log::error!("Sign Check Error | {:?}", err);
            HttpResponse::Ok().json(json!({ "responseText": "登录状态检查失败" }))
        }
    }
}
