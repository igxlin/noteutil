use std::path::PathBuf;

use crate::cli::Cli;
use crate::core;

#[derive(clap::Args, Default)]
pub struct Args {
    #[arg(long)]
    relative_to: Option<PathBuf>,
}

pub fn run(_cli: &Cli, args: &Args) {
    let mut notes = core::note::Filter::new().add(&_cli.root_dir).notes();
    if let Some(base_path) = args.relative_to.as_ref() {
        let base_path = if base_path.is_file() {
            base_path.parent().unwrap()
        } else {
            base_path
        };

        notes = notes
            .into_iter()
            .map(|pathbuf| {
                pathdiff::diff_paths(
                    pathbuf.canonicalize().unwrap(),
                    base_path.canonicalize().unwrap(),
                )
                .unwrap()
            })
            .collect()
    }

    for note in &notes {
        println!("{}", note.display());
    }
}
