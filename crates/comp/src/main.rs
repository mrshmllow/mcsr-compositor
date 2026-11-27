use clap::Parser;
use tracing::{info, trace};

use crate::{
    cli::{Cli, Commands},
    config::ConfigEngine,
};

mod cli;
mod compositor;
mod config;

fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn main() -> Result<(), anyhow::Error> {
    init_tracing();

    let args = Cli::parse();

    let Commands::Wrap {
        arguments,
        config_plugin,
        // config,
        ..
    } = args.command;

    info!(config_plugin = ?config_plugin);
    trace!(arguments = ?arguments);

    let mut conf = ConfigEngine::new(config_plugin)?;

    info!(tick = ?conf.tick());

    // info!(config = ?conf);

    Ok(())
}
