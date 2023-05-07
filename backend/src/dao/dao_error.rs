use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum DaoErrorCode {
    MysqlError(mysql::Error),
    Message(String),
    NotFound,
    Exist,
    TokenError,
    Unknown,
}

pub struct DaoError(pub DaoErrorCode);

impl StdError for DaoError {}

impl fmt::Display for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match &self.0 {
            DaoErrorCode::MysqlError(err) => err.to_string(),
            DaoErrorCode::NotFound => "Not found target from db".to_string(),
            DaoErrorCode::Exist => "Target exist from db".to_string(),
            DaoErrorCode::TokenError => "Some error about token".to_string(),
            DaoErrorCode::Unknown => "Ouccr unknow error".to_string(),
            DaoErrorCode::Message(str) => String::from(str),
        };
        write!(f, "DaoError [{}]", message)
    }
}

impl fmt::Debug for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl DaoError {
    pub fn new(err_code: DaoErrorCode) -> Self {
        Self(err_code)
    }

    pub fn kind(&self) -> &DaoErrorCode {
        &self.0
    }

    pub fn message(str: &str) -> Self {
        Self(DaoErrorCode::Message(String::from(str)))
    }

    pub fn from_mysql_res<T>(res: Result<T, mysql::Error>) -> Result<T, Self> {
        res.or_else(|err| Err(Self::new(DaoErrorCode::MysqlError(err))))
    }

    /// This is for `Conn::exec_first`.
    pub fn from_mysql_res_opt_some<T>(res: Result<Option<T>, mysql::Error>) -> Result<T, Self> {
        match res {
            Ok(opt) => match opt {
                Some(t) => Ok(t),
                None => Err(Self::new(DaoErrorCode::NotFound)),
            },
            Err(err) => Err(Self::new(DaoErrorCode::MysqlError(err))),
        }
    }

    pub fn unknown() -> Self {
        Self(DaoErrorCode::Unknown)
    }
    pub fn not_found() -> Self {
        Self(DaoErrorCode::NotFound)
    }
    pub fn exist() -> Self {
        Self(DaoErrorCode::Exist)
    }
    pub fn token_err() -> Self {
        Self(DaoErrorCode::TokenError)
    }
}

impl From<mysql::Error> for DaoError {
    fn from(value: mysql::Error) -> Self {
        Self(DaoErrorCode::MysqlError(value))
    }
}
