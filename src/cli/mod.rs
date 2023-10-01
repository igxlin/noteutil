use clap::Parser;
use std::path::PathBuf;

mod list;

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(name = "ls")]
    List(list::Args),
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List(args)) => {
            list::run(&cli, args);
        }
        None => {}
    }
}
