//! 💡️ Curation inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃️entries/`).
//!
//! The curation snapshot is `stock: Vec<ObjectKind>` (the catalog) and `curated: Vec<CuratedItem>`
//! (the picked bill of quantities, each `{ objectId, count }`) — no graph, no geometry, so the
//! honest whole-snapshot derivation is a real census over those two lists.

use crate::CurationSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::entries::compute_curation_entries;

pub use super::entries::CurationEntries;

//#region 🔖️Inference
/// 💡️ Everything inferable from a curation snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃️entries/` slug dir).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curation.inference")]
pub struct CurationInference {
    #[derived]
    pub entries: CurationEntries,
}

impl protocol::Inference<CurationSnapshot> for CurationInference {
    fn infer(snapshot: &CurationSnapshot) -> Self {
        Self { entries: compute_curation_entries(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `CurationSnapshot::default()`'s `stock`/`curated` ever stop being empty. Same "match `infer` of
/// the real default, don't derive structurally" trick `AddInference` uses in
/// `📡️spr/🎮️command/🦀️.rs`.
impl Default for CurationInference {
    fn default() -> Self {
        <Self as protocol::Inference<CurationSnapshot>>::infer(&CurationSnapshot::default())
    }
}

impl protocol::InferenceSpec<CurationSnapshot> for CurationInference {
    fn inference_schema_id() -> &'static str {
        "s.sourcing.curation.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.sourcing.curation.inference.entries", reads: &["catalog", "stockExtra", "curated"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🧭️ Trivial zero-sized marker — NOT `semio_framework_plugin::app::SnapshotBuilder<CurationSnapshot,
/// SourcingMutation>` (the `Construction` type alias `🧬️schema/🦀️component.rs` now uses). Targeting
/// `SnapshotBuilder` directly is a genuine orphan-rule violation (E0117): it is a foreign,
/// non-`#[fundamental]` generic struct, so `impl ArtifactInferrer for SnapshotBuilder<Local, Local>`
/// is illegal regardless of the type parameters being local (confirmed by compiling it — see
/// `📓️w4-sourcing-report.md` `## recipeGaps`, matching `📓️w4-sequence-report.md`'s identical
/// finding). `ArtifactInferrer::infer` takes `&Self::Snapshot`, never `&self`, so the impl target is
/// a pure type-level anchor with zero live callers repo-wide — a local marker struct is sufficient.
pub struct CurationInferrer;
impl ArtifactInferrer for CurationInferrer {
    type Snapshot = CurationSnapshot;
    type Inference = CurationInference;
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
pub fn sourcing_catalog_fragment(document: &CurationSnapshot) -> dsl::DslValue {
    let object_kinds: Vec<dsl::DslValue> = crate::stock_of(document)
        .iter()
        .map(|kind| {
            dsl::DslValue::object([
                ("id".to_string(), dsl::DslValue::String(kind.id.clone())),
                ("name".to_string(), dsl::DslValue::String(kind.name.clone())),
                ("label".to_string(), dsl::DslValue::String(kind.name.clone())),
                ("meshUrl".to_string(), dsl::DslValue::Null),
                ("vortices".to_string(), dsl::DslValue::Array(Vec::new())),
            ])
        })
        .collect();
    dsl::DslValue::object([
        ("schema".to_string(), dsl::DslValue::String("manifest".to_string())),
        ("objectKinds".to_string(), dsl::DslValue::Array(object_kinds)),
        ("vortexKinds".to_string(), dsl::DslValue::Array(Vec::new())),
        ("cableKinds".to_string(), dsl::DslValue::Array(Vec::new())),
        ("attractionKinds".to_string(), dsl::DslValue::Array(Vec::new())),
        ("kindCompatibility".to_string(), dsl::DslValue::Array(Vec::new())),
    ])
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.sourcing.curation.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `curation_artifact_schema_descriptor`'s registration.
pub fn curation_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.sourcing.curation.inference",
        inference: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
