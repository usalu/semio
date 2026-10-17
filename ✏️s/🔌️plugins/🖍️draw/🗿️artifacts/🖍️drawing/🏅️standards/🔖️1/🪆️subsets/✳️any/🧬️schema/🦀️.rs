//! 🧬️ Drawing artifact schema — every field of the artifact with its state class.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use crate::stroke::StrokeCap;
use crate::stroke::StrokeJoin;
use crate::fill_rule::FillRule;
use crate::font_family::DrawingFontFamily;
use crate::snapshot::DrawingSnapshot;
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;
#[path="🪪️identity/🦀️.rs"]
pub mod identity;
use identity::{DrawingIdentity,DrawingIdentityAssignment};

/// 🖍️ Constructs an authored path layer from decoded domain segments.
pub fn create_drawing_path_layer(identity: DrawingIdentity, name: &str, segments: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>) -> DrawingLayerNode {
    DrawingLayerNode::Path(DrawingPathBody {
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes::default(),
        },
        segments,
    })
}

/// 🏗️ Constructs a domain document with its initial path and artboard.
pub fn default_drawing_document(id: &str, title: Option<&str>) -> DrawingSnapshot {
    DrawingSnapshot {
        schema: DRAWING_DOCUMENT_SCHEMA.into(),
        id: id.into(),
        title: title.map(Into::into),
        layers: vec![create_drawing_path_layer(DrawingIdentity::admit(("initial-layer").to_string().into()).expect("nonempty authored identity"), "Layer 1", Default::default())].into(),
        assets: Default::default(),
        artboard: Some(DrawingArtboard { width: 1024.0, height: 1024.0 }),
    }
}

/// 📄️ Constructs the empty named domain document.
pub fn empty_drawing_snapshot() -> DrawingSnapshot {
    default_drawing_document("empty", None)
}
//#region 🔖️Artifact
/// 🧬️ drawing document artifact state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingArtifact {
    #[state(artifact)]
    pub schema: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    pub id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    pub title: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    #[state(artifact)]
    pub layers: semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>,
    #[state(artifact)]
    pub assets: semio_framework_value::paged::PagedMap<DrawingImageAsset, {usize::MAX}>,
    #[state(artifact)]
    pub artboard: Option<DrawingArtboard>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DrawingArtifact {
    fn default() -> Self {
        Self { schema: DRAWING_DOCUMENT_SCHEMA.into(), id: Default::default(), title: None, layers: Default::default(), assets: Default::default(), artboard: Some(DrawingArtboard { width: 1024.0, height: 1024.0 }) }
    }
}

