use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mysql::{params, serde_json::json};
use uuid::Uuid;

use crate::{
    dao::{
        dao_error::DaoErrorCode,
        post::{DelPostsRequest, PostSummarySimplify},
        user::{
            UserDb, UserNameRequest, UserNicknameRequest, UserPasswordUpdateRequest,
            UserProfilesUpdateRequest,
        },
    },
    files::file_operate::save_net_file,
    utils::extract_cookie,
};

/// bind path `/user/profiles/get?user_name={name}` with method `get`
pub async fn get_profiles(user_name: web::Query<UserNameRequest>) -> impl Responder {
    let user_res = UserDb::get((
        "user_name=:user_name",
        params! {"user_name"=>&user_name.user_name},
    ));

    match user_res {
        Ok(Some(user)) => HttpResponse::Ok().json(json!({
            "avatar": user.avatar,
            "user_nickname": user.user_nickname,
            "regist_time":user.regist_time,
            "description":user.description
        })),
        Ok(None) => HttpResponse::Ok().json(json!({"responseText":"未找到用户"})),
        Err(err) => {
            log::error!("user_profiles.rs | get_profiles | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
    }
}

/// bind path `/user/profiles/update/avatar/{filename}` with method `post`
pub async fn update_avatar(
    req: HttpRequest,
    file_name: web::Path<String>,
    mut _multipart: Multipart,
) -> impl Responder {
    // get token
    let token = match extract_cookie::<Uuid>(&req, "token") {
        Some(t) => t,
        None => return HttpResponse::Ok().json(json!({"responseText":"非用户访问"})),
    };
    // get user by token
    let user_res = UserDb::get(("token=:token", params! {"token"=>token}));

    match user_res {
        Ok(None) => HttpResponse::Ok()
            .json(json!({"avatar": None as Option<()>, "responseText":"未找到用户"})),
        Err(err) => {
            log::error!(
                "user_profiles.rs | update_avatar | Get user ouccr error | {:?}",
                err
            );
            HttpResponse::Ok()
                .json(json!({"avatar": None as Option<()>, "responseText":"发生错误"}))
        }
        // save avatar file
        Ok(Some(mut user)) => match save_net_file(file_name.into_inner(), _multipart).await {
            Err(err) => {
                log::error!(
                    "user_profiles.rs | update_avatar | Save file ouccr error | {:?}",
                    err
                );
                HttpResponse::Ok()
                    .json(json!({"avatar": None as Option<()>, "responseText":"上传图像失败"}))
            }
            // update database
            Ok(file_path) => match user.update_avatar(&file_path) {
                Ok(_) => HttpResponse::Ok().json(json!({"avatar":user.avatar})),
                Err(err) => {
                    log::error!(
                        "user_profiles.rs | update_avatar | Update database ouccr error | {:?}",
                        err
                    );
                    HttpResponse::Ok()
                        .json(json!({"avatar": None as Option<()>, "responseText":"修改数据库时发生错误"}))
                }
            },
        },
    }
}

/// bind path `/user/profiles/update/nickname?user_nickname={name}` with method `get`
pub async fn update_nickname(
    req: HttpRequest,
    new_value: web::Query<UserNicknameRequest>,
) -> impl Responder {
    // get token
    let token = match extract_cookie::<Uuid>(&req, "token") {
        Some(t) => t,
        None => return HttpResponse::Ok().json(json!({"responseText":"非用户访问"})),
    };
    // get user by token
    let user_res = UserDb::get(("token=:token", params! {"token"=>token}));

    match user_res {
        Ok(None) => HttpResponse::Ok().json(json!({"responseText":"未找到用户"})),
        Err(err) => {
            log::error!("user_profiles.rs | update_nickname | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
        Ok(Some(mut user)) => {
            // update user avatar
            if let Err(err) = user.update_nickname(&new_value.into_inner().user_nickname) {
                log::error!("Update Avatar Error | {:?}", err);
                HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
            } else {
                HttpResponse::Ok().json(json!({"user_nickname":user.user_nickname}))
            }
        }
    }
}

/// bind path `/user/profiles/update/profiles?user_nickname={name}&description={description}` with method `get`
pub async fn update_profiles(
    req: HttpRequest,
    new_profiles: web::Query<UserProfilesUpdateRequest>,
) -> impl Responder {
    // get token
    let token = match extract_cookie::<Uuid>(&req, "token") {
        Some(t) => t,
        None => return HttpResponse::Ok().json(json!({"responseText":"非用户访问"})),
    };
    if String::from("") == new_profiles.user_nickname || new_profiles.user_nickname.len() < 3 {
        return HttpResponse::Ok().json(json!({"responseText":"用户昵称长度不正确"}));
    }
    // get user by token
    let user_res = UserDb::get(("token=:token", params! {"token"=>token}));

    match user_res {
        Ok(None) => HttpResponse::Ok().json(json!({"responseText":"未找到用户"})),
        Err(err) => {
            log::error!("user_profiles.rs | update_profiles | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
        Ok(Some(mut user)) => match user
            .update_nickname_description(&new_profiles.user_nickname, &new_profiles.description)
        {
            Ok(user) => HttpResponse::Ok().json(json!({
                "user_nickname": user.user_nickname,
                "description": user.description
            })),
            Err(err) => {
                log::error!(
                    "user_profiles.rs | update_profiles | Update database ouccr error | {:?}",
                    err
                );
                HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
            }
        },
    }
}

/// bind path `/user/profiles/update/password` with method `post`
pub async fn update_password(up_pwd: web::Json<UserPasswordUpdateRequest>) -> impl Responder {
    match up_pwd.update_password() {
        Ok(_) => HttpResponse::Ok().json(json!({"message":"修改成功"})),
        Err(err) => {
            log::error!(
                "user_profiles.rs | update_password | Update password ouccr error | {:?}",
                err
            );
            match err.0 {
                DaoErrorCode::Message(msg) => {
                    HttpResponse::Ok().json(json!({ "responseText": msg }))
                }
                _ => HttpResponse::Ok().json(json!({"responseText":"修改时发生错误"})),
            }
        }
    }
}

/// bind path `/user/profiles/posts/get` with method `get`
pub async fn get_user_posts(user_name_request: web::Query<UserNameRequest>) -> impl Responder {
    match PostSummarySimplify::get_user_posts(&user_name_request.user_name) {
        Ok(posts) => HttpResponse::Ok().json(json!({ "posts": posts })),
        Err(err) => {
            log::error!("user_profiles.rs | get_user_posts | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
    }
}

/// bind path `/user/profiles/posts/del` with method `post`
pub async fn del_user_posts(
    req: HttpRequest,
    del_posts_request: web::Json<DelPostsRequest>,
) -> impl Responder {
    // verify user identity
    let token = match extract_cookie::<Uuid>(&req, "token") {
        Some(t) => t,
        None => return HttpResponse::Ok().json(json!({"responseText":"非用户访问"})),
    };

    match del_posts_request.del_posts(&token) {
        Ok(_) => HttpResponse::Ok().json(json!({"success":true})),
        Err(err) => {
            log::error!("user_profiles.rs | del_user_posts | {:?}", err);
            HttpResponse::Ok().json(json!({"success":false, "responseText":"发生错误"}))
        }
    }
}
