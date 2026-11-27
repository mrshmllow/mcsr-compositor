use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Wrap Minecraft.
    ///
    /// You should pass this as a launcher wrapper command, ie `comp wrap --`.
    Wrap {
        #[arg(long, value_name = ":lua | :kdl | <CONFIG_PLUGIN>")]
        config_plugin: ConfigPlugin,

        // #[arg(long)]
        // config: PathBuf,
        #[arg(last = true)]
        arguments: Vec<String>,
    },
}

#[derive(Clone, Debug)]
pub enum ConfigPlugin {
    BuiltinLua,
    BuiltinKdl,
    Custom(String),
}

impl FromStr for ConfigPlugin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ":lua" => Ok(Self::BuiltinLua),
            ":kdl" => Ok(Self::BuiltinKdl),
            str => Ok(Self::Custom(str.to_string())),
        }
    }
}
