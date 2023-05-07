use actix_web::{web::Json, HttpResponse, Responder};
use mysql::serde_json::json;

use crate::{
    dao::{
        dao_error::DaoErrorCode,
        email::{CheckEmailCodeRequest, EmailCode, GetEmailCodeRequest},
    },
    utils::email::send_email_code,
};

/// bind path `/user/require-email-code` with method `post`
pub async fn require_email_code(
    get_email_code_request: Json<GetEmailCodeRequest>,
) -> impl Responder {
    let code_res = EmailCode::try_from(get_email_code_request.into_inner());
    match code_res {
        // if gen code success, send the code to email
        Ok(email_code) => match send_email_code(email_code) {
            Ok(_) => HttpResponse::Ok().json(json!({"message":"发送成功"})),
            Err(err) => {
                log::error!("Require Email Code Error in Send | {:?}", err);
                HttpResponse::Ok().json(json!({"responseText":"发送失败"}))
            }
        },
        Err(err) => {
            match err.0 {
                DaoErrorCode::NotFound => {
                    return HttpResponse::Ok().json(json!({"responseText":"未找到用户"}))
                }
                DaoErrorCode::Exist => {
                    return HttpResponse::Ok().json(json!({"responseText":"用户已存在"}))
                }
                _ => (),
            }
            log::error!("Require Email Code Error in From Request | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"获取验证码失败"}))
        }
    }
}

/// bind path `/user/check-email-code` with method `post`
pub async fn check_email_code(check_email_code_req: Json<CheckEmailCodeRequest>) -> impl Responder {
    match EmailCode::query_code(&check_email_code_req.email, Some(check_email_code_req.code)) {
        Ok(mut email_code) => {
            match check_email_code_req.delete_code {
                Some(true) => match email_code.delete_code() {
                    Err(err) => log::error!("email_action.rs | check_email_code | Delete email code ouccr error | {:?}", err),
                    _ => (),
                },
                _ => (),
            };
            HttpResponse::Ok().json(json!({"success": true}))
        },
        Err(err) => match err.0 {
            DaoErrorCode::NotFound => HttpResponse::Ok()
                .json(json! ({"success": false, "responseText":"邮箱或验证码不正确"})),
            _ => {
                log::error!("email_action.rs | check_email_code | {:?}", err);
                HttpResponse::Ok().json(json! ({"success": false,"responseText":"发生错误"}))
            }
        },
    }
}
