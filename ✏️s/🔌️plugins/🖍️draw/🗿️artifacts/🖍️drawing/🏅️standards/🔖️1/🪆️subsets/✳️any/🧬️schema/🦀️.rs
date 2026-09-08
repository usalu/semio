//! 🧬️ Drawing artifact schema — every field of the artifact with its state class.

use crate::{default_drawing_trace_params, default_drawing_transform, ArtifactDsl, DrawingAttributes, DrawingBooleanBody, DrawingEllipse, DrawingGroupBody, DrawingImageBody, DrawingLayerBase, DrawingLine, DrawingMutation, DrawingPathBody, DrawingPolygon, DrawingRect, DrawingShapeBody, DrawingSnapshot, DrawingTextBody, DrawingTraceBody, DrawingTransform, FillStyle, PathSegment, StrokeStyle, DRAWING_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
//#region 🔖️Artifact
/// 🧬️ Full drawing artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub layers: Vec<DrawingLayerNode>,
    #[state(artifact)]
    pub assets: BTreeMap<String, DrawingImageAsset>,
    #[state(artifact)]
    pub artboard: Option<DrawingArtboard>,
    #[state(presence)]
    pub selected_ids: Vec<String>,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(artifact)]
    pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DrawingArtifact {
    fn default() -> Self {
        Self {
            schema: DRAWING_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            title: None,
            layers: Vec::new(),
            assets: BTreeMap::new(),
            artboard: Some(DrawingArtboard { width: 1024.0, height: 1024.0 }),
            selected_ids: Vec::new(),
            active_utility_id: "selectDirect".into(),
            engagement_input: String::new(),
            camera_x: 512.0,
            camera_y: 512.0,
            camera_zoom: 0.75,
            hovered_id: None,
        }
    }
}

