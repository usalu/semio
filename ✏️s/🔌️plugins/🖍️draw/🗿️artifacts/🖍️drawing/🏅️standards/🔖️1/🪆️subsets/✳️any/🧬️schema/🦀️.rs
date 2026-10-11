//! 🧬️ Drawing artifact schema — every field of the artifact with its state class.

use crate::{
    default_drawing_trace_params, default_drawing_transform,  DrawingAttributes, DrawingBooleanBody, DrawingEllipse, DrawingGroupBody, DrawingImageBody, DrawingLayerBase, DrawingLine, DrawingMutation, DrawingPathBody, DrawingPolygon,
    DrawingRect, DrawingShapeBody, DrawingSnapshot, DrawingTextBody, DrawingTraceBody, DrawingTransform, FillStyle, PathSegment, StrokeStyle, DRAWING_DOCUMENT_SCHEMA};
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
        layers: vec![create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit(("initial-layer").to_string().into()).expect("nonempty authored identity"), "Layer 1", Default::default())].into(),
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
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
/// 🧬️ Descriptor for `s.draw.drawing` — twenty handcrafted schema leaves.
pub fn drawing_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.draw.drawing",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
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
    pub font_family: crate::DrawingFontFamily,
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




/// 📄️ Parses the handcrafted DSL fixture once per call — used both for `setActiveExample`'s in-plugin
/// document load and to bridge into the framework's still-JSON-only `App::example`/render-override
/// surfaces, so `SEMIO_DRAW_EXAMPLE_TEXT` stays the single source of truth for the fixture.





pub fn default_layer_base(identity: DrawingIdentity, name: &str) -> DrawingLayerBase {
    DrawingLayerBase { id: identity.into_key(), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() }
}



pub fn create_drawing_group_layer(identity: DrawingIdentity, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Group(DrawingGroupBody {
        isolation:false,
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
        children: Default::default(),
    })
}

pub fn create_drawing_boolean_layer(identity: DrawingIdentity, name: &str, operation: &str, children: semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>) -> DrawingLayerNode {
    DrawingLayerNode::Boolean(DrawingBooleanBody {
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
        operation: operation.into(),
        children,
    })
}

pub fn create_drawing_trace_layer(identity: DrawingIdentity, name: &str, source_key: &str) -> DrawingLayerNode {
    DrawingLayerNode::Trace(DrawingTraceBody {
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
        source_key: source_key.into(),
        params: default_drawing_trace_params(),
    })
}

pub fn create_drawing_shape_layer_rect(identity: DrawingIdentity, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Shape(DrawingShapeBody {
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
        shape_kind: "rect".into(),
        rect: Some(DrawingRect { x: 0.0, y: 0.0, width: 128.0, height: 96.0 }),
        ellipse: None,
        circle: None,
        line: None,
        polygon: None,
    })
}

pub fn create_drawing_text_layer(identity: DrawingIdentity, name: &str) -> DrawingLayerNode {
    DrawingLayerNode::Text(DrawingTextBody {
        base: DrawingLayerBase {
            id: identity.into_key(),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_drawing_transform(),
            attributes: DrawingAttributes { fill_rule: crate::FillRule::Evenodd, fill: Some(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }), stroke: None },
        },
        x: 0.0,
        y: 0.0,
        content: "Text".into(),
        size: 24.0,
        font_family:crate::DrawingFontFamily::Anta,
    })
}

pub fn create_drawing_image_layer(identity: DrawingIdentity, name: &str, image_key: &str) -> DrawingLayerNode {
    DrawingLayerNode::Image(DrawingImageBody {
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
        image_key: image_key.into(),
        width: 256.0,
        height: 256.0,
    })
}





pub fn layer_id(layer: &DrawingLayerNode) -> &semio_framework_value::paged::PagedUtf8<{usize::MAX}> {
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

pub fn find_drawing_layer<'a>(doc: &'a DrawingSnapshot, layer_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<&'a DrawingLayerNode> {
    for layer in &doc.layers {
        if let Some(found) = find_drawing_layer_in_node(layer, layer_id) {
            return Some(found);
        }
    }
    None
}

