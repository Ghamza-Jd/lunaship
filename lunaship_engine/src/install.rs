use crate::error::LunashipError;
use crate::pathbuf;
use crate::specs::DependencySource;
use crate::specs::GitRef;
use crate::specs::Specs;
use std::process::Command;
use std::time::Instant;

pub fn install() -> Result<(), LunashipError> {
    let specs = Specs::load()?;
    for (name, dep) in specs.dependencies.iter() {
        match &dep.source {
            DependencySource::Git { git, git_ref } => {
                resolve_git_dependency(name, git, git_ref, &dep.file)?
            }
        };
    }
    Ok(())
}

pub fn resolve_git_dependency(
    name: &str,
    git: &str,
    _git_ref: &GitRef,
    file: &str,
) -> Result<(), LunashipError> {
    let now = Instant::now().elapsed().as_millis();
    let temp_name = pathbuf![".", "lunaship_tmp", format!("{name}-{now}")];
    Command::new("git")
        .args(&["clone", git, &temp_name.to_string_lossy(), "--depth", "1"])
        .output()?;
    let target = pathbuf![".", "@mod"];
    std::fs::create_dir_all(&target)?;
    std::fs::copy(pathbuf![".", temp_name, file], pathbuf![target, file])?;
    std::fs::remove_dir_all(pathbuf![".", "lunaship_tmp"])?;
    Ok(())
}
