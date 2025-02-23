use crate::error::LunashipError;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Specs {
    pub project: ProjectSpecs,
    pub dependencies: HashMap<String, DependencySpecs>,
    pub scripts: HashMap<String, String>,
}

impl Specs {
    pub fn load() -> Result<Self, LunashipError> {
        let specs = std::fs::read_to_string("lunaship.toml")?;
        let specs: Specs = toml::from_str(&specs)?;
        Ok(specs)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectSpecs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DependencySpecs {
    #[serde(flatten)]
    pub source: DependencySource,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum DependencySource {
    Git {
        git: String,
        #[serde(flatten)]
        git_ref: GitRef,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GitRef {
    #[serde(rename = "branch")]
    Branch(String),
    #[serde(rename = "tag")]
    Tag(String),
    #[serde(rename = "commit")]
    Commit(String),
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn initial_specs() {
        let toml_specs = toml::toml! {
        [project]
        name = "test-project"

        [dependencies]

        [scripts]
        };
        let specs = Specs {
            project: ProjectSpecs {
                name: "test-project".to_string(),
            },
            dependencies: HashMap::new(),
            scripts: HashMap::new(),
        };
        let de_specs = toml::to_string(&toml_specs).unwrap();
        let de_specs = toml::from_str(&de_specs).unwrap();
        assert_eq!(specs, de_specs);
    }

    #[test]
    fn specs_with_git_dependencies() {
        let toml_specs = toml::toml! {
        [project]
        name = "test-project"

        [dependencies]
        json = { git = "https://github.com/rxi/json.lua", tag = "v0.1.2", file = "json.lua" }

        [scripts]
        };
        let specs = Specs {
            project: ProjectSpecs {
                name: "test-project".to_string(),
            },
            dependencies: HashMap::from([(
                "json".to_string(),
                DependencySpecs {
                    source: DependencySource::Git {
                        git: "https://github.com/rxi/json.lua".to_string(),
                        git_ref: GitRef::Tag("v0.1.2".to_string()),
                    },
                    file: "json.lua".to_string(),
                },
            )]),
            scripts: HashMap::new(),
        };
        let de_specs = toml::to_string(&toml_specs).unwrap();
        let de_specs = toml::from_str(&de_specs).unwrap();
        assert_eq!(specs, de_specs);
    }

    #[test]
    fn specs_with_scripts() {
        let toml_specs = toml::toml! {
        [project]
        name = "test-project"

        [dependencies]

        [scripts]
        run = "love ."
        };
        let specs = Specs {
            project: ProjectSpecs {
                name: "test-project".to_string(),
            },
            dependencies: HashMap::new(),
            scripts: HashMap::from([("run".to_string(), "love .".to_string())]),
        };
        let de_specs = toml::to_string(&toml_specs).unwrap();
        let de_specs = toml::from_str(&de_specs).unwrap();
        assert_eq!(specs, de_specs);
    }
}
