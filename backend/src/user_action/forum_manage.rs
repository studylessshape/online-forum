use actix_web::{web, Either, HttpRequest, HttpResponse, Responder};
use mysql_common::{params, serde_json::json};

use crate::{
    dao::{
        post::{DelPostsRequest, PostSearchRequest},
        section::SectionName,
        user::{UserDb, UserIds, UsersInfoForManage},
    },
    utils::extract_cookie,
};

fn admin_check(req: &HttpRequest) -> Result<(), impl Responder> {
    // get token
    let token = match extract_cookie::<uuid::Uuid>(req, "token") {
        Some(t) => t,
        None => {
            return Err(
                HttpResponse::Ok().json(json!({"success":false, "responseText":"非管理员访问"}))
            )
        }
    };
    // check user has authority
    match UserDb::get(("token=:token", params! {"token"=>token})) {
        Ok(user) => {
            if user.auth_id > 1 {
                Err(HttpResponse::Ok()
                    .json(json!({"success":false, "responseText":"非管理员访问"})))
            } else {
                Ok(())
            }
        }
        Err(err) => {
            log::error!("user_manage.rs | del_users | Get user info err | {:?}", err);
            Err(HttpResponse::Ok()
                .json(json!({"success":false, "responseText":"访问数据库出现错误"})))
        }
    }
}

/// bind path `/manage/user/get` with method `get`
pub async fn get_users() -> impl Responder {
    match UsersInfoForManage::get_all() {
        Ok(users) => HttpResponse::Ok().json(json!({ "users": users })),
        Err(err) => {
            log::error!("user_manage.rs | get_users | {:?}", err);
            HttpResponse::Ok().json(json!({ "responseText": err.to_string() }))
        }
    }
}

/// bind path `/manage/user/del` with method `post`
pub async fn del_users(req: HttpRequest, user_ids: web::Json<UserIds>) -> impl Responder {
    // check user has authority
    if let Err(response) = admin_check(&req) {
        return Either::Left(response);
    }
    // delete user
    match user_ids.del_users() {
        Ok(_) => Either::Right(HttpResponse::Ok().json(json!({ "success": true }))),
        Err(err) => {
            log::error!("user_manage.rs | get_users | {:?}", err);
            Either::Right(
                HttpResponse::Ok()
                    .json(json!({ "success":false, "responseText": err.to_string() })),
            )
        }
    }
}

/// bind path `/manage/post/get` with method `get`
pub async fn get_posts(section_name: web::Query<SectionName>) -> impl Responder {
    let post_query_req = PostSearchRequest::from(section_name.0);

    match post_query_req.section_posts() {
        Ok(posts) => HttpResponse::Ok().json(json!({ "posts": posts })),
        Err(err) => {
            log::error!("user_manage.rs | get_posts | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText": "获取失败"}))
        }
    }
}

/// bind path `/manage/post/del` with method `post`
pub async fn del_posts(req: HttpRequest, post_ids: web::Json<DelPostsRequest>) -> impl Responder {
    // check user has authority
    if let Err(response) = admin_check(&req) {
        return Either::Left(response);
    }
    // delete posts
    match post_ids.del_posts_manage() {
        Ok(_) => Either::Right(HttpResponse::Ok().json(json!({ "success": true }))),
        Err(err) => {
            log::error!("user_manage.rs | get_users | {:?}", err);
            Either::Right(
                HttpResponse::Ok()
                    .json(json!({ "success":false, "responseText": err.to_string() })),
            )
        }
    }
}
