use clap::Parser;
use std::path::PathBuf;

use crate::core::config::Config;

mod date;
mod journal;
mod note;
mod template;

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(long)]
    _config: Option<PathBuf>,

    #[arg(long, default_value = ".")]
    root_dir: PathBuf,

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
    Journal(journal::Args),

    Template(template::Args),

    Note(note::Args),
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Journal(args)) => journal::run(&cli, args),
        Some(Commands::Template(args)) => template::run(&cli, args),
        Some(Commands::Note(args)) => note::run(&cli, args),
        None => {}
    }
}
