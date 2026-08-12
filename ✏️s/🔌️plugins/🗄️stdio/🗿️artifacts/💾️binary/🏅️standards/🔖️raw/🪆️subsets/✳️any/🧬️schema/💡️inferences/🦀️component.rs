//! 💡️ BinaryInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📏extent/`). Deliberately NOT a
//! `🗃entries` census like `🎒️zip`'s: `BinarySnapshot` is a single opaque `bytes: Vec<u8>` blob —
//! genuinely the most honest, minimal container in this entire family, with no entry/chunk/box
//! structure of any kind to census. Forcing an "entries" shape onto it would fabricate structure
//! this format doesn't have; this facet instead reports exactly what an opaque byte blob honestly
//! supports — its real extent (byte length, emptiness) plus a real content digest.

use crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::extent::{compute_binary_extent, BinaryExtent};

//#region 🔖️Inference
/// 💡️ Everything inferable from a binary snapshot. One field per named inference under
/// `💡️inferences/` (currently: `extent`, backed by the `📏extent/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary.inference")]
pub struct BinaryInference {
    #[state(inferred)]
    pub extent: BinaryExtent,
}

impl protocol::Inference<BinarySnapshot> for BinaryInference {
    fn infer(snapshot: &BinarySnapshot) -> Self {
        Self { extent: compute_binary_extent(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `BinarySnapshot::default()`'s `bytes` ever stop being empty.
impl Default for BinaryInference {
    fn default() -> Self {
        <Self as protocol::Inference<BinarySnapshot>>::infer(&BinarySnapshot::default())
    }
}

impl protocol::InferenceSpec<BinarySnapshot> for BinaryInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.binary.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.binary.inference.extent", reads: &["bytes"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `extent` is a single length read plus a fold over `bytes`,
/// already O(n) in byte count with no honest per-entity incremental decomposition (there is no
/// "entity" to decompose in an opaque byte blob) — the default `infer_cached` passthrough is
/// exact.
impl ArtifactInferrer for crate::artifacts::binary::standards::v_raw::subsets::any::schema::BinaryBuilder {
    type Snapshot = BinarySnapshot;
    type Inference = BinaryInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.binary.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `binary_artifact_schema_descriptor`'s registration.
pub fn binary_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.binary.inference",
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
        let snapshot = BinarySnapshot::default();
        assert_eq!(BinaryInference::infer(&snapshot), BinaryInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(BinaryInference::infer(&BinarySnapshot::default()), BinaryInference::default());
    }
}
//#endregion 🧪️Tests
