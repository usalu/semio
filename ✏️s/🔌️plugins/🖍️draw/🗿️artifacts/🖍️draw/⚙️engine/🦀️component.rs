//! ⚙️ Draw artifact — headless compute (constitutional: engine).

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use crate::artifacts::draw::{
    default_draw_trace_params, default_draw_transform, DocumentDsl, DrawArtboard, DrawAttributes, DrawBooleanBody, DrawCircle, DrawDocument, DrawEllipse, DrawGroupBody, DrawImageAsset, DrawImageBody, DrawLayerBase, DrawLayerNode, DrawLine,
    DrawPathBody, DrawPolygon, DrawRect, DrawShapeBody, DrawTextBody, DrawTraceBody, DrawTransform, FillStyle, PathSegment, StrokeStyle, DRAW_DOCUMENT_SCHEMA,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

//#region 🔖️Register
/// 🗂️ Registers `DrawDocument`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
/// draw documents without depending on this crate's concrete `Projection`/`Operation` types, plus
/// the 2D export/import media handlers (SVG raster export, DWG export/import).
pub fn register() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::draw::DrawPlayApp>(DRAW_DOCUMENT_SCHEMA);
    semio_framework_os::register_2d_export_handlers("2d.drawing", "draw", draw_document_json_to_svg);
    semio_framework_os::register_os_media_export_handler("2d.drawing", semio_framework_os::OsMediaFormat::Dwg, |doc| {
        let bytes = draw_document_json_to_dwg_bytes(doc)?;
        Ok(semio_framework_os::OsMediaExportResult {
            data: {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.encode(bytes)
            },
            mime_type: semio_framework_os::OsMediaFormat::Dwg.mime_type().into(),
            file_name: "draw.dwg".into(),
            encoding: Some("base64".into()),
        })
    });
    semio_framework_os::register_dwg_import_handler("2d.drawing", draw_document_json_from_dwg);
}
//#endregion 🔖️Register

//#region 🔖️Constants
/// 📄️ The `semio` example document, handcrafted in the `.draw` DSL — {@link semio_draw_example_document}/
/// {@link semio_draw_example_json} are the only ways it should be consumed.
const SEMIO_DRAW_EXAMPLE_TEXT: &str = crate::artifacts::draw::dsl::SEMIO_DRAW_EXAMPLE_TEXT;

static DRAW_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
//#endregion 🔖️Constants

