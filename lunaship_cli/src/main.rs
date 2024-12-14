mod params;
mod prelude;

use clap::Parser;
use lunaship_core::init::InitProject;
use params::LunashipCommand;
use prelude::Fallible;

fn main() -> Fallible<()> {
    let command = params::CliParams::parse().command;
    match command {
        LunashipCommand::Init { name, typ, path } => {
            lunaship_core::init::init_project(InitProject {
                name,
                path,
                typ: typ.into(),
            })?;
        }
    }
    Ok(())
}