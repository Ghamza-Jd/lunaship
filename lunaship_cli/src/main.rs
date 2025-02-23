mod params;
mod prelude;

use clap::Parser;
use lunaship_engine::init::InitProject;
use params::LunashipCommand;
use prelude::Fallible;

fn main() -> Fallible<()> {
    let command = params::CliParams::parse().command;
    match command {
        LunashipCommand::Init { name, typ, path } => {
            lunaship_engine::init::init_project(InitProject {
                name,
                path,
                typ: typ.into(),
            })?;
        }
        LunashipCommand::Install => {
            lunaship_engine::install::install()?;
        }
        LunashipCommand::Run { script_name } => {
            lunaship_engine::run::run(&script_name)?;
        }
        LunashipCommand::Start => {
            lunaship_engine::run::run("start")?;
        }
    }
    Ok(())
}
