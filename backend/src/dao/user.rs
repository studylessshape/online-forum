use chrono::{Duration, Local, NaiveDateTime};
use mysql::{prelude::*, serde_json::json, *};
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
use uuid::Uuid;

use super::{
    connect::get_conn,
    dao_error::{DaoError, DaoErrorCode, DaoResult},
    email::EmailCode,
    get_value, ConditionFor,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIdentity {
    pub user_name: String,
    pub token: Uuid,
}

impl ConditionFor<UserSummary> for UserIdentity {
    fn get_condition(&self) -> (String, Params) {
        (
            String::from("user_name=:user_name"),
            params! {
                "user_name"=>&self.user_name
            },
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserNameAndPwd {
    pub name: String,
    pub password: String,
}

impl ConditionFor<UserSummary> for UserNameAndPwd {
    fn get_condition(&self) -> (String, Params) {
        (
            "(email=:name or user_name=:name) and password=:password".to_string(),
            params! {
                "name"=>&self.name,
                "password"=>&self.password,
            },
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSignUpInfo {
    pub user_name: String,
    pub email: String,
    pub code: i32,
    pub password: String,
}

impl ConditionFor<UserDb> for UserSignUpInfo {
    fn get_condition(&self) -> (String, Params) {
        (
            "user_name=:name and email=:email".to_string(),
            params! {
                "name"=>&self.user_name,
                "email"=>&self.email,
            },
        )
    }
}

impl UserSignUpInfo {
    pub fn sign_up(&self) -> StdResult<UserDb, impl Serialize> {
        let mut conn = get_conn();
        // search the same email and user name
        let query_user_exist_res: Result<Vec<(String, String)>> = conn.exec(
            r"select user_name, email
            from user_table ut
            where ut.user_name=:name or ut.email=:email",
            params! {
                "name"=> &self.user_name,
                "email"=> &self.email,
            },
        );
        // check error
        if let Err(err) = query_user_exist_res {
            log::error!("User Sign Up Dao Error | {:?}", err);
            return Err(json!({"error":["unknow"]}));
        }

        // check repeat section
        let mut re_name = false;
        let mut re_email = false;

        let user_table_res = query_user_exist_res.unwrap();
        if user_table_res.len() >= 1 {
            for user in user_table_res {
                if user.0 == self.user_name {
                    re_name = true;
                }
                if user.1 == self.email {
                    re_email = true;
                }
            }
        }
        // if has repeated section, return the error
        if re_name || re_email {
            log::warn!(
                "Sign up failed, user is already exist! | [\"user_name\": {}, \"email\": {}]",
                self.user_name,
                self.email
            );
            if re_name && re_email {
                return Err(json!({"error":["user_name", "email"]}));
            } else if re_name {
                return Err(json!({"error":["user_name"]}));
            } else if re_email {
                return Err(json!({"error":["email"]}));
            }
        }
        // check email code
        let query_code = EmailCode::query_code(&self.email, Some(self.code));

        match query_code {
            Err(err) => match err.0 {
                DaoErrorCode::NotFound => Err(json!({"error":["code"]})),
                _ => {
                    log::error!("Email Code Search Dao Error | {:?}", err);
                    Err(json!({"error":["unknow"]}))
                }
            },
            Ok(mut code_db) => {
                // insert new user
                let insert_res = conn.exec_drop(
                    r"insert into 
                    user_table(email, user_name, password, regist_time, auth_id)
                    values
                    (:email, :name, :password, :regist_time, :auth_id)",
                    params! {
                        "email"=>&self.email,
                        "name"=>&self.user_name,
                        "password" => &self.password,
                        "regist_time"=>Local::now().naive_local(),
                        "auth_id"=>2,
                    },
                );
                // return the data of user from database
                match insert_res {
                    Ok(_) => {
                        let dyn_self: Box<dyn ConditionFor<UserDb>> = Box::new(self.clone());
                        match UserDb::try_from(dyn_self) {
                            Ok(user) => {
                                if let Err(err) = code_db.delete_code() {
                                    log::error!("Delete email code Error | {:?}", err);
                                }
                                Ok(user)
                            }
                            Err(err) => {
                                log::error!("Get User Dao Error | {:?}", err);
                                Err(json!({"error":["unknow"]}))
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Insert new User Dao Error | {:?}", err);
                        Err(json!({"error":["unknow"]}))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSummary {
    pub user_name: String,
    pub user_nickname: Option<String>,
    pub avatar: Option<String>,
    pub lastest_sign: Option<NaiveDateTime>,
    pub token: Option<Uuid>,
}

impl From<UserDb> for UserSummary {
    fn from(value: UserDb) -> Self {
        Self {
            user_name: value.user_name,
            user_nickname: value.user_nickname,
            avatar: value.avatar,
            lastest_sign: value.lastest_sign,
            token: value.token,
        }
    }
}

impl TryFrom<Box<dyn ConditionFor<UserSummary>>> for UserSummary {
    type Error = DaoError;

    /// Though the condition from param to search in database. Then get the object back to UserSummary
    fn try_from(
        value: Box<dyn ConditionFor<UserSummary>>,
    ) -> StdResult<Self, Self::Error> {
        // get connect with mysql
        let mut conn = get_conn();

        let (condition, para) = value.get_condition();

        let sql = format!(
            r"SELECT user_name, user_nickname, avatar, lastest_sign,token
            FROM user_table
            WHERE {}",
            condition
        );

        // query user summary
        let query_res = conn.exec_first(sql, para).map(|row| {
            row.map(
                |(user_name, user_nickname, avatar, lastest_sign, token)| UserSummary {
                    user_name,
                    user_nickname,
                    avatar,
                    lastest_sign,
                    token,
                },
            )
        });
        match query_res {
            Ok(Some(user)) => Ok(user),
            Err(err) => Err(DaoError(DaoErrorCode::MysqlError(err))),
            _ => Err(DaoError(DaoErrorCode::NotFound)),
        }
    }
}

impl UserSummary {
    /// update user token. This method will update the data in database
    pub fn update_token(&mut self) -> bool {
        // if no token, gen token for user
        if let None = self.token {
            let token = Uuid::new_v4();

            self.token = Some(token);
            self.lastest_sign = Some(Local::now().naive_local());
        } else {
            let now = Local::now().naive_local();
            let duration = now - self.lastest_sign.unwrap();

            // update sign time
            self.lastest_sign = Some(now);
            // if the time span over 30 days, gen new token
            if duration.num_days() >= 30 {
                let token = Uuid::new_v4();

                self.token = Some(token);
            }
        }
        // update database
        let mut conn = get_conn();
        let update_res = conn.exec_drop(
            r"UPDATE user_table
            SET lastest_sign=:lastest_sign, token=:token
            WHERE user_name=:user_name",
            params! {
                "lastest_sign"=>&self.lastest_sign,
                "token"=>&self.token,
                "user_name"=>&self.user_name
            },
        );
        if let Err(err) = update_res {
            log::error!("user.rs | UserSummary::update_token | {:?}", err);
            false
        } else {
            true
        }
    }

    pub fn sign_span(&self) -> Duration {
        if let Some(time) = self.lastest_sign {
            Local::now().naive_local() - time
        } else {
            Duration::zero()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSimplify {
    pub user_name: String,
    pub user_nickname: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserNicknameRequest {
    pub user_nickname: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserNameRequest {
    pub user_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfilesUpdateRequest {
    pub user_nickname: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPasswordUpdateRequest {
    pub email: String,
    pub code: i32,
    pub password: String,
}

impl UserPasswordUpdateRequest {
    pub fn update_password(&self) -> DaoResult<()> {
        // verify email code
        let mut code = match EmailCode::query_code(&self.email, Some(self.code)) {
            Ok(c) => c,
            Err(dao_err) => match dao_err.0 {
                DaoErrorCode::NotFound => return Err(DaoError::message("验证码不正确")),
                _ => return Err(dao_err),
            },
        };
        // delete code after verify
        if let Err(err) = code.delete_code() {
            return Err(err);
        }
        // get user database
        let mut user = UserDb::get(("email=:email", params! {"email"=>&self.email}))?;
        // update password
        match user.update_password(&self.password) {
            Ok(_) => Ok(()),
            Err(err) => Err(DaoError::from(err)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDb {
    pub user_id: u32,
    pub email: String,
    pub user_name: String,
    pub password: String,
    pub regist_time: NaiveDateTime,
    pub auth_id: u32,
    pub user_nickname: Option<String>,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub lastest_sign: Option<NaiveDateTime>,
    pub token: Option<Uuid>,
}

impl TryFrom<Box<dyn ConditionFor<UserDb>>> for UserDb {
    type Error = DaoError;

    fn try_from(value: Box<dyn ConditionFor<UserDb>>) -> StdResult<Self, Self::Error> {
        let mut conn = get_conn();

        let (condition, param) = value.get_condition();

        let sql = format!(
            r"select *
            from user_table
            where {}",
            condition
        );

        let query_res = conn.exec_first(sql, param).map(Self::map);

        match query_res {
            Ok(Some(user)) => Ok(user),
            Err(err) => Err(DaoError(DaoErrorCode::MysqlError(err))),
            _ => Err(DaoError(DaoErrorCode::NotFound)),
        }
    }
}

impl UserDb {
    pub fn map(
        row: Option<(
            u32,
            String,
            String,
            String,
            NaiveDateTime,
            u32,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<NaiveDateTime>,
            Option<Uuid>,
        )>,
    ) -> Option<Self> {
        row.map(
            |(
                user_id,
                email,
                user_name,
                password,
                regist_time,
                auth_id,
                user_nickname,
                avatar,
                description,
                lastest_sign,
                token,
            )| Self {
                user_id,
                email,
                user_name,
                password,
                regist_time,
                auth_id,
                user_nickname,
                avatar,
                description,
                lastest_sign,
                token,
            },
        )
    }
    /// `condition.0` is the string after `where` in sql sentence
    pub fn get(condition: (&str, Params)) -> DaoResult<Self> {
        let (condition, param) = condition;
        let mut conn = get_conn();

        let sql = format!(
            r"select *
            from user_table
            where {}",
            condition
        );

        DaoError::from_mysql_res_opt_some(conn.exec_first(sql, param).map(Self::map))
    }

    pub fn update_nickname(&mut self, new_name: &str) -> Result<&mut Self> {
        let mut conn = get_conn();
        let query_res = conn.exec_drop(
            r"update user_table
            set user_nickname=:name
            where user_id=:id",
            params! {
                "name"=>new_name,
                "id"=>self.user_id,
            },
        );

        match query_res {
            Ok(_) => {
                self.user_nickname = Some(new_name.to_string());
                Ok(self)
            }
            Err(err) => Err(err),
        }
    }

    pub fn update_description(&mut self, new_description: &str) -> Result<&mut Self> {
        let mut conn = get_conn();
        let query_res = conn.exec_drop(
            r"update user_table
            set description=:description
            where user_id=:id",
            params! {
                "description"=>new_description,
                "id"=>self.user_id,
            },
        );

        match query_res {
            Ok(_) => {
                self.description = Some(new_description.to_string());
                Ok(self)
            }
            Err(err) => Err(err),
        }
    }

    pub fn update_avatar(&mut self, new_avatar: &str) -> Result<&mut Self> {
        let mut conn = get_conn();
        let query_res = conn.exec_drop(
            r"update user_table
            set avatar=:avatar
            where user_id=:id",
            params! {
                "avatar"=>new_avatar,
                "id"=>self.user_id,
            },
        );

        match query_res {
            Ok(_) => {
                self.avatar = Some(new_avatar.to_string());
                Ok(self)
            }
            Err(err) => Err(err),
        }
    }

    pub fn update_nickname_description(
        &mut self,
        nickname: &str,
        description: &str,
    ) -> Result<&mut Self> {
        let mut conn = get_conn();
        let query_res = conn.exec_drop(
            r"update user_table
            set user_nickname=:nickname, description=:description
            where user_id=:id",
            params! {
                "nickname"=>nickname,
                "description"=>description,
                "id"=>self.user_id,
            },
        );

        match query_res {
            Ok(_) => {
                self.user_nickname = Some(nickname.to_string());
                self.description = Some(description.to_string());
                Ok(self)
            }
            Err(err) => Err(err),
        }
    }

    pub fn update_password(&mut self, new_password: &str) -> Result<&mut Self> {
        let mut conn = get_conn();
        let query_res = conn.exec_drop(
            r"update user_table
            set password=:password
            where user_id=:id",
            params! {
                "password"=>new_password,
                "id"=>self.user_id,
            },
        );

        match query_res {
            Ok(_) => {
                self.password = new_password.to_string();
                Ok(self)
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersInfoForManage {
    pub user_id: u32,
    pub user_name: String,
    pub user_nickname: Option<String>,
    pub avatar: Option<String>,
    pub regist_time: NaiveDateTime,
    pub lastest_sign: Option<NaiveDateTime>,
    pub lastest_post_time: Option<NaiveDateTime>,
    pub post_count: u32,
    pub lastest_comment_time: Option<NaiveDateTime>,
    pub comment_count: u32,
}

impl FromRow for UsersInfoForManage {
    fn from_row_opt(row: Row) -> StdResult<Self, FromRowError>
    where
        Self: Sized,
    {
        if row.len() < 10 {
            return Err(FromRowError(row));
        }

        Ok(Self {
            user_id: get_value(&row[0], &row)?,
            user_name: get_value(&row[1], &row)?,
            user_nickname: get_value(&row[2], &row)?,
            avatar: get_value(&row[3], &row)?,
            regist_time: get_value(&row[4], &row)?,
            lastest_sign: get_value(&row[5], &row)?,
            lastest_post_time: get_value(&row[6], &row)?,
            post_count: get_value(&row[7], &row)?,
            lastest_comment_time: get_value(&row[8], &row)?,
            comment_count: get_value(&row[9], &row)?,
        })
    }
}

impl UsersInfoForManage {
    pub fn get_all() -> DaoResult<Vec<Self>> {
        let mut conn = get_conn();

        let sql = r"select
                ut.user_id,
                ut.user_name,
                ut.user_nickname,
                ut.avatar,
                ut.regist_time,
                ut.lastest_sign,
                max(pt.post_time) lastest_post_time,
                count(pt.post_id) post_count,
                cut.lastest_comment_time,
                ifnull(cut.comment_count, 0) 
            from
                user_table ut
            left join post_table pt on
                pt.user_id = ut.user_id
            left join (
                select
                    ct.user_id,
                    max(ct.comment_time) lastest_comment_time,
                    count(ct.comment_id) comment_count
                from
                    comment_table ct
                group by ct.user_id) cut on
                cut.user_id=ut.user_id 
            where ut.user_name != 'root'
            group by
                ut.user_id
            order by
                ut.user_id asc";

        DaoError::from_mysql_res(conn.query(sql))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIds {
    pub user_id: Vec<u32>,
}

impl UserIds {
    pub fn del_users(&self) -> DaoResult<()>{
        let mut conn = get_conn();

        let user_conditions = match self.user_id.len() == 0 {
            true => return Err(DaoError::message("No user ids!")),
            false => {
                let mut cdt = String::new();
                for (idx, user_id) in self.user_id.iter().enumerate() {
                    if idx < self.user_id.len() - 1 {
                        cdt.push_str(format!("user_id={} or ", user_id).as_str());
                    } else {
                        cdt.push_str(format!("user_id={}", user_id).as_str());
                    }
                }
                cdt
            },
        };

        let sql = format!(r"DELETE FROM user_table WHERE ({}) and user_name != 'root'", user_conditions);

        DaoError::from_mysql_res(conn.query_drop(sql))
    }
}