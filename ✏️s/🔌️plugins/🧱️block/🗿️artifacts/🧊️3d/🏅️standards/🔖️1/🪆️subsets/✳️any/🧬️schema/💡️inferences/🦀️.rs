//! 💡️ Block3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Unlike puzzle3d, block3d has no parent/child object graph — it is a single kind DEFINITION (one
//! `ObjectKind` plus a catalog of rim `Block3dVortexTemplate`s), so the honest whole-snapshot
//! inference here is a geometric bounding box + vertex count over the vortex templates' rim
//! positions, expressed as a plain `Inference` impl (no per-entity `InferredField` caching needed —
//! there is nothing to invalidate incrementally over a flat template list).

use crate::Block3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use dsl::json;
use dsl::os_pack::json::{array, Value};
use super::bounds::{compute_block3d_bounds};
//#region 🔖️Inference
/// 💡️ Everything inferable from a block3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.block3d.inference")]
pub struct Block3dInference {
    #[derived]
    pub bounds: Block3dBounds,
}

impl protocol::Inference<Block3dSnapshot> for Block3dInference {
    fn infer(snapshot: &Block3dSnapshot) -> Self {
        Self { bounds: compute_block3d_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<Block3dSnapshot> for Block3dInference {
    fn inference_schema_id() -> &'static str {
        "s.block.block3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block3d.inference.bounds", reads: &["vortices"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Block3dBuilder {
    type Snapshot = Block3dSnapshot;
    type Inference = Block3dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️PuzzleCatalogFragment
/// 🌐️ Resolves the active representation's mesh url — the first representation whose `tags` all
/// appear in `wanted_tags`, or the first representation overall, or `None` for an empty catalog.
pub fn resolve_active_mesh_url<'a>(definition: &'a Block3dSnapshot, wanted_tags: &[&str]) -> Option<&'a str> {
    definition
        .representations
        .iter()
        .find(|representation| wanted_tags.iter().all(|tag| representation.tags.iter().any(|other| other == tag)))
        .or_else(|| definition.representations.first())
        .and_then(|representation| representation.mesh_url.as_deref())
}

/// 🌉️ Maps this `ObjectKind` definition into the `s/plugin/puzzle` 3d catalog shape (`objectKinds`/
/// `vortexKinds`/`cableKinds`/`attractionKinds` — see `Puzzle3dKindCatalogs`), the seam puzzle imports
/// through its `Kit×Type` media port. The active representation's mesh (first row, or the first
/// matching `wanted_tags`) becomes the catalog row's `meshUrl`.
pub fn puzzle3d_catalog_fragment(definition: &Block3dSnapshot, wanted_tags: &[&str]) -> Value {
    let vec3 = |v: [f64; 3]| array(v.iter().map(|c| Value::from(*c)));
    let vortices: Vec<Value> = definition.vortices.iter().map(|vortex| json!({ "id": vortex.id.as_str(), "vortexKind": vortex.vortex_kind.as_str(), "position": vec3(vortex.position), "direction": vec3(vortex.direction), "radius": vortex.radius })).collect();
    let object_kind = json!({
        "id": definition.object_kind.id.as_str(),
        "name": definition.object_kind.name.as_str(),
        "label": definition.object_kind.label.as_str(),
        "meshUrl": resolve_active_mesh_url(definition, wanted_tags),
        "vortices": vortices,
    });
    let vortex_kinds: Vec<Value> = crate::vortex_kinds_of(definition).iter().map(|kind| json!({ "id": kind.id.as_str(), "name": kind.name.as_str(), "label": kind.label.as_str(), "color": kind.color.as_str(), "defaultCableKind": kind.default_cable_kind.as_str() })).collect();
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| json!({ "source": rule.source.as_str(), "target": rule.target.as_str(), "bidirectional": rule.bidirectional })).collect();
    json!({
        "schema": "manifest",
        "objectKinds": [object_kind],
        "vortexKinds": vortex_kinds,
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": kind_compatibility,
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block3d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block3d_artifact_schema_descriptor`'s registration.
pub fn block3d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.block.block3d.inference",
        inference: ::semio_framework_schema::FacetLeaves {
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::Block3dBounds;
//#endregion 🔁️Re-exports
