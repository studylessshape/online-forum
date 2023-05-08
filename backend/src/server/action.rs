use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};
use clap::Parser;

use crate::{
    files,
    server::{ServeCommandArgs, SERVER_CONFIGURATION, SERVER_CONFIGURATION_PATH},
    user_action,
    utils::get_local_addr,
    visit_action,
};

use super::entity::ServerConfiguration;

fn config_all_action(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index_route)))
        .configure(user_action::config_user_action)
        .configure(visit_action::config_visit_action)
        .configure(files::config_file);
}

async fn index_route() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

pub fn read_config() -> Option<ServerConfiguration> {
    let args = ServeCommandArgs::parse();

    log::info!("Read config file......");
    *SERVER_CONFIGURATION.lock().unwrap() =
        match ServerConfiguration::read_or_create(SERVER_CONFIGURATION_PATH) {
            Ok(conf) => conf,
            Err(err) => {
                log::error!(
                    "Read `{}` ouccr error! Will use default configuration to start server!",
                    SERVER_CONFIGURATION_PATH
                );
                log::error!("Error: {:?}", err);
                Default::default()
            }
        };

    let mut server_config = SERVER_CONFIGURATION.lock().unwrap().clone();

    // valid the email account
    if !server_config.email_account.is_valid() {
        log::warn!(
            "Please config your email and password in file `{}`",
            SERVER_CONFIGURATION_PATH
        );
        return None;
    }
    // test the database config
    {
        let db_config_res = server_config.database_config.test_connection();
        if let Err(err) = db_config_res {
            log::error!("{}", err);
            log::error!("Database config is invalid!");
            return None;
        }
    }
    // get port, origin and method from args
    server_config.port = args.port;
    if args.origins.is_some() {
        server_config.allow_origins = args.origins.unwrap();
    }
    if server_config.allow_origins.len() == 0 {
        log::warn!("Allow all origin will not accpet any credentials!");
    }
    if args.methods.is_some() {
        server_config.allow_methods = args.methods.unwrap();
    }

    log::info!(
        "Admin account [name=\"root\", password=\"{}\"]",
        server_config.replace_root_password()
    );

    Some(server_config)
}

pub async fn server_run(server_config: ServerConfiguration) -> std::io::Result<()> {
    let port = server_config.port;
    let address = get_local_addr().unwrap().ip();

    log::info!(
        "Start server on http://{}:{} (public) and http://localhost:{} (local)",
        address,
        port,
        port
    );

    HttpServer::new(move || {
        let cors = config_corsss(&server_config);
        App::new()
            // add cors
            .wrap(cors)
            // config action
            .configure(config_all_action)
            // add middle ware
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    // bind local host
    .bind(("127.0.0.1", port))?
    // bind server ip
    .bind((address, port))?
    .run()
    .await
}

pub fn config_corsss(server_config: &ServerConfiguration) -> Cors {
    let mut cors = Cors::default()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    cors = if server_config.allow_origins.len() > 0 {
        for origin in server_config.allow_origins.iter() {
            cors = cors.allowed_origin(origin);
        }
        cors
    } else {
        cors.allow_any_origin()
    };

    if server_config.allow_methods.len() > 0 {
        cors.allowed_methods(
            server_config
                .allow_methods
                .iter()
                .map(|str| str.as_str())
                .collect::<Vec<&str>>(),
        )
    } else {
        cors.allow_any_method()
    }
}
