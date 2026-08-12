//! 💡️ ZipInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃entries/`, a real census over the
//! archive's decompressed `entries` — the natural container-level facet a ZIP central directory
//! already exists to answer).

use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::entries::{compute_zip_entries, ZipEntries};

//#region 🔖️Inference
/// 💡️ Everything inferable from a zip snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃entries/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip.inference")]
pub struct ZipInference {
    #[state(inferred)]
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
impl ArtifactInferrer for crate::artifacts::zip::standards::v2_0::subsets::any::schema::ZipBuilder {
    type Snapshot = ZipSnapshot;
    type Inference = ZipInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.zip.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `zip_artifact_schema_descriptor`'s registration.
pub fn zip_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.zip.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[test]
    fn inference_determinism_law() {
        let snapshot = ZipSnapshot::default();
        assert_eq!(ZipInference::infer(&snapshot), ZipInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(ZipInference::infer(&ZipSnapshot::default()), ZipInference::default());
    }
}
//#endregion 🧪️Tests
