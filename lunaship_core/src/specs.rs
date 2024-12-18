use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specs {
    project: ProjectSpecs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSpecs {
    name: String,
}
