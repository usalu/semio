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
use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    dec_canvas, dec_layer, dec_list, dec_node, dec_path_segment, dec_point2, dec_rgba, dec_str, dec_style, dec_transform, decode_option, enc_bool,
    enc_canvas, enc_layer, enc_list, enc_node, enc_path_segment, enc_point2, enc_rgba, enc_str, enc_style, enc_transform, encode_option, hex_decode,
    hex_encode, parse_bool, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot,
};
use protocol::command::DiffAlgebra;
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioDrawingMutation` block below
/// calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in scope
/// in production code too, not merely under `#[cfg(test)]` (same fix flow's/brep's own
/// mutations facet needed).
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
/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION drawing wave: real hand-rolled
/// `OpText`/`OpBinary`, replacing the old whole-enum `serde_json` passthrough (a real
/// JSON-transfer-ban policy violation, confirmed and now fixed — this facet was on a strictly
/// LESS-real starting point than the sibling `🔺️diff` facet's own pre-wave hex/bracket text).
/// Grammar: `keyword arg=value ...` (space-separated), reusing the sibling `📸️snapshot` facet's
/// real hex/bracket value primitives (`enc_str`/`enc_rgba`/`enc_point2`/`enc_transform`/
/// `enc_path_segment`/`enc_node`/`enc_style`/`enc_layer`/...) rather than re-deriving a second
/// independent copy — one source of truth for the entity encoding, same convention brep's own
/// mutations facet established (importing from its sibling `schema::snapshot`).
fn enc_node_path(np: &NodePath) -> String {
    format!("[{},{}]", np.layer, enc_list(&np.path, |i: &usize| i.to_string()))
}
fn dec_node_path(s: &str) -> Result<NodePath, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layer, path] = parts.as_slice() else { return Err(format!("node path: expected 2 fields, got {}", parts.len())) };
    Ok(NodePath {
        layer: layer.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?,
        path: dec_list(path, |v| v.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string()))?,
    })
}

