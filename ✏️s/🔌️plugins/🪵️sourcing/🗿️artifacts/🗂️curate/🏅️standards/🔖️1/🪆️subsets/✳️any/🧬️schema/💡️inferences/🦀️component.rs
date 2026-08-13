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

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this app's stock (its `"catalogue.kinds"`-shaped rows) into the `s/plugin/puzzle` 3d catalog
/// shape (`objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`/`kindCompatibility` — see
/// `block_3d::puzzle3d_catalog_fragment`, the sibling producer this mirrors byte-for-byte in shape), the
/// seam puzzle imports through its `Kit×Type` `kit:in` media port. Sourcing's `ObjectKind` carries no
/// mesh URL (geometry is a procedural `GeometryRecipe`, not an asset reference) or vortex/attachment
/// data, so every row's `meshUrl` is `null` and `vortices` is empty — puzzle's importer treats a missing
/// mesh as "no visual representation yet", not an error.
pub fn sourcing_catalog_fragment(document: &CurateSnapshot) -> Value {
    let object_kinds: Vec<Value> = document.stock.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.name, "meshUrl": Value::Null, "vortices": Vec::<Value>::new() })).collect();
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

    //#region 🧪️PuzzleCatalogFragment
    fn sample_document() -> CurateSnapshot {
        CurateSnapshot { stock: crate::artifacts::curate::schema::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() }
    }

    #[test]
    fn sourcing_catalog_fragment_maps_stock_into_the_puzzle3d_kit_catalog_shape() {
        let document = sample_document();
        let fragment = sourcing_catalog_fragment(&document);
        assert_eq!(fragment["schema"], "manifest");
        let object_kinds = fragment["objectKinds"].as_array().expect("objectKinds array");
        assert_eq!(object_kinds.len(), document.stock.len());
        assert_eq!(object_kinds[0]["id"], document.stock[0].id);
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
