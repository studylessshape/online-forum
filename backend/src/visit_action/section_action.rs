use actix_web::{web, HttpResponse, Responder};
use mysql::serde_json::json;

use crate::dao::{section::{SectionName, SectionNames, SectionPageRequest}, post::PostSimplify};

/// bind path `/section/get-status` with method `POST`
pub async fn get_section_status(section_names: web::Json<SectionNames>) -> impl Responder {
    // get status
    match section_names.get_status() {
        Ok(vec_status) => {
            if vec_status.len() == 0 {
                return HttpResponse::Ok().json(json!({"responseText": "查找失败"}));
            }
            HttpResponse::Ok().json(json!({ "statuses": vec_status }))
        }
        Err(err) => {
            log::error!("section_action.rs | get_section_status | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
    }
}

/// bind path `/section/get-total-post-count` with method `get`
pub async fn get_section_total_post_count(section_name: web::Query<SectionName>) -> impl Responder {
    match section_name.get_total_post_count() {
        Ok(status) => HttpResponse::Ok().json(json!({
            "section_name":status.section_name,
            "count": status.total_post_count
        })),
        Err(err) => {
            log::error!(
                "section_action.rs | get_section_total_post_count | {:?}",
                err
            );
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
    }
}

/// bind path `/section/get-per-page-post` with method `post`
pub async fn get_per_page_post(section_page: web::Json<SectionPageRequest>) -> impl Responder {
    match section_page.get_posts() {
        Ok(data) => HttpResponse::Ok()
            .json(json!({"section_name": &section_page.section_name, "posts":data})),
        Err(err) => {
            log::error!("section_action.rs | get_per_page_post | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText":"发生错误"}))
        }
    }
}

/// bind path `/section/get-lastest-post` with method `get`
pub async fn get_lastest_post(section_name: web::Query<SectionName>) -> impl Responder {
    match section_name.get_lastest_post() {
        Ok(post_opt) => {
            match post_opt {
                Some(post) => HttpResponse::Ok().json(json!({"post": PostSimplify::from(post)})),
                None => HttpResponse::Ok().json(json!({"post": null})),
            }
        },
        Err(err) => {
            log::error!("section_action.rs | get_lastest_post | {:?}", err);
            HttpResponse::Ok().json(json!({"responseText": "发生错误"}))
        }
    }
}