use std::error::Error;

#[derive(clap::Args, Default)]
pub struct Args {
    #[arg(long)]
    lsp: bool,

    #[arg(long)]
    http: bool,
}

pub fn run(cx: noteutil::Context, args: &Args) -> Result<(), Box<dyn Error>> {
    let rt = tokio::runtime::Runtime::new()?;

    let mut tasks = Vec::new();
    if args.lsp {
        tasks.push(rt.spawn(noteutil::lsp::serve(cx)));
    }
    if args.http {
        tasks.push(rt.spawn(noteutil::http::Server::new().serve()));
    }

    rt.block_on(async move {
        futures::future::join_all(tasks).await;
    });

    Ok(())
}
