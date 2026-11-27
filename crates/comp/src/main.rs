use clap::Parser;
use tracing::{info, trace};

use crate::cli::{Cli, Commands};

mod cli;
mod config;

fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn main() {
    init_tracing();

    let args = Cli::parse();

    let Commands::Wrap {
        arguments,
        config_plugin,
        config,
        ..
    } = args.command;

    info!(config_plugin = ?config_plugin, config = ?config);

    trace!(arguments = ?arguments);
    info!("Hello, world!");
}
