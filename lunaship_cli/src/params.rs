use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use lunaship_engine::init::LuaProjectType as CoreLuaProjectType;

#[derive(Debug, Parser)]
pub struct CliParams {
    #[command(subcommand)]
    pub command: LunashipCommand,
}

#[derive(Debug, Subcommand)]
pub enum LunashipCommand {
    /// Initialize a new project
    Init {
        /// Name of the project
        #[arg(short, long = "name")]
        name: String,
        /// Type of the project
        typ: LuaProjectType,
        /// Project's path
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Install the dependencies specified in lunaship.toml
    Install,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LuaProjectType {
    Love,
}

impl From<LuaProjectType> for CoreLuaProjectType {
    fn from(typ: LuaProjectType) -> Self {
        match typ {
            LuaProjectType::Love => CoreLuaProjectType::Love,
        }
    }
}
