use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub id: String,
    pub version: String,
    pub description: String,
    pub architecture: String,
    pub author: String,
}

impl Metadata {
    pub fn new(
        name: impl ToString,
        id: impl ToString,
        version: impl ToString,
        description: impl ToString,
        architecture: impl ToString,
        author: impl ToString
    ) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            architecture: architecture.to_string(),
            author: author.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    
    pub fn from_from(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}