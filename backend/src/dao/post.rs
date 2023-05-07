use chrono::{Local, NaiveDateTime};
use mysql::{
    params,
    prelude::{FromRow, Queryable},
    Value,
};
use serde::{Deserialize, Serialize};

use super::{connect::get_conn, dao_error::DaoError, get_value, user::UserSimplify};

/// Include author info
#[derive(Debug, Deserialize, Serialize)]
pub struct PostSummary {
    pub post_id: i32,
    pub post_title: String,
    pub post_summary: String,
    pub post_time: NaiveDateTime,
    pub post_update_time: Option<NaiveDateTime>,
    pub author: UserSimplify,
    pub comment_count: i32,
}

impl FromRow for PostSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        if row.len() != 9 {
            return Err(mysql::FromRowError(row));
        }

        Ok(PostSummary {
            post_id: get_value(&row[0], &row)?,
            post_title: get_value(&row[1], &row)?,
            post_summary: get_value(&row[2], &row)?,
            post_time: get_value(&row[3], &row)?,
            post_update_time: get_value(&row[4], &row)?,
            author: UserSimplify {
                user_name: get_value(&row[5], &row)?,
                user_nickname: get_value(&row[6], &row)?,
                avatar: get_value(&row[7], &row)?,
            },
            comment_count: get_value(&row[8], &row)?,
        })
    }
}

impl PostSummary {
    pub fn get_posts(
        section_name: &str,
        offset: i32,
        count: i32,
        before_time: Option<NaiveDateTime>,
    ) -> Result<Vec<PostSummary>, DaoError> {
        let mut conn = get_conn();

        let sql = format!(
            r"select pt.post_id, pt.post_title, left(pt.post_content, 60) post_summary, pt.post_time, pt.post_update_time, 
            ut.user_name, ut.user_nickname, ut.avatar,
            ifnull(cc.comment_count, 0) comment_count
            from post_table pt
            join user_table ut on pt.user_id = ut.user_id
            left join (select ct.post_id, count(ct.post_id) comment_count
                from comment_table ct
                group by post_id) cc
            on cc.post_id = pt.post_id
            where :before_time and pt.section_id = (
                select ps.section_id 
                from post_section ps 
                where ps.section_name = :section_name
            )
            order by pt.post_id desc
            limit :offset, :count"
        );

        let params = params! {
            "before_time" => match before_time {
                Some(time) => Value::from(time),
                None => Value::from(true),
            },
            "section_name" => section_name,
            "offset" => offset,
            "count" => count,
        };

        DaoError::from_mysql_res(conn.exec(sql, params))
    }
}

/// Don't include author info
#[derive(Debug, Deserialize, Serialize)]
pub struct PostSummarySimplify {
    pub post_id: i32,
    pub post_title: String,
    pub post_summary: String,
    pub section_name: String,
    pub section_name_zh: String,
    pub post_time: NaiveDateTime,
    pub post_update_time: Option<NaiveDateTime>,
    pub comment_count: i32,
}

impl FromRow for PostSummarySimplify {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        if row.len() != 8 {
            return Err(mysql::FromRowError(row));
        }

        Ok(Self {
            post_id: get_value(&row[0], &row)?,
            post_title: get_value(&row[1], &row)?,
            post_summary: get_value(&row[2], &row)?,
            section_name: get_value(&row[3], &row)?,
            section_name_zh: get_value(&row[4], &row)?,
            post_time: get_value(&row[5], &row)?,
            post_update_time: get_value(&row[6], &row)?,
            comment_count: get_value(&row[7], &row)?,
        })
    }
}

