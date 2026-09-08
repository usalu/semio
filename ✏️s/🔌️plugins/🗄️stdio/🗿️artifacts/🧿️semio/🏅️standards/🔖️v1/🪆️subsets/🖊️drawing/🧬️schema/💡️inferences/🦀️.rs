//! 💡️ SemioDrawing inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING's pattern,
//! authored here per DKM #2550 since IIF explicitly excluded `🧊️brep`/`🖊️drawing`/`🔺️mesh` and
//! deferred them to DKM). Directory shape mirrors `🧬️mutations/`: this file is the family-root
//! assembly (never `mod`'s/includes the slug dirs directly — `🦀️.rs` is the sole mounting
//! mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `🎛️flattened-scene/`).
//!
//! `flattenedScene` is the direct schema-level replacement for the framework's own (deleted-by-
//! this-ticket) `◻️2d/🗄️store/🦀️.rs` `DrawingEngine::compute`/`DrawingStore::flatten_
//! handle` — world transforms composed down through nested `Group`s, plus each entity's style
//! reference resolved into the real value. No other field of `SemioDrawingSnapshot` has an honest
//! dependency chain to author yet (`canvas` and `styles` themselves are already fully persisted,
//! not derived).

use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use std::collections::BTreeMap;

use super::flattened_scene::{DrawFlattenedScene, FlattenedNode};

//#region 🔖️Inference
/// 💡️ Everything inferable from a drawing snapshot. One field per named inference under
/// `💡️inferences/` (currently: `flattenedScene`, backed by the `🎛️flattened-scene/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing.inference")]
pub struct SemioDrawingInference {
    #[derived]
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
impl ArtifactInferrer for crate::standards::v1::subsets::drawing::schema::SemioDrawingBuilder {
    type Snapshot = SemioDrawingSnapshot;
    type Inference = SemioDrawingInference;

    async fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = session;
        let flattened_scene = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(snapshot, Some(cache)).into_iter().collect();
        SemioDrawingInference { flattened_scene }
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.drawing.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `semio_drawing_artifact_schema_descriptor`'s own
/// registration (`../🦀️.rs`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_drawing_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.drawing.inference",
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