impl DrawingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> DrawingSnapshot {
        DrawingSnapshot { schema: self.schema.clone(), id: self.id.clone(), title: self.title.clone(), layers: self.layers.clone(), assets: self.assets.clone(), artboard: self.artboard.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: DrawingSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, title: snapshot.title, layers: snapshot.layers, assets: snapshot.assets, artboard: snapshot.artboard, ..Self::default() }
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
/// 🧬️ Descriptor for `s.draw.drawing` — twenty handcrafted schema leaves.
pub fn drawing_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.draw.drawing",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️Construction
/// 🏗️ `DrawingSnapshot`/`DrawingMutation`'s `ArtifactBuilder` is the ordinary `Mutation`/`MutationDiff`
/// algebra with no custom build logic beyond that — the old hand-rolled `DrawingBuilderConstruction`
/// (`empty`/`from_snapshot`/`from_text`/`from_binary`/`mutate`/`absorb`/`build`) matched
/// `SnapshotBuilder<S, M>`'s own documented "apply `outcome.diff()` once, fold a rejection into a
/// Fatal message" idiom exactly, so this subset writes zero builder boilerplate (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE task 3).
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<DrawingSnapshot, DrawingMutation>;
//#endregion 🏗️Construction

//#region 🔖️Inferrer
/// 💡️ Zero-sized `ArtifactInferrer` anchor for `DrawingInference` (`💡️inferences/🦀️.rs`).
/// Cannot retarget onto `SnapshotBuilder<DrawingSnapshot, DrawingMutation>` (the old `DrawingBuilderFacets`
/// cluster's replacement) — `SnapshotBuilder` is a foreign, non-`#[fundamental]` generic struct, so
/// `impl ArtifactInferrer for SnapshotBuilder<DrawingSnapshot, DrawingMutation>` is an orphan-rule
/// violation (E0117) regardless of the type parameters being local. `ArtifactInferrer::infer` takes
/// `&Self::Snapshot`, never `&self`, so the impl target is a pure type-level anchor — a trivial
/// local marker struct is the correct, and only legal, shape.
pub struct DrawingInferrer;
//#endregion 🔖️Inferrer

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
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingSceneNode {
    pub id: String,
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

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingSceneText {
    pub content: String,
    pub size: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingSceneImage {
    pub src: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
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
fn drawing_id_hex(material: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    material.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// 🪪️ Content-addressed layer/object id — no process-wide counter.
pub fn create_drawing_id(prefix: &str, material: &[u8]) -> String {
    format!("{prefix}-{}", drawing_id_hex(material))
}

/// 📄️ Parses the handcrafted DSL fixture once per call — used both for `setActiveExample`'s in-plugin
/// document load and to bridge into the framework's still-JSON-only `App::example`/render-override
/// surfaces, so `SEMIO_DRAW_EXAMPLE_TEXT` stays the single source of truth for the fixture.
const SEMIO_DRAW_EXAMPLE_TEXT: &str = crate::document_dsl::SEMIO_DRAW_EXAMPLE_TEXT;

pub fn semio_drawing_example_document() -> DrawingSnapshot {
    DrawingSnapshot::parse_dsl(SEMIO_DRAW_EXAMPLE_TEXT).unwrap_or_else(|_| empty_drawing_snapshot())
}

/// 🌉️ JSON bridge for `semio_framework_plugin`'s `App::example`/`VcsArtifactApp::render` override,
/// which hardcode `serde_json::from_str` on their `document_json`/`projection_override_json`
/// parameters (shared framework machinery, out of scope for this DSL migration) — derives the JSON
/// from the DSL fixture rather than keeping a second, redundant JSON copy of it on disk.
pub fn semio_drawing_example_json() -> String {
    dsl::json::to_json_string(&semio_drawing_example_document())
}

pub fn default_layer_base(name: &str) -> DrawingLayerBase {
    DrawingLayerBase { id: create_drawing_id("layer", name.as_bytes()), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() }
}

pub fn create_drawing_path_layer(name: &str, segments: Vec<PathSegment>) -> DrawingLayerNode {
    DrawingLayerNode::Path(DrawingPathBody {
        base: DrawingLayerBase { id: create_drawing_id("path", name.as_bytes()), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() },
        segments,
    })
}

pub fn create_drawing_group_layer(name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Group(DrawingGroupBody {
        base: DrawingLayerBase { id: create_drawing_id("group", name.as_bytes()), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() },
        children: Vec::new(),
    })
}

pub fn create_drawing_boolean_layer(name: &str, operation: &str, children: Vec<String>) -> DrawingLayerNode {
    DrawingLayerNode::Boolean(DrawingBooleanBody {
        base: DrawingLayerBase { id: create_drawing_id("boolean", name.as_bytes()), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() },
        operation: operation.into(),
        children,
    })
}

pub fn create_drawing_trace_layer(name: &str, source_key: &str) -> DrawingLayerNode {
    DrawingLayerNode::Trace(DrawingTraceBody {
        base: DrawingLayerBase { id: create_drawing_id("trace", name.as_bytes()), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() },
        source_key: source_key.into(),
        params: default_drawing_trace_params(),
    })
}

pub fn create_drawing_shape_layer_rect(name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Shape(DrawingShapeBody {
        base: DrawingLayerBase { id: create_drawing_id("shape", name.as_bytes()), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() },
        shape_kind: "rect".into(),
        rect: Some(DrawingRect { x: 0.0, y: 0.0, width: 128.0, height: 96.0 }),
        ellipse: None,
        circle: None,
        line: None,
        polygon: None,
    })
}

pub fn create_drawing_text_layer(name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Text(DrawingTextBody {
        base: DrawingLayerBase {
            id: create_drawing_id("text", name.as_bytes()),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes { fill: Some(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }), stroke: None },
        },
        x: 0.0,
        y: 0.0,
        content: "Text".into(),
        size: 24.0,
    })
}

pub fn create_drawing_image_layer(name: &str, image_key: &str) -> DrawingLayerNode {
    DrawingLayerNode::Image(DrawingImageBody {
        base: DrawingLayerBase { id: create_drawing_id("image", name.as_bytes()), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() },
        image_key: image_key.into(),
        width: 256.0,
        height: 256.0,
    })
}

pub fn default_drawing_document(id: &str, title: Option<&str>) -> DrawingSnapshot {
    DrawingSnapshot {
        schema: DRAWING_DOCUMENT_SCHEMA.into(),
        id: id.into(),
        title: title.map(str::to_string),
        layers: vec![create_drawing_path_layer("Layer 1", Vec::new())],
        assets: Default::default(),
        artboard: Some(DrawingArtboard { width: 1024.0, height: 1024.0 }),
    }
}

pub fn empty_drawing_snapshot() -> DrawingSnapshot {
    default_drawing_document("empty", None)
}

pub fn layer_id(layer: &DrawingLayerNode) -> &str {
    match layer {
        DrawingLayerNode::Shape(shape) => &shape.base.id,
        DrawingLayerNode::Path(path) => &path.base.id,
        DrawingLayerNode::Text(text) => &text.base.id,
        DrawingLayerNode::Image(image) => &image.base.id,
        DrawingLayerNode::Group(group) => &group.base.id,
        DrawingLayerNode::Boolean(boolean) => &boolean.base.id,
        DrawingLayerNode::Trace(trace) => &trace.base.id,
    }
}

pub fn layer_base(layer: &DrawingLayerNode) -> &DrawingLayerBase {
    match layer {
        DrawingLayerNode::Shape(shape) => &shape.base,
        DrawingLayerNode::Path(path) => &path.base,
        DrawingLayerNode::Text(text) => &text.base,
        DrawingLayerNode::Image(image) => &image.base,
        DrawingLayerNode::Group(group) => &group.base,
        DrawingLayerNode::Boolean(boolean) => &boolean.base,
        DrawingLayerNode::Trace(trace) => &trace.base,
    }
}

pub fn layer_kind_label(layer: &DrawingLayerNode) -> String {
    match layer {
        DrawingLayerNode::Shape(shape) => format!("shape:{}", shape.shape_kind),
        DrawingLayerNode::Path(_) => "path".into(),
        DrawingLayerNode::Text(_) => "text".into(),
        DrawingLayerNode::Image(_) => "image".into(),
        DrawingLayerNode::Group(_) => "group".into(),
        DrawingLayerNode::Boolean(_) => "boolean".into(),
        DrawingLayerNode::Trace(_) => "trace".into(),
    }
}

pub fn find_drawing_layer<'a>(doc: &'a DrawingSnapshot, layer_id: &str) -> Option<&'a DrawingLayerNode> {
    for layer in &doc.layers {
        if let Some(found) = find_drawing_layer_in_node(layer, layer_id) {
            return Some(found);
        }
    }
    None
}

fn find_drawing_layer_in_node<'a>(node: &'a DrawingLayerNode, target_id: &str) -> Option<&'a DrawingLayerNode> {
    if layer_id(node) == target_id {
        return Some(node);
    }
    if let DrawingLayerNode::Group(group) = node {
        for child in &group.children {
            if let Some(found) = find_drawing_layer_in_node(child, target_id) {
                return Some(found);
            }
        }
    }
    None
}

pub fn flatten_drawing_layers(layers: &[DrawingLayerNode]) -> Vec<&DrawingLayerNode> {
    let mut out = Vec::new();
    fn walk<'a>(nodes: &'a [DrawingLayerNode], out: &mut Vec<&'a DrawingLayerNode>) {
        for node in nodes {
            out.push(node);
            if let DrawingLayerNode::Group(group) = node {
                walk(&group.children, out);
            }
        }
    }
    walk(layers, &mut out);
    out
}

pub fn drawing_transform_to_matrix(transform: &DrawingTransform) -> [f64; 6] {
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    let a = transform.scale_x * cos;
    let b = transform.scale_x * sin;
    let c = -transform.scale_y * sin;
    let d = transform.scale_y * cos;
    [a, b, c, d, transform.x, transform.y]
}

