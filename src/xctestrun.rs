use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;


#[derive(Deserialize)]
struct Xctestrun {
    #[serde(rename = "rootId")]
    root_id: RootId,
    #[serde(rename = "storage")]
    storage: Storage,
    #[serde(rename = "version")]
    version: Version,
}


#[derive(Deserialize)]
struct RootId {
    #[serde(rename = "hash")]
    hash: String
}

#[derive(Deserialize)]
struct Storage {
    #[serde(rename = "backend")]
    backend: Backend,
    #[serde(rename = "compression")]
    compression: Compression,
}

#[derive(Deserialize)]
struct Version {
    #[serde(rename = "major")]
    major: u32,
    #[serde(rename = "minor")]
    minor: u32,
}


#[derive(Deserialize_enum_str, PartialEq, Debug)]
enum Backend {
    #[serde(rename = "fileBacked2")]
    FileBacked2,
}


#[derive(Deserialize_enum_str, PartialEq, Debug)]
enum Compression {
    #[serde(rename = "standard")]
    Standard,
}

#[cfg(test)]
mod tests {
    use crate::xctestrun::{Xctestrun, Backend, Compression};

    #[test]
    fn it_works() {
        let fixture = include_bytes!("../fixture/xcresult/1017af07-cba3-4c79-ab87-d4c959bda03b.xcresult/Info.plist");
        let xctestrun: Xctestrun = plist::from_bytes(fixture)
            .expect("failed to read xcresult fixture");
        assert_eq!(xctestrun.root_id.hash, "0~bowBLbloy7mFmt4ihiTwC4muw_5xQAk8myCoKrXmgo74YEm6SJWS9mmADtQZEs6FQ6kFWfi7QwWW_UERLHFXmg==");
        
        assert_eq!(xctestrun.version.major, 3);
        assert_eq!(xctestrun.version.minor, 39);

        assert_eq!(xctestrun.storage.backend, Backend::FileBacked2);
        assert_eq!(xctestrun.storage.compression, Compression::Standard);
    }
}

