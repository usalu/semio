use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🧫️Snapshot
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[dsl(extension = "testkit-macro-cfg")]
pub(crate) struct TestConfig {
    pub(crate) selected: Option<String>,
}

impl store::ArtifactDsl for TestConfig {
    const EXTENSION: &'static str = "testkit-macro-cfg";
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl ArtifactPack for TestConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

impl store::ConfigRecord for TestConfig {}
//#endregion 🧫️Snapshot

//#region 🧬️Mutations
#[path="🧬️mutations/🦀️.rs"] pub mod mutations;
pub(crate) use mutations::{ChangeTestConfigSelection,TestConfigMutation};
//#endregion 🧬️Mutations

//#region 🔺️Diff
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(tag="state",content="value",rename_all="camelCase")]
pub(crate) enum TestConfigDiff { Identity, Clear, Set(String) }
impl Default for TestConfigDiff { fn default()->Self{Self::Identity} }
impl MutationDiff<TestConfig> for TestConfigDiff { fn apply(&self,base:&TestConfig)->protocol::MutationApplyResult<TestConfig>{Ok(match self{Self::Identity=>base.clone(),Self::Clear=>TestConfig{selected:None},Self::Set(value)=>TestConfig{selected:Some(value.clone())}})} fn absorb(&mut self,other:Self){if !matches!(other,Self::Identity){*self=other;}} }
//#endregion 🔺️Diff
