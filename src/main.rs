use clap::Parser;
use cli::Args;
use wingman::Wingman;
mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut wm = Wingman::default();

    match &args.command {
        Some(cli::Command::Init { force }) => wm.init(*force)?,
        Some(cli::Command::Build { watch }) => wm.build(*watch).await?,
        Some(cli::Command::Serve { port }) => wm.serve(port).await?,
        None => {}
    }

    Ok(())
}
