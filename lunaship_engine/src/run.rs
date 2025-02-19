use crate::error::LunashipError;
use crate::specs::Specs;
use std::process::Command;

pub fn run(script_name: &str) -> Result<(), LunashipError> {
    let specs = Specs::load()?;
    let script = specs
        .scripts
        .get(script_name)
        .ok_or(LunashipError::ScriptNotFound(script_name.to_string()))?;
    let args = script.split_whitespace().collect::<Vec<_>>();
    Command::new(args[0]).args(&args[1..]).status()?;
    Ok(())
}
