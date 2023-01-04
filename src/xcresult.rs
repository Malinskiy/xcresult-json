use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

#[derive(Deserialize)]
pub struct Xcresult {
    #[serde(rename = "rootId")]
    pub root_id: RootId,
    #[serde(rename = "storage")]
    pub storage: Storage,
    #[serde(rename = "version")]
    pub version: Version,
}

#[derive(Deserialize)]
pub struct RootId {
    #[serde(rename = "hash")]
    pub hash: String
}

#[derive(Deserialize)]
pub struct Storage {
    #[serde(rename = "backend")]
    pub backend: Backend,
    #[serde(rename = "compression")]
    pub compression: Compression,
}

#[derive(Deserialize)]
pub struct Version {
    #[serde(rename = "major")]
    pub major: u32,
    #[serde(rename = "minor")]
    pub minor: u32,
}


#[derive(Deserialize_enum_str, PartialEq, Debug)]
pub enum Backend {
    #[serde(rename = "fileBacked2")]
    FileBacked2,
}


#[derive(Deserialize_enum_str, PartialEq, Debug)]
pub enum Compression {
    #[serde(rename = "standard")]
    Standard,
}

#[cfg(test)]
mod tests {
    use crate::xcresult::{Xcresult, Backend, Compression};

    #[test]
    fn it_works() {
        let fixture = include_bytes!("../fixture/xcresult/1017af07-cba3-4c79-ab87-d4c959bda03b.xcresult/Info.plist");
        let xcresult: Xcresult = plist::from_bytes(fixture)
            .expect("failed to read xcresult fixture");
        assert_eq!(xcresult.root_id.hash, "0~bowBLbloy7mFmt4ihiTwC4muw_5xQAk8myCoKrXmgo74YEm6SJWS9mmADtQZEs6FQ6kFWfi7QwWW_UERLHFXmg==");
        
        assert_eq!(xcresult.version.major, 3);
        assert_eq!(xcresult.version.minor, 39);

        assert_eq!(xcresult.storage.backend, Backend::FileBacked2);
        assert_eq!(xcresult.storage.compression, Compression::Standard);
    }
}

