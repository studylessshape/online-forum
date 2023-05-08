use std::{error, fs, io::Read, io::Write};

use mysql::{Conn, OptsBuilder};
use mysql_common::params;
use serde::{Deserialize, Serialize};

use crate::{dao::user::UserDb, utils::email::is_email};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct EmailAccount {
    pub email: String,
    pub password: String,
    pub smtp_server: String,
}

impl EmailAccount {
    pub fn is_valid(&self) -> bool {
        is_email(&self.email) && self.password.len() > 0 && self.smtp_server.len() > 0
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DataBaseConfig {
    pub name: String,
    pub password: String,
    pub ip_or_host: String,
    pub port: u16,
    pub db_name: String,
}

impl Default for DataBaseConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            password: Default::default(),
            ip_or_host: String::from("localhost"),
            port: 3306,
            db_name: String::from("online_forum"),
        }
    }
}

impl DataBaseConfig {
    pub fn test_connection(&self) -> mysql::Result<Conn> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(&self.ip_or_host))
            .tcp_port(self.port)
            .db_name(Some(&self.db_name))
            .user(Some(&self.name))
            .pass(Some(&self.password));

        Conn::new(opts)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerConfiguration {
    pub port: u16,
    pub allow_origins: Vec<String>,
    pub allow_methods: Vec<String>,
    pub email_account: EmailAccount,
    pub database_config: DataBaseConfig,
    pub admin_password: String,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            port: 8082,
            allow_origins: Default::default(),
            allow_methods: Default::default(),
            email_account: Default::default(),
            database_config: Default::default(),
            admin_password: String::from("123456789"),
        }
    }
}

impl ServerConfiguration {
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = match toml::to_string(self) {
            Ok(c) => c,
            Err(err) => return Err(err.into()),
        };
        fs::write(path, &content).map_err(|err| err.into())
    }

    pub fn read_config(path: &str) -> Result<Self> {
        let read_str = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(err) => return Err(err.into()),
        };
        toml::from_str(&read_str).map_err(|err| err.into())
    }

    pub fn read_or_create(path: &str) -> Result<Self> {
        match fs::File::open(path) {
            Ok(mut file) => {
                let mut content = String::new();
                if let Err(err) = file.read_to_string(&mut content) {
                    return Err(err.into());
                }

                toml::from_str(&content).map_err(|err| err.into())
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => match fs::File::create(path) {
                    Ok(mut file) => {
                        let conf = Self::default();
                        let buf = match toml::to_string(&conf) {
                            Ok(b) => b,
                            Err(err) => return Err(err.into()),
                        };
                        match file.write_all(buf.as_bytes()) {
                            Ok(_) => Ok(conf),
                            Err(err) => Err(err.into()),
                        }
                    }
                    Err(err) => Err(err.into()),
                },
                _ => Err(err.into()),
            },
        }
    }

    pub fn replace_root_password(&self) -> String {
        let mut root = match UserDb::get(("user_name=:name", params! {"name"=>"root"})) {
            Ok(u) => u,
            Err(_) => return self.admin_password.clone(),
        };

        let _ =  root.update_password(&self.admin_password);
        root.password
    }
}