fn find_drawing_layer_in_node<'a>(node: &'a DrawingLayerNode, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<&'a DrawingLayerNode> {
    if layer_id(node).eq_text(target_id) {
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

pub fn selected_drawing_layers<'a>(document: &'a DrawingSnapshot, ids: &[String]) -> Vec<&'a DrawingLayerNode> {
    flatten_drawing_layers(&document.layers).into_iter().filter(|layer| ids.iter().any(|id| id == &layer_base(layer).id || id == &drawing_play_layers_tree_row_id(layer))).collect()
}

/// 🔒️ A lock on any ancestor protects its complete subtree from interactive edits.
pub fn drawing_layer_is_locked(document: &DrawingSnapshot, id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> bool {
    fn visit(layers: &semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), inherited: bool) -> Option<bool> {
        for layer in layers {
            let locked = inherited || layer_base(layer).locked;
            if layer_id(layer).eq_text(id) { return Some(locked); }
            if let DrawingLayerNode::Group(group) = layer {
                if let Some(found) = visit(&group.children, id, locked) { return Some(found); }
            }
        }
        None
    }
    visit(&document.layers, id, false).unwrap_or(false)
}

pub fn flatten_drawing_layers(layers: &semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>) -> Vec<&DrawingLayerNode> {
    let mut out = Vec::new();
    fn walk<'a>(nodes: &'a semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, out: &mut Vec<&'a DrawingLayerNode>) {
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

pub use geometry::affine::{drawing_transform_to_matrix,drawing_matrix_to_transform};

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

pub fn drawing_play_boolean_child_row_id(boolean_id: &(impl semio_framework_value::paged::Utf8Text + std::fmt::Display + ?Sized), child_id: &(impl std::fmt::Display + ?Sized)) -> String {
    format!("drawing-play-layers.boolean-child.{}.{boolean_id}{child_id}", boolean_id.text_bytes())
}

pub fn drawing_play_layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    let (kind, id) = row_id.strip_prefix("drawing-play-layers.")?.split_once('.')?;
    if kind == "boolean-child" {
        let (length, ids) = id.split_once('.')?;
        let child = ids.get(length.parse::<usize>().ok()?..)?;
        return (!child.is_empty()).then(|| child.to_string());
    }
    if !matches!(kind, "group" | "boolean" | "trace" | "path" | "shape" | "text" | "image") || id.is_empty() { return None; }
    Some(id.to_string())
}

pub fn layer_to_path_segments(layer: &DrawingLayerNode) -> Vec<PathSegment> {
    match layer {
        DrawingLayerNode::Path(path) => path.segments.iter().cloned().collect(),
        DrawingLayerNode::Shape(shape) => shape_to_path_segments(shape),
        _ => Vec::new(),
    }
}

fn ellipse_path_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> [PathSegment;6] {
    let arc=|to|PathSegment::Arc {rx:rx.abs(),ry:ry.abs(),rotation:0.0,large_arc:false,sweep:(rx>=0.0)==(ry>=0.0),to};
    [PathSegment::Move {to:[cx,cy-ry]},arc([cx+rx,cy]),arc([cx,cy+ry]),arc([cx-rx,cy]),arc([cx,cy-ry]),PathSegment::Close]
}

/// 🔷️ Reads one primitive contour segment without cloning a polygon.
pub fn shape_path_segment(shape:&DrawingShapeBody,index:usize)->Option<PathSegment> {
    match () {
        () if shape.shape_kind.eq_str("rect")=>shape.rect.as_ref().and_then(|r|[
            PathSegment::Move {to:[r.x,r.y]},PathSegment::Line {to:[r.x+r.width,r.y]},
            PathSegment::Line {to:[r.x+r.width,r.y+r.height]},PathSegment::Line {to:[r.x,r.y+r.height]},PathSegment::Close,
        ].get(index).cloned()),
        () if shape.shape_kind.eq_str("line")=>shape.line.as_ref().and_then(|l|[PathSegment::Move {to:[l.x1,l.y1]},PathSegment::Line {to:[l.x2,l.y2]}].get(index).cloned()),
        () if shape.shape_kind.eq_str("polygon")=>shape.polygon.as_ref().and_then(|p|p.points.get(index).map(|to|if index==0 {PathSegment::Move {to:*to}}else{PathSegment::Line {to:*to}}).or_else(||(!p.points.is_empty()&&index==p.points.len()).then_some(PathSegment::Close))),
        () if shape.shape_kind.eq_str("ellipse")=>shape.ellipse.as_ref().and_then(|e|ellipse_path_segments(e.cx,e.cy,e.rx,e.ry).get(index).cloned()),
        () if shape.shape_kind.eq_str("circle")=>shape.circle.as_ref().and_then(|c|ellipse_path_segments(c.cx,c.cy,c.r,c.r).get(index).cloned()),
        _=>None,
    }
}

