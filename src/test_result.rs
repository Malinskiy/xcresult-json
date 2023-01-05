use crate::cas::{ContentAddressableStorage, ObjectType};
use crate::xcresult::Xcresult;
use anyhow::{bail, Context, Result};
use log::info;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::str;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub struct TestResult {
    path: PathBuf,
}

impl TestResult {
    pub fn new(path: &Path) -> Option<TestResult> {
        if path.exists() && path.is_dir() {
            Some(TestResult {
                path: Path::new(path).to_owned(),
            })
        } else {
            None
        }
    }

    pub fn read(&self) -> Result<Xcresult> {
        let info_plist = self.path.join("Info.plist");
        let xcresult: Xcresult = plist::from_file(info_plist)?;
        Ok(xcresult)
    }

    pub fn convert<'a>(&self, value: &'a mut Value, cas_dir: &Path) -> Result<&'a mut Value> {
        match value {
            Value::Object(object) => {
                let values = object.values_mut();
                for v in values {
                    if v.is_object() {
                        let value_obj = v
                            .as_object()
                            .context("unexpected type of json field: should be object")?;
                        if value_obj.contains_key("_type") {
                            let type_obj = value_obj
                                .get("_type")
                                .context("expected json object to have field _type")?
                                .as_object()
                                .context("expected _type to be a json object")?;
                            let type_name = type_obj
                                .get("_name")
                                .context("expected _name to be defined")?
                                .as_str()
                                .context("expected _name value to be a json string")?;
                            if type_name == "Reference" {
                                let reference_hash = value_obj
                                    .get("id")
                                    .context("expected id for type Reference")?
                                    .as_object()
                                    .context("expected Reference id to be a json object")?
                                    .get("_value")
                                    .context("expected _value to be defined for Reference")?
                                    .as_str()
                                    .context("expected _value to be a json string")?;
                                if value_obj.contains_key("targetType") {
                                    info!("Found embeddable hash {}", reference_hash);
                                    let referenced_value = self.retrieve_json(reference_hash)?;
                                    referenced_value.clone_into(v);
                                } else {
                                    info!("Found raw hash {}", reference_hash);
                                    let raw_obj =
                                        self.retrieve(reference_hash, &ObjectType::Raw)?;
                                    File::create(cas_dir.join(reference_hash))?
                                        .write_all(&raw_obj)?;
                                }
                            }
                        }
                    }
                    self.convert(v, cas_dir)?;
                }
            }
            Value::Array(array) => {
                for val in array {
                    self.convert(val, cas_dir)?;
                }
            }
            Value::Null => {}
            Value::Bool(_) => {}
            Value::Number(_) => {}
            Value::String(_) => {}
        }
        Ok(value)
    }
}

impl ContentAddressableStorage for TestResult {
    fn retrieve(&self, hash: &str, object_type: &ObjectType) -> Result<Vec<u8>> {
        let path = fs::canonicalize(&self.path)?;
        let absolute_path = path
            .as_path()
            .to_str()
            .context("Absolute path is should be available")?;
        let mut cmd = Command::new("xcrun");
        let format = match object_type {
            ObjectType::Raw => "raw",
            ObjectType::Json => "json",
        };
        let bin = cmd.args([
            "xcresulttool",
            "get",
            "--format",
            format,
            "--path",
            absolute_path,
            "--id",
            hash,
        ]);
        let output = bin.output()?;
        let status = output.status;
        if !status.success() {
            bail!("xcresulttool failed");
        }
        let stdout = output.stdout;
        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::TestResult;
    use crate::cas::ContentAddressableStorage;

    #[test]
    fn it_works() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_dir = Path::new(&manifest_dir)
            .join("fixture")
            .join("xcresult")
            .join("1017af07-cba3-4c79-ab87-d4c959bda03b.xcresult");
        let test_result = TestResult::new(&project_dir).unwrap();
        let root_object = test_result.retrieve_json("0~bowBLbloy7mFmt4ihiTwC4muw_5xQAk8myCoKrXmgo74YEm6SJWS9mmADtQZEs6FQ6kFWfi7QwWW_UERLHFXmg==");
        assert!(
            root_object.is_ok(),
            "xcresult read failed, {}",
            root_object.unwrap_err()
        );
        let mut v = root_object.unwrap();
        assert_eq!(v["_type"]["_name"], "ActionsInvocationRecord");
        let temp_cas = tempfile::tempdir().unwrap();

        test_result
            .convert(&mut v, temp_cas.path())
            .expect("conversion should succeed");
    }
}
