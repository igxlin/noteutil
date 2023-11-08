use crate::cli::date;
use crate::cli::Cli;
use crate::core::journal;
use std::path::{Path, PathBuf};

#[derive(clap::Args, Default)]
pub struct Args {
    #[arg(long)]
    journal_only: bool,

    #[arg(long)]
    date: Option<String>,

    #[arg(short = 'p', long = "period")]
    periods: Vec<journal::Period>,

    path: Option<PathBuf>,
}

pub fn run(_cli: &Cli, args: &Args) {
    let path = args.path.clone().unwrap_or(Path::new(".").to_path_buf());
    let mut note_filter = crate::note::Filter::new().add(&path).periods(&args.periods);

    if args.journal_only {
        note_filter = note_filter.journal_only();
    }

    if let Some(date) = args.date.as_deref() {
        let date = date::parse(date).expect("Invalid date");
        note_filter = note_filter.date(&date);
    }

    for note in note_filter.notes() {
        println!("{}", note.display());
    }
}
