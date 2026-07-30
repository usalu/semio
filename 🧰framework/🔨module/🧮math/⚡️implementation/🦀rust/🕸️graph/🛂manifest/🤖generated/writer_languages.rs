// Generated from writer-languages.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const WRITERLANGUAGES_LANGUAGE_JACK: &str = "jack";
pub const WRITERLANGUAGES_LANGUAGE_WIRE: &str = "wire";
pub const WRITERLANGUAGES_LANGUAGE_PLAINTEXT: &str = "plaintext";
pub const WRITERLANGUAGES_LANGUAGE_MARKDOWN: &str = "markdown";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WriterLanguagesLanguageKind {
    #[serde(rename = "jack")]
    Jack,
    #[serde(rename = "wire")]
    Wire,
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "markdown")]
    Markdown,
}

impl WriterLanguagesLanguageKind {
    pub const ALL: &'static [Self] = &[WriterLanguagesLanguageKind::Jack, WriterLanguagesLanguageKind::Wire, WriterLanguagesLanguageKind::Plaintext, WriterLanguagesLanguageKind::Markdown];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Jack => "jack",
            Self::Wire => "wire",
            Self::Plaintext => "plaintext",
            Self::Markdown => "markdown",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "jack" => Ok(Self::Jack),
            "wire" => Ok(Self::Wire),
            "plaintext" => Ok(Self::Plaintext),
            "markdown" => Ok(Self::Markdown),
            other => Err(format!("unknown language kind {other:?} for WriterLanguages")),
        }
    }
}

pub const WRITERLANGUAGES_LANGUAGE_IDS: &[&str] = &["jack", "wire", "plaintext", "markdown"];
pub const WRITERLANGUAGES_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"writer-languages\",\"name\":\"Writer Languages\",\"languageKinds\":[{\"id\":\"jack\",\"name\":\"Jack\",\"properties\":[{\"name\":\"grammarModule\",\"kind\":\"data\",\"valueType\":\"text\"}]},{\"id\":\"wire\",\"name\":\"Wire\",\"properties\":[{\"name\":\"grammarModule\",\"kind\":\"data\",\"valueType\":\"text\"}]},{\"id\":\"plaintext\",\"name\":\"Plain Text\"},{\"id\":\"markdown\",\"name\":\"Markdown\"}]}";

pub fn writer_languages_manifest() -> Manifest {
    serde_json::from_str(WRITERLANGUAGES_MANIFEST_JSON).expect("manifest json")
}
