use crate::error::LunashipError;
use crate::pathbuf;
use crate::specs::DependencySpecs;
use crate::specs::Specs;

pub fn add_dependency(alias: &str, git: &str) -> Result<(), LunashipError> {
    let spec_path = pathbuf![".", "lunaship.toml"];
    let raw_spec = std::fs::read_to_string(&spec_path)?;
    let mut specs = toml::from_str::<Specs>(&raw_spec)?;
    specs.add_dependency(
        alias.to_string(),
        DependencySpecs {
            git: git.to_string(),
        },
    );
    let raw_specs = toml::to_string(&specs)?;
    std::fs::write(spec_path, raw_specs)?;
    Ok(())
}
