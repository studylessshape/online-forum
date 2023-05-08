use backend::server::action::{read_config, server_run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_config = read_config();
    if let None = server_config {
        return Ok(());
    }
    log::info!("Config server......");
    // config server and start
    server_run(server_config.unwrap()).await
}