use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Metadata {
    pub fn new(name: impl ToString, version: impl ToString, description: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string()
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    
    pub fn from_from(json: &str) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}