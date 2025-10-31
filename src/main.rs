mod db;
use clap::Parser;
use std::path::Path;

#[derive(clap::Subcommand)] // この挙列型がサブコマンドの種類と認識させる
enum Actions {
    Add {
        title: String,
        #[arg(short, long)]
        date: String,
    },
    List {
        #[arg(short, long)]
        include_done: bool,
    },
    // prog,
    Cmp {
        id: i64,
    },
}

//
#[derive(Parser)]
#[command(author, version, about = "CLI予定管理ツール TaskMngr", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Actions,
}

const DB_PATH: &str = "my_schedule.db";

fn main() {
    // データベースに接続
    let conn = match db::setup_db(Path::new(DB_PATH)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("データベース接続エラー：{}", e);
            return;
        }
    };

    let cli = Cli::parse();

    match cli.command {
        Actions::Add { title, date } => {
            println!("== Add TASK ==");
            println!("the {} was added on {}", title, date);
            if let Err(e) = db::add_task(&conn, &title, &date) {
                eprintln!("err: {}", e);
            }
        }
        Actions::List { include_done } => {
            println!("== TASK list {} (not availabled now) ==", include_done);
            match db::list_task(&conn, include_done) {
                Ok(tasks) => {
                    for task in tasks {
                        println!("ID:{}", task.id);
                        println!("title:{}", task.title);
                        println!("date:{}", task.date);
                        println!("done:{}", task.done);
                    }
                }

                Err(e) => {
                    eprint!("err: {}", e);
                }
            }
        }
        Actions::Cmp { id } => {
            println!("==compleated TASK {} ! ==", id);
            if let Err(e) = db::complete_task(&conn, id) {
                eprint!("err: {}", e)
            }
        }
    }
}
