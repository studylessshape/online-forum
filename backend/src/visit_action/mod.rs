use actix_web::web::{self, ServiceConfig};

use self::{post_action::*, section_action::*};

pub mod post_action;
pub mod section_action;

pub fn config_visit_action(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/section")
            .service(web::resource("/get-status").route(web::post().to(get_section_status)))
            .service(
                web::resource("/get-total-post-count")
                    .route(web::get().to(get_section_total_post_count)),
            )
            .service(web::resource("/get-per-page-post").route(web::post().to(get_per_page_post)))
            .service(web::resource("/get-lastest-post").route(web::get().to(get_lastest_post))),
    )
    .service(
        web::scope("/post")
            .service(web::resource("/get-post").route(web::get().to(get_post)))
            .service(web::resource("/del-post").route(web::get().to(delete_post)))
            .service(web::resource("/edit-post").route(web::post().to(edit_post)))
            .service(
                web::resource("/get-per-page-comment").route(web::post().to(get_per_page_comments)),
            )
            .service(web::resource("/put-post-comment").route(web::post().to(put_post_comment)))
            .service(
                web::resource("/delete-post-comment").route(web::post().to(delete_post_comment)),
            )
            .service(web::resource("/search").route(web::post().to(search_post))),
    );
}