pub fn drawing_matrix_to_transform(matrix: [f64; 6]) -> DrawingTransform {
    let [a, b, c, d, e, f] = matrix;
    let scale_x = (a * a + b * b).sqrt();
    let rotation = b.atan2(a);
    let det = a * d - b * c;
    let scale_y = if scale_x != 0.0 { det / scale_x } else { 0.0 };
    DrawingTransform { x: e, y: f, scale_x, scale_y, rotation }
}

pub fn drawing_play_layers_tree_row_id(layer: &DrawingLayerNode) -> String {
    let segment = match layer {
        DrawingLayerNode::Group(_) => "group",
        DrawingLayerNode::Boolean(_) => "boolean",
        DrawingLayerNode::Trace(_) => "trace",
        DrawingLayerNode::Path(_) => "path",
        DrawingLayerNode::Shape(_) => "shape",
        DrawingLayerNode::Text(_) => "text",
        DrawingLayerNode::Image(_) => "image",
    };
    format!("drawing-play-layers.{segment}.{}", layer_id(layer))
}

pub fn drawing_play_boolean_child_row_id(boolean_id: &str, child_id: &str) -> String {
    format!("drawing-play-layers.boolean.{boolean_id}.child.{child_id}")
}

pub fn drawing_play_layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    if let Some(rest) = row_id.strip_prefix("drawing-play-layers.") {
        let parts: Vec<&str> = rest.split('.').collect();
        if parts.len() >= 2 {
            return Some(parts[parts.len() - 1].to_string());
        }
    }
    None
}

pub fn layer_to_path_segments(layer: &DrawingLayerNode) -> Vec<PathSegment> {
    match layer {
        DrawingLayerNode::Path(path) => path.segments.clone(),
        DrawingLayerNode::Shape(shape) => shape_to_path_segments(shape),
        _ => Vec::new(),
    }
}

fn ellipse_path_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<PathSegment> {
    let k = 0.552_284_749_8;
    let crx = rx * k;
    let cry = ry * k;
    vec![
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + crx, cy - ry], ctrl2: [cx + rx, cy - cry], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + cry], ctrl2: [cx + crx, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - crx, cy + ry], ctrl2: [cx - rx, cy + cry], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - cry], ctrl2: [cx - crx, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ]
}

fn shape_to_path_segments(shape: &DrawingShapeBody) -> Vec<PathSegment> {
    match shape.shape_kind.as_str() {
        "rect" => shape.rect.as_ref().map(|rect| {
            vec![
                PathSegment::Move { to: [rect.x, rect.y] },
                PathSegment::Line { to: [rect.x + rect.width, rect.y] },
                PathSegment::Line { to: [rect.x + rect.width, rect.y + rect.height] },
                PathSegment::Line { to: [rect.x, rect.y + rect.height] },
                PathSegment::Close,
            ]
        }),
        "line" => shape.line.as_ref().map(|line| vec![PathSegment::Move { to: [line.x1, line.y1] }, PathSegment::Line { to: [line.x2, line.y2] }]),
        "polygon" => shape.polygon.as_ref().and_then(|polygon| {
            if polygon.points.is_empty() {
                return None;
            }
            let mut segments = vec![PathSegment::Move { to: polygon.points[0] }];
            for point in polygon.points.iter().skip(1) {
                segments.push(PathSegment::Line { to: *point });
            }
            segments.push(PathSegment::Close);
            Some(segments)
        }),
        "ellipse" => shape.ellipse.as_ref().map(|ellipse| ellipse_path_segments(ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry)),
        "circle" => shape.circle.as_ref().map(|circle| ellipse_path_segments(circle.cx, circle.cy, circle.r, circle.r)),
        _ => None,
    }
    .unwrap_or_default()
}

pub fn drawing_layer_world_bounds(layer: &DrawingLayerNode) -> Option<(f64, f64, f64, f64)> {
    let local = match layer {
        DrawingLayerNode::Text(text) => {
            let width = (text.content.len() as f64 * text.size * 0.6).max(8.0);
            let height = (text.size * 1.2).max(8.0);
            (text.x, text.y, width, height)
        }
        DrawingLayerNode::Image(image) => (0.0, 0.0, image.width, image.height),
        _ => {
            let segments = layer_to_path_segments(layer);
            if segments.is_empty() {
                return Some((-64.0, -64.0, 128.0, 128.0));
            }
            let mut min_x = f64::INFINITY;
            let mut min_y = f64::INFINITY;
            let mut max_x = f64::NEG_INFINITY;
            let mut max_y = f64::NEG_INFINITY;
            for segment in &segments {
                if let Some(to) = segment_to_point(segment) {
                    min_x = min_x.min(to[0]);
                    min_y = min_y.min(to[1]);
                    max_x = max_x.max(to[0]);
                    max_y = max_y.max(to[1]);
                }
            }
            if !min_x.is_finite() {
                return None;
            }
            (min_x, min_y, max_x - min_x, max_y - min_y)
        }
    };
    let base = layer_base(layer);
    let corners = [(local.0, local.1), (local.0 + local.2, local.1), (local.0 + local.2, local.1 + local.3), (local.0, local.1 + local.3)];
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (x, y) in corners {
        let world = transform_world_point(&base.transform, x, y);
        xs.push(world.0);
        ys.push(world.1);
    }
    Some((
        xs.iter().copied().fold(f64::INFINITY, f64::min),
        ys.iter().copied().fold(f64::INFINITY, f64::min),
        xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) - xs.iter().copied().fold(f64::INFINITY, f64::min),
        ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) - ys.iter().copied().fold(f64::INFINITY, f64::min),
    ))
}

fn segment_to_point(segment: &PathSegment) -> Option<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => Some(*to),
        PathSegment::Close => None,
    }
}