fn shape_to_path_segments(shape:&DrawingShapeBody)->Vec<PathSegment> {
    let mut index=0;
    std::iter::from_fn(||{let segment=shape_path_segment(shape,index);index+=1;segment}).collect()
}

pub fn drawing_layer_world_bounds(layer: &DrawingLayerNode) -> Option<(f64, f64, f64, f64)> {
    drawing_layer_bounds_with_parent(layer,[1.0,0.0,0.0,1.0,0.0,0.0])
}

/// 🌍️ Measures complete geometry in the coordinate system of its ancestor matrix.
pub fn drawing_layer_bounds_with_parent(layer: &DrawingLayerNode, parent: [f64;6]) -> Option<(f64,f64,f64,f64)> {
    let matrix = geometry::multiply(parent, drawing_transform_to_matrix(&layer_base(layer).transform));
    if let DrawingLayerNode::Group(group) = layer {
        return group.children.iter().filter_map(|child| drawing_layer_bounds_with_parent(child, matrix)).reduce(|a, b| {
            let x = a.0.min(b.0);
            let y = a.1.min(b.1);
            (x, y, (a.0+a.2).max(b.0+b.2)-x, (a.1+a.3).max(b.1+b.3)-y)
        });
    }
    let rectangle = match layer {
        DrawingLayerNode::Text(text) => {
            let [width, height] = semio_framework_2d::text::drawing_text_fallback_extent(&text.content, text.size);
            Some((text.x, text.y, width.max(8.0), height.max(8.0)))
        },
        DrawingLayerNode::Image(image) => Some((0.0, 0.0, image.width, image.height)),
        _ => None,
    };
    let segments = if let Some((x, y, w, h)) = rectangle {
        vec![PathSegment::Move { to: [x,y] }, PathSegment::Line { to: [x+w,y] }, PathSegment::Line { to: [x+w,y+h] }, PathSegment::Line { to: [x,y+h] }, PathSegment::Close]
    } else { layer_to_path_segments(layer) };
    path_segments_bounds_with_matrix(&segments, matrix)
}

fn segment_to_point(segment: &PathSegment) -> Option<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => Some(*to),
        PathSegment::Close => None,
    }
}

fn scene_node_for_path(base: &DrawingLayerBase, segments: Vec<PathSegment>) -> DrawingSceneNode {
    DrawingSceneNode {
        id: base.id.to_string_owner(),
        groups:Vec::new(),
        transform: drawing_transform_to_matrix(&base.transform),
        segments,
        fill: base.attributes.fill.clone(),
        stroke: base.attributes.stroke.clone(),
        opacity: base.opacity,
        blend_mode: base.blend_mode.to_string_owner(),
        visible: base.visible,
        fill_rule: Some(base.attributes.fill_rule.as_str().into()),
        text: None,
        image: None,
    }
}

pub fn flatten_drawing_document_to_scene_nodes(doc: &DrawingSnapshot) -> Vec<DrawingSceneNode> {
    flatten_drawing_document_with_transformation(doc,None)
}

