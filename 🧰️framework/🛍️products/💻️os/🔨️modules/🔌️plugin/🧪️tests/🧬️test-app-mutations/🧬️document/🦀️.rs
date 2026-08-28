//#region 🧬️TestDocumentMutationRoot
//! 🧪️ Shared document state and direct mutation fixtures for the Plugin contract.

use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🧫️Snapshot
pub(crate) const MAXIMUM_CHILD_PROBE_BYTES: usize = 4 * 1024 * 1024;
pub(crate) static MAXIMUM_CHILD_CLONES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static MAXIMUM_CHILD_ENCODINGS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[dsl(extension = "testkit-macro")]
pub(crate) struct TestSnapshot {
    pub(crate) count: i32,
    pub(crate) label: String,
}

impl Clone for TestSnapshot {
    fn clone(&self) -> Self {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_CLONES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Self { count: self.count, label: self.label.clone() }
    }
}

impl store::ArtifactDsl for TestSnapshot {
    const EXTENSION: &'static str = "testkit-macro";
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

impl ArtifactPack for TestSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_ENCODINGS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}
//#endregion 🧫️Snapshot

//#region 🔺️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct TestDiff {
    pub(crate) count: Option<i32>,
    pub(crate) label: Option<String>,
}

impl protocol::MutationDiff<TestSnapshot> for TestDiff {
    fn apply(&self, snapshot: &TestSnapshot) -> protocol::MutationApplyResult<TestSnapshot> {
        Ok(TestSnapshot { count: self.count.unwrap_or(snapshot.count), label: self.label.clone().unwrap_or_else(|| snapshot.label.clone()) })
    }

    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
        if other.label.is_some() {
            self.label = other.label;
        }
    }
}
//#endregion 🔺️Diff

//#region 🧬️Mutations
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetCount, SetLabel, TestMutation};
//#endregion 🧬️Mutations
//#endregion 🧬️TestDocumentMutationRoot
