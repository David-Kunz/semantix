use serde::Deserialize;
use std::{collections::HashMap, io, path::Path};

#[derive(Deserialize, Debug)]
pub struct Model {
    pub definitions: HashMap<String, Definition>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "camelCase")]
pub enum Definition {
    Entity(Entity),
}

#[derive(Deserialize, Debug)]
pub struct Entity {
    pub elements: HashMap<String, Element>,
}

#[derive(Deserialize, Debug)]
pub struct Element {
    #[serde(default)]
    pub key: bool,
    #[serde(rename(deserialize = "type"))]
    pub element_type: String,
}

#[derive(Debug)]
pub enum SemantixError {
    IO(io::Error),
    Parse(serde_json::Error),
}

impl From<io::Error> for SemantixError {
    fn from(err: io::Error) -> Self {
        SemantixError::IO(err)
    }
}

impl From<serde_json::Error> for SemantixError {
    fn from(err: serde_json::Error) -> Self {
        SemantixError::Parse(err)
    }
}

pub fn load_model<P: AsRef<Path>>(path: P) -> Result<Model, SemantixError> {
    let content = std::fs::read_to_string(path)?;
    let model: Model = serde_json::from_str(&content)?;
    Ok(model)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_example_model() -> Result<(), SemantixError> {
        let model = load_model("./test/example.json")?;
        dbg!(model);
        Ok(())
    }
}
