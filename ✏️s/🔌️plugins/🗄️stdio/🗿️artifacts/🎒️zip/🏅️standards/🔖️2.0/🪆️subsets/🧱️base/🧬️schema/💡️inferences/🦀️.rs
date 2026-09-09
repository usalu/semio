//! 💡️ ZipInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃entries/`, a real census over the
//! archive's decompressed `entries` — the natural container-level facet a ZIP central directory
//! already exists to answer).

use crate::standards::v2_0::subsets::base::schema::snapshot::ZipSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::entries::compute_zip_entries;
//#region 🔖️Inference
/// 💡️ Everything inferable from a zip snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃entries/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip.inference")]
pub struct ZipInference {
    #[derived]
    pub entries: ZipEntries,
}

impl protocol::Inference<ZipSnapshot> for ZipInference {
    fn infer(snapshot: &ZipSnapshot) -> Self {
        Self { entries: compute_zip_entries(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `ZipSnapshot::default()`'s `entries` ever stop being empty.
impl Default for ZipInference {
    fn default() -> Self {
        <Self as protocol::Inference<ZipSnapshot>>::infer(&ZipSnapshot::default())
    }
}

impl protocol::InferenceSpec<ZipSnapshot> for ZipInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.zip.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.zip.inference.entries", reads: &["entries"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `entries` is a single fold over `entries` (count, byte-size sum,
/// content digest), already O(n) in entry count with no honest per-entity incremental
/// decomposition worth a merkle dep-chain over one flat `Vec<ZipEntry>` — the default
/// `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::standards::v2_0::subsets::base::schema::ZipBuilder {
    type Snapshot = ZipSnapshot;
    type Inference = ZipInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.zip.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `zip_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn zip_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.stdio.zip.inference",
        inference: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::entries::ZipEntries;
//#endregion 🔁️Re-exports
