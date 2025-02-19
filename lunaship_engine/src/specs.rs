use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specs {
    project: ProjectSpecs,
    dependencies: HashMap<String, DependencySpecs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSpecs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DependencySpecs {
    Git {
        url: String,
        #[serde(flatten)]
        git_ref: GitRef,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GitRef {
    #[serde(rename = "branch")]
    Branch(String),
    #[serde(rename = "tag")]
    Tag(String),
    #[serde(rename = "commit")]
    Commit(String),
}
