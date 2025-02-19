use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Specs {
    project: ProjectSpecs,
    dependencies: HashMap<String, DependencySpecs>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectSpecs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DependencySpecs {
    #[serde(flatten)]
    source: DependencySource,
    #[serde(flatten)]
    file_list: FilePicks,
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
pub enum FilePicks {
    #[serde(rename = "white_list")]
    Whitelist(Vec<String>),
    #[serde(rename = "black_list")]
    Blacklist(Vec<String>),
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
    use std::vec;

    #[test]
    fn initial_specs() {
        let toml_specs = toml::toml! {
        [project]
        name = "test-project"

        [dependencies]
        };
        let specs = Specs {
            project: ProjectSpecs {
                name: "test-project".to_string(),
            },
            dependencies: HashMap::new(),
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
        json = { git = "https://github.com/rxi/json.lua", tag = "v0.1.2", white_list = ["json.lua"] }
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
                    file_list: FilePicks::Whitelist(vec!["json.lua".to_string()]),
                },
            )]),
        };
        let de_specs = toml::to_string(&toml_specs).unwrap();
        let de_specs = toml::from_str(&de_specs).unwrap();
        assert_eq!(specs, de_specs);
    }
}
