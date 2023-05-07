use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mysql::serde_json::json;
use uuid::Uuid;

use crate::{
    dao::{
        comment::{CommentDeleteRequest, CommentPageRequest, CommentPutRequest},
        post::{PostEditInfo, PostId, PostSearchRequest},
    },
    utils::extract_cookie,
};

/// bind path `/post/get-post` with method `get`
pub async fn get_post(post_id: web::Query<PostId>) -> impl Responder {
    match post_id.get_post() {
        Ok(post) => HttpResponse::Ok().json(json!({ "post": post })),
        Err(err) => {
            log::error!("post_action.rs | get_post | {:?}", err);
            match err.0 {
                crate::dao::dao_error::DaoErrorCode::NotFound => {
                    HttpResponse::Ok().json(json!({"responseText":"帖子不存在"}))
                }
                _ => HttpResponse::Ok().json(json!({"responseText":"发生错误"})),
            }
        }
    }
}

/// bind path `/post/del-post` with method `get`
pub async fn delete_post(req: HttpRequest, post_id: web::Query<PostId>) -> impl Responder {
    let token_res = match extract_cookie::<Uuid>(&req, "token") {
        Some(token) => token,
        None => {
            return HttpResponse::Ok()
                .json(json!({"success":false,"responseText":"No user or token is invalid"}));
        }
    };

    match post_id.del_post(token_res) {
        Ok(_) => HttpResponse::Ok().json(json!({"success":true})),
        Err(err) => {
            log::error!("post_action.rs | delete_post | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
    }
}

/// bind path `/post/edit-post` with method `post`
pub async fn edit_post(post_edit: web::Json<PostEditInfo>) -> impl Responder {
    match post_edit.update_post() {
        Ok(post_id) => HttpResponse::Ok().json(json!({ "post_id": post_id })),
        Err(err) => {
            log::error!("post_action.rs | edit_post | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText": "发生错误"}))
        }
    }
}

/// bind path `/post/get-per-page-comment` with method `post`
pub async fn get_per_page_comments(comment_page: web::Json<CommentPageRequest>) -> impl Responder {
    match comment_page.get_comments() {
        Ok(comments) => HttpResponse::Ok().json(json!({ "comments": comments })),
        Err(err) => {
            log::error!("post_action.rs | get_per_page_comments | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText": "发生错误"}))
        }
    }
}

/// bind path `/post/put-post-comment` with method `post`
pub async fn put_post_comment(
    req: HttpRequest,
    comment_put: web::Json<CommentPutRequest>,
) -> impl Responder {
    let req_token = match extract_cookie::<Uuid>(&req, "token") {
        Some(token) => token,
        None => {
            return HttpResponse::Ok()
                .json(json!({"success":false,"responseText":"No user or token is invalid"}));
        }
    };

    // check token is equal
    if req_token != comment_put.token {
        return HttpResponse::Ok().json(json!({"success":false,"responseText":"Token is invalid"}));
    }

    match comment_put.put_comment() {
        Ok(_) => HttpResponse::Ok().json(json!({"success":true})),
        Err(err) => {
            log::error!("post_action.rs | put_post_comment | {:?}", err);
            HttpResponse::Ok().json(json!({"success":false, "responseText":"发生错误"}))
        }
    }
}

/// bind path `/post/delete-post-comment` with method `post`
pub async fn delete_post_comment(
    req: HttpRequest,
    comment_delete: web::Json<CommentDeleteRequest>,
) -> impl Responder {
    let req_token = match extract_cookie::<Uuid>(&req, "token") {
        Some(token) => token,
        None => {
            return HttpResponse::Ok()
                .json(json!({"success":false,"responseText":"No user or token is invalid"}));
        }
    };

    // check token is equal
    if req_token != comment_delete.token {
        return HttpResponse::Ok().json(json!({"success":false,"responseText":"Token is invalid"}));
    }

    match comment_delete.delete_comment() {
        Ok(_) => HttpResponse::Ok().json(json!({"success":true})),
        Err(err) => {
            log::error!("post_action.rs | put_post_comment | {:?}", err);
            HttpResponse::Ok().json(json!({"success":false, "responseText":"发生错误"}))
        }
    }
}

/// bind path `/post/search` with method `post`
pub async fn search_post(search_request: web::Json<PostSearchRequest>) -> impl Responder {
    match search_request.search() {
        Ok(posts) => HttpResponse::Ok().json(json!({"posts": posts})),
        Err(err) => {
            log::error!("post_action.rs | search_post | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
    }
}