fn transform_world_point(transform: &DrawingTransform, x: f64, y: f64) -> (f64, f64) {
    let sx = x * transform.scale_x;
    let sy = y * transform.scale_y;
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    (transform.x + sx * cos - sy * sin, transform.y + sx * sin + sy * cos)
}

fn scene_node_for_path(base: &DrawingLayerBase, segments: Vec<PathSegment>) -> DrawingSceneNode {
    DrawingSceneNode {
        id: base.id.clone(),
        transform: drawing_transform_to_matrix(&base.transform),
        segments,
        fill: base.attributes.fill.clone(),
        stroke: base.attributes.stroke.clone(),
        opacity: base.opacity,
        blend_mode: base.blend_mode.clone(),
        visible: base.visible,
        fill_rule: Some("evenodd".into()),
        text: None,
        image: None,
    }
}

pub fn flatten_drawing_document_to_scene_nodes(doc: &DrawingSnapshot) -> Vec<DrawingSceneNode> {
    let mut out = Vec::new();
    fn walk(doc: &DrawingSnapshot, layers: &[DrawingLayerNode], out: &mut Vec<DrawingSceneNode>) {
        for layer in layers {
            let base = layer_base(layer);
            if !base.visible {
                continue;
            }
            match layer {
                DrawingLayerNode::Group(group) => walk(doc, &group.children, out),
                DrawingLayerNode::Boolean(boolean) => {
                    let segments = resolve_boolean_layer_segments(doc, boolean);
                    if segments.is_empty() {
                        continue;
                    }
                    out.push(scene_node_for_path(&boolean.base, segments));
                }
                DrawingLayerNode::Trace(trace) => {
                    let segments = resolve_trace_layer_segments(doc, trace);
                    if segments.is_empty() {
                        continue;
                    }
                    let mut node = scene_node_for_path(&trace.base, segments);
                    if node.fill.is_none() {
                        if let Some(stroke) = node.stroke.take() {
                            node.fill = Some(FillStyle::Solid { color: stroke.color });
                        }
                    }
                    out.push(node);
                }
                DrawingLayerNode::Text(text) => out.push(DrawingSceneNode {
                    id: text.base.id.clone(),
                    transform: drawing_transform_to_matrix(&text.base.transform),
                    segments: Vec::new(),
                    fill: text.base.attributes.fill.clone(),
                    stroke: text.base.attributes.stroke.clone(),
                    opacity: text.base.opacity,
                    blend_mode: text.base.blend_mode.clone(),
                    visible: text.base.visible,
                    fill_rule: None,
                    text: Some(DrawingSceneText { content: text.content.clone(), size: text.size }),
                    image: None,
                }),
                DrawingLayerNode::Image(image) => {
                    let src = doc.assets.get(&image.image_key).map(|asset| if asset.data.starts_with("data:") { asset.data.clone() } else { format!("data:{};base64,{}", asset.mime, asset.data) }).unwrap_or_default();
                    out.push(DrawingSceneNode {
                        id: image.base.id.clone(),
                        transform: drawing_transform_to_matrix(&image.base.transform),
                        segments: Vec::new(),
                        fill: image.base.attributes.fill.clone(),
                        stroke: image.base.attributes.stroke.clone(),
                        opacity: image.base.opacity,
                        blend_mode: image.base.blend_mode.clone(),
                        visible: image.base.visible,
                        fill_rule: None,
                        text: None,
                        image: Some(DrawingSceneImage { src, width: image.width, height: image.height }),
                    });
                }
                _ => {
                    let segments = flatten_curve_segments(&layer_to_path_segments(layer));
                    if segments.is_empty() {
                        continue;
                    }
                    out.push(scene_node_for_path(base, segments));
                }
            }
        }
    }
    walk(doc, &doc.layers, &mut out);
    out
}

pub fn canvas_layer_records(doc: &DrawingSnapshot) -> Vec<DrawingCanvasLayerRecord> {
    flatten_drawing_layers(&doc.layers)
        .into_iter()
        .filter(|layer| !matches!(layer, DrawingLayerNode::Group(_)))
        .map(|layer| {
            let base = layer_base(layer);
            let bounds = drawing_layer_world_bounds(layer);
            DrawingCanvasLayerRecord { id: base.id.clone(), kind: layer_kind_label(layer), name: base.name.clone(), x: bounds.map(|b| b.0), y: bounds.map(|b| b.1), width: bounds.map(|b| b.2), height: bounds.map(|b| b.3) }
        })
        .collect()
}

pub fn clone_drawing_layer_node(node: &DrawingLayerNode, name_suffix: &str) -> DrawingLayerNode {
    let mut cloned = node.clone();
    let id_material = |base: &DrawingLayerBase| format!("{}{name_suffix}{}", base.id, base.name).into_bytes();
    match &mut cloned {
        DrawingLayerNode::Shape(shape) => {
            shape.base.id = create_drawing_id("shape", &id_material(&shape.base));
            shape.base.name = format!("{}{name_suffix}", shape.base.name);
        }
        DrawingLayerNode::Path(path) => {
            path.base.id = create_drawing_id("path", &id_material(&path.base));
            path.base.name = format!("{}{name_suffix}", path.base.name);
        }
        DrawingLayerNode::Text(text) => {
            text.base.id = create_drawing_id("text", &id_material(&text.base));
            text.base.name = format!("{}{name_suffix}", text.base.name);
        }
        DrawingLayerNode::Image(image) => {
            image.base.id = create_drawing_id("image", &id_material(&image.base));
            image.base.name = format!("{}{name_suffix}", image.base.name);
        }
        DrawingLayerNode::Group(group) => {
            group.base.id = create_drawing_id("group", &id_material(&group.base));
            group.base.name = format!("{}{name_suffix}", group.base.name);
            group.children = group.children.iter().map(|child| clone_drawing_layer_node(child, "")).collect();
        }
        DrawingLayerNode::Boolean(boolean) => {
            boolean.base.id = create_drawing_id("boolean", &id_material(&boolean.base));
            boolean.base.name = format!("{}{name_suffix}", boolean.base.name);
        }
        DrawingLayerNode::Trace(trace) => {
            trace.base.id = create_drawing_id("trace", &id_material(&trace.base));
            trace.base.name = format!("{}{name_suffix}", trace.base.name);
        }
    }
    cloned
}

