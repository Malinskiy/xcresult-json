use crate::cas::{ContentAddressableStorage,ObjectType};
use crate::xcresult::Xcresult;
use std::{process::Command, path::{Path, PathBuf}, fs};
use anyhow::{Context, Result, bail};
use std::str;
use serde_json::Value;

pub struct TestResult {
    path: PathBuf,
}

impl TestResult {
    pub fn new(path: &str) -> TestResult {
        return TestResult {
            path: Path::new(path).to_owned(),
        };
    }

    pub fn read(&self) -> Result<Xcresult> {
        let info_plist = self.path.join("Info.plist");
        let xcresult: Xcresult = plist::from_file(&info_plist).expect("invalid Info.plist");
        return Ok(xcresult);
    }

    pub fn convert<'a> (&self, value: &'a mut Value) -> Result<&'a mut Value> {
        match value {
            Value::Object(object) => {
                object.iter_mut().for_each(|(_, mut v)| {
                    if v.is_object() {
                        let value_obj = v.as_object().unwrap();
                        if value_obj.contains_key("_type") {
                            let type_obj = value_obj.get("_type").unwrap().as_object();
                            let type_name = type_obj.unwrap().get("_name").unwrap().as_str().unwrap();
                            if type_name == "Reference" {
                                let reference_hash = value_obj.get("id").unwrap().as_object().unwrap().get("_value").unwrap().as_str().unwrap();
                                if value_obj.contains_key("targetType") {
                                    //println!("Found embeddable hash {}", reference_hash);
                                    let referenced_obj = self.retrieve(reference_hash, &ObjectType::JSON).unwrap();
                                    let mut referenced_value: Value = serde_json::from_str(&referenced_obj).unwrap();
                                    referenced_value.clone_into(v);
                                } else {
                                    //println!("Found raw hash {}", reference_hash);
                                }
                            } 
                        }
                    } 
                    self.convert(v);
                })
            }
            Value::Array(array) => {
               for val in array {
                    self.convert(val);
                } 
            }
            Value::Null => {},
            Value::Bool(_) => {},
            Value::Number(_) => {},
            Value::String(_) => {},
        }
        return Ok(value)
    }
}

impl ContentAddressableStorage for TestResult {
    fn retrieve(&self, hash: &str, object_type: &ObjectType) -> Result<String> {
        let path = fs::canonicalize(&self.path)?;
        let absolute_path = path.as_path().to_str().context("Absolute path is should be available")?;
        let mut cmd = Command::new("xcrun");
        let format = match object_type {
            ObjectType::RAW => "raw",
            ObjectType::JSON => "json",
        };
        let bin = cmd.args(["xcresulttool", "get", "--format", format,"--path", absolute_path, "--id", hash]);
        let output = bin.output()?;
        let status = output.status;
        if !status.success() {
            bail!("xcresulttool failed");
        }
        let stdout = output.stdout;
        let json = str::from_utf8(&stdout)?;
        
        return Ok(json.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use serde_json::Value;

    use crate::cas::{ContentAddressableStorage, ObjectType};
    use super::TestResult;

    #[test]
    fn it_works() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_dir = Path::new(&manifest_dir).join("fixture").join("xcresult").join("1017af07-cba3-4c79-ab87-d4c959bda03b.xcresult");
        let test_result = TestResult::new(project_dir.to_str().unwrap());
        let root_object = test_result.retrieve("0~bowBLbloy7mFmt4ihiTwC4muw_5xQAk8myCoKrXmgo74YEm6SJWS9mmADtQZEs6FQ6kFWfi7QwWW_UERLHFXmg==", &ObjectType::JSON);
        assert_eq!(root_object.is_ok(), true, "xcresult read failed, {}", root_object.unwrap_err());
        let json = root_object.unwrap();
        let mut v: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["_type"]["_name"], "ActionsInvocationRecord");

        test_result.convert(&mut v);
    }

}

