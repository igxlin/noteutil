use std::path::PathBuf;

use rayon::prelude::*;

#[derive(clap::Args, Default)]
pub struct Args {
    #[arg(long)]
    relative_to: Option<PathBuf>,

    #[arg(long)]
    link_to: Option<PathBuf>,
}

pub fn run(ctx: &noteutil::Context, args: &Args) {
    let walkdir_entries: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(&ctx.config.root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    let mut notes: Vec<noteutil::Note> = walkdir_entries
        .into_par_iter()
        .filter(|e| e.path().is_file() && e.path().extension().is_some_and(|ext| ext == "md"))
        .filter_map(|e| noteutil::Note::build(e.path()).ok())
        .collect();

    if let Some(path) = args.link_to.as_ref() {
        notes = notes
            .into_iter()
            .filter(|note| note.link_to(path))
            .collect();
    }

    for note in notes {
        if let Some(base_path) = args.relative_to.as_ref() {
            let base_path = if base_path.is_file() {
                base_path.parent().unwrap()
            } else {
                base_path
            };

            println!(
                "{}",
                pathdiff::diff_paths(
                    note.path.canonicalize().unwrap(),
                    base_path.canonicalize().unwrap(),
                )
                .unwrap()
                .display()
            );
        } else {
            println!("{}", note.path.display());
        }
    }
}
