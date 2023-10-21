use std::path::Path;
use std::path::PathBuf;

use crate::cli::Cli;
use crate::core::journal;

#[derive(clap::Args, Default)]
pub struct Args {
    #[arg(short = 'p', long = "period")]
    periods: Vec<journal::Period>,

    #[arg(long)]
    date: Option<String>,

    #[arg(long)]
    root_dir: Option<PathBuf>,
}

pub fn run(_cli: &Cli, args: &Args) {
    let root_dir = args
        .root_dir
        .clone()
        .unwrap_or(Path::new(".").to_path_buf());

    let today = chrono::Local::now().date_naive();
    let date = match args.date.as_deref() {
        Some(args_date) => crate::cli::date::parse(args_date).expect("Invalid date"),
        None => today,
    };

    for path in journal::paths(date, &args.periods, root_dir) {
        println!("{}", path.display());
    }
}
