use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Wrap Minecraft.
    ///
    /// You should pass this as a launcher wrapper command, ie `comp wrap --`.
    Wrap {
        #[arg(long)]
        config_plugin: PathBuf,

        #[arg(long)]
        config: PathBuf,

        #[arg(last = true)]
        arguments: Vec<String>,
    },
}
