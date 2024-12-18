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
pub struct DependencySpecs {
    pub git: String,
}

impl Specs {
    pub fn add_dependency(&mut self, alias: String, dep_spec: DependencySpecs) {
        self.dependencies.insert(alias, dep_spec);
    }
}
