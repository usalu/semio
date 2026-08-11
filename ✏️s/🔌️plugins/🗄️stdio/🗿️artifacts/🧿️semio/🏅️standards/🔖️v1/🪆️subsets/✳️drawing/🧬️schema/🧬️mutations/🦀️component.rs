//! 🧬️ SemioDrawingMutation — named-variant enum (imperative verbs, gif/svg precedent) covering
//! every mutable field/collection of `SemioDrawingSnapshot`: canvas, styles (upsert/remove), layer
//! ordering/metadata, and per-node ops addressed by `NodePath`. Every variant's `diff()`/
//! `inverse()` is HAND-WRITTEN against the sparse `SemioDrawingDiff` shape — apply-and-capture is
//! banned (schema-design.md's explicit warning: computing diffs via clone+apply+re-diff caused
//! svg's original infinite-mutual-recursion bug).

use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{
    diff_at_path, node_at, DrawCanvasDiff, DrawGroupDiff, DrawLayerDiff, DrawNodeDiff, DrawPathDiff, DrawStyleDiff, DrawTextDiff, NodePath, SemioDrawingDiff,
};
use crate::artifacts::semio::standards::v1::engine::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use protocol::command::DiffAlgebra;
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioDrawingMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: SemioDrawingSnapshot },
    SetCanvasSize { width: f64, height: f64 },
    SetCanvasBackground { background: Option<SemioRgba> },
    /// 🎨️ Upsert: creates the named style if absent, otherwise wholesale-replaces its fields.
    SetStyle { name: String, fill: Option<SemioRgba>, stroke: Option<SemioRgba>, stroke_width: Option<f64>, opacity: Option<f32> },
    RemoveStyle { name: String },
    InsertLayer { index: usize, layer: DrawLayer },
    RemoveLayer { index: usize },
    SetLayerMeta { index: usize, id: String, name: String, visible: bool },
    /// ↔️ Removes the layer at `from` and reinserts it at `to` (positions read against the
    /// CURRENT array, `to` interpreted post-removal — mirrors gif's frame-move semantics).
    MoveLayer { from: usize, to: usize },
    SetGroupTransform { path: NodePath, transform: SemioTransform },
    SetPathSegments { path: NodePath, segments: Vec<PathSegment> },
    /// 🎨️ Sets (or clears, via `None`) the style reference on a `Path`/`Text` node.
    SetNodeStyle { path: NodePath, style: Option<String> },
    SetText { path: NodePath, value: String, at: SemioPoint2 },
    SetImage { path: NodePath, at: SemioPoint2, width: f64, height: f64, mime: String, bytes: Vec<u8> },
    InsertNode { path: NodePath, index: usize, node: DrawNode },
    RemoveNode { path: NodePath, index: usize },
    /// 🔁 Node-KIND change fallback (e.g. `Path` -> `Text`) — mirrors `DrawNodeDiff::Replace`.
    ReplaceNode { path: NodePath, node: DrawNode },
}

impl Mutation<SemioDrawingSnapshot> for SemioDrawingMutation {
    type Diff = SemioDrawingDiff;