//#region 🔖️SceneTypes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawSceneNode {
    pub id: String,
    pub transform: [f64; 6],
    pub segments: Vec<PathSegment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeStyle>,
    pub opacity: f64,
    pub blend_mode: String,
    pub visible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<DrawSceneText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<DrawSceneImage>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawSceneText {
    pub content: String,
    pub size: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawSceneImage {
    pub src: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCanvasLayerRecord {
    pub id: String,
    pub kind: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}
//#endregion 🔖️SceneTypes

//#region 🔖️DocumentHelpers
pub fn create_draw_id(prefix: &str) -> String {
    let next = DRAW_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

/// 📄️ Parses the handcrafted DSL fixture once per call — used both for `setActiveExample`'s in-plugin
/// document load and to bridge into the framework's still-JSON-only `App::example`/render-override
/// surfaces, so `SEMIO_DRAW_EXAMPLE_TEXT` stays the single source of truth for the fixture.
pub fn semio_draw_example_document() -> DrawDocument {
    DrawDocument::parse_dsl(SEMIO_DRAW_EXAMPLE_TEXT).unwrap_or_else(|_| empty_draw_projection())
}

/// 🌉️ JSON bridge for `semio_framework_plugin`'s `App::example`/`VcsDocumentApp::render` override,
/// which hardcode `serde_json::from_str` on their `document_json`/`projection_override_json`
/// parameters (shared framework machinery, out of scope for this DSL migration) — derives the JSON
/// from the DSL fixture rather than keeping a second, redundant JSON copy of it on disk.
pub fn semio_draw_example_json() -> String {
    serde_json::to_string(&semio_draw_example_document()).unwrap_or_default()
}

pub fn default_layer_base(name: &str) -> DrawLayerBase {
    DrawLayerBase { id: create_draw_id("layer"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() }
}

pub fn create_draw_path_layer(name: &str, segments: Vec<PathSegment>) -> DrawLayerNode {
    DrawLayerNode::Path(DrawPathBody {
        base: DrawLayerBase { id: create_draw_id("path"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        segments,
    })
}

pub fn create_draw_group_layer(name: &str) -> DrawLayerNode {
    DrawLayerNode::Group(DrawGroupBody {
        base: DrawLayerBase { id: create_draw_id("group"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        children: Vec::new(),
    })
}

pub fn create_draw_boolean_layer(name: &str, operation: &str, children: Vec<String>) -> DrawLayerNode {
    DrawLayerNode::Boolean(DrawBooleanBody {
        base: DrawLayerBase { id: create_draw_id("boolean"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        operation: operation.into(),
        children,
    })
}

pub fn create_draw_trace_layer(name: &str, source_key: &str) -> DrawLayerNode {
    DrawLayerNode::Trace(DrawTraceBody {
        base: DrawLayerBase { id: create_draw_id("trace"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        source_key: source_key.into(),
        params: default_draw_trace_params(),
    })
}

pub fn create_draw_shape_layer_rect(name: &str) -> DrawLayerNode {
    DrawLayerNode::Shape(DrawShapeBody {
        base: DrawLayerBase { id: create_draw_id("shape"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        shape_kind: "rect".into(),
        rect: Some(DrawRect { x: 0.0, y: 0.0, width: 128.0, height: 96.0 }),
        ellipse: None,
        circle: None,
        line: None,
        polygon: None,
    })
}

pub fn create_draw_text_layer(name: &str) -> DrawLayerNode {
    DrawLayerNode::Text(DrawTextBody {
        base: DrawLayerBase {
            id: create_draw_id("text"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes { fill: Some(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }), stroke: None },
        },
        x: 0.0,
        y: 0.0,
        content: "Text".into(),
        size: 24.0,
    })
}

pub fn create_draw_image_layer(name: &str, image_key: &str) -> DrawLayerNode {
    DrawLayerNode::Image(DrawImageBody {
        base: DrawLayerBase { id: create_draw_id("image"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        image_key: image_key.into(),
        width: 256.0,
        height: 256.0,
    })
}

pub fn default_draw_document(id: &str, title: Option<&str>) -> DrawDocument {
    DrawDocument { schema: DRAW_DOCUMENT_SCHEMA.into(), id: id.into(), title: title.map(str::to_string), layers: vec![create_draw_path_layer("Layer 1", Vec::new())], assets: None, artboard: Some(DrawArtboard { width: 1024.0, height: 1024.0 }) }
}

pub fn empty_draw_projection() -> DrawDocument {
    default_draw_document("empty", None)
}

pub fn layer_id(layer: &DrawLayerNode) -> &str {
    match layer {
        DrawLayerNode::Shape(shape) => &shape.base.id,
        DrawLayerNode::Path(path) => &path.base.id,
        DrawLayerNode::Text(text) => &text.base.id,
        DrawLayerNode::Image(image) => &image.base.id,
        DrawLayerNode::Group(group) => &group.base.id,
        DrawLayerNode::Boolean(boolean) => &boolean.base.id,
        DrawLayerNode::Trace(trace) => &trace.base.id,
    }
}

pub fn layer_base(layer: &DrawLayerNode) -> &DrawLayerBase {
    match layer {
        DrawLayerNode::Shape(shape) => &shape.base,
        DrawLayerNode::Path(path) => &path.base,
        DrawLayerNode::Text(text) => &text.base,
        DrawLayerNode::Image(image) => &image.base,
        DrawLayerNode::Group(group) => &group.base,
        DrawLayerNode::Boolean(boolean) => &boolean.base,
        DrawLayerNode::Trace(trace) => &trace.base,
    }
}

pub fn layer_kind_label(layer: &DrawLayerNode) -> String {
    match layer {
        DrawLayerNode::Shape(shape) => format!("shape:{}", shape.shape_kind),
        DrawLayerNode::Path(_) => "path".into(),
        DrawLayerNode::Text(_) => "text".into(),
        DrawLayerNode::Image(_) => "image".into(),
        DrawLayerNode::Group(_) => "group".into(),
        DrawLayerNode::Boolean(_) => "boolean".into(),
        DrawLayerNode::Trace(_) => "trace".into(),
    }
}

pub fn find_draw_layer<'a>(doc: &'a DrawDocument, layer_id: &str) -> Option<&'a DrawLayerNode> {
    for layer in &doc.layers {
        if let Some(found) = find_draw_layer_in_node(layer, layer_id) {
            return Some(found);
        }
    }
    None
}

fn find_draw_layer_in_node<'a>(node: &'a DrawLayerNode, target_id: &str) -> Option<&'a DrawLayerNode> {
    if layer_id(node) == target_id {
        return Some(node);
    }
    if let DrawLayerNode::Group(group) = node {
        for child in &group.children {
            if let Some(found) = find_draw_layer_in_node(child, target_id) {
                return Some(found);
            }
        }
    }
    None
}

pub fn flatten_draw_layers(layers: &[DrawLayerNode]) -> Vec<&DrawLayerNode> {
    let mut out = Vec::new();
    fn walk<'a>(nodes: &'a [DrawLayerNode], out: &mut Vec<&'a DrawLayerNode>) {
        for node in nodes {
            out.push(node);
            if let DrawLayerNode::Group(group) = node {
                walk(&group.children, out);
            }
        }
    }
    walk(layers, &mut out);
    out
}

pub fn draw_transform_to_matrix(transform: &DrawTransform) -> [f64; 6] {
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    let a = transform.scale_x * cos;
    let b = transform.scale_x * sin;
    let c = -transform.scale_y * sin;
    let d = transform.scale_y * cos;
    [a, b, c, d, transform.x, transform.y]
}

pub fn draw_matrix_to_transform(matrix: [f64; 6]) -> DrawTransform {
    let [a, b, c, d, e, f] = matrix;
    let scale_x = (a * a + b * b).sqrt();
    let rotation = b.atan2(a);
    let det = a * d - b * c;
    let scale_y = if scale_x != 0.0 { det / scale_x } else { 0.0 };
    DrawTransform { x: e, y: f, scale_x, scale_y, rotation }
}

pub fn draw_play_layers_tree_row_id(layer: &DrawLayerNode) -> String {
    let segment = match layer {
        DrawLayerNode::Group(_) => "group",
        DrawLayerNode::Boolean(_) => "boolean",
        DrawLayerNode::Trace(_) => "trace",
        DrawLayerNode::Path(_) => "path",
        DrawLayerNode::Shape(_) => "shape",
        DrawLayerNode::Text(_) => "text",
        DrawLayerNode::Image(_) => "image",
    };
    format!("draw-play-layers.{segment}.{}", layer_id(layer))
}

pub fn draw_play_boolean_child_row_id(boolean_id: &str, child_id: &str) -> String {
    format!("draw-play-layers.boolean.{boolean_id}.child.{child_id}")
}

pub fn draw_play_layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    if let Some(rest) = row_id.strip_prefix("draw-play-layers.") {
        let parts: Vec<&str> = rest.split('.').collect();
        if parts.len() >= 2 {
            return Some(parts[parts.len() - 1].to_string());
        }
    }
    None
}

pub fn layer_to_path_segments(layer: &DrawLayerNode) -> Vec<PathSegment> {
    match layer {
        DrawLayerNode::Path(path) => path.segments.clone(),
        DrawLayerNode::Shape(shape) => shape_to_path_segments(shape),
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

fn shape_to_path_segments(shape: &DrawShapeBody) -> Vec<PathSegment> {
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

pub fn draw_layer_world_bounds(layer: &DrawLayerNode) -> Option<(f64, f64, f64, f64)> {
    let local = match layer {
        DrawLayerNode::Text(text) => {
            let width = (text.content.len() as f64 * text.size * 0.6).max(8.0);
            let height = (text.size * 1.2).max(8.0);
            (text.x, text.y, width, height)
        }
        DrawLayerNode::Image(image) => (0.0, 0.0, image.width, image.height),
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
        xs.iter().cloned().fold(f64::INFINITY, f64::min),
        ys.iter().cloned().fold(f64::INFINITY, f64::min),
        xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - xs.iter().cloned().fold(f64::INFINITY, f64::min),
        ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - ys.iter().cloned().fold(f64::INFINITY, f64::min),
    ))
}

fn segment_to_point(segment: &PathSegment) -> Option<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => Some(*to),
        PathSegment::Close => None,
    }
}

fn transform_world_point(transform: &DrawTransform, x: f64, y: f64) -> (f64, f64) {
    let sx = x * transform.scale_x;
    let sy = y * transform.scale_y;
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    (transform.x + sx * cos - sy * sin, transform.y + sx * sin + sy * cos)
}

fn scene_node_for_path(base: &DrawLayerBase, segments: Vec<PathSegment>) -> DrawSceneNode {
    DrawSceneNode {
        id: base.id.clone(),
        transform: draw_transform_to_matrix(&base.transform),
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

pub fn flatten_draw_document_to_scene_nodes(doc: &DrawDocument) -> Vec<DrawSceneNode> {
    let mut out = Vec::new();
    fn walk(doc: &DrawDocument, layers: &[DrawLayerNode], out: &mut Vec<DrawSceneNode>) {
        for layer in layers {
            let base = layer_base(layer);
            if !base.visible {
                continue;
            }
            match layer {
                DrawLayerNode::Group(group) => walk(doc, &group.children, out),
                DrawLayerNode::Boolean(boolean) => {
                    let segments = resolve_boolean_layer_segments(doc, boolean);
                    if segments.is_empty() {
                        continue;
                    }
                    out.push(scene_node_for_path(&boolean.base, segments));
                }
                DrawLayerNode::Trace(trace) => {
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
                DrawLayerNode::Text(text) => out.push(DrawSceneNode {
                    id: text.base.id.clone(),
                    transform: draw_transform_to_matrix(&text.base.transform),
                    segments: Vec::new(),
                    fill: text.base.attributes.fill.clone(),
                    stroke: text.base.attributes.stroke.clone(),
                    opacity: text.base.opacity,
                    blend_mode: text.base.blend_mode.clone(),
                    visible: text.base.visible,
                    fill_rule: None,
                    text: Some(DrawSceneText { content: text.content.clone(), size: text.size }),
                    image: None,
                }),
                DrawLayerNode::Image(image) => {
                    let src =
                        doc.assets.as_ref().and_then(|assets| assets.get(&image.image_key)).map(|asset| if asset.data.starts_with("data:") { asset.data.clone() } else { format!("data:{};base64,{}", asset.mime, asset.data) }).unwrap_or_default();
                    out.push(DrawSceneNode {
                        id: image.base.id.clone(),
                        transform: draw_transform_to_matrix(&image.base.transform),
                        segments: Vec::new(),
                        fill: image.base.attributes.fill.clone(),
                        stroke: image.base.attributes.stroke.clone(),
                        opacity: image.base.opacity,
                        blend_mode: image.base.blend_mode.clone(),
                        visible: image.base.visible,
                        fill_rule: None,
                        text: None,
                        image: Some(DrawSceneImage { src, width: image.width, height: image.height }),
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

pub fn canvas_layer_records(doc: &DrawDocument) -> Vec<DrawCanvasLayerRecord> {
    flatten_draw_layers(&doc.layers)
        .into_iter()
        .filter(|layer| !matches!(layer, DrawLayerNode::Group(_)))
        .map(|layer| {
            let base = layer_base(layer);
            let bounds = draw_layer_world_bounds(layer);
            DrawCanvasLayerRecord { id: base.id.clone(), kind: layer_kind_label(layer), name: base.name.clone(), x: bounds.map(|b| b.0), y: bounds.map(|b| b.1), width: bounds.map(|b| b.2), height: bounds.map(|b| b.3) }
        })
        .collect()
}

pub fn clone_draw_layer_node(node: &DrawLayerNode, name_suffix: &str) -> DrawLayerNode {
    let mut cloned = node.clone();
    match &mut cloned {
        DrawLayerNode::Shape(shape) => {
            shape.base.id = create_draw_id("shape");
            shape.base.name = format!("{}{name_suffix}", shape.base.name);
        }
        DrawLayerNode::Path(path) => {
            path.base.id = create_draw_id("path");
            path.base.name = format!("{}{name_suffix}", path.base.name);
        }
        DrawLayerNode::Text(text) => {
            text.base.id = create_draw_id("text");
            text.base.name = format!("{}{name_suffix}", text.base.name);
        }
        DrawLayerNode::Image(image) => {
            image.base.id = create_draw_id("image");
            image.base.name = format!("{}{name_suffix}", image.base.name);
        }
        DrawLayerNode::Group(group) => {
            group.base.id = create_draw_id("group");
            group.base.name = format!("{}{name_suffix}", group.base.name);
            group.children = group.children.iter().map(|child| clone_draw_layer_node(child, "")).collect();
        }
        DrawLayerNode::Boolean(boolean) => {
            boolean.base.id = create_draw_id("boolean");
            boolean.base.name = format!("{}{name_suffix}", boolean.base.name);
        }
        DrawLayerNode::Trace(trace) => {
            trace.base.id = create_draw_id("trace");
            trace.base.name = format!("{}{name_suffix}", trace.base.name);
        }
    }
    cloned
}

pub fn layer_base_mut(layer: &mut DrawLayerNode) -> &mut DrawLayerBase {
    match layer {
        DrawLayerNode::Shape(shape) => &mut shape.base,
        DrawLayerNode::Path(path) => &mut path.base,
        DrawLayerNode::Text(text) => &mut text.base,
        DrawLayerNode::Image(image) => &mut image.base,
        DrawLayerNode::Group(group) => &mut group.base,
        DrawLayerNode::Boolean(boolean) => &mut boolean.base,
        DrawLayerNode::Trace(trace) => &mut trace.base,
    }
}

pub fn mutate_draw_layer(doc: &DrawDocument, target_id: &str, mutator: impl FnMut(&mut DrawLayerNode)) -> DrawDocument {
    let mut next = doc.clone();
    let mut mutator = mutator;
    update_layer_in_tree(&mut next.layers, target_id, &mut mutator);
    next
}

pub fn update_layer_in_tree(layers: &mut [DrawLayerNode], target_id: &str, mutator: &mut impl FnMut(&mut DrawLayerNode)) -> bool {
    for layer in layers.iter_mut() {
        if layer_id(layer) == target_id {
            mutator(layer);
            return true;
        }
        if let DrawLayerNode::Group(group) = layer {
            if update_layer_in_tree(&mut group.children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

pub fn remove_layer_from_tree(layers: &mut Vec<DrawLayerNode>, target_id: &str) -> bool {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer) == target_id) {
        layers.remove(index);
        return true;
    }
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if remove_layer_from_tree(&mut group.children, target_id) {
                return true;
            }
        }
    }
    false
}

pub fn extract_layer_node(layers: &mut Vec<DrawLayerNode>, target_id: &str) -> Option<DrawLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer) == target_id) {
        return Some(layers.remove(index));
    }
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if let Some(node) = extract_layer_node(&mut group.children, target_id) {
                return Some(node);
            }
        }
    }
    None
}

pub fn insert_layer(layers: &mut Vec<DrawLayerNode>, parent_id: Option<&str>, index: usize, node: DrawLayerNode) {
    if let Some(parent_id) = parent_id {
        if !insert_layer_in_parent(layers, parent_id, index, node.clone()) {
            layers.push(node);
        }
    } else {
        let at = index.min(layers.len());
        layers.insert(at, node);
    }
}

fn insert_layer_in_parent(layers: &mut [DrawLayerNode], parent_id: &str, index: usize, node: DrawLayerNode) -> bool {
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

pub fn find_draw_layer_location(doc: &DrawDocument, target_id: &str) -> Option<DrawLayerLocation> {
    fn search(layers: &[DrawLayerNode], parent_id: Option<String>, target_id: &str) -> Option<DrawLayerLocation> {
        for (index, layer) in layers.iter().enumerate() {
            if layer_id(layer) == target_id {
                return Some(DrawLayerLocation { parent_id, index });
            }
            if let DrawLayerNode::Group(group) = layer {
                if let Some(found) = search(&group.children, Some(group.base.id.clone()), target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    search(&doc.layers, None, target_id)
}

pub fn create_layer_by_kind(kind: &str) -> DrawLayerNode {
    if let Some(shape_kind) = kind.strip_prefix("shape:") {
        return match shape_kind {
            "rect" => create_draw_shape_layer_rect("Rectangle"),
            "ellipse" => {
                DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawEllipse { cx: 0.0, cy: 0.0, rx: 64.0, ry: 48.0 }), circle: None, line: None, polygon: None })
            }
            "line" => DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 128.0, y2: 0.0 }), polygon: None }),
            "polygon" => DrawLayerNode::Shape(DrawShapeBody {
                base: default_layer_base("Polygon"),
                shape_kind: "polygon".into(),
                rect: None,
                ellipse: None,
                circle: None,
                line: None,
                polygon: Some(DrawPolygon { points: vec![[0.0, 0.0], [64.0, 0.0], [32.0, 48.0]] }),
            }),
            _ => create_draw_shape_layer_rect("Shape"),
        };
    }
    match kind {
        "path" => create_draw_path_layer("Path", Vec::new()),
        "text" => create_draw_text_layer("Text"),
        "image" => create_draw_image_layer("Image", "image-source"),
        "group" => create_draw_group_layer("Group"),
        "boolean" => create_draw_boolean_layer("Boolean", "union", Vec::new()),
        "trace" => create_draw_trace_layer("Trace", "trace-source"),
        _ => create_draw_path_layer("Path", Vec::new()),
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
//#endregion 🔖️DocumentHelpers

//#region 🔖️SegmentGeometry
fn draw_map_point_by_matrix(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

pub fn transform_path_segments(segments: &[PathSegment], transform: &DrawTransform) -> Vec<PathSegment> {
    let matrix = draw_transform_to_matrix(transform);
    segments
        .iter()
        .map(|segment| match segment {
            PathSegment::Move { to } => PathSegment::Move { to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Line { to } => PathSegment::Line { to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: draw_map_point_by_matrix(matrix, *ctrl), to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: draw_map_point_by_matrix(matrix, *ctrl1), ctrl2: draw_map_point_by_matrix(matrix, *ctrl2), to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Close => PathSegment::Close,
        })
        .collect()
}

pub fn scale_path_segments(segments: &[PathSegment], scale_x: f64, scale_y: f64) -> Vec<PathSegment> {
    if scale_x == 1.0 && scale_y == 1.0 {
        return segments.to_vec();
    }
    transform_path_segments(segments, &DrawTransform { x: 0.0, y: 0.0, scale_x, scale_y, rotation: 0.0 })
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

pub fn draw_layer_descendant_leaf_ids(layer: &DrawLayerNode) -> Vec<String> {
    match layer {
        DrawLayerNode::Group(group) => group.children.iter().flat_map(draw_layer_descendant_leaf_ids).collect(),
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
fn to_kernel_segment(segment: &PathSegment) -> kernel_2d_engine::PathSegment {
    use kernel_2d_engine::PathSegment as KernelSegment;
    match segment {
        PathSegment::Move { to } => KernelSegment::Move { to: *to },
        PathSegment::Line { to } => KernelSegment::Line { to: *to },
        PathSegment::Quad { ctrl, to } => KernelSegment::Quad { ctrl: *ctrl, to: *to },
        PathSegment::Cubic { ctrl1, ctrl2, to } => KernelSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => KernelSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        PathSegment::Close => KernelSegment::Close,
    }
}

fn from_kernel_segment(segment: &kernel_2d_engine::PathSegment) -> PathSegment {
    use kernel_2d_engine::PathSegment as KernelSegment;
    match segment {
        KernelSegment::Move { to } => PathSegment::Move { to: *to },
        KernelSegment::Line { to } => PathSegment::Line { to: *to },
        KernelSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        KernelSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        KernelSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        KernelSegment::Close => PathSegment::Close,
    }
}

fn to_kernel_segments(segments: &[PathSegment]) -> Vec<kernel_2d_engine::PathSegment> {
    segments.iter().map(to_kernel_segment).collect()
}

fn from_kernel_segments(segments: &[kernel_2d_engine::PathSegment]) -> Vec<PathSegment> {
    segments.iter().map(from_kernel_segment).collect()
}

/// 🪢️ Resolves a boolean layer's children (each transformed by its own local transform) through the planar kernel.
fn resolve_boolean_layer_segments(doc: &DrawDocument, boolean: &DrawBooleanBody) -> Vec<PathSegment> {
    let child_segments: Vec<Vec<PathSegment>> = boolean
        .children
        .iter()
        .filter_map(|child_id| find_draw_layer(doc, child_id))
        .map(|child| transform_path_segments(&flatten_segments_to_lines(&layer_to_path_segments(child)), &layer_base(child).transform))
        .filter(|segments| !segments.is_empty())
        .collect();
    if child_segments.is_empty() {
        return Vec::new();
    }
    let kernel_inputs: Vec<Vec<kernel_2d_engine::PathSegment>> = child_segments.iter().map(|segments| to_kernel_segments(segments)).collect();
    match kernel_2d_rs::booleans::boolean_paths_many(&kernel_inputs, &boolean.operation) {
        Ok(result) => from_kernel_segments(&result),
        Err(_) => Vec::new(),
    }
}

/// 🖼️ Decodes a (possibly resized) PNG asset into an 8-bit luma buffer, matching the premigration canvas-based decode.
fn decode_draw_image_asset_luma(asset: &DrawImageAsset) -> Option<(u32, u32, Vec<u8>)> {
    let base64_data = match asset.data.strip_prefix("data:") {
        Some(rest) => rest.split_once(',').map(|(_, data)| data).unwrap_or(rest),
        None => asset.data.as_str(),
    };
    let bytes = BASE64.decode(base64_data).ok()?;
    let decoded = image::load_from_memory(&bytes).ok()?;
    let target_width = asset.width.unwrap_or(decoded.width());
    let target_height = asset.height.unwrap_or(decoded.height());
    let rgba = if target_width == decoded.width() && target_height == decoded.height() { decoded.to_rgba8() } else { image::imageops::resize(&decoded.to_rgba8(), target_width, target_height, image::imageops::FilterType::Triangle) };
    let mut luma = vec![0u8; (target_width as usize) * (target_height as usize)];
    for (index, pixel) in rgba.pixels().enumerate() {
        let [r, g, b, a] = pixel.0;
        luma[index] = ((r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) * (a as f64 / 255.0)).round() as u8;
    }
    Some((target_width, target_height, luma))
}

/// 📐️ Premigration artboard resolution: explicit artboard wins, else layer bounds excluding group/boolean/trace kinds.
pub fn resolve_draw_artboard(doc: &DrawDocument) -> Option<DrawArtboard> {
    if let Some(artboard) = &doc.artboard {
        if artboard.width > 0.0 && artboard.height > 0.0 {
            return Some(artboard.clone());
        }
    }
    let mut max_x = 0.0_f64;
    let mut max_y = 0.0_f64;
    for layer in flatten_draw_layers(&doc.layers) {
        if matches!(layer, DrawLayerNode::Trace(_) | DrawLayerNode::Boolean(_) | DrawLayerNode::Group(_)) {
            continue;
        }
        if let Some((x, y, width, height)) = draw_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    if max_x <= 0.0 || max_y <= 0.0 {
        return None;
    }
    Some(DrawArtboard { width: max_x, height: max_y })
}

/// 🔍️ Resolves a trace layer's bitmap source into simplified, artboard-scaled contour segments.
fn resolve_trace_layer_segments(doc: &DrawDocument, trace: &DrawTraceBody) -> Vec<PathSegment> {
    let Some(assets) = &doc.assets else { return Vec::new() };
    let Some(asset) = assets.get(&trace.source_key) else { return Vec::new() };
    let Some((width, height, luma)) = decode_draw_image_asset_luma(asset) else { return Vec::new() };
    let traced = match kernel_2d_rs::trace::trace_bitmap_paths(width, height, &luma, trace.params.threshold, trace.params.simplify_epsilon) {
        Ok(segments) => from_kernel_segments(&segments),
        Err(_) => return Vec::new(),
    };
    let scaled = match resolve_draw_artboard(doc) {
        Some(artboard) if width > 0 && height > 0 => scale_path_segments(&traced, artboard.width / width as f64, artboard.height / height as f64),
        _ => traced,
    };
    filter_path_segments_by_contour_area(&scaled, 6.0)
}
//#endregion 🔖️KernelResolve

//#region 🔖️MediaExport
fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn rgba_to_svg_color(color: [f64; 4]) -> String {
    let r = (color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color[2].clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = color[3].clamp(0.0, 1.0);
    if (a - 1.0).abs() < f64::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({r},{g},{b},{a:.3})")
    }
}

fn fill_style_to_svg(fill: &FillStyle) -> String {
    match fill {
        FillStyle::Solid { color } => rgba_to_svg_color(*color),
        FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => "none".into(),
    }
}

fn path_segments_to_svg_d(segments: &[PathSegment]) -> String {
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("M {} {} ", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("L {} {} ", to[0], to[1])),
            PathSegment::Quad { ctrl, to } => out.push_str(&format!("Q {} {} {} {} ", ctrl[0], ctrl[1], to[0], to[1])),
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                out.push_str(&format!("C {} {} {} {} {} {} ", ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1]));
            }
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => out.push_str(&format!("A {} {} {} {} {} {} {} ", rx, ry, rotation, if *large_arc { 1 } else { 0 }, if *sweep { 1 } else { 0 }, to[0], to[1])),
            PathSegment::Close => out.push('Z'),
        }
    }
    out.trim().to_string()
}

fn resolve_draw_document_artboard(doc: &DrawDocument) -> (u32, u32) {
    if let Some(artboard) = &doc.artboard {
        return (artboard.width.max(1.0).round() as u32, artboard.height.max(1.0).round() as u32);
    }
    let mut max_x: f64 = 1024.0;
    let mut max_y: f64 = 1024.0;
    for layer in flatten_draw_layers(&doc.layers) {
        if let Some((x, y, width, height)) = draw_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// @emoji 💾️ Serializes a draw document to SVG markup and raster dimensions.
pub fn draw_document_to_svg(doc: &DrawDocument) -> (String, u32, u32) {
    let (width, height) = resolve_draw_document_artboard(doc);
    let shapes = flatten_draw_document_to_scene_nodes(doc)
        .into_iter()
        .map(|node| {
            let matrix = node.transform.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" ");
            if let Some(text) = node.text {
                let fill = node.fill.as_ref().map(fill_style_to_svg).unwrap_or_else(|| "black".into());
                return format!(r#"<g transform="matrix({matrix})" opacity="{}"><text x="0" y="{}" font-size="{}" fill="{fill}">{}</text></g>"#, node.opacity, text.size, text.size, escape_svg_text(&text.content));
            }
            if let Some(image) = node.image {
                return format!(r#"<g transform="matrix({matrix})" opacity="{}"><image href="{}" width="{}" height="{}"/></g>"#, node.opacity, image.src, image.width, image.height);
            }
            let d = path_segments_to_svg_d(&node.segments);
            if d.is_empty() {
                return String::new();
            }
            let fill = node.fill.as_ref().map(fill_style_to_svg).unwrap_or_else(|| "none".into());
            let stroke = node.stroke.as_ref().map(|style| rgba_to_svg_color(style.color)).unwrap_or_else(|| "none".into());
            let stroke_width = node.stroke.as_ref().map(|style| style.width).unwrap_or(0.0);
            format!(r#"<g transform="matrix({matrix})" opacity="{}"><path d="{d}" fill="{fill}" stroke="{stroke}" stroke-width="{stroke_width}" fill-rule="evenodd"/></g>"#, node.opacity)
        })
        .filter(|shape| !shape.is_empty())
        .collect::<Vec<_>>()
        .join("");
    let svg = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{shapes}</svg>"#);
    (svg, width, height)
}

pub fn draw_document_json_to_svg(value: &serde_json::Value) -> Result<(String, u32, u32), String> {
    let doc: DrawDocument = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    Ok(draw_document_to_svg(&doc))
}

fn apply_draw_transform_point(m: [f64; 6], p: [f64; 2]) -> [f64; 2] {
    [m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5]]
}

fn draw_path_segment_to_dwg(segment: &PathSegment, transform: [f64; 6]) -> semio_framework_core::DwgPathSegment {
    use semio_framework_core::DwgPathSegment;
    match segment {
        PathSegment::Move { to } => DwgPathSegment::Move { to: apply_draw_transform_point(transform, *to) },
        PathSegment::Line { to } => DwgPathSegment::Line { to: apply_draw_transform_point(transform, *to) },
        PathSegment::Quad { ctrl, to } => DwgPathSegment::Quad { ctrl: apply_draw_transform_point(transform, *ctrl), to: apply_draw_transform_point(transform, *to) },
        PathSegment::Cubic { ctrl1, ctrl2, to } => DwgPathSegment::Cubic { ctrl1: apply_draw_transform_point(transform, *ctrl1), ctrl2: apply_draw_transform_point(transform, *ctrl2), to: apply_draw_transform_point(transform, *to) },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => DwgPathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: apply_draw_transform_point(transform, *to) },
        PathSegment::Close => DwgPathSegment::Close,
    }
}

fn dwg_path_segment_to_draw(segment: &semio_framework_core::DwgPathSegment) -> PathSegment {
    use semio_framework_core::DwgPathSegment;
    match segment {
        DwgPathSegment::Move { to } => PathSegment::Move { to: *to },
        DwgPathSegment::Line { to } => PathSegment::Line { to: *to },
        DwgPathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        DwgPathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        DwgPathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        DwgPathSegment::Close => PathSegment::Close,
    }
}

/// 📐️ Converts a draw document to DWG bytes with native fidelity: circular/elliptical arcs become bulges (not flattened cubics) and text stays a DWG TEXT entity.
pub fn draw_document_json_to_dwg_bytes(value: &serde_json::Value) -> Result<Vec<u8>, String> {
    let doc: DrawDocument = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    let mut path_groups: Vec<Vec<semio_framework_core::DwgPathSegment>> = Vec::new();
    let mut text_entities: Vec<(f64, f64, f64, String)> = Vec::new();
    for node in flatten_draw_document_to_scene_nodes(&doc) {
        if !node.visible {
            continue;
        }
        if let Some(text) = &node.text {
            let at = apply_draw_transform_point(node.transform, [0.0, 0.0]);
            text_entities.push((at[0], at[1], text.size, text.content.clone()));
            continue;
        }
        if node.segments.is_empty() {
            continue;
        }
        path_groups.push(node.segments.iter().map(|segment| draw_path_segment_to_dwg(segment, node.transform)).collect());
    }
    let mut drawing = semio_framework_core::paths_to_dwg_drawing(&path_groups);
    let layer = drawing.ensure_layer("0");
    for (x, y, size, content) in text_entities {
        drawing.entities.push(semio_framework_core::DwgEntity { layer, color: semio_framework_core::DwgColor::ByLayer, geometry: semio_framework_core::DwgGeometry::Text { at: [x, y, 0.0], height: size, rotation: 0.0, content } });
    }
    semio_framework_core::dwg_to_bytes(&drawing)
}

/// 📐️ Rebuilds a draw document from an imported DWG drawing: one path layer per polyline/spline entity, DWG text entities become draw text layers.
pub fn draw_document_json_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<serde_json::Value, String> {
    let mut doc = default_draw_document("imported-dwg", Some("Imported DWG"));
    let mut layers: Vec<DrawLayerNode> = semio_framework_core::dwg_drawing_to_paths(drawing)
        .iter()
        .enumerate()
        .map(|(index, segments)| {
            let draw_segments: Vec<PathSegment> = segments.iter().map(dwg_path_segment_to_draw).collect();
            create_draw_path_layer(&format!("Path {}", index + 1), draw_segments)
        })
        .collect();
    for entity in &drawing.entities {
        if let semio_framework_core::DwgGeometry::Text { at, height, content, .. } = &entity.geometry {
            layers.push(DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Text"), x: at[0], y: at[1], content: content.clone(), size: *height }));
        }
    }
    if layers.is_empty() {
        layers.push(create_draw_path_layer("Layer 1", Vec::new()));
    }
    doc.layers = layers;
    doc.artboard = Some(DrawArtboard { width: (drawing.extmax[0] - drawing.extmin[0]).max(1.0), height: (drawing.extmax[1] - drawing.extmin[1]).max(1.0) });
    serde_json::to_value(&doc).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaExport

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `2d.drawing`
/// `ArtifactKindSpec` literal `create_draw_app` already declares via `.artifact_kind(...)`
/// (schema/media type/export+import formats/presentation fields copied verbatim), plus the
/// app-specific `vector:out` port (see `draw_vector_out_port` below).
pub fn draw_io() -> semio_framework_core::AppIo {
    semio_framework_core::AppIo {
        document_schema: DRAW_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Vector },
        ports: vec![draw_vector_out_port()],
        export_formats: vec![semio_framework_core::OsMediaFormat::Svg, semio_framework_core::OsMediaFormat::Png],
        import_formats: vec![semio_framework_core::OsMediaFormat::Svg, semio_framework_core::OsMediaFormat::Png],
        artifact: semio_framework_core::ArtifactPresentation { id: "2d.drawing".into(), name: "2D Drawing".into(), dimension: "2d".into(), component_kind: "draw".into() },
    }
}

/// 🔌️ `vector:out` — the draw document's current vector content, exported as SVG (workflow port
/// surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). Reuses the existing `2d.drawing`
/// kind (already declared by `create_draw_app`'s `.artifact_kind(...)`) rather than minting a
/// duplicate — `kind_id` just pins this port to that same catalog entry. `Many`/optional: a
/// consumer (e.g. raster's Vector→Raster-converted `image:in`) may connect before the canvas has
/// any content, or fan out to several consumers at once.
pub fn draw_vector_out_port() -> semio_framework_core::MediaPortSpec {
    semio_framework_core::MediaPortSpec {
        id: "vector:out".into(),
        label: "Vector".into(),
        direction: semio_framework_core::MediaPortDirection::Out,
        media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Vector },
        kind_id: Some("2d.drawing".into()),
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🖼️ Exports the current draw document as an SVG `Media` payload for the `vector:out` port —
/// reuses `draw_document_to_svg` (the same renderer the export-svg shell path uses), so there is
/// exactly one SVG renderer.
pub fn draw_vector_media(doc: &DrawDocument) -> Result<semio_framework_core::Media, semio_framework_core::MediaError> {
    let (svg, _width, _height) = draw_document_to_svg(doc);
    Ok(semio_framework_core::Media {
        media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Vector },
        payload: semio_framework_core::MediaPayload::Structured { schema: "2d.drawing".into(), json: svg },
    })
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_document_has_path_layer() {
        let doc = default_draw_document("test", None);
        assert_eq!(doc.layers.len(), 1);
        assert!(matches!(doc.layers[0], DrawLayerNode::Path(_)));
    }

    #[test]
    fn dwg_export_import_round_trips_a_path_and_text_layer() {
        let path_layer = create_draw_path_layer("Outline", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Cubic { ctrl1: [12.0, 2.0], ctrl2: [12.0, 6.0], to: [10.0, 8.0] }, PathSegment::Close]);
        let text_layer = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Label"), x: 1.0, y: 2.0, content: "semio".into(), size: 3.0 });
        let doc = DrawDocument { layers: vec![path_layer, text_layer], ..default_draw_document("dwg-test", None) };
        let value = serde_json::to_value(&doc).unwrap();

        let bytes = draw_document_json_to_dwg_bytes(&value).expect("export dwg");
        assert!(!bytes.is_empty());
        let drawing = semio_framework_core::dwg_from_bytes(&bytes).expect("decode dwg");
        assert!(drawing.entities.iter().any(|entity| matches!(entity.geometry, semio_framework_core::DwgGeometry::Text { .. })));
        assert!(drawing.entities.iter().any(|entity| matches!(entity.geometry, semio_framework_core::DwgGeometry::LwPolyline { .. } | semio_framework_core::DwgGeometry::Spline { .. })));

        let reimported = draw_document_json_from_dwg(&drawing).expect("import dwg");
        let reimported_doc: DrawDocument = serde_json::from_value(reimported).expect("valid draw document");
        assert!(reimported_doc.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Text(text) if text.content == "semio")));
        assert!(reimported_doc.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Path(_))));
    }

    #[test]
    fn scene_nodes_include_shape_bounds() {
        let layer = create_draw_shape_layer_rect("Rect");
        let doc = DrawDocument { layers: vec![layer], ..default_draw_document("scene", None) };
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        assert_eq!(nodes.len(), 1);
        assert!(!nodes[0].segments.is_empty());
    }

    #[test]
    fn resolve_boolean_layer_segments_unions_two_rects() {
        let mut doc = default_draw_document("bool-test", None);
        doc.layers.clear();
        let mut rect_a = create_draw_shape_layer_rect("A");
        if let DrawLayerNode::Shape(shape) = &mut rect_a {
            shape.rect = Some(DrawRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        }
        let id_a = layer_id(&rect_a).to_string();
        let mut rect_b = create_draw_shape_layer_rect("B");
        if let DrawLayerNode::Shape(shape) = &mut rect_b {
            shape.rect = Some(DrawRect { x: 5.0, y: 5.0, width: 10.0, height: 10.0 });
        }
        let id_b = layer_id(&rect_b).to_string();
        doc.layers.push(rect_a);
        doc.layers.push(rect_b);
        let boolean = create_draw_boolean_layer("Union", "union", vec![id_a, id_b]);
        let boolean_id = layer_id(&boolean).to_string();
        doc.layers.push(boolean);
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        let boolean_node = nodes.iter().find(|node| node.id == boolean_id).expect("boolean scene node");
        assert!(!boolean_node.segments.is_empty());
        assert_eq!(boolean_node.fill_rule.as_deref(), Some("evenodd"));
    }

    #[test]
    fn resolve_boolean_layer_segments_flattens_arcs_before_boolean_operation() {
        let mut doc = default_draw_document("bool-arc-test", None);
        doc.layers.clear();
        let path_a =
            create_draw_path_layer("A", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] }, PathSegment::Close]);
        let id_a = layer_id(&path_a).to_string();
        let rect_b = {
            let mut layer = create_draw_shape_layer_rect("B");
            if let DrawLayerNode::Shape(shape) = &mut layer {
                shape.rect = Some(DrawRect { x: 2.0, y: 2.0, width: 4.0, height: 4.0 });
            }
            layer
        };
        let id_b = layer_id(&rect_b).to_string();
        doc.layers.push(path_a);
        doc.layers.push(rect_b);
        let boolean = create_draw_boolean_layer("Union", "union", vec![id_a, id_b]);
        let boolean_id = layer_id(&boolean).to_string();
        doc.layers.push(boolean);
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        let boolean_node = nodes.iter().find(|node| node.id == boolean_id).expect("boolean scene node");
        assert!(!boolean_node.segments.is_empty());
    }

    #[test]
    fn arc_segment_flattens_to_cubics_preserving_endpoints() {
        let segments = vec![PathSegment::Move { to: [10.0, 0.0] }, PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] }];
        let flattened = flatten_curve_segments(&segments);
        assert!(flattened.iter().all(|segment| !matches!(segment, PathSegment::Arc { .. })));
        match flattened.last() {
            Some(PathSegment::Cubic { to, .. }) => {
                assert!((to[0] - 0.0).abs() < 1e-6);
                assert!((to[1] - 10.0).abs() < 1e-6);
            }
            other => panic!("expected trailing cubic segment, got {other:?}"),
        }
    }

    #[test]
    fn resolve_trace_layer_segments_traces_solid_square_png() {
        let mut image_buffer = image::RgbaImage::new(8, 8);
        for y in 2..6 {
            for x in 2..6 {
                image_buffer.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image_buffer).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
        let mut doc = default_draw_document("trace-test", None);
        doc.layers.clear();
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("source".to_string(), DrawImageAsset { mime: "image/png".into(), data: BASE64.encode(&bytes), width: None, height: None });
        doc.assets = Some(assets);
        doc.artboard = Some(DrawArtboard { width: 16.0, height: 16.0 });
        doc.layers.push(create_draw_trace_layer("Trace", "source"));
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        assert_eq!(nodes.len(), 1);
        assert!(!nodes[0].segments.is_empty());
        assert_eq!(nodes[0].fill_rule.as_deref(), Some("evenodd"));
    }

    #[test]
    fn resolve_draw_artboard_skips_group_boolean_trace_kinds() {
        let mut doc = default_draw_document("artboard-test", None);
        doc.artboard = None;
        doc.layers.clear();
        let mut rect = create_draw_shape_layer_rect("R");
        if let DrawLayerNode::Shape(shape) = &mut rect {
            shape.rect = Some(DrawRect { x: 0.0, y: 0.0, width: 20.0, height: 30.0 });
        }
        doc.layers.push(rect);
        doc.layers.push(create_draw_trace_layer("Trace", "missing-source"));
        let artboard = resolve_draw_artboard(&doc).expect("artboard bounds");
        assert_eq!(artboard.width, 20.0);
        assert_eq!(artboard.height, 30.0);
    }

    #[test]
    fn default_draw_document_has_artboard_dimensions() {
        let doc = default_draw_document("blank", None);
        let artboard = doc.artboard.expect("default artboard");
        assert_eq!(artboard.width, 1024.0);
        assert_eq!(artboard.height, 1024.0);
    }

    #[test]
    fn layer_id_base_and_kind_label_cover_all_seven_variants() {
        let shape = create_draw_shape_layer_rect("Shape");
        let path = create_draw_path_layer("Path", Vec::new());
        let text = create_draw_text_layer("Text");
        let image = create_draw_image_layer("Image", "key");
        let group = create_draw_group_layer("Group");
        let boolean = create_draw_boolean_layer("Boolean", "union", Vec::new());
        let trace = create_draw_trace_layer("Trace", "src");
        for (layer, expected_kind) in [(&shape, "shape:rect"), (&path, "path"), (&text, "text"), (&image, "image"), (&group, "group"), (&boolean, "boolean"), (&trace, "trace")] {
            assert_eq!(layer_kind_label(layer), expected_kind);
            assert_eq!(layer_id(layer), layer_base(layer).id.as_str());
        }
    }

    #[test]
    fn find_draw_layer_locates_nested_child_and_returns_none_for_missing() {
        let child = create_draw_shape_layer_rect("Child");
        let child_id = layer_id(&child).to_string();
        let mut group = create_draw_group_layer("Group");
        if let DrawLayerNode::Group(body) = &mut group {
            body.children.push(child);
        }
        let mut doc = default_draw_document("nested", None);
        doc.layers = vec![group];
        assert!(find_draw_layer(&doc, &child_id).is_some());
        assert!(find_draw_layer(&doc, "missing-id").is_none());
    }

    #[test]
    fn flatten_draw_layers_includes_nested_group_children() {
        let child_a = create_draw_shape_layer_rect("A");
        let child_b = create_draw_text_layer("B");
        let mut group = create_draw_group_layer("Group");
        if let DrawLayerNode::Group(body) = &mut group {
            body.children.push(child_a);
            body.children.push(child_b);
        }
        let flat = flatten_draw_layers(std::slice::from_ref(&group));
        assert_eq!(flat.len(), 3);
    }

    #[test]
    fn draw_matrix_to_transform_round_trips_and_handles_zero_scale_x() {
        let transform = DrawTransform { x: 1.0, y: 2.0, scale_x: 2.0, scale_y: 3.0, rotation: std::f64::consts::FRAC_PI_6 };
        let matrix = draw_transform_to_matrix(&transform);
        let back = draw_matrix_to_transform(matrix);
        assert!((back.x - transform.x).abs() < 1e-9);
        assert!((back.y - transform.y).abs() < 1e-9);
        assert!((back.scale_x - transform.scale_x).abs() < 1e-9);
        assert!((back.scale_y - transform.scale_y).abs() < 1e-9);
        assert!((back.rotation - transform.rotation).abs() < 1e-9);

        let degenerate = draw_matrix_to_transform([0.0, 0.0, 5.0, 5.0, 1.0, 2.0]);
        assert_eq!(degenerate.scale_x, 0.0);
        assert_eq!(degenerate.scale_y, 0.0);
    }

    #[test]
    fn draw_play_layers_tree_row_id_formats_and_parses_back() {
        let shape = create_draw_shape_layer_rect("Shape");
        let id = layer_id(&shape).to_string();
        let row_id = draw_play_layers_tree_row_id(&shape);
        assert_eq!(row_id, format!("draw-play-layers.shape.{id}"));
        assert_eq!(draw_play_layer_id_from_tree_row_id(&row_id), Some(id));

        let child_row = draw_play_boolean_child_row_id("bool-1", "child-1");
        assert_eq!(child_row, "draw-play-layers.boolean.bool-1.child.child-1");
        assert_eq!(draw_play_layer_id_from_tree_row_id(&child_row), Some("child-1".to_string()));

        assert_eq!(draw_play_layer_id_from_tree_row_id("not-a-row-id"), None);
        assert_eq!(draw_play_layer_id_from_tree_row_id("draw-play-layers."), None);
    }

    #[test]
    fn layer_to_path_segments_covers_every_shape_kind_and_empty_polygon_and_unknown_kind() {
        let rect = create_draw_shape_layer_rect("Rect");
        assert!(!layer_to_path_segments(&rect).is_empty());

        let line = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0 }), polygon: None });
        assert_eq!(layer_to_path_segments(&line).len(), 2);

        let empty_polygon = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Poly"), shape_kind: "polygon".into(), rect: None, ellipse: None, circle: None, line: None, polygon: Some(DrawPolygon { points: Vec::new() }) });
        assert!(layer_to_path_segments(&empty_polygon).is_empty());

        let polygon =
            DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Poly"), shape_kind: "polygon".into(), rect: None, ellipse: None, circle: None, line: None, polygon: Some(DrawPolygon { points: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]] }) });
        assert_eq!(layer_to_path_segments(&polygon).len(), 4);

        let ellipse = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawEllipse { cx: 0.0, cy: 0.0, rx: 1.0, ry: 1.0 }), circle: None, line: None, polygon: None });
        assert_eq!(layer_to_path_segments(&ellipse).len(), 6);

        let circle = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Circle"), shape_kind: "circle".into(), rect: None, ellipse: None, circle: Some(DrawCircle { cx: 0.0, cy: 0.0, r: 1.0 }), line: None, polygon: None });
        assert_eq!(layer_to_path_segments(&circle).len(), 6);

        let unknown_kind = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Unknown"), shape_kind: "star".into(), rect: None, ellipse: None, circle: None, line: None, polygon: None });
        assert!(layer_to_path_segments(&unknown_kind).is_empty());

        let rect_missing_data = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("RectNoData"), shape_kind: "rect".into(), rect: None, ellipse: None, circle: None, line: None, polygon: None });
        assert!(layer_to_path_segments(&rect_missing_data).is_empty());

        let group = create_draw_group_layer("Group");
        assert!(layer_to_path_segments(&group).is_empty());
    }

    #[test]
    fn draw_layer_world_bounds_covers_text_image_default_and_none_branches() {
        let text = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("T"), x: 0.0, y: 0.0, content: "hi".into(), size: 10.0 });
        let (tx, ty, tw, th) = draw_layer_world_bounds(&text).expect("text bounds");
        assert_eq!((tx, ty), (0.0, 0.0));
        assert!(tw > 0.0 && th > 0.0);

        let image = create_draw_image_layer("Img", "key");
        let (_, _, iw, ih) = draw_layer_world_bounds(&image).expect("image bounds");
        assert_eq!((iw, ih), (256.0, 256.0));

        let empty_path = create_draw_path_layer("Empty", Vec::new());
        let bounds = draw_layer_world_bounds(&empty_path).expect("default bbox");
        assert_eq!(bounds, (-64.0, -64.0, 128.0, 128.0));

        let close_only = create_draw_path_layer("CloseOnly", vec![PathSegment::Close]);
        assert!(draw_layer_world_bounds(&close_only).is_none());
    }

    #[test]
    fn canvas_layer_records_excludes_groups_and_includes_bounds() {
        let child = create_draw_shape_layer_rect("Child");
        let mut group = create_draw_group_layer("Group");
        if let DrawLayerNode::Group(body) = &mut group {
            body.children.push(child);
        }
        let mut doc = default_draw_document("records", None);
        doc.layers = vec![group];
        let records = canvas_layer_records(&doc);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].kind, "shape:rect");
        assert!(records[0].width.is_some());
    }

    #[test]
    fn clone_draw_layer_node_assigns_new_ids_recursively_and_appends_suffix_only_at_top() {
        let shape = create_draw_shape_layer_rect("Rect");
        let clone = clone_draw_layer_node(&shape, " copy");
        assert_ne!(layer_id(&shape), layer_id(&clone));
        assert_eq!(layer_base(&clone).name, "Rect copy");

        let child = create_draw_shape_layer_rect("Child");
        let child_id = layer_id(&child).to_string();
        let mut group = create_draw_group_layer("Group");
        if let DrawLayerNode::Group(body) = &mut group {
            body.children.push(child);
        }
        let group_clone = clone_draw_layer_node(&group, " copy");
        let DrawLayerNode::Group(cloned_body) = &group_clone else { panic!("expected group") };
        assert_eq!(cloned_body.base.name, "Group copy");
        assert_ne!(layer_id(&cloned_body.children[0]), child_id);
        assert_eq!(layer_base(&cloned_body.children[0]).name, "Child");
    }

    #[test]
    fn transform_path_segments_transforms_every_segment_kind() {
        let segments = vec![
            PathSegment::Move { to: [1.0, 0.0] },
            PathSegment::Line { to: [1.0, 0.0] },
            PathSegment::Quad { ctrl: [1.0, 0.0], to: [1.0, 0.0] },
            PathSegment::Cubic { ctrl1: [1.0, 0.0], ctrl2: [1.0, 0.0], to: [1.0, 0.0] },
            PathSegment::Arc { rx: 1.0, ry: 1.0, rotation: 0.0, large_arc: false, sweep: true, to: [1.0, 0.0] },
            PathSegment::Close,
        ];
        let transform = DrawTransform { x: 10.0, y: 20.0, scale_x: 2.0, scale_y: 2.0, rotation: 0.0 };
        let transformed = transform_path_segments(&segments, &transform);
        match &transformed[0] {
            PathSegment::Move { to } => assert_eq!(*to, [12.0, 20.0]),
            other => panic!("expected move, got {other:?}"),
        }
        match &transformed[4] {
            PathSegment::Arc { to, rx, .. } => {
                assert_eq!(*to, [12.0, 20.0]);
                assert_eq!(*rx, 1.0);
            }
            other => panic!("expected arc, got {other:?}"),
        }
        assert!(matches!(transformed[5], PathSegment::Close));
    }

    #[test]
    fn scale_path_segments_returns_untouched_clone_for_identity_scale_and_scales_otherwise() {
        let segments = vec![PathSegment::Move { to: [1.0, 2.0] }, PathSegment::Line { to: [3.0, 4.0] }];
        assert_eq!(scale_path_segments(&segments, 1.0, 1.0), segments);
        let scaled = scale_path_segments(&segments, 2.0, 3.0);
        match &scaled[1] {
            PathSegment::Line { to } => assert_eq!(*to, [6.0, 12.0]),
            other => panic!("expected line, got {other:?}"),
        }
    }

    #[test]
    fn split_path_segments_by_contour_splits_on_move_and_handles_empty_input() {
        let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 0.0] }, PathSegment::Move { to: [5.0, 5.0] }, PathSegment::Line { to: [6.0, 5.0] }, PathSegment::Close];
        let contours = split_path_segments_by_contour(&segments);
        assert_eq!(contours.len(), 2);
        assert_eq!(contours[1].len(), 3);

        let empty_contours = split_path_segments_by_contour(&[]);
        assert_eq!(empty_contours, vec![Vec::<PathSegment>::new()]);
    }

    #[test]
    fn path_segments_bounds_is_none_when_no_segment_carries_an_endpoint() {
        assert!(path_segments_bounds(&[PathSegment::Close]).is_none());
        let bounds = path_segments_bounds(&[PathSegment::Move { to: [1.0, 1.0] }, PathSegment::Line { to: [4.0, 5.0] }]).expect("bounds");
        assert_eq!(bounds, (1.0, 1.0, 3.0, 4.0));
    }

    #[test]
    fn filter_path_segments_by_contour_area_keeps_all_for_non_positive_min_area_and_drops_small_contours() {
        let small = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }, PathSegment::Close];
        let big = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Line { to: [10.0, 10.0] }, PathSegment::Close];
        let mut combined = small.clone();
        combined.extend(big.clone());

        assert_eq!(filter_path_segments_by_contour_area(&combined, 0.0), combined);

        let filtered = filter_path_segments_by_contour_area(&combined, 4.0);
        assert_eq!(filtered, big);
    }

    #[test]
    fn flatten_curve_segments_falls_back_to_line_for_degenerate_arc_and_passes_other_kinds_through() {
        let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Arc { rx: 0.0, ry: 0.0, rotation: 0.0, large_arc: false, sweep: true, to: [5.0, 5.0] }, PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 2.0] }, PathSegment::Close];
        let flattened = flatten_curve_segments(&segments);
        assert!(matches!(flattened[1], PathSegment::Line { to } if to == [5.0, 5.0]));
        assert!(matches!(flattened[2], PathSegment::Quad { .. }));
        assert!(matches!(flattened[3], PathSegment::Close));
    }

    #[test]
    fn flatten_segments_to_lines_samples_quad_and_cubic_into_lines() {
        let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 0.0] }, PathSegment::Cubic { ctrl1: [2.0, 1.0], ctrl2: [3.0, 1.0], to: [4.0, 0.0] }];
        let flattened = flatten_segments_to_lines(&segments);
        assert!(flattened.iter().all(|segment| matches!(segment, PathSegment::Move { .. } | PathSegment::Line { .. })));
        assert_eq!(flattened.len(), 1 + CURVE_LINE_SAMPLE_STEPS * 2);
        match flattened.last().unwrap() {
            PathSegment::Line { to } => assert!((to[0] - 4.0).abs() < 1e-9 && (to[1] - 0.0).abs() < 1e-9),
            other => panic!("expected line, got {other:?}"),
        }
    }

    #[test]
    fn draw_layer_descendant_leaf_ids_flattens_nested_groups_to_leaves() {
        let leaf_a = create_draw_shape_layer_rect("A");
        let leaf_a_id = layer_id(&leaf_a).to_string();
        let leaf_b = create_draw_trace_layer("B", "src");
        let leaf_b_id = layer_id(&leaf_b).to_string();
        let mut inner_group = create_draw_group_layer("Inner");
        if let DrawLayerNode::Group(body) = &mut inner_group {
            body.children.push(leaf_a);
            body.children.push(leaf_b);
        }
        let leaf_c = create_draw_text_layer("C");
        let leaf_c_id = layer_id(&leaf_c).to_string();
        let mut outer_group = create_draw_group_layer("Outer");
        if let DrawLayerNode::Group(body) = &mut outer_group {
            body.children.push(inner_group);
            body.children.push(leaf_c);
        }
        assert_eq!(draw_layer_descendant_leaf_ids(&outer_group), vec![leaf_a_id, leaf_b_id, leaf_c_id]);

        let leaf = create_draw_shape_layer_rect("Solo");
        let leaf_id_value = layer_id(&leaf).to_string();
        assert_eq!(draw_layer_descendant_leaf_ids(&leaf), vec![leaf_id_value]);
    }

    #[test]
    fn resolve_boolean_layer_segments_returns_empty_for_missing_children_and_invalid_operation() {
        let mut doc = default_draw_document("bool-empty", None);
        doc.layers.clear();
        let boolean_missing = DrawBooleanBody { base: default_layer_base("B"), operation: "union".into(), children: vec!["missing".into()] };
        assert!(resolve_boolean_layer_segments(&doc, &boolean_missing).is_empty());

        let mut rect_a = create_draw_shape_layer_rect("A");
        if let DrawLayerNode::Shape(shape) = &mut rect_a {
            shape.rect = Some(DrawRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        }
        let id_a = layer_id(&rect_a).to_string();
        let mut rect_b = create_draw_shape_layer_rect("B");
        if let DrawLayerNode::Shape(shape) = &mut rect_b {
            shape.rect = Some(DrawRect { x: 2.0, y: 2.0, width: 5.0, height: 5.0 });
        }
        let id_b = layer_id(&rect_b).to_string();
        doc.layers.push(rect_a);
        doc.layers.push(rect_b);
        let boolean_invalid = DrawBooleanBody { base: default_layer_base("B"), operation: "not-a-real-op".into(), children: vec![id_a, id_b] };
        assert!(resolve_boolean_layer_segments(&doc, &boolean_invalid).is_empty());
    }

    #[test]
    fn decode_draw_image_asset_luma_handles_data_uri_prefix_resize_and_invalid_inputs() {
        let mut image_buffer = image::RgbaImage::new(4, 4);
        for pixel in image_buffer.pixels_mut() {
            *pixel = image::Rgba([255, 255, 255, 255]);
        }
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image_buffer).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
        let encoded = BASE64.encode(&bytes);

        let data_uri_asset = DrawImageAsset { mime: "image/png".into(), data: format!("data:image/png;base64,{encoded}"), width: None, height: None };
        let (w, h, luma) = decode_draw_image_asset_luma(&data_uri_asset).expect("decode data uri");
        assert_eq!((w, h), (4, 4));
        assert_eq!(luma.len(), 16);
        assert!(luma.iter().all(|&v| v == 255));

        let resized_asset = DrawImageAsset { mime: "image/png".into(), data: encoded.clone(), width: Some(8), height: Some(8) };
        let (rw, rh, rluma) = decode_draw_image_asset_luma(&resized_asset).expect("decode resized");
        assert_eq!((rw, rh), (8, 8));
        assert_eq!(rluma.len(), 64);

        let invalid_base64 = DrawImageAsset { mime: "image/png".into(), data: "not-base64!!".into(), width: None, height: None };
        assert!(decode_draw_image_asset_luma(&invalid_base64).is_none());

        let invalid_image = DrawImageAsset { mime: "image/png".into(), data: BASE64.encode(b"not a png"), width: None, height: None };
        assert!(decode_draw_image_asset_luma(&invalid_image).is_none());
    }

    #[test]
    fn resolve_draw_artboard_falls_back_to_layer_bounds_and_returns_none_when_no_bounds() {
        let mut doc = default_draw_document("artboard-fallback", None);
        doc.artboard = Some(DrawArtboard { width: 0.0, height: 0.0 });
        doc.layers.clear();
        let mut rect = create_draw_shape_layer_rect("R");
        if let DrawLayerNode::Shape(shape) = &mut rect {
            shape.rect = Some(DrawRect { x: 0.0, y: 0.0, width: 15.0, height: 25.0 });
        }
        doc.layers.push(rect);
        let artboard = resolve_draw_artboard(&doc).expect("fallback bounds");
        assert_eq!((artboard.width, artboard.height), (15.0, 25.0));

        doc.artboard = None;
        doc.layers.clear();
        doc.layers.push(create_draw_group_layer("EmptyGroup"));
        assert!(resolve_draw_artboard(&doc).is_none());
    }

    #[test]
    fn resolve_trace_layer_segments_returns_empty_without_assets_or_source_or_valid_decode() {
        let mut doc = default_draw_document("trace-empty", None);
        doc.layers.clear();
        doc.assets = None;
        let trace_no_assets = DrawTraceBody { base: default_layer_base("T"), source_key: "missing".into(), params: default_draw_trace_params() };
        assert!(resolve_trace_layer_segments(&doc, &trace_no_assets).is_empty());

        let mut assets = std::collections::BTreeMap::new();
        assets.insert("present".to_string(), DrawImageAsset { mime: "image/png".into(), data: "not-base64!!".into(), width: None, height: None });
        doc.assets = Some(assets);
        let trace_missing_key = DrawTraceBody { base: default_layer_base("T"), source_key: "missing".into(), params: default_draw_trace_params() };
        assert!(resolve_trace_layer_segments(&doc, &trace_missing_key).is_empty());

        let trace_bad_decode = DrawTraceBody { base: default_layer_base("T"), source_key: "present".into(), params: default_draw_trace_params() };
        assert!(resolve_trace_layer_segments(&doc, &trace_bad_decode).is_empty());
    }

    #[test]
    fn create_layer_by_kind_covers_all_known_kinds_and_fallbacks() {
        assert_eq!(layer_kind_label(&create_layer_by_kind("shape:rect")), "shape:rect");
        assert_eq!(layer_kind_label(&create_layer_by_kind("shape:ellipse")), "shape:ellipse");
        assert_eq!(layer_kind_label(&create_layer_by_kind("shape:line")), "shape:line");
        assert_eq!(layer_kind_label(&create_layer_by_kind("shape:polygon")), "shape:polygon");
        assert_eq!(layer_kind_label(&create_layer_by_kind("shape:unknown")), "shape:rect");
        assert_eq!(layer_kind_label(&create_layer_by_kind("path")), "path");
        assert_eq!(layer_kind_label(&create_layer_by_kind("text")), "text");
        assert_eq!(layer_kind_label(&create_layer_by_kind("image")), "image");
        assert_eq!(layer_kind_label(&create_layer_by_kind("group")), "group");
        assert_eq!(layer_kind_label(&create_layer_by_kind("boolean")), "boolean");
        assert_eq!(layer_kind_label(&create_layer_by_kind("trace")), "trace");
        assert_eq!(layer_kind_label(&create_layer_by_kind("nonsense")), "path");
    }

    #[test]
    fn hex_to_rgba_handles_short_and_long_hex_and_invalid_digits() {
        assert_eq!(hex_to_rgba("#fff", 1.0), [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(hex_to_rgba("#ff0000", 0.5), [1.0, 0.0, 0.0, 0.5]);
        assert_eq!(hex_to_rgba("#zzzzzz", 1.0), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn rgba_to_hex_round_trips_and_clamps_out_of_range_channels() {
        assert_eq!(rgba_to_hex([1.0, 0.0, 0.0, 1.0]), "#ff0000");
        assert_eq!(rgba_to_hex([-1.0, 2.0, 0.5, 1.0]), "#00ff80");
    }

    #[test]
    fn find_draw_layer_location_reports_parent_and_index_or_none_when_missing() {
        let child = create_draw_shape_layer_rect("Child");
        let child_id = layer_id(&child).to_string();
        let mut group = create_draw_group_layer("Group");
        if let DrawLayerNode::Group(body) = &mut group {
            body.children.push(child);
        }
        let group_id = layer_id(&group).to_string();
        let top_level = create_draw_text_layer("Top");
        let top_id = layer_id(&top_level).to_string();
        let mut doc = default_draw_document("locate", None);
        doc.layers = vec![group, top_level];

        let child_location = find_draw_layer_location(&doc, &child_id).expect("child location");
        assert_eq!(child_location.parent_id.as_deref(), Some(group_id.as_str()));
        assert_eq!(child_location.index, 0);

        let top_location = find_draw_layer_location(&doc, &top_id).expect("top location");
        assert_eq!(top_location.parent_id, None);
        assert_eq!(top_location.index, 1);

        assert!(find_draw_layer_location(&doc, "missing").is_none());
    }

    #[test]
    fn draw_document_to_svg_renders_shape_text_image_and_gradient_nodes() {
        let mut rect = create_draw_shape_layer_rect("Rect");
        if let DrawLayerNode::Shape(shape) = &mut rect {
            shape.base.attributes.fill = Some(FillStyle::Solid { color: [1.0, 0.0, 0.0, 0.5] });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "round".into(), join: "round".into(), dash: None });
        }
        let mut gradient_rect = create_draw_shape_layer_rect("Gradient");
        if let DrawLayerNode::Shape(shape) = &mut gradient_rect {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: Vec::new() });
        }
        let text = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("T"), x: 0.0, y: 0.0, content: "<a & b>".into(), size: 12.0 });
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("img".to_string(), DrawImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(4), height: Some(4) });
        let image = create_draw_image_layer("Image", "img");

        let mut doc = default_draw_document("svg-test", None);
        doc.layers = vec![rect, gradient_rect, text, image];
        doc.assets = Some(assets);
        doc.artboard = None;

        let (svg, width, height) = draw_document_to_svg(&doc);
        assert!(width >= 1 && height >= 1);
        assert!(svg.contains("rgba(255,0,0,0.500)"));
        assert!(svg.contains("fill=\"none\""));
        assert!(svg.contains("&lt;a &amp; b&gt;"));
        assert!(svg.contains("data:image/png;base64,aGVsbG8="));

        let json_error = draw_document_json_to_svg(&serde_json::json!({"bad": true}));
        assert!(json_error.is_err());
    }

    #[test]
    fn draw_document_json_to_dwg_bytes_errors_on_invalid_json_and_skips_invisible_layers() {
        let bad_json = serde_json::json!({"not": "a document"});
        assert!(draw_document_json_to_dwg_bytes(&bad_json).is_err());

        let mut hidden = create_draw_shape_layer_rect("Hidden");
        layer_base_mut(&mut hidden).visible = false;
        let mut doc = default_draw_document("hidden-only", None);
        doc.layers = vec![hidden];
        let value = serde_json::to_value(&doc).unwrap();
        let bytes = draw_document_json_to_dwg_bytes(&value).expect("export empty dwg");
        let drawing = semio_framework_core::dwg_from_bytes(&bytes).expect("decode dwg");
        assert!(drawing.entities.is_empty());
    }

    #[test]
    fn draw_document_json_from_dwg_falls_back_to_single_empty_layer_when_no_entities() {
        let drawing = semio_framework_core::DwgDrawing::default();
        let value = draw_document_json_from_dwg(&drawing).expect("import empty dwg");
        let doc: DrawDocument = serde_json::from_value(value).expect("valid document");
        assert_eq!(doc.layers.len(), 1);
        assert!(matches!(&doc.layers[0], DrawLayerNode::Path(body) if body.segments.is_empty()));
        assert_eq!(doc.artboard, Some(DrawArtboard { width: 1.0, height: 1.0 }));
    }
}
//#endregion 🧪️Tests
