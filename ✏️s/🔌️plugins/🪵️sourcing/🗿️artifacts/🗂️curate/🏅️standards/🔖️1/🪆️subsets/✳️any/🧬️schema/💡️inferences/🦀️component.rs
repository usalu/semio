//! 💡️ Curate inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃entries/`).
//!
//! The curate snapshot is `stock: Vec<ObjectKind>` (the catalog) and `curated: Vec<CuratedItem>`
//! (the picked bill of quantities, each `{ objectId, count }`) — no graph, no geometry, so the
//! honest whole-snapshot derivation is a real census over those two lists.

use crate::artifacts::curate::CurateSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::entries::compute_curate_entries;

pub use super::entries::CurateEntries;

//#region 🔖️Inference
/// 💡️ Everything inferable from a curate snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃entries/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curate.inference")]
pub struct CurateInference {
    #[state(inferred)]
    pub entries: CurateEntries,
}

impl protocol::Inference<CurateSnapshot> for CurateInference {
    fn infer(snapshot: &CurateSnapshot) -> Self {
        Self { entries: compute_curate_entries(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `CurateSnapshot::default()`'s `stock`/`curated` ever stop being empty. Same "match `infer` of
/// the real default, don't derive structurally" trick `AddInference` uses in
/// `📡️spr/🎮️command/🦀️component.rs`.
impl Default for CurateInference {
    fn default() -> Self {
        <Self as protocol::Inference<CurateSnapshot>>::infer(&CurateSnapshot::default())
    }
}

impl protocol::InferenceSpec<CurateSnapshot> for CurateInference {
    fn inference_schema_id() -> &'static str {
        "s.sourcing.curate.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.sourcing.curate.inference.entries", reads: &["stock", "curated"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::curate::standards::v1::subsets::any::schema::CurateBuilder {
    type Snapshot = CurateSnapshot;
    type Inference = CurateInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.sourcing.curate.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `curate_artifact_schema_descriptor`'s registration.
pub fn curate_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.sourcing.curate.inference",
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
    use crate::artifacts::curate::CuratedItem;
    use protocol::Inference;

    fn picked_snapshot() -> CurateSnapshot {
        CurateSnapshot {
            curated: vec![CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 4 }, CuratedItem { object_id: "window-fixed-150x150".into(), count: 6 }],
            ..CurateSnapshot::default()
        }
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = picked_snapshot();
        assert_eq!(CurateInference::infer(&snapshot), CurateInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(CurateInference::infer(&CurateSnapshot::default()), CurateInference::default());
    }

    #[test]
    fn entries_counts_curated_lines_and_total_quantity() {
        let inferred = CurateInference::infer(&picked_snapshot());
        assert_eq!(inferred.entries.entry_count, 2);
        assert_eq!(inferred.entries.total_count, 10);
    }
}
//#endregion 🧪️Tests
