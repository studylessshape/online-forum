use std::fs;

use actix_files::Files;
use actix_web::web::{self, ServiceConfig};

use self::file_action::upload_image;

pub mod file_action;
pub mod file_operate;

/// This part set this static files and some method about files such as upload image
pub fn config_file(cfg: &mut ServiceConfig) {
    let _ = fs::create_dir_all("static/resources");

    cfg.service(Files::new("/resources", "static/resources/").use_hidden_files())
        .service(
            web::scope("/files")
                .service(web::resource("/image/{file_name}").route(web::post().to(upload_image))),
        );
}