    fn diff(&self, base: &SemioDrawingSnapshot) -> Self::Diff {
        match self {
            SemioDrawingMutation::NoMutation => SemioDrawingDiff::default(),
            SemioDrawingMutation::SetSnapshot { snapshot } => SemioDrawingDiff::between(base, snapshot),
            SemioDrawingMutation::SetCanvasSize { width, height } => {
                let width_d = if base.canvas.width != *width { Some(*width) } else { None };
                let height_d = if base.canvas.height != *height { Some(*height) } else { None };
                if width_d.is_none() && height_d.is_none() {
                    SemioDrawingDiff::default()
                } else {
                    SemioDrawingDiff { canvas: Some(DrawCanvasDiff { width: width_d, height: height_d, background: None }), styles: None, layers: None }
                }
            }
            SemioDrawingMutation::SetCanvasBackground { background } => {
                if base.canvas.background == *background {
                    SemioDrawingDiff::default()
                } else {
                    SemioDrawingDiff { canvas: Some(DrawCanvasDiff { width: None, height: None, background: Some(background.clone()) }), styles: None, layers: None }
                }
            }
            SemioDrawingMutation::SetStyle { name, fill, stroke, stroke_width, opacity } => match base.styles.iter().find(|s| &s.name == name) {
                Some(old) => {
                    let diff = DrawStyleDiff {
                        fill: if &old.fill != fill { Some(*fill) } else { None },
                        stroke: if &old.stroke != stroke { Some(*stroke) } else { None },
                        stroke_width: if &old.stroke_width != stroke_width { Some(*stroke_width) } else { None },
                        opacity: if &old.opacity != opacity { Some(*opacity) } else { None },
                    };
                    if diff.fill.is_none() && diff.stroke.is_none() && diff.stroke_width.is_none() && diff.opacity.is_none() {
                        SemioDrawingDiff::default()
                    } else {
                        SemioDrawingDiff { canvas: None, styles: Some(NamedTripleDiff { removed: vec![], modified: vec![crate::artifacts::semio::standards::v1::engine::triples::NamedModified { key: name.clone(), diff }], added: vec![] }), layers: None }
                    }
                }
                None => SemioDrawingDiff {
                    canvas: None,
                    styles: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![DrawStyle { name: name.clone(), fill: *fill, stroke: *stroke, stroke_width: *stroke_width, opacity: *opacity }] }),
                    layers: None,
                },
            },
            SemioDrawingMutation::RemoveStyle { name } => {
                if base.styles.iter().any(|s| &s.name == name) {
                    SemioDrawingDiff { canvas: None, styles: Some(NamedTripleDiff { removed: vec![name.clone()], modified: vec![], added: vec![] }), layers: None }
                } else {
                    SemioDrawingDiff::default()
                }
            }
            SemioDrawingMutation::InsertLayer { index, layer } => {
                SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: vec![], modified: vec![], added: vec![IndexAdded { index: *index, item: layer.clone() }] }) }
            }
            SemioDrawingMutation::RemoveLayer { index } => {
                if base.layers.get(*index).is_some() {
                    SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: vec![*index], modified: vec![], added: vec![] }) }
                } else {
                    SemioDrawingDiff::default()
                }
            }
            SemioDrawingMutation::SetLayerMeta { index, id, name, visible } => match base.layers.get(*index) {
                Some(old) => {
                    let diff = DrawLayerDiff {
                        id: if &old.id != id { Some(id.clone()) } else { None },
                        name: if &old.name != name { Some(name.clone()) } else { None },
                        visible: if old.visible != *visible { Some(*visible) } else { None },
                        root: None,
                    };
                    if diff.id.is_none() && diff.name.is_none() && diff.visible.is_none() {
                        SemioDrawingDiff::default()
                    } else {
                        SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: vec![], modified: vec![IndexModified { index: *index, diff }], added: vec![] }) }
                    }
                }
                None => SemioDrawingDiff::default(),
            },
            SemioDrawingMutation::MoveLayer { from, to } => match base.layers.get(*from) {
                Some(layer) => SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: vec![*from], modified: vec![], added: vec![IndexAdded { index: *to, item: layer.clone() }] }) },
                None => SemioDrawingDiff::default(),
            },
            SemioDrawingMutation::SetGroupTransform { path, transform } => diff_at_path(path, DrawNodeDiff::Group(DrawGroupDiff { transform: Some(*transform), children: None })),
            SemioDrawingMutation::SetPathSegments { path, segments } => diff_at_path(path, DrawNodeDiff::Path(DrawPathDiff { segments: Some(segments.clone()), style: None })),
            SemioDrawingMutation::SetNodeStyle { path, style } => match node_at(base, path) {
                Some(DrawNode::Path { .. }) => diff_at_path(path, DrawNodeDiff::Path(DrawPathDiff { segments: None, style: Some(style.clone()) })),
                Some(DrawNode::Text { .. }) => diff_at_path(path, DrawNodeDiff::Text(DrawTextDiff { value: None, at: None, style: Some(style.clone()) })),
                _ => SemioDrawingDiff::default(),
            },
            SemioDrawingMutation::SetText { path, value, at } => diff_at_path(path, DrawNodeDiff::Text(DrawTextDiff { value: Some(value.clone()), at: Some(*at), style: None })),
            SemioDrawingMutation::SetImage { path, at, width, height, mime, bytes } => {
                diff_at_path(path, DrawNodeDiff::Image(crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::DrawImageDiff { at: Some(*at), width: Some(*width), height: Some(*height), mime: Some(mime.clone()), bytes: Some(bytes.clone()) }))
            }
            SemioDrawingMutation::InsertNode { path, index, node } => diff_at_path(path, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![], modified: vec![], added: vec![IndexAdded { index: *index, item: node.clone() }] }) })),
            SemioDrawingMutation::RemoveNode { path, index } => diff_at_path(path, DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: vec![*index], modified: vec![], added: vec![] }) })),
            SemioDrawingMutation::ReplaceNode { path, node } => diff_at_path(path, DrawNodeDiff::Replace { node: node.clone() }),
        }
    }

    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<Self> {
        match self {
            SemioDrawingMutation::NoMutation => vec![SemioDrawingMutation::NoMutation],
            SemioDrawingMutation::SetSnapshot { .. } => vec![SemioDrawingMutation::SetSnapshot { snapshot: base.clone() }],
            SemioDrawingMutation::SetCanvasSize { .. } => vec![SemioDrawingMutation::SetCanvasSize { width: base.canvas.width, height: base.canvas.height }],
            SemioDrawingMutation::SetCanvasBackground { .. } => vec![SemioDrawingMutation::SetCanvasBackground { background: base.canvas.background }],
            SemioDrawingMutation::SetStyle { name, .. } => match base.styles.iter().find(|s| &s.name == name) {
                Some(old) => vec![SemioDrawingMutation::SetStyle { name: old.name.clone(), fill: old.fill, stroke: old.stroke, stroke_width: old.stroke_width, opacity: old.opacity }],
                None => vec![SemioDrawingMutation::RemoveStyle { name: name.clone() }],
            },
            SemioDrawingMutation::RemoveStyle { name } => match base.styles.iter().find(|s| &s.name == name) {
                Some(old) => vec![SemioDrawingMutation::SetStyle { name: old.name.clone(), fill: old.fill, stroke: old.stroke, stroke_width: old.stroke_width, opacity: old.opacity }],
                None => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::InsertLayer { index, .. } => vec![SemioDrawingMutation::RemoveLayer { index: *index }],
            SemioDrawingMutation::RemoveLayer { index } => match base.layers.get(*index) {
                Some(old) => vec![SemioDrawingMutation::InsertLayer { index: *index, layer: old.clone() }],
                None => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::SetLayerMeta { index, .. } => match base.layers.get(*index) {
                Some(old) => vec![SemioDrawingMutation::SetLayerMeta { index: *index, id: old.id.clone(), name: old.name.clone(), visible: old.visible }],
                None => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::MoveLayer { from, to } => vec![SemioDrawingMutation::MoveLayer { from: *to, to: *from }],
            SemioDrawingMutation::SetGroupTransform { path, .. } => match node_at(base, path) {
                Some(DrawNode::Group { transform, .. }) => vec![SemioDrawingMutation::SetGroupTransform { path: path.clone(), transform: *transform }],
                _ => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::SetPathSegments { path, .. } => match node_at(base, path) {
                Some(DrawNode::Path { segments, .. }) => vec![SemioDrawingMutation::SetPathSegments { path: path.clone(), segments: segments.clone() }],
                _ => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::SetNodeStyle { path, .. } => {
                let old_style = match node_at(base, path) {
                    Some(DrawNode::Path { style, .. }) => Some(style.clone()),
                    Some(DrawNode::Text { style, .. }) => Some(style.clone()),
                    _ => None,
                };
                match old_style {
                    Some(s) => vec![SemioDrawingMutation::SetNodeStyle { path: path.clone(), style: s }],
                    None => vec![SemioDrawingMutation::NoMutation],
                }
            }
            SemioDrawingMutation::SetText { path, .. } => match node_at(base, path) {
                Some(DrawNode::Text { value, at, .. }) => vec![SemioDrawingMutation::SetText { path: path.clone(), value: value.clone(), at: *at }],
                _ => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::SetImage { path, .. } => match node_at(base, path) {
                Some(DrawNode::Image { at, width, height, mime, bytes }) => vec![SemioDrawingMutation::SetImage { path: path.clone(), at: *at, width: *width, height: *height, mime: mime.clone(), bytes: bytes.clone() }],
                _ => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::InsertNode { path, index, .. } => vec![SemioDrawingMutation::RemoveNode { path: path.clone(), index: *index }],
            SemioDrawingMutation::RemoveNode { path, index } => match node_at(base, path) {
                Some(DrawNode::Group { children, .. }) => match children.get(*index) {
                    Some(node) => vec![SemioDrawingMutation::InsertNode { path: path.clone(), index: *index, node: node.clone() }],
                    None => vec![SemioDrawingMutation::NoMutation],
                },
                _ => vec![SemioDrawingMutation::NoMutation],
            },
            SemioDrawingMutation::ReplaceNode { path, .. } => match node_at(base, path) {
                Some(node) => vec![SemioDrawingMutation::ReplaceNode { path: path.clone(), node: node.clone() }],
                None => vec![SemioDrawingMutation::NoMutation],
            },
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_drawing_mutation(snapshot: &mut SemioDrawingSnapshot, mutation: &SemioDrawingMutation) -> SemioDrawingDiff {
    let diff = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioDrawingDiff as protocol::MutationDiff<SemioDrawingSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — one-line `serde_json` round trip of the whole enum, the
/// same "JSON-pack passthrough" honesty boundary the subset's own `ArtifactPack`/`ArtifactDsl`
/// snapshot codec already uses (see that file's doc comment). Deliberately NOT
/// `#[derive(dsl::DslOps)]` + `#[dsl(block)]` — that path requires the embedded snapshot type to
/// itself implement `dsl::DslField` end to end, real work this hand-rolled codec sidesteps
/// without inventing a second op grammar (mirrors `WriterDiff`'s precedent).
impl protocol::OpText for SemioDrawingMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl protocol::OpBinary for SemioDrawingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
    use protocol::MutationDiff;

    fn base() -> SemioDrawingSnapshot {
        SemioDrawingSnapshot {
            schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
            canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
            styles: vec![DrawStyle { name: "s1".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None }],
            layers: vec![DrawLayer {
                id: "l0".into(),
                name: "base".into(),
                visible: true,
                root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }], style: Some("s1".into()) }] },
            }],
        }
    }

    fn all_variants(base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        let path = NodePath { layer: 0, path: vec![0] };
        vec![
            SemioDrawingMutation::NoMutation,
            SemioDrawingMutation::SetSnapshot { snapshot: base.clone() },
            SemioDrawingMutation::SetCanvasSize { width: 99.0, height: 44.0 },
            SemioDrawingMutation::SetCanvasBackground { background: Some(SemioRgba { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }) },
            SemioDrawingMutation::SetStyle { name: "s1".into(), fill: None, stroke: Some(SemioRgba { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }), stroke_width: Some(2.0), opacity: Some(0.5) },
            SemioDrawingMutation::SetStyle { name: "s2".into(), fill: None, stroke: None, stroke_width: None, opacity: None },
            SemioDrawingMutation::RemoveStyle { name: "s1".into() },
            SemioDrawingMutation::InsertLayer { index: 1, layer: DrawLayer { id: "l1".into(), name: "new".into(), visible: true, root: DrawNode::default() } },
            SemioDrawingMutation::RemoveLayer { index: 0 },
            SemioDrawingMutation::SetLayerMeta { index: 0, id: "l0b".into(), name: "renamed".into(), visible: false },
            SemioDrawingMutation::MoveLayer { from: 0, to: 0 },
            SemioDrawingMutation::SetGroupTransform { path: NodePath { layer: 0, path: vec![] }, transform: SemioTransform { translation: crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 }, ..SemioTransform::identity() } },
            SemioDrawingMutation::SetPathSegments { path: path.clone(), segments: vec![PathSegment::LineTo { to: SemioPoint2 { x: 5.0, y: 5.0 } }, PathSegment::Close] },
            SemioDrawingMutation::SetNodeStyle { path: path.clone(), style: None },
            SemioDrawingMutation::SetText { path: NodePath { layer: 0, path: vec![] }, value: "won't apply (kind mismatch, no-op)".into(), at: SemioPoint2 { x: 0.0, y: 0.0 } },
            SemioDrawingMutation::SetImage { path: NodePath { layer: 0, path: vec![] }, at: SemioPoint2 { x: 0.0, y: 0.0 }, width: 1.0, height: 1.0, mime: "image/png".into(), bytes: vec![1] },
            SemioDrawingMutation::InsertNode { path: NodePath { layer: 0, path: vec![] }, index: 0, node: DrawNode::Text { value: "new".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None } },
            SemioDrawingMutation::RemoveNode { path: NodePath { layer: 0, path: vec![] }, index: 0 },
            SemioDrawingMutation::ReplaceNode { path: path.clone(), node: DrawNode::Text { value: "replaced".into(), at: SemioPoint2 { x: 2.0, y: 2.0 }, style: None } },
        ]
    }

    /// 🧪️ mutation_diff_law: `m.diff(base).apply(base) == { apply(&mut s,m); s }`, matching
    /// returned diff, for every variant.
    #[test]
    fn mutation_diff_law_every_variant() {
        let b = base();
        for m in all_variants(&b) {
            let diff = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(&m, &b);
            let mut applied = b.clone();
            let returned = apply_semio_drawing_mutation(&mut applied, &m);
            assert_eq!(diff, returned, "diff() must match apply_semio_drawing_mutation's returned diff for {m:?}");
            assert_eq!(<SemioDrawingDiff as MutationDiff<SemioDrawingSnapshot>>::apply(&diff, &b), applied, "diff.apply(base) must match the mutated snapshot for {m:?}");
        }
    }

    /// 🧪️ inverse_law: every variant's inverse mutation restores `base` when applied after it.
    #[test]
    fn inverse_law_every_variant() {
        let b = base();
        for m in all_variants(&b) {
            let mut round = b.clone();
            let _ = apply_semio_drawing_mutation(&mut round, &m);
            let inv = <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::inverse(&m, &b);
            assert_eq!(inv.len(), 1, "every variant returns exactly one inverse mutation for {m:?}");
            let _ = apply_semio_drawing_mutation(&mut round, &inv[0]);
            assert_eq!(round, b, "inverse must restore base for {m:?} (inverse {:?})", inv[0]);
        }
    }

    #[test]
    fn op_text_binary_roundtrip_law() {
        let b = base();
        for m in all_variants(&b) {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioDrawingMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioDrawingMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
