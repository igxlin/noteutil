use std::error::Error;

mod journal;
mod note;
mod server;
mod template;

#[derive(clap::Subcommand)]
pub enum Command {
    Journal(journal::Args),
    Template(template::Args),
    Note(note::Args),
    Server(server::Args),
}

pub fn run(ctx: &noteutil::Context, cmd: &Option<Command>) -> Result<(), Box<dyn Error>> {
    match &cmd {
        Some(Command::Journal(args)) => journal::run(&ctx, args),
        Some(Command::Template(args)) => template::run(&ctx, args),
        Some(Command::Note(args)) => note::run(&ctx, args),
        Some(Command::Server(args)) => server::run(ctx.clone(), args)?,
        None => {}
    }

    Ok(())
}