impl DrawingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> DrawingSnapshot {
        DrawingSnapshot { schema: self.schema.clone(), id: self.id.clone(), title: self.title.clone(), layers: self.layers.clone(), assets: self.assets.clone(), artboard: self.artboard.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: DrawingSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, title: snapshot.title, layers: snapshot.layers, assets: snapshot.assets, artboard: snapshot.artboard }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: DrawingSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.layers = snapshot.layers;
        self.assets = snapshot.assets;
        self.artboard = snapshot.artboard;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
//#endregion 🔖️Descriptor
//#region 🏗️Construction
//#endregion 🏗️Construction

// 🧬️ The old hand-rolled `derived_construction`/`derived_analysis` modules and the
// `derive_artifact_facets!(DrawingBuilderFacets { .. })` macro invocation (generated `DrawingBuilder`/
// `DrawingAnalyzer`/`DrawingComposer`) are deleted outright (design.md §3: all io now goes exclusively
// through the `io_mechanism` registry — see `🚪️io/🦀️.rs`'s `pub fn io()`). Confirmed
// zero external callers of any of the three generated types before deletion (grep, this pass).

//#region 🔖️DocumentHelpers
/// 🌱️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 3: pure helpers over document types
/// live in `🧬️schema/`). Every external call site now reads `crate::schema::…`
/// (the artifact root's own pre-existing `pub mod schema { pub use super::standards::v1::subsets::
/// any::schema::*; }` shim keeps that path resolving).

//#region 🔖️SceneTypes
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingSceneNode {
    pub id: String,
    #[value(default)]
    pub groups: Vec<DrawingSceneGroup>,
    pub transform: [f64; 6],
    pub segments: Vec<PathSegment>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeStyle>,
    pub opacity: f64,
    pub blend_mode: String,
    pub visible: bool,
    #[value(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub text: Option<DrawingSceneText>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub image: Option<DrawingSceneImage>,
}

/// 🧩️ One isolated ancestor compositing scope; leaf matrices already include its transform.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase")]
pub struct DrawingSceneGroup {
    pub id:String,
    pub opacity:f64,
    pub blend_mode:String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingSceneText {
    pub content: String,
    pub size: f64,
    pub font_family: crate::font_family::DrawingFontFamily,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingSceneImage {
    pub asset_id: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingCanvasLayerRecord {
    pub id: String,
    pub kind: String,
    pub name: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}
//#endregion 🔖️SceneTypes

//#region 🔖️Tree

#[path = "🏭️layer-factories/🦀️.rs"]
mod layer_factories;
pub use layer_factories::*;

#[path = "🌳️layer-tree/🦀️.rs"]
mod layer_tree;
pub use layer_tree::*;

pub use crate::affine::{drawing_transform_to_matrix, drawing_matrix_to_transform};

//#endregion 🔖️Tree

//#region 🔖️SegmentGeometry

//#endregion 🔖️SegmentGeometry

//#region 🔖️KernelResolve

/// 🖼️ Project intrinsic ordered RGBA samples into the first-party raster vocabulary.
pub fn drawing_image_samples(asset: &DrawingImageAsset) -> Option<semio_framework_pixels::RasterImage> {
    let count=(asset.width as usize).checked_mul(asset.height as usize)?;
    if count==0||count>16_777_216||asset.samples.len()!=count{return None;}
    let pixels=asset.samples.iter().flat_map(|sample|sample.iter().copied()).collect();
    Some(semio_framework_pixels::RasterImage{width:asset.width,height:asset.height,pixels})
}

//#endregion 🔖️KernelResolve

/// 🔎 Returns whether `s.draw.drawing` is present in the process-local schema registry. Relocated from
/// `⚙️engine` alongside `default_drawing_document` (same rule; mirrors `s.lowpoly.lowpoly`'s identical move).
pub fn artifact_schema_registered() -> bool {
    ::semio_framework_schema_registry::artifact_schema_descriptor_registered("s.draw.drawing")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use crate::DrawingArtboard;
pub use crate::DrawingImageAsset;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::DrawingLayerNode;
//#endregion 🔁️Re-exports

/// 📏️ Admits finite vector coordinates and nonnegative arc radii.
pub fn valid_path_segment(segment: &PathSegment) -> bool {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } => to.iter().all(|v| v.is_finite()),
        PathSegment::Quad { ctrl, to } => ctrl.iter().chain(to).all(|v| v.is_finite()),
        PathSegment::Cubic { ctrl1, ctrl2, to } => ctrl1.iter().chain(ctrl2).chain(to).all(|v| v.is_finite()),
        PathSegment::Arc { rx, ry, rotation, to, .. } => *rx >= 0.0 && *ry >= 0.0 && [*rx,*ry,*rotation,to[0],to[1]].iter().all(|v| v.is_finite()),
        PathSegment::Close => true,
    }
}

#[path = "🖊️stroke/🦀️.rs"]
pub mod stroke;

#[path="🔷️shape/✏️coordinates/🦀️.rs"]
pub mod shape_geometry;

#[path = "🎨️fill/🦀️.rs"]
pub mod fill;

#[path = "🎨️fill/🌀️rule/🦀️.rs"]
pub mod fill_rule;

#[path="📝️text/🔤️family/🦀️.rs"]
pub mod font_family;

pub const DRAWING_DOCUMENT_SCHEMA: &str = "drawing.document";

pub const DRAWING_BLEND_MODES: &[&str] = &["normal", "multiply", "screen", "overlay", "darken", "lighten", "colorDodge", "colorBurn", "hardLight", "softLight", "difference", "exclusion", "hue", "saturation", "color", "luminosity"];

pub const DRAWING_BOOLEAN_OPERATIONS: &[&str] = &["union", "difference", "intersection", "xor"];

pub const DRAWING_SHAPE_KINDS: &[&str] = &["rect", "ellipse", "circle", "line", "polygon"];

pub const DRAWING_UTILITY_IDS: &[&str] = &["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"];

// No `#[dsl(keyword = ...)]` on `DrawingTransform`/`DrawingTraceParams`/`DrawingArtboard`: every field of
// these types is itself `#[dsl(block)]`, which already supplies the bare leading keyword from the
// FIELD's own name — an inner keyword too would double it (`transform { transform x=0 ... }`),
// same reasoning as `note`'s `NoteImageAsset`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingTransform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub shear: f64,
    /// 📐️ Radians — `engine`'s compose/decompose matrix helpers call `.cos()`/`.sin()`
    /// directly on this field with no `to_radians()` conversion.
    #[dsl(angle = "rad")]
    pub rotation: f64,
}

// No keyword either: reached only through `semio_framework_value::list::PagedList<GradientStop, {usize::MAX}>` (a plain, un-tagged list) —
// `parse_record_body` self-terminates on the first unrecognized key regardless, the same reasoning
// verified for `note`'s `NoteImageAsset` nested inside a `Map` value slot.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(tag = "kind", rename_all = "camelCase"))]
pub enum FillStyle {
    Solid {
        color: [f64; 4],
    },
    LinearGradient {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        #[dsl(table)]
        stops: semio_framework_value::list::PagedList<GradientStop, {usize::MAX}>,
    },
    RadialGradient {
        cx: f64,
        cy: f64,
        r: f64,
        #[dsl(table)]
        stops: semio_framework_value::list::PagedList<GradientStop, {usize::MAX}>,
    },
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct StrokeStyle {
    pub color: [f64; 4],
    pub width: f64,
    pub cap: StrokeCap,
    pub join: StrokeJoin,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub dash: Option<semio_framework_value::list::PagedList<f64, {usize::MAX}>>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingAttributes {
    #[value(default)]
    #[cfg_attr(any(test, feature = "testkit"), serde(default))]
    pub fill_rule: FillRule,
    // `fill` is a sum type (`FillStyle` has several tagged variants), so it uses
    // `#[dsl(statements, block)]` — see `dsl::DslVariants`'s doc comment on `OptionStatements`.
    // `stroke` is a single record type, so a plain `#[dsl(block)]` scalar Option suffices.
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(statements, block)]
    pub fill: Option<FillStyle>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub stroke: Option<StrokeStyle>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingTraceParams {
    pub threshold: f64,
    pub simplify_epsilon: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingImageAsset {
    pub width: u32,
    pub height: u32,
    pub samples: semio_framework_value::list::PagedList<[u8; 4], {usize::MAX}>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingLayerBase {
    pub id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub name: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub blend_mode: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(block)]
    pub transform: DrawingTransform,
    #[value(default)]
    #[cfg_attr(any(test, feature = "testkit"), serde(default))]
    #[dsl(block)]
    pub attributes: DrawingAttributes,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingEllipse {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingCircle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingLine {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingPolygon {
    pub points: semio_framework_value::list::PagedList<[f64; 2], {usize::MAX}>,
}

// Each body carries its own `#[dsl(keyword = ...)]` — required by the single-field tuple
// ("newtype") variants of `DrawingLayerNode` below, which delegate their entire `RecordSpec` to the
// inner body's own spec (see `dsl::__rt::newtype_variant_spec`) rather than wrapping it in one more
// layer. `base: DrawingLayerBase` carries BOTH `#[value(flatten)]` (splices into the JSON-shaped
// `ToValue`/`FromValue` tree) and `#[dsl(block)]` (the text/binary DSL grammar has no
// flatten-splice primitive; a bare nested `base { ... }` line is its declarative equivalent).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "shape")]
pub struct DrawingShapeBody {
    #[value(flatten)]
    #[cfg_attr(any(test, feature = "testkit"), serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub shape_kind: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub rect: Option<DrawingRect>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub ellipse: Option<DrawingEllipse>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub circle: Option<DrawingCircle>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub line: Option<DrawingLine>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub polygon: Option<DrawingPolygon>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "path")]
pub struct DrawingPathBody {
    #[value(flatten)]
    #[cfg_attr(any(test, feature = "testkit"), serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    #[dsl(statements, block)]
    pub segments: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "text")]
pub struct DrawingTextBody {
    #[value(flatten)]
    #[cfg_attr(any(test, feature = "testkit"), serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub x: f64,
    pub y: f64,
    pub content: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub size: f64,
    pub font_family: DrawingFontFamily,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "image")]
pub struct DrawingImageBody {
    #[value(flatten)]
    #[cfg_attr(any(test, feature = "testkit"), serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub image_key: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "group")]
pub struct DrawingGroupBody {
    #[value(flatten)]
    #[cfg_attr(any(test, feature = "testkit"), serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    #[value(default,skip_serializing_if="group_isolation_disabled")]
    #[cfg_attr(any(test, feature = "testkit"),serde(default,skip_serializing_if="group_isolation_disabled"))]
    pub isolation:bool,
    #[dsl(statements, block)]
    pub children: semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>,
}

fn group_isolation_disabled(value:&bool)->bool {!*value}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "boolean")]
pub struct DrawingBooleanBody {
    #[value(flatten)]
    #[cfg_attr(any(test, feature = "testkit"), serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub operation: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub children: semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "trace")]
pub struct DrawingTraceBody {
    #[value(flatten)]
    #[cfg_attr(any(test, feature = "testkit"), serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub source_key: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(block)]
    pub params: DrawingTraceParams,
}

/// 🌿️ Projects a layer body whose `base` is flattened into the body object, in the exact field order of the `ToValue` wire (base fields first, then the own fields that are present).
macro_rules! flattened_layer_body_tree {
    (@present $body:ident) => { true };
    (@present $body:ident, $present:expr) => { ($present)($body) };
    ($body:ty { $($field:ident: $key:literal $(if $present:expr)?),* $(,)? }) => {
        impl semio_framework_pack_json::ArtifactCanonicalJsonTree for $body {
            fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
                let semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(base) = semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_node(&self.base)? else { return Err(flattened_layer_body_absent()); };
                Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(base $(+ usize::from(flattened_layer_body_tree!(@present self $(, $present)?)))*))
            }
            fn canonical_tree_child(&self, mut ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, semio_framework_value::ValueError> {
                let semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(base) = semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_node(&self.base)? else { return Err(flattened_layer_body_absent()); };
                if ordinal < base { return semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_child(&self.base, ordinal); }
                ordinal -= base;
                $(if flattened_layer_body_tree!(@present self $(, $present)?) { if ordinal == 0 { return Ok(&self.$field); } ordinal -= 1; })*
                Err(flattened_layer_body_absent())
            }
            fn canonical_tree_key(&self, mut ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
                let semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(base) = semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_node(&self.base)? else { return Err(flattened_layer_body_absent()); };
                if ordinal < base { return semio_framework_pack_json::ArtifactCanonicalJsonTree::canonical_tree_key(&self.base, ordinal); }
                ordinal -= base;
                $(if flattened_layer_body_tree!(@present self $(, $present)?) { if ordinal == 0 { return Ok($key.into()); } ordinal -= 1; })*
                Err(flattened_layer_body_absent())
            }
        }
    };
}

fn flattened_layer_body_absent() -> semio_framework_value::ValueError { semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical layer body ordinal is absent") }

flattened_layer_body_tree!(DrawingShapeBody { shape_kind: "shapeKind", rect: "rect" if |body: &DrawingShapeBody| body.rect.is_some(), ellipse: "ellipse" if |body: &DrawingShapeBody| body.ellipse.is_some(), circle: "circle" if |body: &DrawingShapeBody| body.circle.is_some(), line: "line" if |body: &DrawingShapeBody| body.line.is_some(), polygon: "polygon" if |body: &DrawingShapeBody| body.polygon.is_some() });

flattened_layer_body_tree!(DrawingPathBody { segments: "segments" });

flattened_layer_body_tree!(DrawingTextBody { x: "x", y: "y", content: "content", size: "size", font_family: "fontFamily" });

flattened_layer_body_tree!(DrawingImageBody { image_key: "imageKey", width: "width", height: "height" });

flattened_layer_body_tree!(DrawingGroupBody { isolation: "isolation" if |body: &DrawingGroupBody| !group_isolation_disabled(&body.isolation), children: "children" });

flattened_layer_body_tree!(DrawingBooleanBody { operation: "operation", children: "children" });

flattened_layer_body_tree!(DrawingTraceBody { source_key: "sourceKey", params: "params" });

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind")]
#[cfg_attr(any(test, feature = "testkit"), serde(tag = "kind"))]
pub enum DrawingLayerNode {
    #[value(rename = "shape")]
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "shape"))]
    Shape(DrawingShapeBody),
    #[value(rename = "path")]
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "path"))]
    Path(DrawingPathBody),
    #[value(rename = "text")]
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "text"))]
    Text(DrawingTextBody),
    #[value(rename = "image")]
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "image"))]
    Image(DrawingImageBody),
    #[value(rename = "group")]
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "group"))]
    Group(DrawingGroupBody),
    #[value(rename = "boolean")]
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "boolean"))]
    Boolean(DrawingBooleanBody),
    #[value(rename = "trace")]
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "trace"))]
    Trace(DrawingTraceBody),
}