/// 🌱️ `[hex(schema),canvas,[style,...],[layer,...]]` — the same whole-snapshot embed shape
/// `enc_brep_snapshot`/`dec_brep_snapshot` established for `SetSnapshot`, generalized to drawing's
/// `canvas`/`styles`/`layers` fields.
fn enc_drawing_snapshot(s: &SemioDrawingSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_canvas(&s.canvas), enc_list(&s.styles, enc_style), enc_list(&s.layers, enc_layer))
}
fn dec_drawing_snapshot(s: &str) -> Result<SemioDrawingSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, canvas, styles, layers] = parts.as_slice() else { return Err(format!("snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(SemioDrawingSnapshot { schema: dec_str(schema)?, canvas: dec_canvas(canvas)?, styles: dec_list(styles, dec_style)?, layers: dec_list(layers, dec_layer)? })
}

fn parse_num<T: std::str::FromStr>(s: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    s.parse::<T>().map_err(|e| e.to_string())
}

fn print_semio_drawing_mutation(m: &SemioDrawingMutation) -> String {
    match m {
        SemioDrawingMutation::NoMutation => "no-mutation".to_string(),
        SemioDrawingMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_drawing_snapshot(snapshot)),
        SemioDrawingMutation::SetCanvasSize { width, height } => format!("set-canvas-size width={width} height={height}"),
        SemioDrawingMutation::SetCanvasBackground { background } => format!("set-canvas-background background={}", encode_option(background, enc_rgba)),
        SemioDrawingMutation::SetStyle { name, fill, stroke, stroke_width, opacity } => format!(
            "set-style name={} fill={} stroke={} stroke-width={} opacity={}",
            enc_str(name),
            encode_option(fill, enc_rgba),
            encode_option(stroke, enc_rgba),
            encode_option(stroke_width, |v: &f64| v.to_string()),
            encode_option(opacity, |v: &f32| v.to_string()),
        ),
        SemioDrawingMutation::RemoveStyle { name } => format!("remove-style name={}", enc_str(name)),
        SemioDrawingMutation::InsertLayer { index, layer } => format!("insert-layer index={index} layer={}", enc_layer(layer)),
        SemioDrawingMutation::RemoveLayer { index } => format!("remove-layer index={index}"),
        SemioDrawingMutation::SetLayerMeta { index, id, name, visible } => format!("set-layer-meta index={index} id={} name={} visible={}", enc_str(id), enc_str(name), enc_bool(*visible)),
        SemioDrawingMutation::MoveLayer { from, to } => format!("move-layer from={from} to={to}"),
        SemioDrawingMutation::SetGroupTransform { path, transform } => format!("set-group-transform path={} transform={}", enc_node_path(path), enc_transform(transform)),
        SemioDrawingMutation::SetPathSegments { path, segments } => format!("set-path-segments path={} segments={}", enc_node_path(path), enc_list(segments, enc_path_segment)),
        SemioDrawingMutation::SetNodeStyle { path, style } => format!("set-node-style path={} style={}", enc_node_path(path), encode_option(style, |s| enc_str(s))),
        SemioDrawingMutation::SetText { path, value, at } => format!("set-text path={} value={} at={}", enc_node_path(path), enc_str(value), enc_point2(at)),
        SemioDrawingMutation::SetImage { path, at, width, height, mime, bytes } => {
            format!("set-image path={} at={} width={width} height={height} mime={} bytes={}", enc_node_path(path), enc_point2(at), enc_str(mime), hex_encode(bytes))
        }
        SemioDrawingMutation::InsertNode { path, index, node } => format!("insert-node path={} index={index} node={}", enc_node_path(path), enc_node(node)),
        SemioDrawingMutation::RemoveNode { path, index } => format!("remove-node path={} index={index}", enc_node_path(path)),
        SemioDrawingMutation::ReplaceNode { path, node } => format!("replace-node path={} node={}", enc_node_path(path), enc_node(node)),
    }
}
fn parse_semio_drawing_mutation(line: &str) -> Result<SemioDrawingMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioDrawingMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("drawing mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("drawing mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioDrawingMutation::SetSnapshot { snapshot: dec_drawing_snapshot(arg("snapshot")?)? }),
        "set-canvas-size" => Ok(SemioDrawingMutation::SetCanvasSize { width: parse_num(arg("width")?)?, height: parse_num(arg("height")?)? }),
        "set-canvas-background" => Ok(SemioDrawingMutation::SetCanvasBackground { background: decode_option(arg("background")?, dec_rgba)? }),
        "set-style" => Ok(SemioDrawingMutation::SetStyle {
            name: dec_str(arg("name")?)?,
            fill: decode_option(arg("fill")?, dec_rgba)?,
            stroke: decode_option(arg("stroke")?, dec_rgba)?,
            stroke_width: decode_option(arg("stroke-width")?, parse_num::<f64>)?,
            opacity: decode_option(arg("opacity")?, parse_num::<f32>)?,
        }),
        "remove-style" => Ok(SemioDrawingMutation::RemoveStyle { name: dec_str(arg("name")?)? }),
        "insert-layer" => Ok(SemioDrawingMutation::InsertLayer { index: parse_num(arg("index")?)?, layer: dec_layer(arg("layer")?)? }),
        "remove-layer" => Ok(SemioDrawingMutation::RemoveLayer { index: parse_num(arg("index")?)? }),
        "set-layer-meta" => Ok(SemioDrawingMutation::SetLayerMeta { index: parse_num(arg("index")?)?, id: dec_str(arg("id")?)?, name: dec_str(arg("name")?)?, visible: parse_bool(arg("visible")?)? }),
        "move-layer" => Ok(SemioDrawingMutation::MoveLayer { from: parse_num(arg("from")?)?, to: parse_num(arg("to")?)? }),
        "set-group-transform" => Ok(SemioDrawingMutation::SetGroupTransform { path: dec_node_path(arg("path")?)?, transform: dec_transform(arg("transform")?)? }),
        "set-path-segments" => Ok(SemioDrawingMutation::SetPathSegments { path: dec_node_path(arg("path")?)?, segments: dec_list(arg("segments")?, dec_path_segment)? }),
        "set-node-style" => Ok(SemioDrawingMutation::SetNodeStyle { path: dec_node_path(arg("path")?)?, style: decode_option(arg("style")?, dec_str)? }),
        "set-text" => Ok(SemioDrawingMutation::SetText { path: dec_node_path(arg("path")?)?, value: dec_str(arg("value")?)?, at: dec_point2(arg("at")?)? }),
        "set-image" => Ok(SemioDrawingMutation::SetImage {
            path: dec_node_path(arg("path")?)?,
            at: dec_point2(arg("at")?)?,
            width: parse_num(arg("width")?)?,
            height: parse_num(arg("height")?)?,
            mime: dec_str(arg("mime")?)?,
            bytes: hex_decode(arg("bytes")?)?,
        }),
        "insert-node" => Ok(SemioDrawingMutation::InsertNode { path: dec_node_path(arg("path")?)?, index: parse_num(arg("index")?)?, node: dec_node(arg("node")?)? }),
        "remove-node" => Ok(SemioDrawingMutation::RemoveNode { path: dec_node_path(arg("path")?)?, index: parse_num(arg("index")?)? }),
        "replace-node" => Ok(SemioDrawingMutation::ReplaceNode { path: dec_node_path(arg("path")?)?, node: dec_node(arg("node")?)? }),
        other => Err(format!("drawing mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioDrawingMutation {
    fn print_op(&self) -> String {
        print_semio_drawing_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_semio_drawing_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🏷️ Ordinal table, same declaration order as `SemioDrawingMutation`'s own enum variants and
/// `parse_semio_drawing_mutation`'s keyword match — the real binary `tag` field's source of truth.
const OP_KEYWORDS: [&str; 18] = [
    "no-mutation",
    "set-snapshot",
    "set-canvas-size",
    "set-canvas-background",
    "set-style",
    "remove-style",
    "insert-layer",
    "remove-layer",
    "set-layer-meta",
    "move-layer",
    "set-group-transform",
    "set-path-segments",
    "set-node-style",
    "set-text",
    "set-image",
    "insert-node",
    "remove-node",
    "replace-node",
];
fn variant_ordinal(m: &SemioDrawingMutation) -> u8 {
    match m {
        SemioDrawingMutation::NoMutation => 0,
        SemioDrawingMutation::SetSnapshot { .. } => 1,
        SemioDrawingMutation::SetCanvasSize { .. } => 2,
        SemioDrawingMutation::SetCanvasBackground { .. } => 3,
        SemioDrawingMutation::SetStyle { .. } => 4,
        SemioDrawingMutation::RemoveStyle { .. } => 5,
        SemioDrawingMutation::InsertLayer { .. } => 6,
        SemioDrawingMutation::RemoveLayer { .. } => 7,
        SemioDrawingMutation::SetLayerMeta { .. } => 8,
        SemioDrawingMutation::MoveLayer { .. } => 9,
        SemioDrawingMutation::SetGroupTransform { .. } => 10,
        SemioDrawingMutation::SetPathSegments { .. } => 11,
        SemioDrawingMutation::SetNodeStyle { .. } => 12,
        SemioDrawingMutation::SetText { .. } => 13,
        SemioDrawingMutation::SetImage { .. } => 14,
        SemioDrawingMutation::InsertNode { .. } => 15,
        SemioDrawingMutation::RemoveNode { .. } => 16,
        SemioDrawingMutation::ReplaceNode { .. } => 17,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_semio_drawing_mutation` (empty for
/// `no-mutation`) — the binary frame's `tag` byte already carries the keyword.
fn print_semio_drawing_mutation_args(m: &SemioDrawingMutation) -> String {
    match print_semio_drawing_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old whole-enum `serde_json::to_vec` shortcut. `format u8`
/// (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see [`OP_KEYWORDS`]) are two
/// REAL fixed fields; the variant's own `key=value ...` argument payload follows as one opaque
/// trailing `bytes` chain — reusing the already-real, already-tested `print_semio_drawing_mutation`/
/// `parse_semio_drawing_mutation` text codec rather than re-deriving a second independent encoding.
impl protocol::OpBinary for SemioDrawingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_semio_drawing_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Shared fixture + representative `SemioDrawingMutation` cases (one per variant, incl.
/// `NoMutation`) — single source of truth for this facet's own tests AND
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
fn fixture() -> SemioDrawingSnapshot {
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
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

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioDrawingMutation> {
    let base = fixture();
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
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    fn base() -> SemioDrawingSnapshot {
        fixture()
    }

    fn all_variants(base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        let _ = base;
        demo_mutation_cases()
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
