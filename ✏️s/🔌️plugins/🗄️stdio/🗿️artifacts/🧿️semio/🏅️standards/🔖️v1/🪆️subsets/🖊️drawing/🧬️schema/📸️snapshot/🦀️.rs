//! 🧬️ SemioDrawingSnapshot — canvas + name-keyed styles + ordered layers, each a recursive
//! `DrawNode`{Path{segments}/Text/Group{transform,children}/Image} scene graph — from svg;
//! replaces DwgDrawing-as-neutral. Real, complete-per-spec-row shape (master plan "drawing" row):
//! no `serde_json::Value`, no bare tuples/nested fixed arrays (geometry fields reuse
//! `engine::geometry`'s named structs throughout).

use crate::standards::v1::subsets::base::schema::geometry::native::{NativeF64,NativeF32};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};


use framework_schema::ArtifactSchema;

//#region 🔖️PathSegment
/// ✏️ A single SVG-style path command — the honest, complete production set for `Path.segments`
/// (no `*OCTET`/size-eos catch-all: every field a real drawn quantity).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PathSegment {
    MoveTo {
        to: SemioPoint2,
    },
    LineTo {
        to: SemioPoint2,
    },
    CubicTo {
        c1: SemioPoint2,
        c2: SemioPoint2,
        to: SemioPoint2,
    },
    QuadTo {
        c: SemioPoint2,
        to: SemioPoint2,
    },
    /// 🌙️ Elliptical arc, SVG `A rx ry x-rotation large-arc sweep x y` shape.
    ArcTo {
        rx: f64,
        ry: f64,
        x_rotation: f64,
        large_arc: bool,
        sweep: bool,
        to: SemioPoint2,
    },
    Close,
}
//#endregion 🔖️PathSegment

//#region 🔖️DrawNode
/// 🖍️ Owned by the `drawing` subset: the recursive scene-graph node, matching svg's
/// `SvgNodeDiff` recursive-diff template per the master plan. `style` fields are a referential
/// `Option<String>` into `SemioDrawingSnapshot.styles` by name (checked by `SemioDrawingValidator`
/// — dangling references are a real referential-invariant breach, not silently tolerated).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DrawNode {
    Path {
        segments: Vec<PathSegment>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Text {
        value: String,
        at: SemioPoint2,
        #[value(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Group {
        transform: SemioTransform,
        #[value(default)]
        children: Vec<DrawNode>,
    },
    /// 🖼️ Raster payload embedded verbatim (typed raw retention — real bytes, not a lie).
    Image { at: SemioPoint2, width: f64, height: f64, mime: String, bytes: Vec<u8> },
}

impl Default for DrawNode {
    fn default() -> Self {
        DrawNode::Group { transform: SemioTransform::identity(), children: Vec::new() }
    }
}
//#endregion 🔖️DrawNode

//#region 🔖️Style
/// 🎨️ A named presentation style, referenced by `DrawNode::Path`/`Text.style`. Name-keyed
/// (`NamedTripleDiff<String, DrawStyleDiff, DrawStyle>` in the diff facet).
/// 🩹 `Default` derived (not just decoration) — required transitively as the `T` of
/// `triples::NamedTripleDiff<String, DrawStyleDiff, DrawStyle>`'s generated `Deserialize` impl
/// (serde-derive's bound inference for `#[value(default)]` fields on a generic container reaches
/// every type parameter, not just the immediately-defaulted field's own type).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawStyle {
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<SemioRgba>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<SemioRgba>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
}
//#endregion 🔖️Style

//#region 🔖️Layer
/// 🗂️ One ordered layer (index-keyed z-order, `IndexedTripleDiff<DrawLayerDiff, DrawLayer>` in
/// the diff facet — mirrors gif-frame ordering precedent).
/// 🩹 `Default` derived for the same reason as `DrawStyle` above (needed as the `T` of
/// `triples::IndexedTripleDiff<DrawLayerDiff, DrawLayer>`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawLayer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    #[value(serialize_controlled_with="component::nodes::encode_node",deserialize_controlled_with="component::nodes::decode_node",retire_with="component::nodes::retire_node")]
    pub root: DrawNode,
}
//#endregion 🔖️Layer

//#region 🔖️Canvas
/// 🖼️ Document-level viewport/backdrop.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawCanvas {
    pub width: f64,
    pub height: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<SemioRgba>,
}

impl Default for DrawCanvas {
    fn default() -> Self {
        Self { width: 0.0, height: 0.0, background: None }
    }
}
//#endregion 🔖️Canvas

//#region 🔖️Ids
pub const STDIO_SEMIODRAWING_DOCUMENT_SCHEMA: &str = "stdio.semio.drawing";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing")]
pub struct SemioDrawingSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub canvas: DrawCanvas,
    #[state(artifact)]
    #[value(default)]
    pub styles: Vec<DrawStyle>,
    #[state(artifact)]
    #[value(default)]
    pub layers: Vec<DrawLayer>,
}

impl Default for SemioDrawingSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(), canvas: DrawCanvas::default(), styles: Vec::new(), layers: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives









































//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives





































//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.drawing` document — exercises every `PathSegment`/`DrawNode` variant
/// at least once (incl. nested `Group.children` recursion) plus every `Option<T>` field non-`None`.
/// Single source of truth for `📚️examples/🖍️sketch/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_drawing_snapshot() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 100.0, height: 50.0, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: vec![DrawStyle { name: "s1".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(2.0), opacity: None }],
        layers: vec![DrawLayer {
            id: "l0".into(),
            name: "base".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path {
                        segments: vec![
                            PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } },
                            PathSegment::LineTo { to: SemioPoint2 { x: 10.0, y: 10.0 } },
                            PathSegment::CubicTo { c1: SemioPoint2 { x: 1.0, y: 1.0 }, c2: SemioPoint2 { x: 2.0, y: 2.0 }, to: SemioPoint2 { x: 3.0, y: 3.0 } },
                            PathSegment::QuadTo { c: SemioPoint2 { x: 4.0, y: 4.0 }, to: SemioPoint2 { x: 5.0, y: 5.0 } },
                            PathSegment::ArcTo { rx: 1.0, ry: 2.0, x_rotation: 0.0, large_arc: true, sweep: false, to: SemioPoint2 { x: 6.0, y: 6.0 } },
                            PathSegment::Close,
                        ],
                        style: Some("s1".into()),
                    },
                    DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 5.0, y: 5.0 }, style: None },
                    DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: 8.0, height: 8.0, mime: "image/png".into(), bytes: vec![1, 2, 3] },
                    DrawNode::Group { transform: SemioTransform::identity(), children: Vec::new() },
                ],
            },
        }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
//#endregion 🔁️Re-exports






#[path="🧩️component/🦀️.rs"]
pub mod component;
