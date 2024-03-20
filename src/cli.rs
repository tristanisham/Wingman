use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initalize a new Wingman project.
    Init {
        /// Forces creating new project files, regardless of the dir's current contents.
        #[arg(short, long)]
        force: bool,
    },
    /// Build your Wingman project in the specified distribution directory.
    Build {
        /// Enable the filewatcher to build your site as you modify your sourcecode.
        #[arg(short, long)]
        watch: bool,
    }
}

