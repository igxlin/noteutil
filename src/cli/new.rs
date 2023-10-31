use crate::cli::Cli;
use std::path::PathBuf;

#[derive(clap::Args, Default)]
pub struct Args {
    template: String,

    #[arg(long)]
    root_dir: Option<PathBuf>,
}

pub fn run(_cli: &Cli, args: &Args) {
    let template_path = _cli.root_dir.join("templates").join(args.template.as_str());

    let mut tera = tera::Tera::default();
    tera.add_template_file(template_path, Some(args.template.as_str()))
        .expect("invalid template");

    let context = tera::Context::new();

    println!("{}", tera.render(args.template.as_str(), &context).unwrap());
}
