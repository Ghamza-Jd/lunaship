use crate::error::LunashipError;
use crate::pathbuf;
use crate::templates;
use askama::Template;
use std::path::PathBuf;

pub struct InitProject {
    pub name: String,
    pub typ: LuaProjectType,
    pub path: PathBuf,
}

pub enum LuaProjectType {
    Love,
}

pub fn init_project(project: InitProject) -> Result<(), LunashipError> {
    init_common_files(&project)?;
    init_love_files(&project)?;
    Ok(())
}

fn init_common_files(project: &InitProject) -> Result<(), LunashipError> {
    let dir_path = pathbuf![&project.path];
    std::fs::create_dir_all(&dir_path)?;

    let lunaship_toml_path = pathbuf![&dir_path, "lunaship.toml"];
    let gitignore_path = pathbuf![&dir_path, ".gitignore"];

    let lunaship_toml_content = templates::LunashipProject {
        project_name: project.name.clone(),
    }
    .render()?;
    let gitignore_content = templates::GitIgnore.render()?;

    std::fs::write(lunaship_toml_path, lunaship_toml_content)?;
    std::fs::write(gitignore_path, gitignore_content)?;
    Ok(())
}

fn init_love_files(project: &InitProject) -> Result<(), LunashipError> {
    let dir_path = pathbuf![&project.path];
    std::fs::create_dir_all(&dir_path)?;

    let lua_rc_path = pathbuf![&dir_path, ".luarc.json"];
    let main_path = pathbuf![&dir_path, "main.lua"];
    let conf_path = pathbuf![&dir_path, "conf.lua"];
    let lunaship_toml_path = pathbuf![&dir_path, "lunaship.toml"];

    let lua_rc_content = templates::LoveLuaRc.render()?;
    let main_content = templates::LoveMain {
        project_name: project.name.clone(),
    }
    .render()?;
    let conf_content = templates::LoveConf {
        project_name: project.name.clone(),
    }
    .render()?;
    let lunaship_toml_content = templates::LunashipLoveProject {
        project_name: project.name.clone(),
    }
    .render()?;

    std::fs::write(lua_rc_path, lua_rc_content)?;
    std::fs::write(main_path, main_content)?;
    std::fs::write(conf_path, conf_content)?;
    std::fs::write(lunaship_toml_path, lunaship_toml_content)?;
    Ok(())
}
