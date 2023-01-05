use anyhow::{Context, Result};
use serde_json::Value;
use std::str;

pub trait ContentAddressableStorage {
    fn retrieve(&self, hash: &str, object_type: &ObjectType) -> Result<Vec<u8>>;

    fn retrieve_json(&self, hash: &str) -> Result<Value> {
        let referenced_obj = self
            .retrieve(hash, &ObjectType::Json)
            .context("expected cas object to be a valid json")?;
        let json = str::from_utf8(&referenced_obj)?;
        let referenced_value: Value = serde_json::from_str(json)?;
        Ok(referenced_value)
    }
}

#[derive(Debug)]
pub enum ObjectType {
    Raw,
    Json,
}
