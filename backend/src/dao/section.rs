use chrono::NaiveDateTime;
use mysql::{params, prelude::Queryable};
use serde::{Deserialize, Serialize};

use super::{
    connect::get_conn,
    dao_error::DaoError,
    post::{PostSummary, PostDb},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct SectionNames {
    pub names: Vec<String>,
}

impl SectionNames {
    /// If not found section or ouccr error, it's will return error
    pub fn get_status(&self) -> Result<Vec<SectionStatus>, DaoError> {
        let mut res = Vec::new();
        for name in &self.names {
            match SectionStatus::get_status(name) {
                Err(err) => return Err(err),
                Ok(status) => res.push(status),
            }
        }
        Ok(res)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SectionStatus {
    pub section_name: String,
    pub last_update_time: Option<NaiveDateTime>,
    pub today_post_count: i32,
    pub total_post_count: i32,
}

impl SectionStatus {
    /// through `post_section.section_name` select row
    pub fn get_status(section_name: &str) -> Result<Self, DaoError> {
        // get section id
        match Self::get_section_id(section_name) {
            Err(err) => Err(err),
            // then get today count of section's posts
            Ok(section_id) => match Self::get_section_today_count(section_id) {
                Err(err) => Err(err),
                Ok(today_count) => {
                    let mut conn = get_conn();
                    // get other status and to one table
                    let query_res = conn.exec_first(
                        r"select max(pt.post_time) last_update_time, :today_count today_count, count(*) total_count
                        from post_table pt
                        where section_id=:section_id", 
                        params!{
                            "today_count"=>today_count,
                            "section_id"=>section_id
                        }
                    ).map(|row| row.map(|(last_update_time, today_post_count, total_post_count)| Self {
                        section_name: section_name.to_string(),
                        last_update_time,
                        today_post_count,
                        total_post_count
                    }));
                    DaoError::from_mysql_res_opt_some(query_res)
                }
            },
        }
    }

    fn get_section_id(section_name: &str) -> Result<i32, DaoError> {
        let mut conn = get_conn();
        let res = conn.exec_first(
            r"select section_id
            from post_section
            where section_name=:name",
            params! {
                "name"=>section_name
            },
        );
        DaoError::from_mysql_res_opt_some(res)
    }

    fn get_section_today_count(section_id: i32) -> Result<i32, DaoError> {
        let mut conn = get_conn();

        let res = conn.exec_first(
            r"select count(post_id)
            from post_table
            where convert(post_time, date)=current_date() and section_id=:section_id",
            params! {
                "section_id"=>section_id
            },
        );
        DaoError::from_mysql_res_opt_some(res)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SectionName {
    pub section_name: String,
}

impl SectionName {
    pub fn get_total_post_count(&self) -> Result<SectionStatus, DaoError> {
        let mut conn = get_conn();

        let query_res = conn
            .exec_first(
                r"select count(*) total_post_count
            from post_table
            where section_id=(
                select section_id
                from post_section
                where section_name=:section_name
            )",
                params! {
                    "section_name"=>&self.section_name
                },
            )
            .map(|row| {
                row.map(|total_post_count| SectionStatus {
                    section_name: self.section_name.clone(),
                    last_update_time: None,
                    today_post_count: -1,
                    total_post_count,
                })
            });

        DaoError::from_mysql_res_opt_some(query_res)
    }

    pub fn get_lastest_post(&self) -> Result<Option<PostDb>, DaoError> {
        PostDb::lastest_post(&self.section_name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SectionPageRequest {
    pub section_name: String,
    pub current_page: i32,
    pub per_page_count: i32,
    pub first_post_datetime: Option<NaiveDateTime>,
}

impl SectionPageRequest {
    pub fn get_posts(&self) -> Result<Vec<PostSummary>, DaoError> {
        PostSummary::get_posts(
            &self.section_name,
            (self.current_page - 1) * self.per_page_count,
            self.per_page_count,
            self.first_post_datetime,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SectionDb {
    pub section_id: i32,
    pub section_name: String,
    pub section_name_zh: String,
}
