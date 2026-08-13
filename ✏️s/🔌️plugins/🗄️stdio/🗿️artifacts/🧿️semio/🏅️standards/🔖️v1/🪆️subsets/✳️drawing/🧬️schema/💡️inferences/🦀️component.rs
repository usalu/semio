//! 💡️ SemioDrawing inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING's pattern,
//! authored here per DKM #2550 since IIF explicitly excluded `✳️brep`/`✳️drawing`/`✳️mesh` and
//! deferred them to DKM). Directory shape mirrors `🧬️mutations/`: this file is the family-root
//! assembly (never `mod`'s/includes the slug dirs directly — `📦️glue.rs` is the sole mounting
//! mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `🎛flattened-scene/`).
//!
//! `flattenedScene` is the direct schema-level replacement for the framework's own (deleted-by-
//! this-ticket) `◻2d/🗄️store/🦀️component.rs` `DrawingEngine::compute`/`DrawingStore::flatten_
//! handle` — world transforms composed down through nested `Group`s, plus each entity's style
//! reference resolved into the real value. No other field of `SemioDrawingSnapshot` has an honest
//! dependency chain to author yet (`canvas` and `styles` themselves are already fully persisted,
//! not derived).

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::flattened_scene::{DrawFlattenedScene, FlattenedNode};

//#region 🔖️Inference
/// 💡️ Everything inferable from a drawing snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flattenedScene`, backed by the `🎛flattened-scene/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing.inference")]
pub struct SemioDrawingInference {
    #[state(inferred)]
    pub flattened_scene: BTreeMap<String, FlattenedNode>,
}

impl protocol::Inference<SemioDrawingSnapshot> for SemioDrawingInference {
    fn infer(snapshot: &SemioDrawingSnapshot) -> Self {
        Self { flattened_scene: store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(snapshot, None).into_iter().collect() }
    }
}

impl protocol::InferenceSpec<SemioDrawingSnapshot> for SemioDrawingInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.drawing.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.drawing.inference.flattenedScene", reads: &["layers", "styles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::drawing::schema::SemioDrawingBuilder {
    type Snapshot = SemioDrawingSnapshot;
    type Inference = SemioDrawingInference;

    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = session;
        let flattened_scene = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(snapshot, Some(cache)).into_iter().collect();
        SemioDrawingInference { flattened_scene }
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.drawing.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `semio_drawing_artifact_schema_descriptor`'s own
/// registration (`../🦀️component.rs`).
pub fn semio_drawing_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.drawing.inference",
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
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioTransform};
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, PathSegment, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
    use protocol::Inference;

    fn fixture() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
            canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
            styles: Vec::new(),
            layers: vec![DrawLayer {
                id: "l0".into(),
                name: "base".into(),
                visible: true,
                root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None }] },
            }],
        }
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = fixture();
        assert_eq!(SemioDrawingInference::infer(&snapshot), SemioDrawingInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(SemioDrawingInference::infer(&SemioDrawingSnapshot::default()), SemioDrawingInference::default());
    }

    #[test]
    fn inference_matches_direct_infer_field_call() {
        let snapshot = fixture();
        let inferred = SemioDrawingInference::infer(&snapshot);
        let direct = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, None);
        for (key, value) in &direct {
            assert_eq!(inferred.flattened_scene.get(key), Some(value), "inference must match infer_field exactly for {key}");
        }
    }
}
//#endregion 🧪️Tests
