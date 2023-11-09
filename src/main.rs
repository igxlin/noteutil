use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

mod cmd;

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(long)]
    config: Option<PathBuf>,

    #[arg(long, default_value = ".")]
    root_dir: PathBuf,

    #[command(subcommand)]
    command: Option<cmd::Command>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = noteutil::Context::default();
    let cli = Cli::parse();

    if let Some(config_path) = cli.config {
        ctx.config = noteutil::Config::from_file(&config_path).unwrap_or_else(|_| {
            panic!("Invalid path: {:?}", config_path);
        });
    }
    ctx.root_dir = cli.root_dir;

    cmd::run(&ctx, &cli.command)?;

    Ok(())
}
