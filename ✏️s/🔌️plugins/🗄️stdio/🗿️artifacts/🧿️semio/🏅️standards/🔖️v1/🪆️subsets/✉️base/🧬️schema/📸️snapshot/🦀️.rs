//! 🧬️ SemioSnapshot — the envelope union over all 13 domain subsets — every semio artifact round-trips through this shape.
//! W2b closer: the 13 imports below now resolve to each subset's REAL, W2a/W2b-completed
//! snapshot type (brep/mesh/model/value/cad/drawing landed in W2a; document/image/video/audio/
//! animation/presentation/flow landed in W2b) — this file's own shape (an untagged-by-us
//! `SemioSubsetSnapshot` enum + the thin `SemioSnapshot{schema, subset}` wrapper) needed no
//! structural change from the W1b scaffold to pick that up, since only the referenced types'
//! internals grew, not their names/paths.

use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;

/// 🌐️ The envelope union of all 18 semio subset snapshot types (master plan: "SemioSnapshot =
/// tagged union of the 18" — `text` (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W2a) is the 14th arm,
/// `table`/`graph` (W2b) the 15th/16th, `object`/`kit` (W2c, the two COMPOSITE subsets) the
/// 17th/18th. Wrapped by `SemioSnapshot` below (a struct, not the enum
/// directly — keeps `#[derive(ArtifactSchema)]` on a proven struct shape; see the W1b manifest for
/// why).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "subset", rename_all = "camelCase")]
pub enum SemioSubsetSnapshot {
    Brep(SemioBrepSnapshot),
    Mesh(SemioMeshSnapshot),
    Model(SemioModelSnapshot),
    Value(SemioValueSnapshot),
    Document(SemioDocumentSnapshot),
    Cad(SemioCadSnapshot),
    Drawing(SemioDrawingSnapshot),
    Image(SemioImageSnapshot),
    Video(SemioVideoSnapshot),
    Audio(SemioAudioSnapshot),
    Animation(SemioAnimationSnapshot),
    Presentation(SemioPresentationSnapshot),
    Flow(SemioFlowSnapshot),
    Text(SemioTextSnapshot),
    Table(SemioTableSnapshot),
    Graph(SemioGraphSnapshot),
    Object(SemioObjectSnapshot),
    Kit(SemioKitSnapshot),
}

impl Default for SemioSubsetSnapshot {
    fn default() -> Self {
        SemioSubsetSnapshot::Brep(SemioBrepSnapshot::default())
    }
}

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio")]
pub struct SemioSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub subset: SemioSubsetSnapshot,
}

impl Default for SemioSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SubsetDispatch



//#endregion 🔖️SubsetDispatch

//#region 🔖️TextPrimitives








//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives








//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge











//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio` document — wraps `flow`'s own real demo snapshot (2 nodes, 1
/// edge, incl. a negative coordinate) so this facet's fixtures/conformance tests exercise a real,
/// already-nontrivial nested payload rather than an all-default stub. Single source of truth for
/// `📚️examples/🌐️envelope/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and this facet's
/// own conformance-law tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_semio_snapshot() -> SemioSnapshot {
    use crate::standards::v1::subsets::flow::schema::snapshot::demo_flow_snapshot;
    SemioSnapshot { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset: SemioSubsetSnapshot::Flow(demo_flow_snapshot()) }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests






semio_framework_value::artifact_retire_struct!(SemioSnapshot{schema,subset});
impl semio_framework_value::retirement::RetireOwned for SemioSubsetSnapshot{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;match self{
 Self::Brep(v)=>v.retirement(),Self::Mesh(v)=>v.retirement(),Self::Model(v)=>v.retirement(),Self::Value(v)=>v.retirement(),Self::Document(v)=>v.retirement(),Self::Cad(v)=>v.retirement(),Self::Drawing(v)=>v.retirement(),Self::Image(v)=>v.retirement(),Self::Video(v)=>v.retirement(),Self::Audio(v)=>v.retirement(),Self::Animation(v)=>v.retirement(),Self::Presentation(v)=>v.retirement(),Self::Flow(v)=>v.retirement(),Self::Text(v)=>v.retirement(),Self::Table(v)=>v.retirement(),Self::Graph(v)=>v.retirement(),Self::Object(v)=>v.retirement(),Self::Kit(v)=>v.retirement()
 }}
}

#[cfg(test)]
#[path="🧪️tests/🛫️native/🦀️.rs"]
pub(crate) mod native_output_tests;

/// 🏷️ The wire tag naming which of the 13 domain subsets is carried — shared by the text DSL's
/// `subset=<tag>` header line and used to select which subset's own REAL codec to delegate to.
/// `pub(crate)` (not private) since `💡️inferences/🏷️kind/🦀️.rs`
/// (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING) is a sibling
/// module, not a descendant, and needs this same dispatch as its own honest derivation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn subset_tag(s: &SemioSubsetSnapshot) -> &'static str {
    match s {
        SemioSubsetSnapshot::Brep(_) => "brep",
        SemioSubsetSnapshot::Mesh(_) => "mesh",
        SemioSubsetSnapshot::Model(_) => "model",
        SemioSubsetSnapshot::Value(_) => "value",
        SemioSubsetSnapshot::Document(_) => "document",
        SemioSubsetSnapshot::Cad(_) => "cad",
        SemioSubsetSnapshot::Drawing(_) => "drawing",
        SemioSubsetSnapshot::Image(_) => "image",
        SemioSubsetSnapshot::Video(_) => "video",
        SemioSubsetSnapshot::Audio(_) => "audio",
        SemioSubsetSnapshot::Animation(_) => "animation",
        SemioSubsetSnapshot::Presentation(_) => "presentation",
        SemioSubsetSnapshot::Flow(_) => "flow",
        SemioSubsetSnapshot::Text(_) => "text",
        SemioSubsetSnapshot::Table(_) => "table",
        SemioSubsetSnapshot::Graph(_) => "graph",
        SemioSubsetSnapshot::Object(_) => "object",
        SemioSubsetSnapshot::Kit(_) => "kit",
    }
}

/// 🔢️ The binary sibling of [`subset_tag`] — a real, individually protocol-walkable `u8` ordinal
/// (0-13, enum declaration order), used by the binary pack header instead of a length-prefixed name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn subset_ordinal(s: &SemioSubsetSnapshot) -> u8 {
    match s {
        SemioSubsetSnapshot::Brep(_) => 0,
        SemioSubsetSnapshot::Mesh(_) => 1,
        SemioSubsetSnapshot::Model(_) => 2,
        SemioSubsetSnapshot::Value(_) => 3,
        SemioSubsetSnapshot::Document(_) => 4,
        SemioSubsetSnapshot::Cad(_) => 5,
        SemioSubsetSnapshot::Drawing(_) => 6,
        SemioSubsetSnapshot::Image(_) => 7,
        SemioSubsetSnapshot::Video(_) => 8,
        SemioSubsetSnapshot::Audio(_) => 9,
        SemioSubsetSnapshot::Animation(_) => 10,
        SemioSubsetSnapshot::Presentation(_) => 11,
        SemioSubsetSnapshot::Flow(_) => 12,
        SemioSubsetSnapshot::Text(_) => 13,
        SemioSubsetSnapshot::Table(_) => 14,
        SemioSubsetSnapshot::Graph(_) => 15,
        SemioSubsetSnapshot::Object(_) => 16,
        SemioSubsetSnapshot::Kit(_) => 17,
    }
}
