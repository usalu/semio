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
use serde_json::{json, Value};

use super::entries::compute_curate_entries;

pub use super::entries::CurateEntries;

//#region 🔖️Inference
/// 💡️ Everything inferable from a curate snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃entries/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curate.inference")]
pub struct CurateInference {
    #[derived]
    pub entries: CurateEntries,
}

impl protocol::Inference<CurateSnapshot> for CurateInference {
    async fn infer(snapshot: &CurateSnapshot) -> Self {
        Self { entries: compute_curate_entries(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `CurateSnapshot::default()`'s `stock`/`curated` ever stop being empty. Same "match `infer` of
/// the real default, don't derive structurally" trick `AddInference` uses in
/// `📡️spr/🎮️command/🦀️component.rs`.
impl Default for CurateInference {
    async fn default() -> Self {
        <Self as protocol::Inference<CurateSnapshot>>::infer(&CurateSnapshot::default())
    }
}

impl protocol::InferenceSpec<CurateSnapshot> for CurateInference {
    async fn inference_schema_id() -> &'static str {
        "s.sourcing.curate.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.sourcing.curate.inference.entries", reads: &["catalog", "stockExtra", "curated"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧭️ Trivial zero-sized marker — NOT `semio_framework_plugin::app::SnapshotBuilder<CurateSnapshot,
/// SourcingMutation>` (the `Construction` type alias `🧬️schema/🦀️component.rs` now uses). Targeting
/// `SnapshotBuilder` directly is a genuine orphan-rule violation (E0117): it is a foreign,
/// non-`#[fundamental]` generic struct, so `impl ArtifactInferrer for SnapshotBuilder<Local, Local>`
/// is illegal regardless of the type parameters being local (confirmed by compiling it — see
/// `📓️w4-sourcing-report.md` `## recipeGaps`, matching `📓️w4-sequence-report.md`'s identical
/// finding). `ArtifactInferrer::infer` takes `&Self::Snapshot`, never `&self`, so the impl target is
/// a pure type-level anchor with zero live callers repo-wide — a local marker struct is sufficient.
pub struct CurateInferrer;
impl ArtifactInferrer for CurateInferrer {
    type Snapshot = CurateSnapshot;
    type Inference = CurateInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this app's stock (its `"catalogue.kinds"`-shaped rows) into the `s/plugin/puzzle` 3d catalog
/// shape (`objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`/`kindCompatibility` — see
/// `block_3d::puzzle3d_catalog_fragment`, the sibling producer this mirrors byte-for-byte in shape), the
/// seam puzzle imports through its `Kit×Type` `kit:in` media port. Sourcing's `ObjectKind` carries no
/// mesh URL (geometry is a procedural `GeometryRecipe`, not an asset reference) or vortex/attachment
/// data, so every row's `meshUrl` is `null` and `vortices` is empty — puzzle's importer treats a missing
/// mesh as "no visual representation yet", not an error.
pub async fn sourcing_catalog_fragment(document: &CurateSnapshot) -> Value {
    let object_kinds: Vec<Value> = crate::artifacts::curate::stock_of(document).iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.name, "meshUrl": Value::Null, "vortices": Vec::<Value>::new() })).collect();
    json!({
        "schema": "manifest",
        "objectKinds": object_kinds,
        "vortexKinds": Vec::<Value>::new(),
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": Vec::<Value>::new(),
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.sourcing.curate.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `curate_artifact_schema_descriptor`'s registration.
pub async fn curate_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
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

    async fn picked_snapshot() -> CurateSnapshot {
        CurateSnapshot {
            curated: vec![CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 4 }, CuratedItem { object_id: "window-fixed-150x150".into(), count: 6 }],
            ..CurateSnapshot::default()
        }
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = picked_snapshot();
        assert_eq!(CurateInference::infer(&snapshot), CurateInference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(CurateInference::infer(&CurateSnapshot::default()), CurateInference::default());
    }

    #[test]
    async fn entries_counts_curated_lines_and_total_quantity() {
        let inferred = CurateInference::infer(&picked_snapshot());
        assert_eq!(inferred.entries.entry_count, 2);
        assert_eq!(inferred.entries.total_count, 10);
    }

    //#region 🧪️PuzzleCatalogFragment
    async fn sample_document() -> CurateSnapshot {
        crate::artifacts::curate::curate_snapshot_from_stock(crate::artifacts::curate::schema::demo_stock(), Vec::new())
    }

    #[test]
    async fn sourcing_catalog_fragment_maps_stock_into_the_puzzle3d_kit_catalog_shape() {
        let document = sample_document();
        let stock = crate::artifacts::curate::stock_of(&document);
        let fragment = sourcing_catalog_fragment(&document);
        assert_eq!(fragment["schema"], "manifest");
        let object_kinds = fragment["objectKinds"].as_array().expect("objectKinds array");
        assert_eq!(object_kinds.len(), stock.len());
        assert_eq!(object_kinds[0]["id"], stock[0].id);
        assert_eq!(object_kinds[0]["meshUrl"], Value::Null);
        assert!(object_kinds[0]["vortices"].as_array().unwrap().is_empty());
        assert!(fragment["vortexKinds"].as_array().unwrap().is_empty());
        assert!(fragment["cableKinds"].as_array().unwrap().is_empty());
        assert!(fragment["attractionKinds"].as_array().unwrap().is_empty());
        assert!(fragment["kindCompatibility"].as_array().unwrap().is_empty());
    }
    //#endregion 🧪️PuzzleCatalogFragment
}
//#endregion 🧪️Tests
