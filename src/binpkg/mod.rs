use serde_json::Value;
use crate::binpkg::metadata::Metadata;

pub mod create;
pub mod read;
mod metadata;

pub struct BinPkg {
    pub header: Value,
    pub metadata: Metadata,
}

impl BinPkg {
    pub fn create(metadata: Metadata) -> Self {

    }
}