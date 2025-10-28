use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};
use std::path::Path;

#[derive(Debug)]
pub struct TASK {
    pub id: i64,
    pub title: String,
    pub date: NaiveDate,
    pub prog: bool,
}

// データベースのセットアップ
pub fn setup_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS schedules (
            id      INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            date    TEXT NOT NULL,
            prog    INTEGER NOT NULL DEFAULT 0
        )",
        (),
    )?;

    Ok(conn)
}

pub fn add_TASK(conn: &Connection, title: &str, date: &str, prog: &bool) -> Result<()> {
    conn.execute(
        "INSERT INTO task (title,date,prog) VALUES(?1,?2)",
        params![title, date, prog],
    )?;
    Ok(())
}

pub fn list_TASK(conn: &Connection, include_done: bool) -> Result<Vec<tasks>> {
    let mut stmt = if include_done {
        // all
        conn.prepare("SELECT id, title, date, prog FROM task ORDER BY date ASC")?
    } else {
        conn.prepare("SELECT id, title, date, prog FROM task WHERE prog = 0 ORDER BY date ASC")?
    };

    let task_iter = stmt.query_map(params![], |row| {
        // row.get() でカラムの値を指定の型で取得
        Ok(TASK_item {
            id: row.get(0)?,
            title: row.get(1)?,
            // 簡略化のためNaiveDateへのパスは省略しているらしい？
            date: row.get(2).unwrap_or_default(),
            // SQLiteの INTEGER (0/1) を Rustの bool に変換
            prog: row.get::<_, i32>(3)? != 0,
        })
    })?;

    let tasks = task_iter.filter_map(std::result::Result::ok).collect();

    Ok(tasks)
}

pub fn complete_schedule(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE task SET done = 1 WHERE id = ?1", params![id])?;
    Ok(())
}
