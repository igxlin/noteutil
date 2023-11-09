use std::path::Path;
use std::path::PathBuf;

use crate::cli::Cli;

#[derive(clap::Args, Default)]
pub struct Args {
    #[arg(short = 'p', long = "period")]
    periods: Vec<crate::journal::Period>,

    #[arg(long)]
    date: Option<String>,

    #[arg(long)]
    root_dir: Option<PathBuf>,
}

pub fn run(ctx: crate::Context, _cli: &Cli, args: &Args) {
    let root_dir = args
        .root_dir
        .clone()
        .unwrap_or(Path::new(".").to_path_buf());

    let today = chrono::Local::now().date_naive();
    let date = match args.date.as_deref() {
        Some(args_date) => crate::cli::date::parse(args_date).expect("Invalid date"),
        None => today,
    };

    for path in crate::journal::paths(ctx, date, &args.periods, root_dir, &_cli.config()) {
        println!("{}", path.display());
    }
}
