use crate::error::LunashipError;
use crate::pathbuf;
use serde_json::json;
use std::fs::read_dir;
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
    let reg = handlebars::Handlebars::new();
    let handlebars_params = json!({
        "project_name": project.name
    });
    let common_dir_path = pathbuf![".", "templates", "common"];
    let common_files = read_dir(&common_dir_path)?;
    let common_files = common_files
        .flat_map(|entry| match entry {
            Ok(entry) => Some(entry.path()),
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    for path in common_files {
        let file_name = match path.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => return Err(LunashipError::FileNameError),
        };
        let content = std::fs::read_to_string(&path)?;
        let rendered_content = reg.render_template(&content, &handlebars_params)?;
        let file_name = strip_hbs(&file_name);
        let dir_path = pathbuf![&project.path];
        std::fs::create_dir_all(&dir_path)?;
        let new_file_path = pathbuf![dir_path, file_name];
        std::fs::write(new_file_path, rendered_content)?;
    }

    match project.typ {
        LuaProjectType::Love => {
            let love_dir_path = pathbuf![".", "templates", "love"];
            let love_files = read_dir(&love_dir_path)?;
            let love_files = love_files
                .flat_map(|entry| match entry {
                    Ok(entry) => Some(entry.path()),
                    Err(_) => None,
                })
                .collect::<Vec<_>>();
            for path in love_files {
                let file_name = match path.file_name() {
                    Some(file_name) => file_name.to_string_lossy().to_string(),
                    None => return Err(LunashipError::FileNameError),
                };
                let content = std::fs::read_to_string(&path)?;
                let rendered_content = reg.render_template(&content, &handlebars_params)?;
                let file_name = strip_hbs(&file_name);
                let dir_path = pathbuf![&project.path];
                std::fs::create_dir_all(&dir_path)?;
                let new_file_path = pathbuf![dir_path, file_name];
                std::fs::write(new_file_path, rendered_content)?;
            }
        }
    }

    Ok(())
}

fn strip_hbs(file_name: &str) -> String {
    file_name
        .split('.')
        .filter(|ss| ss != &"hbs")
        .collect::<Vec<_>>()
        .join(".")
}