/// ↔️ Preview world-space transforms through the same group traversal as the committed scene.
pub fn flatten_drawing_document_with_transformation(doc: &DrawingSnapshot, transformation: Option<&(Vec<String>,[f64;6])>) -> Vec<DrawingSceneNode> {
    let mut out = Vec::new();
    fn walk(doc: &DrawingSnapshot, layers: &semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, parent: [f64; 6], transformation: Option<&(Vec<String>,[f64;6])>, groups:&mut Vec<DrawingSceneGroup>, out: &mut Vec<DrawingSceneNode>) {
        for layer in layers {
            let base = layer_base(layer);
            if !base.visible {
                continue;
            }
            let mut parent=parent;
            if let Some((ids,matrix))=transformation {
                if ids.iter().any(|id| base.id.eq_str(id)) { parent=geometry::multiply(*matrix,parent); }
            }
            let first = out.len();
            match layer {
                DrawingLayerNode::Group(group) => {
                    let isolated=group.isolation || base.opacity!=1.0 || base.blend_mode!="normal";
                    if isolated {groups.push(DrawingSceneGroup {id:base.id.to_string_owner(),opacity:base.opacity,blend_mode:base.blend_mode.to_string_owner()});}
                    walk(doc, &group.children, geometry::multiply(parent, drawing_transform_to_matrix(&base.transform)), transformation, groups, out);
                    if isolated {groups.pop();}
                    continue;
                }
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
                    id: text.base.id.to_string_owner(),
                    groups:Vec::new(),
                    transform: geometry::multiply(drawing_transform_to_matrix(&text.base.transform), [1.0, 0.0, 0.0, 1.0, text.x, text.y]),
                    segments: Default::default(),
                    fill: text.base.attributes.fill.clone(),
                    stroke: text.base.attributes.stroke.clone(),
                    opacity: text.base.opacity,
                    blend_mode: text.base.blend_mode.to_string_owner(),
                    visible: text.base.visible,
                    fill_rule: None,
                    text: Some(DrawingSceneText { content: text.content.to_string_owner(), size: text.size,font_family:text.font_family }),
                    image: None,
                }),
                DrawingLayerNode::Image(image) => {
                    let asset_id = image.image_key.to_string_owner();
                    out.push(DrawingSceneNode {
                        id: image.base.id.to_string_owner(),
                        groups:Vec::new(),
                        transform: drawing_transform_to_matrix(&image.base.transform),
                        segments: Default::default(),
                        fill: image.base.attributes.fill.clone(),
                        stroke: image.base.attributes.stroke.clone(),
                        opacity: image.base.opacity,
                        blend_mode: image.base.blend_mode.to_string_owner(),
                        visible: image.base.visible,
                        fill_rule: None,
                        text: None,
                        image: Some(DrawingSceneImage { asset_id, width: image.width, height: image.height }),
                    });
                }
                _ => {
                    let segments = layer_to_path_segments(layer);
                    if segments.is_empty() {
                        continue;
                    }
                    out.push(scene_node_for_path(base, segments));
                }
            }
            for node in &mut out[first..] { node.transform = geometry::multiply(parent, node.transform); node.groups=groups.clone(); }
        }
    }
    walk(doc, &doc.layers, [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], transformation, &mut Vec::new(), &mut out);
    out
}

pub fn canvas_layer_records(doc: &DrawingSnapshot) -> Vec<DrawingCanvasLayerRecord> {
    flatten_drawing_layers(&doc.layers)
        .into_iter()
        .filter(|layer| !matches!(layer, DrawingLayerNode::Group(_)))
        .map(|layer| {
            let base = layer_base(layer);
            let bounds = drawing_layer_world_bounds(layer);
            DrawingCanvasLayerRecord { id: base.id.to_string_owner(), kind: layer_kind_label(layer), name: base.name.to_string_owner(), x: bounds.map(|b| b.0), y: bounds.map(|b| b.1), width: bounds.map(|b| b.2), height: bounds.map(|b| b.3) }
        })
        .collect()
}

