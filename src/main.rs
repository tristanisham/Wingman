use clap::Parser;
use cli::Args;
use wingman::Wingman;
mod cli;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let wm = Wingman::default();

    match &args.command {
        Some(cli::Command::Init { force }) => wm.init(*force)?,
        Some(cli::Command::Build { watch }) => wm.build(*watch)?,
        None => {}
    }

    Ok(())
}
