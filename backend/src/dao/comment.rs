use chrono::{Local, NaiveDateTime};
use mysql::{
    params,
    prelude::{FromRow, Queryable}
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{connect::get_conn, dao_error::DaoError, get_value, user::UserSimplify};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentView {
    pub comment_id: i32,
    pub user: UserSimplify,
    pub comment_time: NaiveDateTime,
    pub comment_content: String,
}

impl FromRow for CommentView {
    /// This search on database, need row like this, the column count is 11 at least:
    /// ```sql
    /// select cut.comment_id, cut.user_name, cut.user_nickname, cut.avatar, cut.comment_time, cut.comment_content
    /// ```
    ///
    /// `cut.comment_id, cut.user_name, cut...` is this comment information.
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        if row.len() < 6 {
            return Err(mysql::FromRowError(row));
        }

        Ok(Self {
            comment_id: get_value(&row[0], &row)?,
            user: UserSimplify {
                user_name: get_value(&row[1], &row)?,
                user_nickname: get_value(&row[2], &row)?,
                avatar: get_value(&row[3], &row)?,
            },
            comment_time: get_value(&row[4], &row)?,
            comment_content: get_value(&row[5], &row)?,
        })
    }
}

impl CommentView {
    pub fn get_comments(post_id: i32, offset: i32, count: i32) -> Result<Vec<Self>, DaoError> {
        let mut conn = get_conn();

        let sql = r"select cut.comment_id, cut.user_name, cut.user_nickname, cut.avatar, cut.comment_time, cut.comment_content
        from (
          select ct.comment_id, ct.post_id, ct.user_id, ct.comment_time, ct.comment_content, ut.user_name, ut.user_nickname, ut.avatar 
          from comment_table ct 
          left join user_table ut on ct.user_id=ut.user_id
        ) cut
        where cut.post_id=:post_id
        order by cut.comment_time
        limit :offset, :count";

        let param = params! {
            "post_id"=>post_id,
            "offset"=>offset,
            "count"=>count,
        };

        DaoError::from_mysql_res(conn.exec(sql, param))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentPageRequest {
    pub post_id: i32,
    pub page: i32,
    pub per_count: i32,
}

impl CommentPageRequest {
    pub fn get_comments(&self) -> Result<Vec<CommentView>, DaoError> {
        CommentView::get_comments(
            self.post_id,
            (self.page - 1) * self.per_count,
            self.per_count,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentPutRequest {
    pub post_id: i32,
    pub token: Uuid,
    pub comment_content: String,
}

impl CommentPutRequest {
    pub fn put_comment(&self) -> Result<(), DaoError> {
        let mut conn = get_conn();

        let sql = r"insert comment_table(post_id,user_id,comment_time,comment_content)
            values (:post_id, 
                (select ut.user_id from user_table ut where ut.token=:token), 
                :now_time,
                :comment_content
            )";

        let param = params! {
            "post_id"=>self.post_id,
            "token"=>self.token,
            "now_time"=>Local::now().naive_local(),
            "comment_content"=>&self.comment_content
        };

        DaoError::from_mysql_res(conn.exec_drop(sql, param))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentDeleteRequest {
    pub comment_id: i32,
    pub token: Uuid,
}

impl CommentDeleteRequest {
    pub fn delete_comment(&self) -> Result<(), DaoError> {
        let mut conn = get_conn();

        let sql = r"delete from comment_table where comment_id=:comment_id";

        let param = params! {"comment_id"=>self.comment_id};

        DaoError::from_mysql_res(conn.exec_drop(sql, param))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentDb {
    pub comment_id: i32,
    pub post_id: i32,
    pub user_id: u32,
    pub comment_time: NaiveDateTime,
    pub comment_content: String,
}
