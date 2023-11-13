use std::path::PathBuf;

#[derive(clap::Args, Default)]
pub struct Args {
    template: String,

    #[arg(long)]
    root_dir: Option<PathBuf>,

    #[arg(short, long)]
    outfile: Option<PathBuf>,
}

pub fn run(ctx: &noteutil::Context, args: &Args) {
    let template_path = ctx
        .config
        .root_dir
        .join("templates")
        .join(args.template.as_str());

    let mut tera = tera::Tera::default();
    tera.add_template_file(template_path, Some(args.template.as_str()))
        .expect("invalid template");

    let context = tera::Context::new();
    let content = tera
        .render(args.template.as_str(), &context)
        .expect("Unable to apply template");

    match &args.outfile {
        Some(outfile) => {
            if let Err(why) = std::fs::write(outfile, content.as_bytes()) {
                println!("Unable to write to file {}: {}", outfile.display(), why);
            }
        }
        None => println!("{}", content),
    }
}
