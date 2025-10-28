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
        is_all: bool,
    },
    // prog,
    Cmp {
        title: String,
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
        }
        Actions::List { is_all } => {
            println!("== TASK list {} (not availabled now) ==", is_all);
        }
        Actions::Cmp { title } => {
            println!("==compleated TASL {} ! ==", title)
        }
    }
}
