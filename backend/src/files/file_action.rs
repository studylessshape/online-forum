use actix_multipart::Multipart;
use actix_web::{web, Responder, HttpResponse};
use mysql::serde_json::json;

use super::file_operate::save_net_file;


/// route to path "/files/image/{file_name}" bind method "POST"
pub async fn upload_image(path: web::Path<String>, mut _multipart: Multipart) -> impl Responder {
    match save_net_file(path.into_inner(), _multipart).await {
        Ok(file_path) => HttpResponse::Ok().json(json!({ "src": file_path })),
        Err(err) => {
            log::error!("file_action.rs | upload_image | {:?}", err);
            HttpResponse::Ok().json(json!({"src": None as Option<()>, "responseText":"上传失败"}))
        },
    }
}