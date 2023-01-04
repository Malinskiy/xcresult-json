use anyhow::Result;

pub trait ContentAddressableStorage {
    fn retrieve(&self, hash: &str, object_type: &ObjectType) -> Result<String>;
}

#[derive(Debug)]
pub enum ObjectType {
    RAW,
    JSON,
}
