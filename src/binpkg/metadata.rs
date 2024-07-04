pub struct Metadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Metadata {
    pub fn new(name: String, version: String, description: String) -> Self {
        Self {
            name, 
            version,
            description
        }
    }
}