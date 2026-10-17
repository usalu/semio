//! 🧳️ The persisted `stdio.gltf` snapshot root: the one type that owns the sqlite, pack and text snapshot codecs, kept apart from the element types it aggregates.

use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfDocument, GltfSourceForm};
use framework_schema::ArtifactSchema;
#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GLTF_DOCUMENT_SCHEMA: &str = "stdio.gltf";

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.gltf` snapshot: the fully typed [`GltfDocument`] plus `buffers`: the
/// resolved raw bytes for each `document.buffers[i]` (index-aligned), since a `.glb`-sourced
/// buffer may have no `uri` at all and its bytes must live somewhere other than the JSON.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub document: GltfDocument,
    #[state(artifact)]
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub buffers: Vec<Vec<u8>>,
    #[state(artifact)]
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub source_form: GltfSourceForm,
}

impl Default for GltfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document: GltfDocument::default(), buffers: Vec::new(), source_form: GltfSourceForm::Json }
    }
}
//#endregion 🔖️Snapshot
