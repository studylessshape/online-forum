use actix_web::web::{self, ServiceConfig};

use self::{email_action::*, user_profiles::*, user_sign_in::*, user_sign_up::*};

pub mod email_action;
pub mod user_profiles;
pub mod user_sign_in;
pub mod user_sign_up;

pub fn config_user_action(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/user")
            // user sign in
            .service(web::resource("/sign-in").route(web::post().to(sign_in)))
            .service(web::resource("/online-check").route(web::post().to(sign_check)))
            // user sign up
            .service(web::resource("/require-email-code").route(web::post().to(require_email_code)))
            .service(web::resource("/check-email-code").route(web::post().to(check_email_code)))
            .service(web::resource("/sign-up").route(web::post().to(sign_up)))
            // user profiles update
            .service(
                web::scope("/profiles")
                    .service(
                        web::scope("/update")
                            .service(
                                web::resource("/nickname").route(web::get().to(update_nickname)),
                            )
                            .service(
                                web::resource("/avatar/{file_name}")
                                    .route(web::post().to(update_avatar)),
                            )
                            .service(
                                web::resource("/profiles").route(web::get().to(update_profiles)),
                            )
                            .service(
                                web::resource("/password").route(web::post().to(update_password)),
                            ),
                    )
                    .service(web::resource("/get").route(web::get().to(get_profiles)))
                    .service(
                        web::scope("/posts")
                            .service(web::resource("/get").route(web::get().to(get_user_posts)))
                            .service(web::resource("/del").route(web::post().to(del_user_posts))),
                    ),
            ),
    );
}