impl PostSummarySimplify {
    /// Get posts that user publish, limit the length less than 100.
    pub fn get_user_posts(user_name: &str) -> Result<Vec<Self>, DaoError> {
        let mut conn = get_conn();

        let sql = r"select
                pt.post_id,
                pt.post_title,
                left(pt.post_content,
                100) post_summary,
                ps.section_name,
                ps.section_name_zh,
                pt.post_time,
                pt.post_update_time,
                ifnull(cc.comment_count, 0) comment_count
            from post_table pt
            left join (select ct.post_id, count(ct.post_id) comment_count
                from comment_table ct
                group by post_id) cc
            on cc.post_id = pt.post_id
            left join post_section ps on ps.section_id = pt.section_id
            where pt.user_id  = (select ut.user_id 
                from user_table ut 
                where ut.user_name=:name)
            order by pt.post_id desc
            limit 0,100";

        let param = params! { "name"=>user_name };

        DaoError::from_mysql_res(conn.exec(sql, param))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostId {
    post_id: i32,
}

impl PostId {
    pub fn get_post(&self) -> Result<PostView, DaoError> {
        PostView::get_post(self.post_id)
    }

    pub fn del_post(&self, token: uuid::Uuid) -> Result<(), DaoError> {
        let mut conn = get_conn();

        conn.exec_drop(
            r"delete from post_table
            where post_id=:post_id and user_id=(select ut.user_id from user_table ut where ut.token=:token)", 
            params! {
                "post_id"=>self.post_id,
                "token"=>token
            }
        ).or_else(|err| Err(DaoError::from(err)))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostView {
    pub post_title: String,
    pub post_time: NaiveDateTime,
    pub post_update_time: Option<NaiveDateTime>,
    pub post_content: String,
    pub author: UserSimplify,
}

impl FromRow for PostView {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        if row.len() != 7 {
            return Err(mysql::FromRowError(row));
        }

        Ok(Self {
            post_title: get_value(&row[0], &row)?,
            post_time: get_value(&row[1], &row)?,
            post_update_time: get_value(&row[2], &row)?,
            post_content: get_value(&row[3], &row)?,
            author: UserSimplify {
                user_name: get_value(&row[4], &row)?,
                user_nickname: get_value(&row[5], &row)?,
                avatar: get_value(&row[6], &row)?,
            },
        })
    }
}

impl PostView {
    pub fn get_post(id: i32) -> Result<Self, DaoError> {
        let mut conn = get_conn();

        let res = conn.exec_first(
            r"select pt.post_title, pt.post_time, pt.post_update_time, pt.post_content,ut.user_name,ut.user_nickname,ut.avatar
            from post_table pt
            join user_table ut on pt.user_id=ut.user_id
            where pt.post_id=:post_id", 
            params! {
                "post_id"=>id,
            });
        DaoError::from_mysql_res_opt_some(res)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostEditInfo {
    pub post_id: Option<i32>,
    pub post_title: String,
    pub post_content: String,
    pub section_name: String,
    pub token: uuid::Uuid,
}

impl PostEditInfo {
    /// Update post whatever add or modify. It will return post's id or error.
    pub fn update_post(&self) -> Result<i32, DaoError> {
        match self.post_id {
            Some(post_id) => self.modify_post(post_id),
            None => self.add_post(),
        }
    }

    fn add_post(&self) -> Result<i32, DaoError> {
        let mut conn = get_conn();

        let add_sql = r"insert post_table(user_id, section_id, post_title, post_content, post_time)
        values(
            (select ut.user_id
            from user_table ut
            where ut.token=:token),
            (select ps.section_id
            from post_section ps
            where ps.section_name=:section_name),
            :post_title, :post_content, :now
        )";

        let add_param = params! {
            "token"=>self.token,
            "section_name"=>&self.section_name,
            "post_title"=>&self.post_title,
            "post_content"=>&self.post_content,
            "now"=>Local::now().naive_local(),
        };

        let add_res = conn.exec_drop(add_sql, add_param);

        match add_res {
            Ok(_) => DaoError::from_mysql_res_opt_some(conn.query_first("select last_insert_id()")),
            Err(err) => Err(DaoError::from(err)),
        }
    }

    fn modify_post(&self, post_id: i32) -> Result<i32, DaoError> {
        let mut conn = get_conn();

        let sql = r"update post_table 
            set post_title=:post_title, post_content=:post_content, post_update_time=:update_time
            where post_id=:post_id and user_id=(select ut.user_id from user_table ut where ut.token=:token)";

        let param = params! {
            "post_title"=>&self.post_title,
            "post_content"=>&self.post_content,
            "update_time"=>Local::now().naive_local(),
            "post_id"=>post_id,
            "token"=>self.token,
        };

        let res = conn.exec_drop(sql, param);
        match res {
            Ok(_) => Ok(post_id),
            Err(err) => Err(DaoError::from(err)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostDb {
    pub post_id: i32,
    pub user_id: u32,
    pub section_id: i32,
    pub post_title: String,
    pub post_time: NaiveDateTime,
    pub post_update_time: Option<NaiveDateTime>,
    pub post_content: String,
}

impl FromRow for PostDb {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        if row.len() != 7 {
            return Err(mysql::FromRowError(row));
        }

        Ok(Self {
            post_id: get_value(&row[0], &row)?,
            user_id: get_value(&row[1], &row)?,
            section_id: get_value(&row[2], &row)?,
            post_title: get_value(&row[3], &row)?,
            post_time: get_value(&row[4], &row)?,
            post_update_time: get_value(&row[5], &row)?,
            post_content: get_value(&row[6], &row)?,
        })
    }
}

impl PostDb {
    pub fn lastest_post(section_name: &str) -> Result<Option<Self>, DaoError> {
        let mut conn = get_conn();

        let sql = r"select *
        from post_table pt
        where pt.section_id = (select ps.section_id from post_section ps where ps.section_name=:section_name)
        order by pt.post_time desc 
        limit 0,1;";

        let param = params! {
            "section_name"=>section_name
        };

        DaoError::from_mysql_res(conn.exec_first(sql, param))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSimplify {
    pub post_title: String,
    pub post_content: String,
    pub post_time: NaiveDateTime,
    pub post_update_time: Option<NaiveDateTime>,
}

impl From<PostDb> for PostSimplify {
    fn from(value: PostDb) -> Self {
        Self {
            post_title: value.post_title,
            post_content: value.post_content,
            post_time: value.post_time,
            post_update_time: value.post_update_time,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DelPostsRequest {
    pub all_post_id: Vec<i32>,
}

impl DelPostsRequest {
    pub fn del_posts(&self, token: &uuid::Uuid) -> Result<(), DaoError> {
        let mut conn = get_conn();

        let mut id_clause = String::new();

        for (idx, post_id) in self.all_post_id.iter().enumerate() {
            if idx != self.all_post_id.len() - 1 {
                id_clause.push_str(&format!("post_id={} or ", post_id));
            } else {
                id_clause.push_str(&format!("post_id={}", post_id));
            }
        }

        let sql = format!(
            r"DELETE FROM post_table pt
            WHERE pt.user_id=(select user_id from user_table where token=:token) and ({})",
            id_clause
        );

        let param = params! {"token"=>token};

        DaoError::from_mysql_res(conn.exec_drop(sql, param))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSearchRequest {
    pub keywords: String,
    pub title: bool,
    pub content: bool,
    pub section_name: Vec<String>,
}

impl PostSearchRequest {
    pub fn search(&self) -> Result<Vec<PostSearchResult>, DaoError> {
        let mut conn = get_conn();
        // if `section_name.len() > 0`, will create conditon for sectoin_name
        let section_condition = if self.section_name.len() == 0 {
            String::from("true")
        } else {
            let mut t_str = String::new();
            for (idx, section_name) in self.section_name.iter().enumerate() {
                if idx == self.section_name.len() - 1 {
                    t_str.push_str(&format!("ps.section_name='{}'", section_name));
                } else {
                    t_str.push_str(&format!("ps.section_name='{}' or ", section_name));
                }
            }
            t_str
        };
        
        let sql = format!(
            r"select
                pt.post_id,
                pt.post_title,
                left(pt.post_content,
                100) post_summary,
                pt.post_time,
                pt.post_update_time,
                ut.user_name,
                ut.user_nickname,
                ut.avatar,
                ps.section_name,
                ps.section_name_zh,
                count(ct.comment_id) comment_count
            from
                post_table pt
            left join user_table ut on
                pt.user_id = ut.user_id
            left join post_section ps on
                ps.section_id = pt.section_id
            left join comment_table ct on
                ct.post_id = pt.post_id
            where
                ((post_title like :keywords
                    and :is_title)
                or (post_content like :keywords
                    and :is_content))
                and ({})
            group by
                pt.post_id
            order by
                char_length(post_title) asc,
                char_length(post_content) asc,
                post_time desc",
            section_condition
        );

        let param = params! { 
            "keywords"=> format!("%{}%", self.keywords),
            "is_title"=> self.title,
            "is_content"=> self.content,
        };

        DaoError::from_mysql_res(conn.exec(sql, param))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSearchResult {
    pub post_id: i32,
    pub post_title: String,
    pub post_summary: String,
    pub post_time: NaiveDateTime,
    pub post_update_time: Option<NaiveDateTime>,
    pub author: UserSimplify,
    pub section_name: String,
    pub section_name_zh: String,
}

impl FromRow for PostSearchResult {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        Ok(Self {
            post_id: get_value(&row[0], &row)?,
            post_title: get_value(&row[1], &row)?,
            post_summary: get_value(&row[2], &row)?,
            post_time: get_value(&row[3], &row)?,
            post_update_time: get_value(&row[4], &row)?,
            author: UserSimplify {
                user_name: get_value(&row[5], &row)?,
                user_nickname: get_value(&row[6], &row)?,
                avatar: get_value(&row[7], &row)?,
            },
            section_name: get_value(&row[8], &row)?,
            section_name_zh: get_value(&row[9], &row)?,
        })
    }
}