// 🖊️ Keywords/field order are a genuine SUBSET of SVG path data's absolute commands
// (`M`/`L`/`Q`/`C`/`A`/`Z`), each field `#[dsl(positional)]` so a segment prints as compact
// command-then-args tokens — `M 1.25,196.933 L 36.25,161.125 ... Z` — instead of `move to=1.25,196.933`.
// Field order per variant mirrors the SVG spec's own argument order (e.g. `A rx ry rotation
// large-arc-flag sweep-flag x,y`) so it reads as real SVG path syntax, just space- instead of
// comma/space-mixed-delimited between commands.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(tag = "kind", rename_all = "camelCase"))]
pub enum PathSegment {
    #[dsl(key = "M")]
    Move {
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "L")]
    Line {
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "Q")]
    Quad {
        #[dsl(positional)]
        ctrl: [f64; 2],
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "C")]
    Cubic {
        #[dsl(positional)]
        ctrl1: [f64; 2],
        #[dsl(positional)]
        ctrl2: [f64; 2],
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "A")]
    Arc {
        #[dsl(positional)]
        rx: f64,
        #[dsl(positional)]
        ry: f64,
        /// 📐️ Degrees — `arc_segment_to_cubics`'s `rotation_deg` parameter calls `.to_radians()`
        /// on this value, matching SVG path data's `A rx ry x-axis-rotation ...` convention.
        #[dsl(positional)]
        #[dsl(angle = "deg")]
        rotation: f64,
        #[dsl(positional)]
        #[value(rename = "largeArc")]
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "largeArc"))]
        large_arc: bool,
        #[dsl(positional)]
        sweep: bool,
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "Z")]
    Close,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct DrawingArtboard {
    pub width: f64,
    pub height: f64,
}

