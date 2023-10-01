use crate::cli::Cli;
use crate::core;
use crate::core::note::JournalPeriod;
use std::path::{Path, PathBuf};

#[derive(clap::Args, Default)]
pub struct Args {
    #[arg(long)]
    journal_only: bool,

    #[arg(long)]
    date: Option<String>,

    #[arg(long)]
    period: Option<JournalPeriod>,

    path: Option<PathBuf>,
}

fn parse_date(value: &str) -> Result<chrono::NaiveDate, anyhow::Error> {
    if value.eq("today") {
        return Ok(chrono::Local::now().date_naive());
    }

    if let Ok(date) = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return Ok(date);
    }

    return Err(anyhow::anyhow!("Invalid date: {}", value));
}

pub fn run(_cli: &Cli, args: &Args) {
    let path = args.path.clone().unwrap_or(Path::new(".").to_path_buf());
    let mut note_filter = core::note::Filter::new()
        .add(&path)
        .period(args.period.clone().unwrap_or(JournalPeriod::All));

    if args.journal_only {
        note_filter = note_filter.journal_only();
    }

    if let Some(date) = args.date.as_deref() {
        let date = parse_date(date).expect("Invalid date");
        note_filter = note_filter.date(&date);
    }

    for note in note_filter.notes() {
        println!("{}", note.display());
    }
}
