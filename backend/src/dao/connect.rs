use mysql::*;

use crate::server::SERVER_CONFIGURATION as server_config;

pub fn get_conn() -> Conn {
    let config = &server_config.lock().unwrap().database_config;

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(&config.ip_or_host))
        .tcp_port(config.port)
        .db_name(Some(&config.db_name))
        .user(Some(&config.name))
        .pass(Some(&config.password));

    Conn::new(opts).unwrap()
}