pub fn default_drawing_transform() -> DrawingTransform {
    DrawingTransform { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0, shear: 0.0 }
}

pub fn default_drawing_trace_params() -> DrawingTraceParams {
    DrawingTraceParams { threshold: 0.5, simplify_epsilon: 1.5 }
}

/// 🏷️ The `2d.drawing` artifact kind declaration — lifted out of the old bundle manifest's
/// `.artifact_kind(...)` call so the app's manifest stitch can reuse it verbatim.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.drawing".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D Drawing", "2D-Zeichnung"),
        source_format: "drawing.document".into(),
        component_kind: "drawing".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        schema: "drawing.document".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.svg".into(), "stdio.png".into()],
        import_stdio_kinds: vec![],
    }
}

/// 🎯️ The `s.draw.drawing@1/*` surface dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §2.1) — lives at the ARTIFACT level, not under `editor`/`viewer`, specifically so a
/// viewer file can read it without ever importing through the sibling editor module. `artifact_kind`
/// matches this file's own `definition()` capability row `"s.draw.schema.artifact"` → descriptor
/// `"s.draw.drawing"`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const DRAWING_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.draw.drawing", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

/// 🔢️ A selection label's number, `(en, de)`: two decimals at most, trailing zeros trimmed, a German decimal comma.
pub fn drawing_label_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 🔠️ A selection label's counted noun, `(en, de)`: "1 layer" / "1 Ebene", "3 layers" / "3 Ebenen".
pub fn drawing_label_layers(count: usize) -> (String, String) {
    match count {
        1 => ("1 layer".to_string(), "1 Ebene".to_string()),
        count => (format!("{count} layers"), format!("{count} Ebenen")),
    }
}

//#region 🪢️TaxonomyMounts
#[path = "🧮️geometry/↗️affine/🦀️.rs"]
pub mod affine;
#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod text {
        #[path = "../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
}
//#endregion 🪢️TaxonomyMounts