pub fn layer_base_mut(layer: &mut DrawingLayerNode) -> &mut DrawingLayerBase {
    match layer {
        DrawingLayerNode::Shape(shape) => &mut shape.base,
        DrawingLayerNode::Path(path) => &mut path.base,
        DrawingLayerNode::Text(text) => &mut text.base,
        DrawingLayerNode::Image(image) => &mut image.base,
        DrawingLayerNode::Group(group) => &mut group.base,
        DrawingLayerNode::Boolean(boolean) => &mut boolean.base,
        DrawingLayerNode::Trace(trace) => &mut trace.base,
    }
}

pub fn mutate_drawing_layer(doc: &DrawingSnapshot, target_id: &str, mutator: impl FnMut(&mut DrawingLayerNode)) -> DrawingSnapshot {
    let mut next = doc.clone();
    let mut mutator = mutator;
    update_layer_in_tree(&mut next.layers, target_id, &mut mutator);
    next
}

pub fn update_layer_in_tree(layers: &mut [DrawingLayerNode], target_id: &str, mutator: &mut impl FnMut(&mut DrawingLayerNode)) -> bool {
    for layer in layers.iter_mut() {
        if layer_id(layer) == target_id {
            mutator(layer);
            return true;
        }
        if let DrawingLayerNode::Group(group) = layer {
            if update_layer_in_tree(&mut group.children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

pub fn remove_layer_from_tree(layers: &mut Vec<DrawingLayerNode>, target_id: &str) -> bool {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer) == target_id) {
        layers.remove(index);
        return true;
    }
    for layer in layers.iter_mut() {
        if let DrawingLayerNode::Group(group) = layer {
            if remove_layer_from_tree(&mut group.children, target_id) {
                return true;
            }
        }
    }
    false
}

pub fn extract_layer_node(layers: &mut Vec<DrawingLayerNode>, target_id: &str) -> Option<DrawingLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer) == target_id) {
        return Some(layers.remove(index));
    }
    for layer in layers.iter_mut() {
        if let DrawingLayerNode::Group(group) = layer {
            if let Some(node) = extract_layer_node(&mut group.children, target_id) {
                return Some(node);
            }
        }
    }
    None
}

pub fn insert_layer(layers: &mut Vec<DrawingLayerNode>, parent_id: Option<&str>, index: usize, node: DrawingLayerNode) {
    if let Some(parent_id) = parent_id {
        if !insert_layer_in_parent(layers, parent_id, index, node.clone()) {
            layers.push(node);
        }
    } else {
        let at = index.min(layers.len());
        layers.insert(at, node);
    }
}

fn insert_layer_in_parent(layers: &mut [DrawingLayerNode], parent_id: &str, index: usize, node: DrawingLayerNode) -> bool {
    for layer in layers.iter_mut() {
        if let DrawingLayerNode::Group(group) = layer {
            if group.base.id == parent_id {
                let at = index.min(group.children.len());
                group.children.insert(at, node);
                return true;
            }
            if insert_layer_in_parent(&mut group.children, parent_id, index, node.clone()) {
                return true;
            }
        }
    }
    false
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingLayerLocation {
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

pub fn find_drawing_layer_location(doc: &DrawingSnapshot, target_id: &str) -> Option<DrawingLayerLocation> {
    fn search(layers: &[DrawingLayerNode], parent_id: Option<String>, target_id: &str) -> Option<DrawingLayerLocation> {
        for (index, layer) in layers.iter().enumerate() {
            if layer_id(layer) == target_id {
                return Some(DrawingLayerLocation { parent_id, index });
            }
            if let DrawingLayerNode::Group(group) = layer {
                if let Some(found) = search(&group.children, Some(group.base.id.clone()), target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    search(&doc.layers, None, target_id)
}

pub fn create_layer_by_kind(kind: &str) -> DrawingLayerNode {
    if let Some(shape_kind) = kind.strip_prefix("shape:") {
        return match shape_kind {
            "rect" => create_drawing_shape_layer_rect("Rectangle"),
            "ellipse" => {
                DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawingEllipse { cx: 0.0, cy: 0.0, rx: 64.0, ry: 48.0 }), circle: None, line: None, polygon: None })
            }
            "line" => DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawingLine { x1: 0.0, y1: 0.0, x2: 128.0, y2: 0.0 }), polygon: None }),
            "polygon" => DrawingLayerNode::Shape(DrawingShapeBody {
                base: default_layer_base("Polygon"),
                shape_kind: "polygon".into(),
                rect: None,
                ellipse: None,
                circle: None,
                line: None,
                polygon: Some(DrawingPolygon { points: vec![[0.0, 0.0], [64.0, 0.0], [32.0, 48.0]] }),
            }),
            _ => create_drawing_shape_layer_rect("Shape"),
        };
    }
    match kind {
        "path" => create_drawing_path_layer("Path", Vec::new()),
        "text" => create_drawing_text_layer("Text"),
        "image" => create_drawing_image_layer("Image", "image-source"),
        "group" => create_drawing_group_layer("Group"),
        "boolean" => create_drawing_boolean_layer("Boolean", "union", Vec::new()),
        "trace" => create_drawing_trace_layer("Trace", "trace-source"),
        _ => create_drawing_path_layer("Path", Vec::new()),
    }
}

pub fn hex_to_rgba(hex: &str, alpha: f64) -> [f64; 4] {
    let normalized = hex.trim_start_matches('#');
    let value = if normalized.len() == 3 { normalized.chars().map(|c| format!("{c}{c}")).collect::<String>() } else { normalized.to_string() };
    let parse = |start: usize| u8::from_str_radix(&value[start..start + 2], 16).unwrap_or(0) as f64 / 255.0;
    [parse(0), parse(2), parse(4), alpha]
}

