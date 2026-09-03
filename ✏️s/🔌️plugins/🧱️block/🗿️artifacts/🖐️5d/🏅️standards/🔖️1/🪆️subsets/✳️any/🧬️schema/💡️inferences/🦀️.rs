//! 💡️ Block5d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! Like block2d/block3d, block5d has no parent/child object graph — it is a single kind DEFINITION
//! (one `PartKind` plus a catalog of rim `Block5dGripTemplate`s, each carrying both a 2d polar
//! placement and a 3d cartesian placement — see `Block5dGripTemplate`'s doc). The honest
//! whole-snapshot inference here mirrors block3d: a 3d bounding box + vertex count over the grip
//! templates' `position`/`radius3d` fields (the part's 3d-projection rim geometry), expressed as a
//! plain `Inference` impl (no per-entity `InferredField` caching needed).

use crate::artifacts::block5d::Block5dSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use dsl::json;
use dsl::os_pack::json::{array, Value};

use super::bounds::{compute_block5d_bounds, Block5dBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a block5d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.block5d.inference")]
pub struct Block5dInference {
    #[derived]
    pub bounds: Block5dBounds,
}

impl protocol::Inference<Block5dSnapshot> for Block5dInference {
    async fn infer(snapshot: &Block5dSnapshot) -> Self {
        Self { bounds: compute_block5d_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<Block5dSnapshot> for Block5dInference {
    async fn inference_schema_id() -> &'static str {
        "s.block.block5d.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.block.block5d.inference.bounds", reads: &["grips"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::block5d::standards::v1::subsets::any::schema::Block5dBuilder {
    type Snapshot = Block5dSnapshot;
    type Inference = Block5dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this `PartKind` definition into the `s/plugin/puzzle` 5d catalog shape
/// (`Puzzle5dKindCatalogs`: `parts`/`grips`/`fasteners`/`ropes`), the seam puzzle imports through its
/// `Kit×Type` media port. Block owns no fastener/rope-kind rows, so those arrays stay empty here.
pub async fn puzzle5d_catalog_fragment(definition: &Block5dSnapshot) -> Value {
    let vec3 = |v: [f64; 3]| array(v.iter().map(|c| Value::from(*c)));
    let grips: Vec<Value> = definition
        .grips
        .iter()
        .map(|grip| {
            json!({
                "gripKind": grip.grip_kind.as_str(),
                "2d": { "angle": grip.angle, "gripKind": grip.grip_kind.as_str(), "radius": grip.radius_2d },
                "3d": { "position": vec3(grip.position), "direction": vec3(grip.direction), "radius": grip.radius_3d },
            })
        })
        .collect();
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.clone());
    let part = json!({
        "id": definition.part_kind.id.as_str(),
        "name": definition.part_kind.name.as_str(),
        "label": definition.part_kind.label.as_str(),
        "meshUrl": mesh_url,
        "grips": grips,
    });
    let grip_kinds: Vec<Value> = definition.grip_kinds.iter().map(|kind| json!({ "id": kind.id.as_str(), "name": kind.name.as_str(), "label": kind.label.as_str(), "color": kind.color.as_str(), "defaultRopeKind": kind.default_rope_kind.as_str() })).collect();
    json!({
        "schema": "manifest",
        "parts": [part],
        "grips": grip_kinds,
        "fasteners": Vec::<Value>::new(),
        "ropes": Vec::<Value>::new(),
        "kindCompatibility": definition.compatibility.iter().map(|rule| json!({ "source": rule.source.as_str(), "target": rule.target.as_str(), "bidirectional": rule.bidirectional })).collect::<Vec<Value>>(),
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Descriptor
/// 💡️ Registers `s.block.block5d.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `block5d_artifact_schema_descriptor`'s registration.
pub fn block5d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.block.block5d.inference",
        inference: schema::FacetLeaves {
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
mod tests {
    use super::*;
    use crate::artifacts::block5d::{Block5dGripTemplate, BLOCK_5D_SCHEMA};
    use crate::BlockKindIdentity;
    use protocol::Inference;

    //#region 🧸️Fixtures
    async fn grip(id: &str, position: [f64; 3], radius_3d: f64) -> Block5dGripTemplate {
        Block5dGripTemplate { id: id.into(), grip_kind: "rope".into(), angle: 0.0, radius_2d: 0.0, position, direction: [0.0, 1.0, 0.0], radius_3d }
    }

    async fn snapshot_with_grips(grips: Vec<Block5dGripTemplate>) -> Block5dSnapshot {
        Block5dSnapshot { part_kind: BlockKindIdentity { id: "capsule".into(), name: "capsule".into(), label: "Capsule".into(), ..Default::default() }, grips, ..Block5dSnapshot::default() }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = snapshot_with_grips(vec![grip("g0", [1.0, 2.0, 3.0], 0.5), grip("g1", [-1.0, 0.0, 4.0], 0.25)]);
        assert_eq!(Block5dInference::infer(&snapshot), Block5dInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(Block5dInference::infer(&Block5dSnapshot::default()), Block5dInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn bounds_match_grip_positions_inflated_by_radius_3d() {
        let snapshot = snapshot_with_grips(vec![grip("g0", [1.0, 2.0, 3.0], 0.5), grip("g1", [-1.0, 0.0, 4.0], 0.25)]);
        let inferred = Block5dInference::infer(&snapshot);
        let bounds = inferred.bounds.bounding_box.expect("non-empty grips produce a bounding box");
        assert_eq!(bounds.min, [-1.25, -0.25, 2.5]);
        assert_eq!(bounds.max, [1.5, 2.5, 4.25]);
        assert_eq!(inferred.bounds.vertex_count, 2);
    }
    //#endregion 🧪️InferenceLaws

    //#region 🧪️PuzzleCatalogFragment
    #[semio_framework_async_macros::async_test]
    async fn puzzle5d_catalog_fragment_maps_grips() {
        let mut definition = Block5dSnapshot { schema: BLOCK_5D_SCHEMA.into(), part_kind: BlockKindIdentity { id: "left".into(), name: "left".into(), label: "Left".into(), ..Default::default() }, ..Block5dSnapshot::default() };
        definition.grips.push(Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -1.57, radius_2d: 0.36, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 });
        let fragment = puzzle5d_catalog_fragment(&definition);
        assert_eq!(fragment["parts"][0]["id"], "left");
        assert_eq!(fragment["parts"][0]["grips"][0]["gripKind"], "b-l");
    }
    //#endregion 🧪️PuzzleCatalogFragment
}
//#endregion 🧪️Tests