pub fn clone_drawing_layer_node(node: &DrawingLayerNode, name_suffix: &str, identities: &semio_framework_value::list::PagedList<DrawingIdentityAssignment,{usize::MAX}>) -> Result<DrawingLayerNode,semio_framework_value::ValueError> {
    fn identify(node: &mut DrawingLayerNode, identities: &semio_framework_value::list::PagedList<DrawingIdentityAssignment,{usize::MAX}>, ids: &mut BTreeMap<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, semio_framework_value::paged::PagedUtf8<{usize::MAX}>>) -> Result<(),semio_framework_value::ValueError> {
        let base=layer_base_mut(node);let old=base.id.clone();
        let mut matches=identities.iter().filter(|assignment|assignment.source==old);
        let assignment=matches.next().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing clone identity missing"))?;
        if matches.next().is_some()||assignment.target.is_empty()||assignment.target==old||ids.contains_key(&old)||ids.values().any(|target|*target==assignment.target){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing clone identity repeated or invalid"));}
        base.id=assignment.target.clone();ids.insert(old,base.id.clone());
        if let DrawingLayerNode::Group(group)=node{for child in &mut group.children{identify(child,identities,ids)?;}}Ok(())
    }
    fn references(node:&mut DrawingLayerNode,ids:&BTreeMap<semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>>){
        match node{DrawingLayerNode::Boolean(boolean)=>{for child in &mut boolean.children{if let Some(id)=ids.get(child){*child=id.clone();}}},DrawingLayerNode::Group(group)=>{for child in &mut group.children{references(child,ids);}},_=>{}}
    }
    let mut cloned=node.clone();let mut ids=BTreeMap::new();identify(&mut cloned,identities,&mut ids)?;
    if identities.len()!=ids.len()||ids.values().any(|target|ids.contains_key(target)){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing clone identity census differs"));}
    layer_base_mut(&mut cloned).name.push_str(name_suffix);references(&mut cloned,&ids);Ok(cloned)
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

pub fn mutate_drawing_layer(doc: &DrawingSnapshot, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), mutator: impl FnMut(&mut DrawingLayerNode)) -> DrawingSnapshot {
    let mut next = doc.clone();
    let mut mutator = mutator;
    update_layer_in_tree(&mut next.layers, target_id, &mut mutator);
    next
}

pub fn update_layer_in_tree(layers: &mut semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), mutator: &mut impl FnMut(&mut DrawingLayerNode)) -> bool {
    for layer in layers.iter_mut() {
        if layer_id(layer).eq_text(target_id) {
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

pub fn remove_layer_from_tree(layers: &mut semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> bool {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer).eq_text(target_id)) {
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

pub fn extract_layer_node(layers: &mut semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<DrawingLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer).eq_text(target_id)) {
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

pub fn insert_layer(layers: &mut semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<&str>, index: usize, node: DrawingLayerNode) {
    if let Some(parent_id) = parent_id {
        if !insert_layer_in_parent(layers, parent_id, index, node.clone()) {
            layers.push(node);
        }
    } else {
        let at = index.min(layers.len());
        layers.insert(at, node);
    }
}

fn insert_layer_in_parent(layers: &mut semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: &str, index: usize, node: DrawingLayerNode) -> bool {
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

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingLayerLocation {
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub index: usize,
}

pub fn find_drawing_layer_location(doc: &DrawingSnapshot, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<DrawingLayerLocation> {
    fn search(layers: &semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<DrawingLayerLocation> {
        for (index, layer) in layers.iter().enumerate() {
            if layer_id(layer).eq_text(target_id) {
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

pub fn create_layer_by_kind(identity: DrawingIdentity, kind: &str) -> DrawingLayerNode {
    if let Some(shape_kind) = kind.strip_prefix("shape:") {
        return match shape_kind {
            "rect" => create_drawing_shape_layer_rect(identity, "Rectangle"),
            "ellipse" => DrawingLayerNode::Shape(DrawingShapeBody {
                base: default_layer_base(identity, "Ellipse"),
                shape_kind: "ellipse".into(),
                rect: None,
                ellipse: Some(DrawingEllipse { cx: 0.0, cy: 0.0, rx: 64.0, ry: 48.0 }),
                circle: None,
                line: None,
                polygon: None,
            }),
            "line" => DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(identity, "Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawingLine { x1: 0.0, y1: 0.0, x2: 128.0, y2: 0.0 }), polygon: None }),
            "polygon" => DrawingLayerNode::Shape(DrawingShapeBody {
                base: default_layer_base(identity, "Polygon"),
                shape_kind: "polygon".into(),
                rect: None,
                ellipse: None,
                circle: None,
                line: None,
                polygon: Some(DrawingPolygon { points: vec![[0.0, 0.0], [64.0, 0.0], [32.0, 48.0]].into() }),
            }),
            _ => create_drawing_shape_layer_rect(identity, "Shape"),
        };
    }
    match kind {
        "path" => create_drawing_path_layer(identity, "Path", Vec::new().into()),
        "text" => create_drawing_text_layer(identity, "Text"),
        "image" => create_drawing_image_layer(identity, "Image", "image-source"),
        "group" => create_drawing_group_layer(identity, "Group"),
        "boolean" => create_drawing_boolean_layer(identity, "Boolean", "union", Vec::new().into()),
        "trace" => create_drawing_trace_layer(identity, "Trace", "trace-source"),
        _ => create_drawing_path_layer(identity, "Path", Vec::new().into()),
    }
}


//#endregion 🔖️Tree

//#region 🔖️SegmentGeometry
fn drawing_map_point_by_matrix(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

pub fn transform_path_segments(segments: &[PathSegment], transform: &DrawingTransform) -> Vec<PathSegment> {
    transform_path_by_matrix(segments, drawing_transform_to_matrix(transform))
}

pub fn transform_path_by_matrix(segments: &[PathSegment], matrix: [f64; 6]) -> Vec<PathSegment> {
    flatten_curve_segments(segments)
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
    transform_path_segments(segments, &DrawingTransform { x: 0.0, y: 0.0, scale_x, scale_y, rotation: 0.0, shear: 0.0 })
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

#[path = "🧮️geometry/🦀️.rs"]
pub mod geometry;
#[path = "🎬️scene/📷️raster/🦀️.rs"]
pub mod scene_raster;
#[path = "🎬️scene/📋️prepare/🦀️.rs"]
pub mod scene_preparation;
#[path = "🎬️scene/🔤️text/🦀️.rs"]
pub mod scene_text;
#[path = "🎬️scene/🔀️booleans/🦀️.rs"]
pub mod scene_booleans;
#[path = "🎬️scene/🔍️trace/🦀️.rs"]
pub mod scene_trace;
#[path = "🎬️scene/🧹️retire/🦀️.rs"]
pub mod scene_retirement;
#[path = "🎬️scene/🎨️paint/🦀️.rs"]
pub mod scene_paint;
#[path = "🎬️scene/📍️placement/🦀️.rs"]
pub mod scene_placement;

pub fn path_segments_bounds(segments: &[PathSegment]) -> Option<(f64, f64, f64, f64)> {
    path_segments_bounds_with_matrix(segments,[1.0,0.0,0.0,1.0,0.0,0.0])
}

pub(crate) fn path_segments_bounds_with_matrix(segments: &[PathSegment], matrix: [f64;6]) -> Option<(f64, f64, f64, f64)> {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    let mut current = [0.0; 2];
    let mut start = current;
    for segment in segments {
        if matches!(segment, PathSegment::Close) && !min[0].is_finite() { continue; }
        let bounds = geometry::segment_bounds(segment,current,start,matrix);
        match segment {
            PathSegment::Move { to } => { current=*to; start=*to; }
            PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => current=*to,
            PathSegment::Close => current=start,
        }
        for axis in 0..2 { min[axis] = min[axis].min(bounds[axis]); max[axis] = max[axis].max(bounds[axis]+bounds[axis+2]); }
    }
    min.iter().all(|n| n.is_finite()).then_some((min[0], min[1], max[0]-min[0], max[1]-min[1]))
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

fn arc_approx_unit_arc(ang1: f64, ang2: f64) -> ([f64; 2], [f64; 2], [f64; 2]) {
    let a = (4.0 / 3.0) * ((ang2 - ang1) / 4.0).tan();
    let (sin1, cos1) = ang1.sin_cos();
    let (sin2, cos2) = ang2.sin_cos();
    ([cos1 - sin1 * a, sin1 + cos1 * a], [cos2 + sin2 * a, sin2 - cos2 * a], [cos2, sin2])
}

/// 🌙️ Converts one SVG endpoint-parameterized arc into cubic Bézier control triples (SVG spec F.6.5).
fn arc_segment_to_cubics(from: [f64; 2], rx: f64, ry: f64, rotation_deg: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Vec<([f64; 2], [f64; 2], [f64; 2])> {
    let Some(arc) = geometry::arc_geometry(from, [rx.abs(),ry.abs()], rotation_deg, large_arc, sweep, to) else { return Vec::new() };
    let [rx, ry] = arc.radii;
    let [cx, cy] = arc.center;
    let (sin_phi, cos_phi) = arc.rotation.sin_cos();
    let ang1 = arc.start;
    let ang2 = arc.sweep;
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
pub(crate) fn to_kernel_segment(segment: &PathSegment) -> semio_framework_2d::PathSegment {
    use semio_framework_2d::PathSegment as KernelSegment;
    match segment {
        PathSegment::Move { to } => KernelSegment::Move { to: *to },
        PathSegment::Line { to } => KernelSegment::Line { to: *to },
        PathSegment::Quad { ctrl, to } => KernelSegment::Quad { ctrl: *ctrl, to: *to },
        PathSegment::Cubic { ctrl1, ctrl2, to } => KernelSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => KernelSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        PathSegment::Close => KernelSegment::Close,
    }
}

pub(crate) fn from_kernel_segment(segment: &semio_framework_2d::PathSegment) -> PathSegment {
    use semio_framework_2d::PathSegment as KernelSegment;
    match segment {
        KernelSegment::Move { to } => PathSegment::Move { to: *to },
        KernelSegment::Line { to } => PathSegment::Line { to: *to },
        KernelSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        KernelSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        KernelSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        KernelSegment::Close => PathSegment::Close,
    }
}

fn to_kernel_segments(segments: &[PathSegment]) -> Vec<semio_framework_2d::PathSegment> {
    segments.iter().map(to_kernel_segment).collect()
}

fn from_kernel_segments(segments: &[semio_framework_2d::PathSegment]) -> Vec<PathSegment> {
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
    let kernel_inputs: Vec<Vec<semio_framework_2d::PathSegment>> = child_segments.iter().map(|segments| to_kernel_segments(segments)).collect();
    let operation = crate::DRAWING_BOOLEAN_OPERATIONS.iter().copied().find(|operation| boolean.operation.eq_str(operation));
    let Some(operation) = operation else { return Vec::new(); };
    match semio_framework_2d::booleans::boolean_paths_many(&kernel_inputs, operation) {
        Ok(result) => from_kernel_segments(&result),
        Err(_) => Vec::new(),
    }
}

/// 🖼️ Project intrinsic ordered RGBA samples into the first-party raster vocabulary.
pub fn drawing_image_samples(asset: &DrawingImageAsset) -> Option<semio_framework_pixels::RasterImage> {
    let count=(asset.width as usize).checked_mul(asset.height as usize)?;
    if count==0||count>16_777_216||asset.samples.len()!=count{return None;}
    let pixels=asset.samples.iter().flat_map(|sample|sample.iter().copied()).collect();
    Some(semio_framework_pixels::RasterImage{width:asset.width,height:asset.height,pixels})
}

/// 🌗️ Alpha-weighted luminance is pure sample algebra independent of physical admission.
fn drawing_image_asset_luma(asset: &DrawingImageAsset) -> Option<(u32,u32,Vec<u8>)> {
    let count=(asset.width as usize).checked_mul(asset.height as usize)?;
    if count==0||count>16_777_216||asset.samples.len()!=count{return None;}
    let luma=asset.samples.iter().map(|[r,g,b,a]|((299.0*(*r as f64)+587.0*(*g as f64)+114.0*(*b as f64))*(*a as f64)/255000.0).round()as u8).collect();
    Some((asset.width,asset.height,luma))
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
    let Some((width, height, luma)) = drawing_image_asset_luma(asset) else { return Vec::new() };
    let traced = match semio_framework_2d::trace::trace_bitmap_paths(width, height, &luma, trace.params.threshold, trace.params.simplify_epsilon) {
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

#[path="🎬️scene/🪪️identity/🦀️.rs"]
pub mod scene_identity;

#[path="🎬️scene/👁️view/🦀️.rs"]
pub mod scene_view;

#[path="📝️text/🔤️family/🦀️.rs"]
pub mod font_family;
