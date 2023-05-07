use actix_web::web::ServiceConfig;

pub mod dao;
pub mod user_action;
pub mod utils;
pub mod visit_action;
pub mod files;
pub mod server_config;

pub fn config_all_action(cfg: &mut ServiceConfig) {
    cfg.configure(user_action::config_user_action)
        .configure(visit_action::config_visit_action)
        .configure(files::config_file);
}