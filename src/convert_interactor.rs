use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::Path,
};

use anyhow::Result;
use log::info;

use crate::{cas::ContentAddressableStorage, test_result::TestResult};

pub struct ConvertInteractor {}

impl ConvertInteractor {
    pub fn new() -> ConvertInteractor {
        ConvertInteractor {}
    }

    pub fn execute(&self, input: &Path, output: &Path) -> Result<()> {
        info!("Processing bundle {}", input.display());
        let test_result = TestResult::new(input).expect("invalid input: xcresult is malformed");
        let xcresult = test_result
            .read()
            .expect("invalid input: Info.plist is malformed");
        let root_id = xcresult.root_id;
        let mut root_obj = test_result
            .retrieve_json(&root_id.hash)
            .expect("failure retrieving root cas object");
        if output.exists() {
            info!("output already exists. cleaning...");
            remove_dir_all(output).expect("");
        }
        create_dir_all(output).expect("error creating output folder");

        let cas_dir = output.join("cas");
        create_dir_all(&cas_dir).expect("error creating cas folder");

        let converted = test_result
            .convert(&mut root_obj, &cas_dir)
            .expect("convertion failed");

        let mut xcresult_json_file =
            File::create(output.join("xcresult.json")).expect("error creating xcresult.json");
        let xcresult_json =
            serde_json::to_string(converted).expect("error unwrapping nested jsons");
        xcresult_json_file
            .write_all(xcresult_json.as_bytes())
            .expect("error writing xcresult.json");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rstest::rstest;

    use super::ConvertInteractor;

    #[rstest]
    #[case("1017af07-cba3-4c79-ab87-d4c959bda03b.xcresult")]
    #[case("174cd1de-31c6-43c4-ae04-fce7b5b54826.xcresult")]
    #[case("2e75af03-2c35-45c2-bdee-9d2fe15aeec2.xcresult")]
    #[case("cdadd5ff-38e2-4a3a-a3f0-969ce0971eb0.xcresult")]
    #[case("d4cab44e-4004-4f55-98a0-e3f5530fcb81.xcresult")]
    fn it_works(#[case] id: &str) {
        let interactor = ConvertInteractor::new();

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let project_dir = Path::new(&manifest_dir)
            .join("fixture")
            .join("xcresult")
            .join(id);
        
        let temp_output = tempfile::tempdir().unwrap();

        let result = interactor.execute(&project_dir, temp_output.path());
        assert!(result.is_ok(), "should successfully parse xcresult");
    }
}
