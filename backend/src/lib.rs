use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

pub mod dao;
pub mod files;
pub mod server_config;
pub mod user_action;
pub mod utils;
pub mod visit_action;

pub fn config_all_action(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index_route)))
        .configure(user_action::config_user_action)
        .configure(visit_action::config_visit_action)
        .configure(files::config_file);
}

async fn index_route() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
