//! 💡️ Block2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Like block3d, block2d has no parent/child object graph — it is a single kind DEFINITION (one
//! `NodeKind` plus a catalog of rim `Block2dHandleTemplate`s placed by polar `angle`/`radius`), so
//! the honest whole-snapshot inference here is a geometric bounding box + vertex count over the
//! handle templates' rim positions (converted from polar to cartesian), expressed as a plain
//! `Inference` impl (no per-entity `InferredField` caching needed).

use crate::Block2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use dsl::json;
use dsl::os_pack::json::Value;

use super::bounds::{compute_block2d_bounds};
//#region 🔖️Inference
/// 💡️ Everything inferable from a block2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.block2d.inference")]
pub struct Block2dInference {
    #[derived]
    pub bounds: Block2dBounds,
}

impl protocol::Inference<Block2dSnapshot> for Block2dInference {
    fn infer(snapshot: &Block2dSnapshot) -> Self {
        Self { bounds: compute_block2d_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<Block2dSnapshot> for Block2dInference {
    fn inference_schema_id() -> &'static str {
        "s.block.block2d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block2d.inference.bounds", reads: &["handles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Block2dBuilder {
    type Snapshot = Block2dSnapshot;
    type Inference = Block2dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this `NodeKind` definition into the `s/plugin/puzzle` 2d manifest shape (`portKinds`/
/// `wireKinds`/`edgeKinds`/`nodeKinds`/`kindCompatibility` — see
/// `s/plugin/puzzle/app/2d/manifest/🔣️.json`), the seam puzzle imports through
/// its `Kit×Type` media port. Block owns no wire/edge-kind rows (`AGENTS.md`: referenced by
/// `default_wire_kind` only), so those arrays stay empty here — a merge keeps the puzzle manifest's
/// existing rows.
pub fn puzzle2d_manifest_fragment(definition: &Block2dSnapshot) -> Value {
    let port_kinds: Vec<Value> = definition.handle_kinds.iter().map(|kind| json!({ "id": kind.id.as_str(), "name": kind.name.as_str(), "presentation": { "color": kind.color.as_str(), "defaultWireKind": kind.default_wire_kind.as_str() } })).collect();
    let handles: Vec<Value> = definition.handles.iter().map(|handle| json!({ "handleKind": handle.handle_kind.as_str(), "angle": handle.angle, "radius": handle.radius })).collect();
    let node_kind = json!({
        "id": definition.node_kind.id.as_str(),
        "name": definition.node_kind.name.as_str(),
        "presentation": {
            "meshUrl": null,
            "handles": handles,
        },
    });
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| json!({ "bidirectional": rule.bidirectional, "specificity": "handle", "source": rule.source.as_str(), "target": rule.target.as_str() })).collect();
    json!({
        "schema": "manifest",
        "id": definition.node_kind.id.as_str(),
        "name": definition.node_kind.name.as_str(),
        "axes": { "portModel": "ported", "directedness": "directed" },
        "portKinds": port_kinds,
        "wireKinds": Vec::<Value>::new(),
        "edgeKinds": Vec::<Value>::new(),
        "nodeKinds": [node_kind],
        "kindCompatibility": kind_compatibility,
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block2d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block2d_artifact_schema_descriptor`'s registration.
pub fn block2d_artifact_inference_descriptor() -> ::semio_framework_schema::ArtifactInferenceDescriptor {
    ::semio_framework_schema::ArtifactInferenceDescriptor {
        id: "s.block.block2d.inference",
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
pub use super::bounds::Block2dBounds;
//#endregion 🔁️Re-exports
