use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};
use std::path::Path;

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub date: NaiveDate,
    pub done: bool,
}

// データベースのセットアップ
pub fn setup_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id      INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            date    TEXT NOT NULL,
            done INTEGER NOT DEFAULT 0
        )",
        (),
    )?;

    Ok(conn)
}

// TASK追加
pub fn add_task(conn: &Connection, title: &str, date: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO task (title,date,) VALUES(?1,?2)",
        params![title, date],
    )?;
    Ok(())
}

// TASK list
pub fn list_task(conn: &Connection, include_done: bool) -> Result<Vec<Task>> {
    let mut stmt = if include_done {
        // all
        conn.prepare("SELECT id, title, date, FROM tasks ORDER BY date ASC")?
    } else {
        conn.prepare("SELECT id, title, date, done FROM tasks WHERE done = 0 ORDER BY date ASC")?
    };

    let task_iter = stmt.query_map(params![], |row| {
        // gemini に考えてもらった　date のパース処理を追加
        let date_str: String = row.get(2)?;
        let parsed_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") // データベースに "%Y-%m-%d" 形式で保存されていると仮定
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?; // row.get() でカラムの値を指定の型で取得

        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            date: parsed_date,
            done: row.get::<_, i32>(3)? != 0,
        })
    })?;

    let tasks = task_iter.filter_map(std::result::Result::ok).collect();

    Ok(tasks)
}

pub fn complete_task(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE task SET done = 1 WHERE id = ?1", params![id])?;
    Ok(())
}
