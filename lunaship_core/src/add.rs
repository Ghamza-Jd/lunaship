use crate::error::LunashipError;
use crate::pathbuf;
use crate::specs::Specs;

pub fn add_dependency(alias: &str, git: &str) -> Result<(), LunashipError> {
    println!("Adding dependency: {} from {}", alias, git);
    let raw_spec = std::fs::read_to_string(pathbuf![".", "lunaship.toml"])?;
    let specs = toml::from_str::<Specs>(&raw_spec)?;
    println!("{specs:#?}");
    Ok(())
}
