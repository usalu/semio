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

use super::entries::compute_curation_entries;

pub use super::entries::CurationEntries;

//#region 🔖️Inference
/// 💡️ Everything inferable from a curation snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃️entries/` slug dir).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curation.inference")]
pub struct CurationInference {
    #[derived]
    pub entries: CurationEntries,
}

impl protocol::Inference<CurationSnapshot> for CurationInference {
    fn infer(snapshot: &CurationSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { entries: compute_curation_entries(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `CurationSnapshot::default()`'s `stock`/`curated` ever stop being empty. Same "match `infer` of
/// the real default, don't derive structurally" trick `AddInference` uses in
/// `📡️spr/🎮️command/🦀️.rs`.
impl Default for CurationInference {
    fn default() -> Self {
        let snapshot = &CurationSnapshot::default();

        Self { entries: compute_curation_entries(snapshot) }
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

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this app's stock (its `"catalogue.kinds"`-shaped rows) into the `s/plugin/puzzle` 3d catalog
/// shape (`objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`/`kindCompatibility` — see
/// `block_3d::puzzle3d_catalog_fragment`, the sibling producer this mirrors byte-for-byte in shape), the
/// seam puzzle imports through its `Kit×Type` `kit:in` media port. Sourcing's `ObjectKind` carries no
/// `GeometryRecipe::Glb` rows carry the same `/mesh/…` routes as puzzle 3d and cad; procedural recipes
/// leave `meshUrl` null. Sourcing has no vortex templates, so `vortices` stays empty.
pub fn sourcing_catalog_fragment(document: &CurationSnapshot) -> semio_framework_value::DslValue {
    let object_kinds: Vec<semio_framework_value::DslValue> = crate::stock_of(document)
        .iter()
        .map(|kind| {
            let mesh_url = crate::schema::geometry_mesh_url(&kind.geometry).map(|url| semio_framework_value::DslValue::String(url.to_string())).unwrap_or(semio_framework_value::DslValue::Null);
            semio_framework_value::DslValue::object([
                ("id".to_string(), semio_framework_value::DslValue::String(kind.id.clone())),
                ("name".to_string(), semio_framework_value::DslValue::String(kind.name.clone())),
                ("label".to_string(), semio_framework_value::DslValue::String(kind.name.clone())),
                ("meshUrl".to_string(), mesh_url),
                ("vortices".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
            ])
        })
        .collect();
    semio_framework_value::DslValue::object([
        ("schema".to_string(), semio_framework_value::DslValue::String("manifest".to_string())),
        ("objectKinds".to_string(), semio_framework_value::DslValue::Array(object_kinds)),
        ("vortexKinds".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
        ("cableKinds".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
        ("attractionKinds".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
        ("kindCompatibility".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
    ])
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.sourcing.curation.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `curation_artifact_schema_descriptor`'s registration.
pub fn curation_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.sourcing.curation.inference",
        inference: semio_framework_schema_registry::FacetLeaves {
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
