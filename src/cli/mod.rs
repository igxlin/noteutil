use clap::Parser;
use std::path::PathBuf;

use crate::core::config::Config;

mod date;
mod journal;
mod list;

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(long)]
    _config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn config(&self) -> Config {
        if self._config.is_none() {
            return Config::default();
        }

        let path = self._config.as_ref().unwrap().as_path();
        Config::from_file(path).expect("Invalid config file")
    }
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(name = "ls")]
    List(list::Args),

    Journal(journal::Args),
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List(args)) => {
            list::run(&cli, args);
        }
        Some(Commands::Journal(args)) => {
            journal::run(&cli, args);
        }
        None => {}
    }
}
