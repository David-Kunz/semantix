use serde::{
    de::{self, Visitor},
    Deserialize,
};
use serde_json::Value;
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

#[derive(Debug, Default)]
pub struct Element {
    pub key: bool,
    pub element_type: String,
    pub annotations: HashMap<String, Value>,
}

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ElementVisitor {}

        impl<'de> Visitor<'de> for ElementVisitor {
            type Value = Element;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Could not deserialize element")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut element = Element {
                    ..Default::default()
                };

                while let Some(key) = map.next_key::<String>()? {
                    if key.starts_with('@') {
                        element.annotations.insert(key, map.next_value()?);
                    } else {
                        match key.as_str() {
                            "type" => {
                                element.element_type = map.next_value()?;
                            }
                            "key" => {
                                element.key = map.next_value()?;
                            }
                            _ => {}
                        }
                    }
                }

                if element.element_type.is_empty() {
                    return Err(de::Error::missing_field("type"));
                }

                Ok(element)
            }
        }
        deserializer.deserialize_map(ElementVisitor {})
    }
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
