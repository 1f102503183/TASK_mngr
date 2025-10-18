use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "CLI予定ツール TaskMngr", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

enum Commands {
    // add task
    Add {
        title: String,
        #[arg(short, long)]
        due: Option<String>,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {title, due} => {
            println!("== Add TASK ==");
            println!("the TASK title : {}",title);

            if let Some(d) = due {
                println!("limit: {}",d);
            }else {
                println!("limit: Nan");
                            }
        }
        Commands::List => {
            println!("== TASK list (not availabled now) ==");
        }
    }
}
