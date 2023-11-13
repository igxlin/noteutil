use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

mod cmd;

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(long)]
    config: Option<PathBuf>,

    #[arg(long)]
    root_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<cmd::Command>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = noteutil::Context::default();
    let cli = Cli::parse();

    match noteutil::Config::from_default_locations() {
        Ok(config) => ctx.config = config,
        Err(err) => println!("Failed to load configuration: {}", err),
    }

    if let Some(config_path) = cli.config {
        ctx.config = noteutil::Config::from_file(&config_path).unwrap_or_else(|_| {
            panic!("Invalid path: {:?}", config_path);
        });
    }

    if let Some(root_dir) = cli.root_dir {
        ctx.config.root_dir = root_dir;
    }

    cmd::run(&ctx, &cli.command)?;

    Ok(())
}
