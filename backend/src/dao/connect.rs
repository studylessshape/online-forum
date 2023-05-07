use mysql::*;

pub fn get_conn() -> Conn {
    let opts = OptsBuilder::new()
        .db_name(Some("online_forum"))
        .user(Some("admin"))
        .pass(Some("a664554724"));

    Conn::new(opts).unwrap()
}