pub fn rgba_to_hex(color: [f64; 4]) -> String {
    let channel = |value: f64| format!("{:02x}", (value.clamp(0.0, 1.0) * 255.0).round() as u8);
    format!("#{}{}{}", channel(color[0]), channel(color[1]), channel(color[2]))
}
//#endregion 🔖️Tree

//#region 🔖️SegmentGeometry
fn drawing_map_point_by_matrix(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

pub fn transform_path_segments(segments: &[PathSegment], transform: &DrawingTransform) -> Vec<PathSegment> {
    let matrix = drawing_transform_to_matrix(transform);
    segments
        .iter()
        .map(|segment| match segment {
            PathSegment::Move { to } => PathSegment::Move { to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Line { to } => PathSegment::Line { to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: drawing_map_point_by_matrix(matrix, *ctrl), to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: drawing_map_point_by_matrix(matrix, *ctrl1), ctrl2: drawing_map_point_by_matrix(matrix, *ctrl2), to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: drawing_map_point_by_matrix(matrix, *to) },
            PathSegment::Close => PathSegment::Close,
        })
        .collect()
}

pub fn scale_path_segments(segments: &[PathSegment], scale_x: f64, scale_y: f64) -> Vec<PathSegment> {
    if scale_x == 1.0 && scale_y == 1.0 {
        return segments.to_vec();
    }
    transform_path_segments(segments, &DrawingTransform { x: 0.0, y: 0.0, scale_x, scale_y, rotation: 0.0 })
}

pub fn split_path_segments_by_contour(segments: &[PathSegment]) -> Vec<Vec<PathSegment>> {
    let mut contours = Vec::new();
    let mut current: Vec<PathSegment> = Vec::new();
    for segment in segments {
        if matches!(segment, PathSegment::Move { .. }) && !current.is_empty() {
            contours.push(std::mem::take(&mut current));
        }
        current.push(segment.clone());
    }
    if !current.is_empty() {
        contours.push(current);
    }
    if contours.is_empty() {
        contours.push(Vec::new());
    }
    contours
}

pub fn path_segments_bounds(segments: &[PathSegment]) -> Option<(f64, f64, f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for segment in segments {
        if let Some(to) = segment_to_point(segment) {
            min_x = min_x.min(to[0]);
            min_y = min_y.min(to[1]);
            max_x = max_x.max(to[0]);
            max_y = max_y.max(to[1]);
        }
    }
    if !min_x.is_finite() {
        return None;
    }
    Some((min_x, min_y, max_x - min_x, max_y - min_y))
}

pub fn filter_path_segments_by_contour_area(segments: &[PathSegment], min_area: f64) -> Vec<PathSegment> {
    if min_area <= 0.0 {
        return segments.to_vec();
    }
    let mut kept = Vec::new();
    for contour in split_path_segments_by_contour(segments) {
        let Some((_, _, width, height)) = path_segments_bounds(&contour) else { continue };
        if width * height < min_area {
            continue;
        }
        kept.extend(contour);
    }
    kept
}

fn arc_ellipse_point(unit: [f64; 2], rx: f64, ry: f64, cos_phi: f64, sin_phi: f64, cx: f64, cy: f64) -> [f64; 2] {
    let x = unit[0] * rx;
    let y = unit[1] * ry;
    [cos_phi * x - sin_phi * y + cx, sin_phi * x + cos_phi * y + cy]
}

fn arc_vector_angle(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    let sign = if ux * vy - uy * vx < 0.0 { -1.0 } else { 1.0 };
    let dot = (ux * vx + uy * vy).clamp(-1.0, 1.0);
    sign * dot.acos()
}

fn arc_approx_unit_arc(ang1: f64, ang2: f64) -> ([f64; 2], [f64; 2], [f64; 2]) {
    let a = (4.0 / 3.0) * ((ang2 - ang1) / 4.0).tan();
    let (sin1, cos1) = ang1.sin_cos();
    let (sin2, cos2) = ang2.sin_cos();
    ([cos1 - sin1 * a, sin1 + cos1 * a], [cos2 + sin2 * a, sin2 - cos2 * a], [cos2, sin2])
}

/// 🌙️ Converts one SVG endpoint-parameterized arc into cubic Bézier control triples (SVG spec F.6.5).
fn arc_segment_to_cubics(from: [f64; 2], rx: f64, ry: f64, rotation_deg: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Vec<([f64; 2], [f64; 2], [f64; 2])> {
    if rx.abs() < 1e-9 || ry.abs() < 1e-9 {
        return Vec::new();
    }
    let mut rx = rx.abs();
    let mut ry = ry.abs();
    let phi = rotation_deg.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let dx = (from[0] - to[0]) / 2.0;
    let dy = (from[1] - to[1]) / 2.0;
    let pxp = cos_phi * dx + sin_phi * dy;
    let pyp = -sin_phi * dx + cos_phi * dy;
    if pxp == 0.0 && pyp == 0.0 {
        return Vec::new();
    }
    let lambda = (pxp * pxp) / (rx * rx) + (pyp * pyp) / (ry * ry);
    if lambda > 1.0 {
        let factor = lambda.sqrt();
        rx *= factor;
        ry *= factor;
    }
    let rx_sq = rx * rx;
    let ry_sq = ry * ry;
    let pxp_sq = pxp * pxp;
    let pyp_sq = pyp * pyp;
    let mut radicand = rx_sq * ry_sq - rx_sq * pyp_sq - ry_sq * pxp_sq;
    if radicand < 0.0 {
        radicand = 0.0;
    }
    radicand /= rx_sq * pyp_sq + ry_sq * pxp_sq;
    let coef = radicand.sqrt() * if large_arc == sweep { -1.0 } else { 1.0 };
    let centerxp = coef * (rx / ry) * pyp;
    let centeryp = coef * -(ry / rx) * pxp;
    let cx = cos_phi * centerxp - sin_phi * centeryp + (from[0] + to[0]) / 2.0;
    let cy = sin_phi * centerxp + cos_phi * centeryp + (from[1] + to[1]) / 2.0;
    let vx1 = (pxp - centerxp) / rx;
    let vy1 = (pyp - centeryp) / ry;
    let vx2 = (-pxp - centerxp) / rx;
    let vy2 = (-pyp - centeryp) / ry;
    let ang1 = arc_vector_angle(1.0, 0.0, vx1, vy1);
    let mut ang2 = arc_vector_angle(vx1, vy1, vx2, vy2);
    if !sweep && ang2 > 0.0 {
        ang2 -= std::f64::consts::TAU;
    }
    if sweep && ang2 < 0.0 {
        ang2 += std::f64::consts::TAU;
    }
    let mut ratio = ang2.abs() / std::f64::consts::FRAC_PI_2;
    if (1.0 - ratio).abs() < 1e-7 {
        ratio = 1.0;
    }
    let segment_count = ratio.ceil().max(1.0) as usize;
    let delta = ang2 / segment_count as f64;
    let mut cubics = Vec::with_capacity(segment_count);
    let mut angle = ang1;
    for _ in 0..segment_count {
        let (unit_ctrl1, unit_ctrl2, unit_to) = arc_approx_unit_arc(angle, angle + delta);
        cubics.push((arc_ellipse_point(unit_ctrl1, rx, ry, cos_phi, sin_phi, cx, cy), arc_ellipse_point(unit_ctrl2, rx, ry, cos_phi, sin_phi, cx, cy), arc_ellipse_point(unit_to, rx, ry, cos_phi, sin_phi, cx, cy)));
        angle += delta;
    }
    cubics
}

/// 🌙️ Flattens `Arc` segments into `Cubic` runs so downstream consumers (booleans, canvas hosts) never see SVG endpoint arcs.
pub fn flatten_curve_segments(segments: &[PathSegment]) -> Vec<PathSegment> {
    let mut out = Vec::with_capacity(segments.len());
    let mut cursor = [0.0, 0.0];
    for segment in segments {
        match segment {
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                let cubics = arc_segment_to_cubics(cursor, *rx, *ry, *rotation, *large_arc, *sweep, *to);
                if cubics.is_empty() {
                    out.push(PathSegment::Line { to: *to });
                } else {
                    for (ctrl1, ctrl2, point) in cubics {
                        out.push(PathSegment::Cubic { ctrl1, ctrl2, to: point });
                    }
                }
                cursor = *to;
            }
            other => {
                if let Some(to) = segment_to_point(other) {
                    cursor = to;
                }
                out.push(other.clone());
            }
        }
    }
    out
}

pub fn drawing_layer_descendant_leaf_ids(layer: &DrawingLayerNode) -> Vec<String> {
    match layer {
        DrawingLayerNode::Group(group) => group.children.iter().flat_map(drawing_layer_descendant_leaf_ids).collect(),
        _ => vec![layer_id(layer).to_string()],
    }
}

const CURVE_LINE_SAMPLE_STEPS: usize = 16;

fn sample_quad_points(from: [f64; 2], ctrl: [f64; 2], to: [f64; 2], steps: usize) -> Vec<[f64; 2]> {
    (1..=steps)
        .map(|step| {
            let t = step as f64 / steps as f64;
            let mt = 1.0 - t;
            [mt * mt * from[0] + 2.0 * mt * t * ctrl[0] + t * t * to[0], mt * mt * from[1] + 2.0 * mt * t * ctrl[1] + t * t * to[1]]
        })
        .collect()
}

fn sample_cubic_points(from: [f64; 2], ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2], steps: usize) -> Vec<[f64; 2]> {
    (1..=steps)
        .map(|step| {
            let t = step as f64 / steps as f64;
            let mt = 1.0 - t;
            [mt * mt * mt * from[0] + 3.0 * mt * mt * t * ctrl1[0] + 3.0 * mt * t * t * ctrl2[0] + t * t * t * to[0], mt * mt * mt * from[1] + 3.0 * mt * mt * t * ctrl1[1] + 3.0 * mt * t * t * ctrl2[1] + t * t * t * to[1]]
        })
        .collect()
}

/// 📏️ Flattens `Quad`/`Cubic`/`Arc` segments into `Line` segments — the planar boolean kernel only understands polygons.
pub fn flatten_segments_to_lines(segments: &[PathSegment]) -> Vec<PathSegment> {
    let curved = flatten_curve_segments(segments);
    let mut out = Vec::with_capacity(curved.len());
    let mut cursor = [0.0, 0.0];
    for segment in &curved {
        match segment {
            PathSegment::Quad { ctrl, to } => {
                for point in sample_quad_points(cursor, *ctrl, *to, CURVE_LINE_SAMPLE_STEPS) {
                    out.push(PathSegment::Line { to: point });
                }
                cursor = *to;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                for point in sample_cubic_points(cursor, *ctrl1, *ctrl2, *to, CURVE_LINE_SAMPLE_STEPS) {
                    out.push(PathSegment::Line { to: point });
                }
                cursor = *to;
            }
            other => {
                if let Some(to) = segment_to_point(other) {
                    cursor = to;
                }
                out.push(other.clone());
            }
        }
    }
    out
}
//#endregion 🔖️SegmentGeometry

//#region 🔖️KernelResolve
fn to_kernel_segment(segment: &PathSegment) -> semio_s_2d::PathSegment {
    use semio_s_2d::PathSegment as KernelSegment;
    match segment {
        PathSegment::Move { to } => KernelSegment::Move { to: *to },
        PathSegment::Line { to } => KernelSegment::Line { to: *to },
        PathSegment::Quad { ctrl, to } => KernelSegment::Quad { ctrl: *ctrl, to: *to },
        PathSegment::Cubic { ctrl1, ctrl2, to } => KernelSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => KernelSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        PathSegment::Close => KernelSegment::Close,
    }
}

fn from_kernel_segment(segment: &semio_s_2d::PathSegment) -> PathSegment {
    use semio_s_2d::PathSegment as KernelSegment;
    match segment {
        KernelSegment::Move { to } => PathSegment::Move { to: *to },
        KernelSegment::Line { to } => PathSegment::Line { to: *to },
        KernelSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        KernelSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        KernelSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        KernelSegment::Close => PathSegment::Close,
    }
}

fn to_kernel_segments(segments: &[PathSegment]) -> Vec<semio_s_2d::PathSegment> {
    segments.iter().map(to_kernel_segment).collect()
}

fn from_kernel_segments(segments: &[semio_s_2d::PathSegment]) -> Vec<PathSegment> {
    segments.iter().map(from_kernel_segment).collect()
}

/// 🪢️ Resolves a boolean layer's children (each transformed by its own local transform) through the planar kernel.
fn resolve_boolean_layer_segments(doc: &DrawingSnapshot, boolean: &DrawingBooleanBody) -> Vec<PathSegment> {
    let child_segments: Vec<Vec<PathSegment>> = boolean
        .children
        .iter()
        .filter_map(|child_id| find_drawing_layer(doc, child_id))
        .map(|child| transform_path_segments(&flatten_segments_to_lines(&layer_to_path_segments(child)), &layer_base(child).transform))
        .filter(|segments| !segments.is_empty())
        .collect();
    if child_segments.is_empty() {
        return Vec::new();
    }
    let kernel_inputs: Vec<Vec<semio_s_2d::PathSegment>> = child_segments.iter().map(|segments| to_kernel_segments(segments)).collect();
    match semio_s_2d::booleans::boolean_paths_many(&kernel_inputs, &boolean.operation) {
        Ok(result) => from_kernel_segments(&result),
        Err(_) => Vec::new(),
    }
}

/// 🖼️ Decodes a (possibly resized) PNG asset into an 8-bit luma buffer, matching the premigration canvas-based decode.
fn decode_drawing_image_asset_luma(asset: &DrawingImageAsset) -> Option<(u32, u32, Vec<u8>)> {
    let base64_data = match asset.data.strip_prefix("data:") {
        Some(rest) => rest.split_once(',').map_or(rest, |(_, data)| data),
        None => asset.data.as_str(),
    };
    let bytes = base64_codec::base64_standard_decode(base64_data).ok()?;
    let decoded = semio_framework_pixels::decode_png(&bytes).ok()?;
    let target_width = asset.width.unwrap_or(decoded.width);
    let target_height = asset.height.unwrap_or(decoded.height);
    let rgba = if target_width == decoded.width && target_height == decoded.height { decoded } else { semio_framework_pixels::resize_bilinear(&decoded, target_width, target_height) };
    let mut luma = vec![0u8; (target_width as usize) * (target_height as usize)];
    for (index, pixel) in rgba.pixels.as_chunks::<4>().0.iter().enumerate() {
        let [r, g, b, a] = [pixel[0], pixel[1], pixel[2], pixel[3]];
        luma[index] = ((r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) * (a as f64 / 255.0)).round() as u8;
    }
    Some((target_width, target_height, luma))
}

/// 📐️ Premigration artboard resolution: explicit artboard wins, else layer bounds excluding group/boolean/trace kinds.
pub fn resolve_drawing_artboard(doc: &DrawingSnapshot) -> Option<DrawingArtboard> {
    if let Some(artboard) = &doc.artboard {
        if artboard.width > 0.0 && artboard.height > 0.0 {
            return Some(artboard.clone());
        }
    }
    let mut max_x = 0.0_f64;
    let mut max_y = 0.0_f64;
    for layer in flatten_drawing_layers(&doc.layers) {
        if matches!(layer, DrawingLayerNode::Trace(_) | DrawingLayerNode::Boolean(_) | DrawingLayerNode::Group(_)) {
            continue;
        }
        if let Some((x, y, width, height)) = drawing_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    if max_x <= 0.0 || max_y <= 0.0 {
        return None;
    }
    Some(DrawingArtboard { width: max_x, height: max_y })
}

/// 🔍️ Resolves a trace layer's bitmap source into simplified, artboard-scaled contour segments.
fn resolve_trace_layer_segments(doc: &DrawingSnapshot, trace: &DrawingTraceBody) -> Vec<PathSegment> {
    let assets = &doc.assets;
    if assets.is_empty() {
        return Vec::new();
    };
    let Some(asset) = assets.get(&trace.source_key) else { return Vec::new() };
    let Some((width, height, luma)) = decode_drawing_image_asset_luma(asset) else { return Vec::new() };
    let traced = match semio_s_2d::trace::trace_bitmap_paths(width, height, &luma, trace.params.threshold, trace.params.simplify_epsilon) {
        Ok(segments) => from_kernel_segments(&segments),
        Err(_) => return Vec::new(),
    };
    let scaled = match resolve_drawing_artboard(doc) {
        Some(artboard) if width > 0 && height > 0 => scale_path_segments(&traced, artboard.width / width as f64, artboard.height / height as f64),
        _ => traced,
    };
    filter_path_segments_by_contour_area(&scaled, 6.0)
}
//#endregion 🔖️KernelResolve

/// 🔎 Returns whether `s.draw.drawing` is present in the process-local schema registry. Relocated from
/// `⚙️engine` alongside `default_drawing_document` (same rule; mirrors `s.lowpoly.lowpoly`'s identical move).
pub fn artifact_schema_registered() -> bool {
    ::framework_schema::artifact_schema_descriptor_registered("s.draw.drawing")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::DrawingLayerNode;
pub use crate::DrawingImageAsset;
pub use crate::DrawingArtboard;
//#endregion 🔁️Re-exports
