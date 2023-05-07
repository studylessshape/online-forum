use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use backend::{
    config_all_action,
    server_config::{
        entity::ServerConfiguration, ServeCommandArgs, SERVER_CONFIGURATION,
        SERVER_CONFIGURATION_PATH,
    },
    utils::get_local_addr,
};
use clap::Parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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

    if !server_config.email_account.is_vaild() {
        log::warn!(
            "Please config your email and password in file `{}`",
            SERVER_CONFIGURATION_PATH
        );
        return Ok(());
    }

    server_config.port = args.port;
    if args.origins.len() > 0 {
        server_config.allow_origins = args.origins;
    }
    if args.methods.len() > 0 {
        server_config.allow_methods = args.methods;
    }
    log::info!("Config server......");
    server_run(server_config).await
}

async fn server_run(server_config: ServerConfiguration) -> std::io::Result<()> {
    log::info!(
        "Start server on http://{}:{}",
        get_local_addr().unwrap().ip(),
        server_config.port
    );

    HttpServer::new(move || {
        App::new()
            // add cors
            .wrap(config_corsss(&server_config))
            // config action
            .configure(config_all_action)
            // add middle ware
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8082))?
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
