use chrono::NaiveDate;
use rusqlite::{Connection, Result};
use std::path::Path;

#[derive(Debug)]
pub struct TASK {
    pub id: i64,
    pub title: String,
    pub date: NaiveDate,
}

// データベースのセットアップ
pub fn setup_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS schedules (
            id      INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            date    TEXT NOT NULL,
        )",
        (),
    )?;

    Ok(conn)
}

// pub fn add_TASK(conn: &Connection, title: &str, date: &str) -> Result<()> {
//     conn.execute(
//         "INSERT INTO TASK (title,date) VALUES(?1,?2)",
//         params![title,date],
//     )?;
//     Ok(())
// }
