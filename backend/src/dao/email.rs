use chrono::{Local, NaiveDateTime};
use mysql::{params, prelude::Queryable};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{connect::get_conn, dao_error::DaoError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetEmailCodeRequest {
    pub email: String,
    pub check_user_rule: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum EmailCodeCheckUserRule {
    NoCheck,
    NoExist,
    IsExist,
}

impl Default for EmailCodeCheckUserRule {
    fn default() -> Self {
        Self::NoCheck
    }
}

impl GetEmailCodeRequest {
    const NO_CHECK: &str = "no-check";
    const NO_EXIST: &str = "no-exist";
    const IS_EXIST: &str = "is-exist";

    pub fn get_user_rule(&self) -> EmailCodeCheckUserRule {
        match &self.check_user_rule {
            Some(rule) => match rule.as_str() {
                Self::NO_CHECK => EmailCodeCheckUserRule::NoCheck,
                Self::NO_EXIST => EmailCodeCheckUserRule::NoExist,
                Self::IS_EXIST => EmailCodeCheckUserRule::IsExist,
                _ => Default::default(),
            },
            None => Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckEmailCodeRequest {
    pub email: String,
    pub code: i32,
    pub delete_code: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailCode {
    pub email: String,
    pub code: i32,
    pub get_date: NaiveDateTime,
}

impl TryFrom<GetEmailCodeRequest> for EmailCode {
    type Error = DaoError;

    fn try_from(req: GetEmailCodeRequest) -> Result<Self, Self::Error> {
        let mut conn = get_conn();
        // check the rule
        let user_rule = req.get_user_rule();
        if user_rule != EmailCodeCheckUserRule::NoCheck {
            let query_res: Result<Option<String>, mysql::Error> = conn.exec_first(
                r"select user_name
                from user_table ut
                where ut.email=:email",
                params! {
                    "email"=>&req.email
                },
            );
            // across the rule, return result
            match query_res {
                Ok(res) => match res {
                    Some(_) => {
                        if user_rule == EmailCodeCheckUserRule::NoExist {
                            return Err(DaoError::exist());
                        }
                    }
                    None => {
                        if user_rule == EmailCodeCheckUserRule::IsExist {
                            return Err(DaoError::not_found());
                        }
                    }
                },
                Err(err) => return Err(DaoError::from(err)),
            }
        }
        // generate code
        Self::gen_code(&req.email)
    }
}

impl EmailCode {
    pub fn map(row: Option<(String, i32, NaiveDateTime)>) -> Option<Self> {
        row.map(|(email, code, get_date)| Self {
            email,
            code,
            get_date,
        })
    }

    pub fn query_code(email: &str, code: Option<i32>) -> Result<Self, DaoError> {
        let mut conn = get_conn();

        let code_con = match code {
            Some(c) => format!("ec.code={}", c),
            None => String::from("true"),
        };

        let sql = format!(
            r"select *
            from email_code ec
            where ec.email=:email and {}",
            code_con
        );

        let res = conn
            .exec_first(sql, params! {"email"=>email, })
            .map(|row| Self::map(row));
        DaoError::from_mysql_res_opt_some(res)
    }

    pub fn get_code(email: &str) -> Result<Self, DaoError> {
        let mut conn = get_conn();
        let res = conn
            .exec_first(
                r"select *
            from email_code ec
            where ec.email=:email",
                params! {
                    "email"=>email
                },
            )
            .map(|row| Self::map(row));

        DaoError::from_mysql_res_opt_some(res)
    }

    fn gen_code(email: &str) -> Result<Self, DaoError> {
        let mut conn = get_conn();

        let query_res = Self::get_code(email);

        // generate code
        let mut rng = rand::thread_rng();
        let code = rng.gen_range(1000..9999);

        let now = Local::now().naive_local();
        if let Ok(mut item) = query_res {
            // if has code, replace it
            let _ = conn.exec_drop(
                r"UPDATE email_code
                    SET code=:code, get_date=:now
                    WHERE email=:email",
                params! {
                    "code"=>code,
                    "now"=>now,
                    "email"=>email
                },
            );
            item.code = code;
            item.get_date = now;
            Ok(item)
        } else {
            // if don't, generate new one
            let _ = conn.exec_drop(
                r"INSERT INTO email_code(email, code, get_date)
                    VALUES
                    (:email, :code, :now)",
                params! {
                    "email"=>email,
                    "code"=>code,
                    "now"=>now,
                },
            );
            // through select get new code
            Self::get_code(email)
        }
    }

    pub fn delete_code(&mut self) -> Result<(), DaoError> {
        let mut conn = get_conn();
        let delete_res = conn.exec_drop(
            "delete 
            from email_code
            where email=:email",
            params! {
                "email"=>&self.email
            },
        );
        if let Ok(_) = delete_res {
            self.email = "".to_string();
            self.code = 0;
        }
        DaoError::from_mysql_res(delete_res)
    }
}
