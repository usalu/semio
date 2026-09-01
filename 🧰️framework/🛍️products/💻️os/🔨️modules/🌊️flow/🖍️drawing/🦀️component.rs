//! 🖍️ Flow 2D drawing kernel — ephemeral node-evaluation scene-graph kernel (content-addressed via
//! the OS `EngineCache`, mirroring `📐️brep-geometry`'s own in-process `Brep` kernel precedent) plus
//! its JSON bridge for the flow `draw` operator extension.
//!
//! 🪦 Relocated verbatim from the framework's `🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs`
//! (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS) — that file was a
//! **parallel non-artifact store** living in shared framework surface, duplicating what should be
//! (and, for the persisted drawing document, now is) `✳️drawing`'s real `ArtifactStore` + 17
//! mutation triads + `🎛flattened-scene` inference. This kernel is NOT that persisted document: it
//! is flow's own ephemeral, per-evaluation scratch geometry (shapes/booleans/gradients/text/trace/
//! DWG round-trip built and discarded while a node graph runs) — the same ephemeral-compute role
//! `📐️brep-geometry`'s own `Brep` kernel already legitimately plays for brep, unaffected by this
//! ticket. It now lives here, private to flow, rather than as shared framework API: nothing outside
//! flow's own two files (`💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs`, this file, and
//! `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`) ever referenced it. `PathSegment`/
//! `Vec2`/`DrawingError` remain shared framework surface
//! (`semio_framework_2d::{...}`) — genuinely generic geometry-kernel primitives also used by the
//! framework's own `booleans`/`trace` pure-function kernels and by the unrelated `🖍️draw` plugin.

use neural_engine as neural;

use std::collections::HashSet;
use std::fmt;
use std::sync::{LazyLock, Mutex};

use neural::EvalError;
use serde::{Deserialize, Serialize};

// #region 🔖️KernelTypes
/// 🧭️ Drawing entity kind carried by a handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DrawingKind {
    Rect,
    Ellipse,
    Circle,
    Line,
    Polygon,
    Path,
    Text,
    Group,
}

/// 🧭️ Opaque content-addressed drawing handle (hex-encoded OS engine key).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DrawingHandle(pub String);

impl DrawingHandle {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🎨️ Gradient color stop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

/// 🪣️ Fill style.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FillStyle {
    Solid { color: [f64; 4] },
    LinearGradient { x1: f64, y1: f64, x2: f64, y2: f64, stops: Vec<GradientStop> },
    RadialGradient { cx: f64, cy: f64, r: f64, stops: Vec<GradientStop> },
}

/// 🖌️ Stroke style.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrokeStyle {
    pub color: [f64; 4],
    pub width: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    #[serde(default)]
    pub dash: Vec<f64>,
}

/// 🔚️ Line cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// 🔗️ Line join.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// 📐️ Affine 2D transform `[a,b,c,d,e,f]` mapping `(x,y)` to `(a*x+c*y+e, b*x+d*y+f)`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Affine2D(pub [f64; 6]);

impl Default for Affine2D {
    fn default() -> Self {
        Self([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }
}

impl Affine2D {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn translate(dx: f64, dy: f64) -> Self {
        Self([1.0, 0.0, 0.0, 1.0, dx, dy])
    }

    pub fn rotate(angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self([cos, sin, -sin, cos, 0.0, 0.0])
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self([sx, 0.0, 0.0, sy, 0.0, 0.0])
    }

    pub fn multiply(self, other: Self) -> Self {
        let [a, b, c, d, e, f] = self.0;
        let [a2, b2, c2, d2, e2, f2] = other.0;
        Self([a * a2 + c * b2, b * a2 + d * b2, a * c2 + c * d2, b * c2 + d * d2, a * e2 + c * f2 + e, b * e2 + d * f2 + f])
    }

    pub fn transform_point(self, point: semio_framework_2d::Vec2) -> semio_framework_2d::Vec2 {
        let [a, b, c, d, e, f] = self.0;
        [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
    }
}

/// 🧩️ Scene-graph node variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DrawingNode {
    Rect { x: f64, y: f64, width: f64, height: f64 },
    Ellipse { cx: f64, cy: f64, rx: f64, ry: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Polygon { points: Vec<semio_framework_2d::Vec2> },
    Path { segments: Vec<semio_framework_2d::PathSegment> },
    Text { x: f64, y: f64, content: String, size: f64 },
    Group { children: Vec<String> },
}

/// 🖼️ Flattened scene node for preview and export.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneNode {
    pub transform: Affine2D,
    pub node: DrawingNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeStyle>,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<Vec<semio_framework_2d::PathSegment>>,
}

fn default_opacity() -> f64 {
    1.0
}

/// 🎬️ Serializable drawing scene transfer type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrawingScene {
    pub width: f64,
    pub height: f64,
    pub nodes: Vec<SceneNode>,
}

impl Default for DrawingScene {
    fn default() -> Self {
        Self { width: 512.0, height: 512.0, nodes: Vec::new() }
    }
}
// #endregion 🔖️KernelTypes

// #region 🔖️KernelTrait
/// 🔌️ Model-free synchronous 2D drawing kernel interface for flow's ephemeral node-evaluation
/// contract, implemented only by [`DrawingStore`] below.
pub trait DrawingKernel {
    // #region Primitives
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn polygon(&mut self, points: &[semio_framework_2d::Vec2]) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Primitives

    // #region Paths
    fn polyline_path(&mut self, points: &[semio_framework_2d::Vec2]) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Paths

    // #region Style
    fn set_fill(&mut self, handle: &DrawingHandle, fill: FillStyle) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn set_stroke(&mut self, handle: &DrawingHandle, stroke: StrokeStyle) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn linear_gradient_fill(&mut self, handle: &DrawingHandle, x1: f64, y1: f64, x2: f64, y2: f64, stops: &[GradientStop]) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Style

    // #region Transforms
    fn translate(&mut self, handle: &DrawingHandle, dx: f64, dy: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn rotate(&mut self, handle: &DrawingHandle, angle: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn scale(&mut self, handle: &DrawingHandle, sx: f64, sy: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Transforms

    // #region Group
    fn group(&mut self, children: &[DrawingHandle]) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Group

    // #region Booleans
    fn bool_union(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn bool_difference(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn bool_intersection(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn bool_xor(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn bool_op_many(&mut self, operation: &str, handles: &[DrawingHandle]) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    fn boolean_segments(&self, a: &[semio_framework_2d::PathSegment], b: &[semio_framework_2d::PathSegment], operation: &str) -> Result<Vec<semio_framework_2d::PathSegment>, semio_framework_2d::DrawingError>;
    // #endregion Booleans

    // #region Trace
    fn trace_bitmap(&mut self, width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Trace

    // #region Text
    fn text(&mut self, x: f64, y: f64, content: &str, size: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Text

    // #region Clip
    fn apply_clip(&mut self, target: &DrawingHandle, clip: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Clip

    // #region Export
    fn flatten_scene(&self, handle: &DrawingHandle) -> Result<DrawingScene, semio_framework_2d::DrawingError>;
    fn export_svg(&self, handle: &DrawingHandle) -> Result<String, semio_framework_2d::DrawingError>;
    fn export_pdf(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError>;
    fn export_dwg(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError>;
    fn import_dwg(&mut self, data: &[u8]) -> Result<DrawingHandle, semio_framework_2d::DrawingError>;
    // #endregion Export

    // #region Core
    fn kind(&self, handle: &DrawingHandle) -> Result<DrawingKind, semio_framework_2d::DrawingError>;
    fn dispose(&mut self, handle: &DrawingHandle);
    fn retain_sync(&mut self, live: &HashSet<String>);
    // #endregion Core
}
// #endregion 🔖️KernelTrait

// #region 🔖️Engine
use semio_framework_2d::Engine as _;

/// ⚙️ Content-addressed drawing node engine (`ENGINE_ID = "s.2d.drawing"`).
pub struct DrawingEngine;

impl semio_framework_2d::Engine for DrawingEngine {
    const ENGINE_ID: &'static str = "s.2d.drawing";

    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, semio_framework_2d::EngineFault> {
        let node = StoredNode::decode_pack(input).map_err(semio_framework_2d::EngineFault::InvalidInput)?;
        StoredNode::encode_pack(&node).map_err(semio_framework_2d::EngineFault::InvalidInput)
    }
}

const ENGINE_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;

fn map_engine_fault(fault: semio_framework_2d::EngineFault) -> semio_framework_2d::DrawingError {
    match fault {
        semio_framework_2d::EngineFault::Evicted => semio_framework_2d::DrawingError::MissingHandle("engine cache evicted handle".into()),
        semio_framework_2d::EngineFault::InvalidInput(message) => semio_framework_2d::DrawingError::InvalidInput(message),
        semio_framework_2d::EngineFault::Compute(message) => semio_framework_2d::DrawingError::Operation(message),
        semio_framework_2d::EngineFault::UnknownEngine(message) => semio_framework_2d::DrawingError::Operation(message),
    }
}

fn drawing_handle_from_key(key: semio_framework_2d::EngineKey) -> DrawingHandle {
    DrawingHandle(hex_encode(&key.0))
}

fn kernel_handle_for_drawing(handle: &DrawingHandle) -> Result<semio_framework_2d::KernelEngineHandle, semio_framework_2d::DrawingError> {
    let bytes = hex_decode(handle.as_str()).map_err(|_| semio_framework_2d::DrawingError::InvalidInput("invalid drawing handle hex".into()))?;
    if bytes.len() != 32 {
        return Err(semio_framework_2d::DrawingError::InvalidInput("drawing handle length".into()));
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes);
    Ok(semio_framework_2d::KernelEngineHandle { key: semio_framework_2d::EngineKey(key), engine_id: DrawingEngine::ENGINE_ID.to_string() })
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(text: &str) -> Result<Vec<u8>, ()> {
    if !text.len().is_multiple_of(2) {
        return Err(());
    }
    let mut out = Vec::with_capacity(text.len() / 2);
    for index in (0..text.len()).step_by(2) {
        let byte = u8::from_str_radix(&text[index..index + 2], 16).map_err(|_| ())?;
        out.push(byte);
    }
    Ok(out)
}
// #endregion 🔖️Engine

// #region 🔖️Store
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct StoredNode {
    kind: DrawingKind,
    node: DrawingNode,
    transform: Affine2D,
    fill: Option<FillStyle>,
    stroke: Option<StrokeStyle>,
    clip: Option<Vec<semio_framework_2d::PathSegment>>,
    opacity: f64,
}

impl StoredNode {
    fn encode_pack(node: &StoredNode) -> Result<Vec<u8>, String> {
        serde_json::to_vec(node).map_err(|error| error.to_string())
    }

    fn decode_pack(bytes: &[u8]) -> Result<StoredNode, String> {
        serde_json::from_slice(bytes).map_err(|error| error.to_string())
    }
}

/// 🗄️ [`DrawingKernel`] using engine derive through the WIT `engine-derive`/`engine-read` guest↔host
/// boundary (process-local [`semio_framework_2d::EngineCache`]) — flow's own ephemeral node-evaluation kernel,
/// not a persisted artifact store.
pub struct DrawingStore {
    cache: semio_framework_2d::EngineCache,
    live: HashSet<String>,
}

impl Default for DrawingStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawingStore {
    pub fn new() -> Self {
        let mut cache = semio_framework_2d::EngineCache::new(ENGINE_CACHE_BUDGET_BYTES);
        cache.register(DrawingEngine);
        Self { cache, live: HashSet::new() }
    }

    /// 🧩 Attach to an existing host-owned cache (registers [`DrawingEngine`]).
    pub fn with_engine_cache(mut cache: semio_framework_2d::EngineCache) -> Self {
        cache.register(DrawingEngine);
        Self { cache, live: HashSet::new() }
    }

    /// 🧠 Mutable engine cache backing the WIT `engine-derive`/`engine-read` guest↔host boundary.
    pub fn engine_cache_mut(&mut self) -> &mut semio_framework_2d::EngineCache {
        &mut self.cache
    }

    fn derive_node(&mut self, node: StoredNode) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let pack = StoredNode::encode_pack(&node).map_err(semio_framework_2d::DrawingError::InvalidInput)?;
        let kernel_handle = self.cache.derive(DrawingEngine::ENGINE_ID, &pack).map_err(map_engine_fault)?;
        let drawing = drawing_handle_from_key(kernel_handle.key);
        self.live.insert(drawing.as_str().to_string());
        Ok(drawing)
    }

    fn register(&mut self, kind: DrawingKind, node: DrawingNode) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.derive_node(StoredNode { kind, node, transform: Affine2D::identity(), fill: None, stroke: None, clip: None, opacity: 1.0 })
    }

    fn fork(&mut self, source: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let entry = self.entry(source)?.clone();
        self.derive_node(entry)
    }

    fn with_mutated<F>(&mut self, source: &DrawingHandle, mutate: F) -> Result<DrawingHandle, semio_framework_2d::DrawingError>
    where
        F: FnOnce(&mut StoredNode),
    {
        let mut entry = self.entry(source)?.clone();
        mutate(&mut entry);
        self.derive_node(entry)
    }

    fn entry(&self, handle: &DrawingHandle) -> Result<StoredNode, semio_framework_2d::DrawingError> {
        if !self.live.contains(handle.as_str()) {
            return Err(semio_framework_2d::DrawingError::MissingHandle(handle.as_str().to_string()));
        }
        let kernel_handle = kernel_handle_for_drawing(handle)?;
        let pack = self.cache.read(&kernel_handle).map_err(map_engine_fault)?;
        StoredNode::decode_pack(&pack).map_err(semio_framework_2d::DrawingError::InvalidInput)
    }

    pub fn registry_len(&self) -> usize {
        self.live.len()
    }

    pub fn dispose_sync(&mut self, handle: &DrawingHandle) {
        self.live.remove(handle.as_str());
    }

    pub fn retain_sync(&mut self, live: &HashSet<String>) {
        self.live.retain(|handle| live.contains(handle));
    }

    fn node_to_segments(node: &DrawingNode) -> Vec<semio_framework_2d::PathSegment> {
        match node {
            DrawingNode::Rect { x, y, width, height } => rect_segments(*x, *y, *width, *height),
            DrawingNode::Ellipse { cx, cy, rx, ry } => ellipse_segments(*cx, *cy, *rx, *ry),
            DrawingNode::Circle { cx, cy, r } => ellipse_segments(*cx, *cy, *r, *r),
            DrawingNode::Line { x1, y1, x2, y2 } => vec![semio_framework_2d::PathSegment::Move { to: [*x1, *y1] }, semio_framework_2d::PathSegment::Line { to: [*x2, *y2] }],
            DrawingNode::Polygon { points } => polygon_segments(points),
            DrawingNode::Path { segments } => segments.clone(),
            DrawingNode::Text { .. } => Vec::new(),
            DrawingNode::Group { .. } => Vec::new(),
        }
    }

    fn flatten_handle(&self, handle: &DrawingHandle, parent: Affine2D) -> Result<Vec<SceneNode>, semio_framework_2d::DrawingError> {
        let entry = self.entry(handle)?;
        let transform = parent.multiply(entry.transform);
        match &entry.node {
            DrawingNode::Group { children } => children.iter().try_fold(Vec::new(), |mut acc, child| {
                let nested = self.flatten_handle(&DrawingHandle(child.clone()), transform)?;
                acc.extend(nested);
                Ok::<_, semio_framework_2d::DrawingError>(acc)
            }),
            _ => Ok(vec![SceneNode { transform, node: entry.node.clone(), fill: entry.fill.clone(), stroke: entry.stroke.clone(), opacity: entry.opacity, clip: entry.clip.clone() }]),
        }
    }

    pub fn flatten_scene_sync(&self, handle: &DrawingHandle) -> Result<DrawingScene, semio_framework_2d::DrawingError> {
        let nodes = self.flatten_handle(handle, Affine2D::identity())?;
        let (width, height) = scene_bounds(&nodes);
        Ok(DrawingScene { width, height, nodes })
    }

    pub fn export_svg_sync(&self, handle: &DrawingHandle) -> Result<String, semio_framework_2d::DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_svg(&scene))
    }

    pub fn export_pdf_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        Ok(serialize_pdf(&scene))
    }

    pub fn export_dwg_sync(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError> {
        let scene = self.flatten_scene_sync(handle)?;
        let mut drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        for node in &scene.nodes {
            if node.opacity <= 0.0 {
                continue;
            }
            if let DrawingNode::Circle { cx, cy, r } = &node.node {
                let center = affine_apply_point(node.transform, [*cx, *cy]);
                drawing.entities.push(semio_s_plugin_stdio::artifacts::dwg::DwgEntity {
                    layer,
                    color: semio_s_plugin_stdio::artifacts::dwg::DwgColor::ByLayer,
                    geometry: semio_s_plugin_stdio::artifacts::dwg::DwgGeometry::Circle { center: [center[0], center[1], 0.0], radius: r * node.transform.0[0].abs(), normal: [0.0, 0.0, 1.0] },
                });
                continue;
            }
            if let DrawingNode::Text { x, y, content, size } = &node.node {
                let at = affine_apply_point(node.transform, [*x, *y]);
                drawing.entities.push(semio_s_plugin_stdio::artifacts::dwg::DwgEntity {
                    layer,
                    color: semio_s_plugin_stdio::artifacts::dwg::DwgColor::ByLayer,
                    geometry: semio_s_plugin_stdio::artifacts::dwg::DwgGeometry::Text { at: [at[0], at[1], 0.0], height: *size, rotation: 0.0, content: content.clone() },
                });
                continue;
            }
            if let Some(segments) = scene_node_world_segments(node) {
                let dwg_segments: Vec<semio_s_plugin_stdio::artifacts::dwg::DwgPathSegment> = segments.iter().map(engine_segment_to_dwg).collect();
                let mut sub = semio_s_plugin_stdio::artifacts::dwg::paths_to_dwg_drawing(&[dwg_segments]);
                drawing.entities.append(&mut sub.entities);
            }
        }
        semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&drawing).map_err(semio_framework_2d::DrawingError::InvalidInput)
    }

    pub fn import_dwg_sync(&mut self, data: &[u8]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::dwg_from_bytes(data).map_err(semio_framework_2d::DrawingError::InvalidInput)?;
        let mut children = Vec::new();
        for path in semio_s_plugin_stdio::artifacts::dwg::dwg_drawing_to_paths(&drawing) {
            let segments: Vec<semio_framework_2d::PathSegment> = path.iter().map(dwg_segment_to_engine).collect();
            if segments.len() < 2 {
                continue;
            }
            children.push(self.register(DrawingKind::Path, DrawingNode::Path { segments })?);
        }
        for entity in &drawing.entities {
            if let semio_s_plugin_stdio::artifacts::dwg::DwgGeometry::Text { at, height, content, .. } = &entity.geometry {
                children.push(self.register(DrawingKind::Text, DrawingNode::Text { x: at[0], y: at[1], content: content.clone(), size: *height })?);
            }
        }
        if children.is_empty() {
            return self.register(DrawingKind::Path, DrawingNode::Path { segments: vec![semio_framework_2d::PathSegment::Move { to: [0.0, 0.0] }, semio_framework_2d::PathSegment::Line { to: [0.0, 0.0] }] });
        }
        if children.len() == 1 {
            return Ok(children.into_iter().next().expect("children.len() == 1 checked above"));
        }
        self.register(DrawingKind::Group, DrawingNode::Group { children: children.iter().map(|h| h.as_str().to_string()).collect() })
    }
}
// #endregion 🔖️Store

fn affine_apply_point(m: Affine2D, p: semio_framework_2d::Vec2) -> semio_framework_2d::Vec2 {
    let a = m.0;
    [a[0] * p[0] + a[2] * p[1] + a[4], a[1] * p[0] + a[3] * p[1] + a[5]]
}

fn scene_node_world_segments(node: &SceneNode) -> Option<Vec<semio_framework_2d::PathSegment>> {
    use semio_framework_2d::PathSegment;
    let segments = match &node.node {
        DrawingNode::Path { segments } => segments.clone(),
        DrawingNode::Line { x1, y1, x2, y2 } => vec![PathSegment::Move { to: [*x1, *y1] }, PathSegment::Line { to: [*x2, *y2] }],
        DrawingNode::Polygon { points } => {
            if points.is_empty() {
                return None;
            }
            let mut segments = vec![PathSegment::Move { to: points[0] }];
            for p in &points[1..] {
                segments.push(PathSegment::Line { to: *p });
            }
            segments.push(PathSegment::Close);
            segments
        }
        DrawingNode::Rect { x, y, width, height } => {
            vec![PathSegment::Move { to: [*x, *y] }, PathSegment::Line { to: [*x + *width, *y] }, PathSegment::Line { to: [*x + *width, *y + *height] }, PathSegment::Line { to: [*x, *y + *height] }, PathSegment::Close]
        }
        DrawingNode::Ellipse { cx, cy, rx, ry } => vec![
            PathSegment::Move { to: [*cx + *rx, *cy] },
            PathSegment::Arc { rx: *rx, ry: *ry, rotation: 0.0, large_arc: true, sweep: true, to: [*cx - *rx, *cy] },
            PathSegment::Arc { rx: *rx, ry: *ry, rotation: 0.0, large_arc: true, sweep: true, to: [*cx + *rx, *cy] },
            PathSegment::Close,
        ],
        DrawingNode::Circle { .. } | DrawingNode::Text { .. } | DrawingNode::Group { .. } => return None,
    };
    Some(
        segments
            .into_iter()
            .map(|segment| match segment {
                PathSegment::Move { to } => PathSegment::Move { to: affine_apply_point(node.transform, to) },
                PathSegment::Line { to } => PathSegment::Line { to: affine_apply_point(node.transform, to) },
                PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: affine_apply_point(node.transform, ctrl), to: affine_apply_point(node.transform, to) },
                PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: affine_apply_point(node.transform, ctrl1), ctrl2: affine_apply_point(node.transform, ctrl2), to: affine_apply_point(node.transform, to) },
                PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to: affine_apply_point(node.transform, to) },
                PathSegment::Close => PathSegment::Close,
            })
            .collect(),
    )
}

fn engine_segment_to_dwg(segment: &semio_framework_2d::PathSegment) -> semio_s_plugin_stdio::artifacts::dwg::DwgPathSegment {
    use semio_framework_2d::PathSegment;
    use semio_s_plugin_stdio::artifacts::dwg::DwgPathSegment;
    match segment {
        PathSegment::Move { to } => DwgPathSegment::Move { to: *to },
        PathSegment::Line { to } => DwgPathSegment::Line { to: *to },
        PathSegment::Quad { ctrl, to } => DwgPathSegment::Quad { ctrl: *ctrl, to: *to },
        PathSegment::Cubic { ctrl1, ctrl2, to } => DwgPathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => DwgPathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        PathSegment::Close => DwgPathSegment::Close,
    }
}

fn dwg_segment_to_engine(segment: &semio_s_plugin_stdio::artifacts::dwg::DwgPathSegment) -> semio_framework_2d::PathSegment {
    use semio_framework_2d::PathSegment;
    use semio_s_plugin_stdio::artifacts::dwg::DwgPathSegment;
    match segment {
        DwgPathSegment::Move { to } => PathSegment::Move { to: *to },
        DwgPathSegment::Line { to } => PathSegment::Line { to: *to },
        DwgPathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        DwgPathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        DwgPathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        DwgPathSegment::Close => PathSegment::Close,
    }
}

// #region 🔖️Geometry
fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
    vec![PathSegment::Move { to: [x, y] }, PathSegment::Line { to: [x + width, y] }, PathSegment::Line { to: [x + width, y + height] }, PathSegment::Line { to: [x, y + height] }, PathSegment::Close]
}

fn polygon_segments(points: &[semio_framework_2d::Vec2]) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
    let mut segments = Vec::new();
    if let Some(first) = points.first() {
        segments.push(PathSegment::Move { to: *first });
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
        if points.len() > 2 {
            segments.push(PathSegment::Close);
        }
    }
    segments
}

fn ellipse_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
    const K: f64 = 0.5522847498;
    let ox = rx * K;
    let oy = ry * K;
    vec![
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + ox, cy - ry], ctrl2: [cx + rx, cy - oy], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + oy], ctrl2: [cx + ox, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - ox, cy + ry], ctrl2: [cx - rx, cy + oy], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - oy], ctrl2: [cx - ox, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ]
}

fn polyline_segments(points: &[semio_framework_2d::Vec2]) -> Vec<semio_framework_2d::PathSegment> {
    use semio_framework_2d::PathSegment;
    let mut segments = Vec::new();
    if let Some(first) = points.first() {
        segments.push(PathSegment::Move { to: *first });
        for point in points.iter().skip(1) {
            segments.push(PathSegment::Line { to: *point });
        }
    }
    segments
}

fn scene_bounds(nodes: &[SceneNode]) -> (f64, f64) {
    let mut max_x = 512.0_f64;
    let mut max_y = 512.0_f64;
    for node in nodes {
        match &node.node {
            DrawingNode::Rect { x, y, width, height } => {
                max_x = max_x.max(x + width);
                max_y = max_y.max(y + height);
            }
            DrawingNode::Circle { cx, cy, r } => {
                max_x = max_x.max(cx + r);
                max_y = max_y.max(cy + r);
            }
            DrawingNode::Ellipse { cx, cy, rx, ry } => {
                max_x = max_x.max(cx + rx);
                max_y = max_y.max(cy + ry);
            }
            DrawingNode::Text { x, y, size, .. } => {
                max_x = max_x.max(x + size * 4.0);
                max_y = max_y.max(y + size);
            }
            _ => {}
        }
    }
    (max_x.max(1.0), max_y.max(1.0))
}
// #endregion 🔖️Geometry

// #region 🔖️Export
fn color_css(color: [f64; 4]) -> String {
    let r = (color[0] * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (color[1] * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (color[2] * 255.0).round().clamp(0.0, 255.0) as u8;
    let a = color[3].clamp(0.0, 1.0);
    if (a - 1.0).abs() < f64::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({r},{g},{b},{a:.3})")
    }
}

fn segments_to_svg_d(segments: &[semio_framework_2d::PathSegment]) -> String {
    use semio_framework_2d::PathSegment;
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("M {} {} ", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("L {} {} ", to[0], to[1])),
            PathSegment::Quad { ctrl, to } => out.push_str(&format!("Q {} {} {} {} ", ctrl[0], ctrl[1], to[0], to[1])),
            PathSegment::Cubic { ctrl1, ctrl2, to } => out.push_str(&format!("C {} {} {} {} {} {} ", ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1])),
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                out.push_str(&format!("A {} {} {} {} {} {} {} ", rx, ry, rotation, if *large_arc { 1 } else { 0 }, if *sweep { 1 } else { 0 }, to[0], to[1]));
            }
            PathSegment::Close => out.push('Z'),
        }
    }
    out.trim().to_string()
}

fn serialize_svg(scene: &DrawingScene) -> String {
    let mut body = String::new();
    for node in &scene.nodes {
        match &node.node {
            DrawingNode::Text { x, y, content, size } => {
                let [tx, ty] = node.transform.transform_point([*x, *y]);
                let fill = node.fill.as_ref().map_or_else(
                    || "black".into(),
                    |f| match f {
                        FillStyle::Solid { color } => color_css(*color),
                        _ => "black".into(),
                    },
                );
                body.push_str(&format!(r#"<text x="{tx}" y="{ty}" font-size="{size}" fill="{fill}">{content}</text>"#));
            }
            _ => {
                let segments = DrawingStore::node_to_segments(&node.node);
                if segments.is_empty() {
                    continue;
                }
                let d = segments_to_svg_d(&segments);
                let mut attrs = format!(r#"d="{d}""#);
                if let Some(fill) = &node.fill {
                    match fill {
                        FillStyle::Solid { color } => attrs.push_str(&format!(r#" fill="{}""#, color_css(*color))),
                        FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                            let id = format!("lg{}", body.len());
                            body.push_str(&format!(r#"<defs><linearGradient id="{id}" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</linearGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                        FillStyle::RadialGradient { cx, cy, r, stops } => {
                            let id = format!("rg{}", body.len());
                            body.push_str(&format!(r#"<defs><radialGradient id="{id}" cx="{cx}" cy="{cy}" r="{r}">"#));
                            for stop in stops {
                                body.push_str(&format!(r#"<stop offset="{}" stop-color="{}"/>"#, stop.offset, color_css(stop.color)));
                            }
                            body.push_str("</radialGradient></defs>");
                            attrs.push_str(&format!(r#" fill="url(#{id})""#));
                        }
                    }
                } else {
                    attrs.push_str(r#" fill="none""#);
                }
                if let Some(stroke) = &node.stroke {
                    attrs.push_str(&format!(r#" stroke="{}" stroke-width="{}""#, color_css(stroke.color), stroke.width));
                }
                body.push_str(&format!("<path {attrs}/>"));
            }
        }
    }
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">{body}</svg>"#, scene.width, scene.height, scene.width, scene.height)
}

fn segments_to_pdf_operations(segments: &[semio_framework_2d::PathSegment]) -> String {
    use semio_framework_2d::PathSegment;
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("{} {} m\n", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("{} {} l\n", to[0], to[1])),
            PathSegment::Close => out.push_str("h\n"),
            _ => {}
        }
    }
    out
}

fn serialize_pdf(scene: &DrawingScene) -> Vec<u8> {
    let mut stream = String::new();
    stream.push_str("0.1 0.1 0.1 rg\n");
    for node in &scene.nodes {
        let segments = DrawingStore::node_to_segments(&node.node);
        if segments.is_empty() {
            continue;
        }
        stream.push_str(&segments_to_pdf_operations(&segments));
        if node.fill.is_some() {
            stream.push_str("f\n");
        } else if node.stroke.is_some() {
            stream.push_str("S\n");
        }
    }
    let content = stream.as_bytes();
    let objects = [
        "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n",
        "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n",
        &format!("3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R /Resources << >> >>\nendobj\n", scene.width, scene.height),
        &format!("4 0 obj\n<< /Length {} >>\nstream\n", content.len()),
    ];
    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = Vec::new();
    for object in &objects[..3] {
        offsets.push(pdf.len());
        pdf.push_str(object);
    }
    offsets.push(pdf.len());
    pdf.push_str(objects[3]);
    pdf.push_str(std::str::from_utf8(content).unwrap_or(""));
    pdf.push_str("\nendstream\nendobj\n");
    let xref = pdf.len();
    pdf.push_str("xref\n0 5\n0000000000 65535 f \n");
    for offset in offsets {
        pdf.push_str(&format!("{:010} 00000 n \n", offset));
    }
    pdf.push_str(&format!("trailer\n<< /Size 5 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n"));
    pdf.into_bytes()
}
// #endregion 🔖️Export

// #region 🔖️KernelImpl
impl DrawingKernel for DrawingStore {
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Rect, DrawingNode::Rect { x, y, width, height })
    }

    fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Ellipse, DrawingNode::Ellipse { cx, cy, rx, ry })
    }

    fn circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Circle, DrawingNode::Circle { cx, cy, r })
    }

    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Line, DrawingNode::Line { x1, y1, x2, y2 })
    }

    fn polygon(&mut self, points: &[semio_framework_2d::Vec2]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if points.len() < 3 {
            return Err(semio_framework_2d::DrawingError::InvalidInput("polygon needs at least 3 points".into()));
        }
        self.register(DrawingKind::Polygon, DrawingNode::Polygon { points: points.to_vec() })
    }

    fn polyline_path(&mut self, points: &[semio_framework_2d::Vec2]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if points.len() < 2 {
            return Err(semio_framework_2d::DrawingError::InvalidInput("polyline needs at least 2 points".into()));
        }
        self.register(DrawingKind::Path, DrawingNode::Path { segments: polyline_segments(points) })
    }

    fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Path, DrawingNode::Path { segments: rect_segments(x, y, width, height) })
    }

    fn set_fill(&mut self, handle: &DrawingHandle, fill: FillStyle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.fill = Some(fill))
    }

    fn set_stroke(&mut self, handle: &DrawingHandle, stroke: StrokeStyle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.stroke = Some(stroke))
    }

    fn linear_gradient_fill(&mut self, handle: &DrawingHandle, x1: f64, y1: f64, x2: f64, y2: f64, stops: &[GradientStop]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.set_fill(handle, FillStyle::LinearGradient { x1, y1, x2, y2, stops: stops.to_vec() })
    }

    fn translate(&mut self, handle: &DrawingHandle, dx: f64, dy: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.transform = entry.transform.multiply(Affine2D::translate(dx, dy)))
    }

    fn rotate(&mut self, handle: &DrawingHandle, angle: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.transform = entry.transform.multiply(Affine2D::rotate(angle)))
    }

    fn scale(&mut self, handle: &DrawingHandle, sx: f64, sy: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.with_mutated(handle, |entry| entry.transform = entry.transform.multiply(Affine2D::scale(sx, sy)))
    }

    fn group(&mut self, children: &[DrawingHandle]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if children.is_empty() {
            return Err(semio_framework_2d::DrawingError::InvalidInput("group needs children".into()));
        }
        let handles: Vec<String> = children.iter().map(|h| h.as_str().to_string()).collect();
        self.register(DrawingKind::Group, DrawingNode::Group { children: handles })
    }

    fn bool_union(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "union")
    }

    fn bool_difference(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "difference")
    }

    fn bool_intersection(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "intersection")
    }

    fn bool_xor(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.bool_operation(a, b, "xor")
    }

    fn bool_op_many(&mut self, operation: &str, handles: &[DrawingHandle]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        if handles.is_empty() {
            return Err(semio_framework_2d::DrawingError::InvalidInput("boolean operation needs at least one handle".into()));
        }
        if handles.len() == 1 {
            return self.fork(&handles[0]);
        }
        let mut segments_list: Vec<Vec<semio_framework_2d::PathSegment>> = Vec::new();
        for handle in handles {
            segments_list.push(DrawingStore::node_to_segments(&self.entry(handle)?.node));
        }
        let merged = semio_framework_2d::booleans::boolean_paths_many(&segments_list, operation)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments: merged })
    }

    fn boolean_segments(&self, a: &[semio_framework_2d::PathSegment], b: &[semio_framework_2d::PathSegment], operation: &str) -> Result<Vec<semio_framework_2d::PathSegment>, semio_framework_2d::DrawingError> {
        semio_framework_2d::booleans::boolean_paths(a, b, operation)
    }

    fn trace_bitmap(&mut self, width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let segments = semio_framework_2d::trace::trace_bitmap_paths(width, height, mask_or_luma, threshold, simplify_epsilon)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments })
    }

    fn text(&mut self, x: f64, y: f64, content: &str, size: f64) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.register(DrawingKind::Text, DrawingNode::Text { x, y, content: content.to_string(), size })
    }

    fn apply_clip(&mut self, target: &DrawingHandle, clip: &DrawingHandle) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let clip_segments = DrawingStore::node_to_segments(&self.entry(clip)?.node);
        self.with_mutated(target, |entry| entry.clip = Some(clip_segments))
    }

    fn flatten_scene(&self, handle: &DrawingHandle) -> Result<DrawingScene, semio_framework_2d::DrawingError> {
        self.flatten_scene_sync(handle)
    }

    fn export_svg(&self, handle: &DrawingHandle) -> Result<String, semio_framework_2d::DrawingError> {
        self.export_svg_sync(handle)
    }

    fn export_pdf(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError> {
        self.export_pdf_sync(handle)
    }

    fn export_dwg(&self, handle: &DrawingHandle) -> Result<Vec<u8>, semio_framework_2d::DrawingError> {
        self.export_dwg_sync(handle)
    }

    fn import_dwg(&mut self, data: &[u8]) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        self.import_dwg_sync(data)
    }

    fn kind(&self, handle: &DrawingHandle) -> Result<DrawingKind, semio_framework_2d::DrawingError> {
        Ok(self.entry(handle)?.kind)
    }

    fn dispose(&mut self, handle: &DrawingHandle) {
        self.dispose_sync(handle);
    }

    fn retain_sync(&mut self, live: &HashSet<String>) {
        DrawingStore::retain_sync(self, live);
    }
}

impl DrawingStore {
    fn bool_operation(&mut self, a: &DrawingHandle, b: &DrawingHandle, operation: &str) -> Result<DrawingHandle, semio_framework_2d::DrawingError> {
        let a_segments = DrawingStore::node_to_segments(&self.entry(a)?.node);
        let b_segments = DrawingStore::node_to_segments(&self.entry(b)?.node);
        let merged = semio_framework_2d::booleans::boolean_paths(&a_segments, &b_segments, operation)?;
        self.register(DrawingKind::Path, DrawingNode::Path { segments: merged })
    }
}
// #endregion 🔖️KernelImpl

// #region 🖍️DrawingKernel
static DRAWING_KERNEL: LazyLock<Mutex<DrawingStore>> = LazyLock::new(|| Mutex::new(DrawingStore::new()));

fn drawing_kernel() -> &'static Mutex<DrawingStore> {
    &DRAWING_KERNEL
}

/// 🖊️ Runs `f` against the process-wide 2D drawing kernel.
pub fn with_drawing_kernel<T>(f: impl FnOnce(&mut DrawingStore) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = drawing_kernel().lock().map_err(|_| EvalError::InvalidInput("draw kernel lock poisoned".into()))?;
    f(&mut guard)
}

/// 🧯️ Internal error type for drawing JSON-bridging helpers.
#[derive(Debug)]
enum DrawingKernelError {
    Json(serde_json::Error),
    Invalid(String),
}

impl fmt::Display for DrawingKernelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => error.fmt(formatter),
            Self::Invalid(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for DrawingKernelError {}

impl From<serde_json::Error> for DrawingKernelError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

/// 🧹️ Retains only drawing handles referenced by the current evaluation outputs.
pub fn retain_drawing_handles(live: &[String]) {
    let live_set: HashSet<String> = live.iter().cloned().collect();
    if let Ok(mut guard) = drawing_kernel().lock() {
        guard.retain_sync(&live_set);
    }
}

/// 🎬️ Flattens a drawing handle to JSON scene payload.
pub fn render_scene_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match store.flatten_scene(&drawing) {
                Ok(scene) => serde_json::to_string(&scene).unwrap_or_else(|_| "{}".into()),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📄️ Exports a drawing handle as SVG JSON wrapper.
pub fn export_svg_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match store.export_svg(&drawing) {
                Ok(svg) => serde_json::json!({ "svg": svg }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📑️ Exports a drawing handle as base64 PDF JSON wrapper.
pub fn export_pdf_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match store.export_pdf(&drawing) {
                Ok(pdf) => serde_json::json!({ "pdf": drawing_base64_encode(&pdf) }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📐️ Exports a drawing handle as base64 DWG JSON wrapper.
pub fn export_dwg_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match store.export_dwg(&drawing) {
                Ok(dwg) => serde_json::json!({ "dwg": drawing_base64_encode(&dwg) }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📐️ Imports a base64 DWG payload into the in-process draw kernel, returning the new drawing handle JSON wrapper.
pub fn import_dwg_json(data_base64: &str) -> String {
    let Ok(bytes) = drawing_base64_decode(data_base64) else {
        return serde_json::json!({ "error": "invalid base64 dwg payload" }).to_string();
    };
    drawing_kernel()
        .lock()
        .ok()
        .map(|mut store| match store.import_dwg(&bytes) {
            Ok(handle) => serde_json::json!({ "handle": handle.as_str() }).to_string(),
            Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 🗑️ Disposes a drawing handle owned by the in-process draw kernel.
pub fn dispose_drawing(handle: &str) {
    if let Ok(mut store) = drawing_kernel().lock() {
        store.dispose(&DrawingHandle(handle.to_string()));
    }
}

/// 🔍️ Autotraces a bitmap mask into path segments JSON.
pub fn trace_bitmap_json(width: u32, height: u32, mask: &[u8], threshold: f64, simplify_epsilon: f64) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .and_then(|mut store| match store.trace_bitmap(width, height, mask, threshold, simplify_epsilon) {
            Ok(handle) => match store.flatten_scene(&handle) {
                Ok(scene) => {
                    let segments = scene.nodes.into_iter().find_map(|node| if let DrawingNode::Path { segments } = node.node { Some(segments) } else { None });
                    segments.map(|segs| serde_json::json!({ "segments": segs }).to_string())
                }
                Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
            },
            Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 🔀️ Boolean-combines two path segment arrays.
pub fn boolean_segments_json(a_json: &str, b_json: &str, operation: &str) -> String {
    let parse = |json: &str| -> Result<Vec<semio_framework_2d::PathSegment>, DrawingKernelError> {
        let parsed: serde_json::Value = serde_json::from_str(json)?;
        if let Some(error) = parsed.get("error").and_then(|v| v.as_str()) {
            return Err(DrawingKernelError::Invalid(error.to_string()));
        }
        let segments_value = parsed.get("segments").cloned().ok_or_else(|| DrawingKernelError::Invalid("missing segments".to_string()))?;
        serde_json::from_value(segments_value).map_err(DrawingKernelError::from)
    };
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| match (parse(a_json), parse(b_json)) {
            (Ok(a), Ok(b)) => match store.boolean_segments(&a, &b, operation) {
                Ok(segments) => serde_json::json!({ "segments": segments }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            },
            (Err(error), _) | (_, Err(error)) => serde_json::json!({ "error": error.to_string() }).to_string(),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

fn drawing_base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 63) as usize] as char);
        out.push(TABLE[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { TABLE[((triple >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[(triple & 63) as usize] as char } else { '=' });
    }
    out
}

fn drawing_base64_decode(data: &str) -> Result<Vec<u8>, DrawingKernelError> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut lookup = [255u8; 256];
    for (index, &byte) in TABLE.iter().enumerate() {
        lookup[byte as usize] = index as u8;
    }
    let cleaned: Vec<u8> = data.bytes().filter(|byte| *byte != b'=' && !byte.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(cleaned.len() * 3 / 4);
    for chunk in cleaned.chunks(4) {
        let mut values = [0u8; 4];
        for (index, &byte) in chunk.iter().enumerate() {
            let value = lookup[byte as usize];
            if value == 255 {
                return Err(DrawingKernelError::Invalid("invalid base64 character".to_string()));
            }
            values[index] = value;
        }
        let triple = ((values[0] as u32) << 18) | ((values[1] as u32) << 12) | ((values[2] as u32) << 6) | (values[3] as u32);
        out.push((triple >> 16) as u8);
        if chunk.len() > 2 {
            out.push((triple >> 8) as u8);
        }
        if chunk.len() > 3 {
            out.push(triple as u8);
        }
    }
    Ok(out)
}
// #endregion 🖍️DrawingKernel

// #region 🔖️Tests
#[cfg(test)]
mod drawing_kernel_tests {
    use super::*;

    #[test]
    fn rect_exports_svg() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 10.0, 20.0).expect("rect");
        let svg = store.export_svg_sync(&rect).expect("svg");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("10"));
    }

    #[test]
    fn rect_exports_pdf() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 10.0, 20.0).expect("rect");
        let pdf = store.export_pdf_sync(&rect).expect("pdf");
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[test]
    fn group_flattens_children() {
        let mut store = DrawingStore::new();
        let a = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let b = store.circle(10.0, 10.0, 3.0).unwrap();
        let group = store.group(&[a, b]).unwrap();
        let scene = store.flatten_scene_sync(&group).unwrap();
        assert_eq!(scene.nodes.len(), 2);
    }

    #[test]
    fn dwg_export_import_round_trips_a_group() {
        let mut store = DrawingStore::new();
        let rect = store.rect_path(0.0, 0.0, 5.0, 5.0).unwrap();
        let circle = store.circle(10.0, 10.0, 3.0).unwrap();
        let group = store.group(&[rect, circle]).unwrap();

        let bytes = store.export_dwg_sync(&group).expect("export dwg");
        assert!(!bytes.is_empty());

        let imported = store.import_dwg_sync(&bytes).expect("import dwg");
        let scene = store.flatten_scene_sync(&imported).expect("flatten imported scene");
        assert!(!scene.nodes.is_empty());
    }

    // #region Geometry primitives export
    #[test]
    fn ellipse_exports_svg_with_cubic_curves() {
        let mut store = DrawingStore::new();
        let ellipse = store.ellipse(5.0, 5.0, 4.0, 2.0).unwrap();
        let svg = store.export_svg_sync(&ellipse).expect("svg");
        assert!(svg.contains("C "));
        assert!(svg.contains("Z"));
    }

    #[test]
    fn line_exports_svg_move_and_line() {
        let mut store = DrawingStore::new();
        let line = store.line(0.0, 0.0, 10.0, 10.0).unwrap();
        let svg = store.export_svg_sync(&line).expect("svg");
        assert!(svg.contains("M 0 0"));
        assert!(svg.contains("L 10 10"));
    }

    #[test]
    fn polygon_exports_closed_svg_path() {
        let mut store = DrawingStore::new();
        let polygon = store.polygon(&[[0.0, 0.0], [4.0, 0.0], [2.0, 4.0]]).unwrap();
        let svg = store.export_svg_sync(&polygon).expect("svg");
        assert!(svg.contains("Z"));
    }

    #[test]
    fn polyline_path_exports_open_path_without_close() {
        let mut store = DrawingStore::new();
        let polyline = store.polyline_path(&[[0.0, 0.0], [4.0, 0.0], [2.0, 4.0]]).unwrap();
        let svg = store.export_svg_sync(&polyline).expect("svg");
        assert!(!svg.contains("Z"));
    }

    #[test]
    fn polygon_errors_on_too_few_points() {
        let mut store = DrawingStore::new();
        let err = store.polygon(&[[0.0, 0.0], [1.0, 1.0]]).unwrap_err();
        assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
    }

    #[test]
    fn polyline_path_errors_on_too_few_points() {
        let mut store = DrawingStore::new();
        let err = store.polyline_path(&[[0.0, 0.0]]).unwrap_err();
        assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
    }
    // #endregion Geometry primitives export

    // #region Style
    #[test]
    fn set_fill_solid_renders_opaque_hex_color() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let filled = store.set_fill(&rect, FillStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] }).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains(r##"fill="#ff0000""##));
    }

    #[test]
    fn set_fill_with_alpha_renders_rgba() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let filled = store.set_fill(&rect, FillStyle::Solid { color: [0.0, 1.0, 0.0, 0.5] }).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains("rgba(0,255,0,0.500)"));
    }

    #[test]
    fn linear_gradient_fill_renders_gradient_defs() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let stops = vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 1.0] }];
        let filled = store.linear_gradient_fill(&rect, 0.0, 0.0, 5.0, 5.0, &stops).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains("<linearGradient"));
        assert!(svg.contains("fill=\"url(#lg"));
    }

    #[test]
    fn set_fill_radial_gradient_renders_defs() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let stops = vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }];
        let fill = FillStyle::RadialGradient { cx: 2.5, cy: 2.5, r: 2.0, stops };
        let filled = store.set_fill(&rect, fill).unwrap();
        let svg = store.export_svg_sync(&filled).expect("svg");
        assert!(svg.contains("<radialGradient"));
        assert!(svg.contains("fill=\"url(#rg"));
    }

    #[test]
    fn set_stroke_renders_stroke_attributes() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let stroke = StrokeStyle { color: [0.0, 0.0, 1.0, 1.0], width: 2.0, cap: LineCap::Round, join: LineJoin::Round, dash: vec![] };
        let stroked = store.set_stroke(&rect, stroke).unwrap();
        let svg = store.export_svg_sync(&stroked).expect("svg");
        assert!(svg.contains(r##"stroke="#0000ff""##));
        assert!(svg.contains(r#"stroke-width="2""#));
    }
    // #endregion Style

    // #region Transforms
    #[test]
    fn translate_moves_exported_geometry() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let moved = store.translate(&rect, 10.0, 20.0).unwrap();
        let scene = store.flatten_scene_sync(&moved).unwrap();
        assert_eq!(scene.nodes[0].transform.transform_point([0.0, 0.0]), [10.0, 20.0]);
    }

    #[test]
    fn rotate_and_scale_compose_into_transform() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let rotated = store.rotate(&rect, std::f64::consts::FRAC_PI_2).unwrap();
        let scaled = store.scale(&rotated, 2.0, 2.0).unwrap();
        let scene = store.flatten_scene_sync(&scaled).unwrap();
        let [x, y] = scene.nodes[0].transform.transform_point([1.0, 0.0]);
        assert!((x - 0.0).abs() < 1e-9);
        assert!((y - 2.0).abs() < 1e-9);
    }
    // #endregion Transforms

    // #region Group and clip
    #[test]
    fn group_errors_on_empty_children() {
        let mut store = DrawingStore::new();
        let err = store.group(&[]).unwrap_err();
        assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
    }

    #[test]
    fn apply_clip_stores_clip_segments_on_flatten() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let circle = store.circle(2.0, 2.0, 1.0).unwrap();
        let clipped = store.apply_clip(&rect, &circle).unwrap();
        let scene = store.flatten_scene_sync(&clipped).unwrap();
        assert!(scene.nodes[0].clip.as_ref().is_some_and(|segments| !segments.is_empty()));
    }
    // #endregion Group and clip

    // #region Text
    #[test]
    fn text_with_fill_renders_colored_text_element() {
        let mut store = DrawingStore::new();
        let text = store.text(1.0, 2.0, "hi", 12.0).unwrap();
        let colored = store.set_fill(&text, FillStyle::Solid { color: [0.0, 0.0, 1.0, 1.0] }).unwrap();
        let svg = store.export_svg_sync(&colored).expect("svg");
        assert!(svg.contains(r##"<text x="1" y="2" font-size="12" fill="#0000ff">hi</text>"##));
    }

    #[test]
    fn text_without_fill_defaults_to_black() {
        let mut store = DrawingStore::new();
        let text = store.text(0.0, 0.0, "plain", 10.0).unwrap();
        let svg = store.export_svg_sync(&text).expect("svg");
        assert!(svg.contains(r#"fill="black">plain"#));
    }

    #[test]
    fn text_with_gradient_fill_falls_back_to_black_in_svg() {
        let mut store = DrawingStore::new();
        let text = store.text(0.0, 0.0, "grad", 10.0).unwrap();
        let stops = vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }];
        let gradient = store.linear_gradient_fill(&text, 0.0, 0.0, 1.0, 1.0, &stops).unwrap();
        let svg = store.export_svg_sync(&gradient).expect("svg");
        assert!(svg.contains(r#"fill="black">grad"#));
    }
    // #endregion Text

    // #region Boolean operations via kernel trait
    #[test]
    fn bool_union_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
        let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
        let merged = store.bool_union(&a, &b).unwrap();
        assert_eq!(store.kind(&merged).unwrap(), DrawingKind::Path);
    }

    #[test]
    fn bool_difference_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
        let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
        let diff = store.bool_difference(&a, &b).unwrap();
        let scene = store.flatten_scene_sync(&diff).unwrap();
        assert!(!scene.nodes.is_empty());
    }

    #[test]
    fn bool_intersection_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
        let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
        let intersection = store.bool_intersection(&a, &b).unwrap();
        let scene = store.flatten_scene_sync(&intersection).unwrap();
        assert!(!scene.nodes.is_empty());
    }

    #[test]
    fn bool_xor_via_kernel_trait() {
        let mut store = DrawingStore::new();
        let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
        let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
        let xor = store.bool_xor(&a, &b).unwrap();
        let scene = store.flatten_scene_sync(&xor).unwrap();
        assert!(!scene.nodes.is_empty());
    }

    #[test]
    fn bool_op_many_single_handle_is_content_addressed() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let forked = store.bool_op_many("union", std::slice::from_ref(&rect)).unwrap();
        assert_eq!(forked.as_str(), rect.as_str());
        assert_eq!(store.kind(&forked).unwrap(), DrawingKind::Rect);
    }

    #[test]
    fn bool_op_many_merges_multiple_handles() {
        let mut store = DrawingStore::new();
        let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
        let b = store.rect_path(5.0, 0.0, 10.0, 10.0).unwrap();
        let c = store.rect_path(0.0, 5.0, 10.0, 10.0).unwrap();
        let merged = store.bool_op_many("union", &[a, b, c]).unwrap();
        assert_eq!(store.kind(&merged).unwrap(), DrawingKind::Path);
    }

    #[test]
    fn bool_op_many_errors_on_empty_handles() {
        let mut store = DrawingStore::new();
        let err = store.bool_op_many("union", &[]).unwrap_err();
        assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
    }

    #[test]
    fn boolean_segments_trait_delegates_to_booleans_module() {
        let store = DrawingStore::new();
        let a = rect_segments(0.0, 0.0, 10.0, 10.0);
        let b = rect_segments(5.0, 5.0, 10.0, 10.0);
        let merged = store.boolean_segments(&a, &b, "union").expect("union");
        assert!(!merged.is_empty());
    }
    // #endregion Boolean operations via kernel trait

    // #region Trace via kernel trait
    #[test]
    fn trace_bitmap_trait_delegates_to_trace_module() {
        let mut store = DrawingStore::new();
        let width = 6_u32;
        let height = 6_u32;
        let mut mask = vec![0_u8; (width * height) as usize];
        for y in 1..5 {
            for x in 1..5 {
                mask[(y * width + x) as usize] = 255;
            }
        }
        let traced = store.trace_bitmap(width, height, &mask, 0.5, 0.5).unwrap();
        assert_eq!(store.kind(&traced).unwrap(), DrawingKind::Path);
    }
    // #endregion Trace via kernel trait

    // #region Registry lifecycle
    #[test]
    fn registry_len_tracks_inserted_handles() {
        let mut store = DrawingStore::new();
        assert_eq!(store.registry_len(), 0);
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        assert_eq!(store.registry_len(), 1);
        store.set_fill(&rect, FillStyle::Solid { color: [1.0, 1.0, 1.0, 1.0] }).unwrap();
        assert_eq!(store.registry_len(), 2);
    }

    #[test]
    fn dispose_sync_removes_handle_from_registry() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        store.dispose_sync(&rect);
        assert_eq!(store.registry_len(), 0);
        let err = store.kind(&rect).unwrap_err();
        assert!(matches!(err, semio_framework_2d::DrawingError::MissingHandle(_)));
    }

    #[test]
    fn retain_sync_keeps_only_live_handles() {
        let mut store = DrawingStore::new();
        let a = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
        let b = store.circle(1.0, 1.0, 1.0).unwrap();
        let live: HashSet<String> = [a.as_str().to_string()].into_iter().collect();
        store.retain_sync(&live);
        assert!(store.kind(&a).is_ok());
        assert!(store.kind(&b).is_err());
    }

    #[test]
    fn missing_handle_errors_on_set_fill_and_translate() {
        let mut store = DrawingStore::new();
        let bogus = DrawingHandle("not-valid-hex".to_string());
        let fill_err = store.set_fill(&bogus, FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }).unwrap_err();
        assert!(matches!(fill_err, semio_framework_2d::DrawingError::MissingHandle(_)));
        let translate_err = store.translate(&bogus, 1.0, 1.0).unwrap_err();
        assert!(matches!(translate_err, semio_framework_2d::DrawingError::MissingHandle(_)));
    }

    #[test]
    fn flatten_scene_errors_on_missing_handle() {
        let store = DrawingStore::new();
        let bogus = DrawingHandle("not-valid-hex".to_string());
        let err = store.flatten_scene(&bogus).unwrap_err();
        assert!(matches!(err, semio_framework_2d::DrawingError::MissingHandle(_)));
    }
    // #endregion Registry lifecycle

    // #region DWG export/import branches
    #[test]
    fn export_dwg_includes_circle_and_text_entities() {
        let mut store = DrawingStore::new();
        let circle = store.circle(5.0, 5.0, 3.0).unwrap();
        let text = store.text(0.0, 0.0, "hi", 5.0).unwrap();
        let group = store.group(&[circle, text]).unwrap();
        let bytes = store.export_dwg_sync(&group).expect("export dwg");
        let imported = store.import_dwg_sync(&bytes).expect("import dwg");
        let scene = store.flatten_scene_sync(&imported).expect("flatten imported scene");
        assert_eq!(scene.nodes.len(), 2);
    }

    #[test]
    fn import_dwg_of_single_path_skips_group_wrapper() {
        let mut store = DrawingStore::new();
        let rect = store.rect_path(0.0, 0.0, 5.0, 5.0).unwrap();
        let bytes = store.export_dwg_sync(&rect).expect("export dwg");
        let imported = store.import_dwg_sync(&bytes).expect("import dwg");
        assert_eq!(store.kind(&imported).unwrap(), DrawingKind::Path);
    }

    #[test]
    fn import_dwg_of_empty_drawing_returns_degenerate_path() {
        let mut store = DrawingStore::new();
        let empty = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let bytes = semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&empty).expect("encode empty dwg");
        let imported = store.import_dwg_sync(&bytes).expect("import empty dwg");
        assert_eq!(store.kind(&imported).unwrap(), DrawingKind::Path);
    }
    // #endregion DWG export/import branches

    // #region Scene bounds
    #[test]
    fn scene_bounds_grows_to_fit_text_and_shapes() {
        let mut store = DrawingStore::new();
        let rect = store.rect(600.0, 0.0, 10.0, 10.0).unwrap();
        let text = store.text(0.0, 700.0, "wide label", 20.0).unwrap();
        let group = store.group(&[rect, text]).unwrap();
        let scene = store.flatten_scene_sync(&group).unwrap();
        assert!(scene.width >= 610.0);
        assert!(scene.height >= 720.0);
    }
    // #endregion Scene bounds

    // #region Engine derive
    #[test]
    fn drawing_engine_id_matches_os_contract() {
        assert_eq!(DrawingEngine::ENGINE_ID, "s.2d.drawing");
    }

    #[test]
    fn derive_twice_same_node_is_same_handle() {
        let mut store = DrawingStore::new();
        let first = store.rect(1.0, 2.0, 3.0, 4.0).unwrap();
        let second = store.rect(1.0, 2.0, 3.0, 4.0).unwrap();
        assert_eq!(first.as_str(), second.as_str());
        assert_eq!(store.registry_len(), 2);
    }

    #[test]
    fn handles_are_hex_engine_keys() {
        let mut store = DrawingStore::new();
        let rect = store.rect(0.0, 0.0, 1.0, 1.0).unwrap();
        assert_eq!(rect.as_str().len(), 64);
        assert!(rect.as_str().chars().all(|ch| ch.is_ascii_hexdigit()));
    }
    // #endregion Engine derive

    /// 📐️ Migrated from `🧰️framework/🔨️modules/◻2d/⚙️engine/🦀️.rs` alongside `Affine2D`
    /// itself (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS).
    #[test]
    fn affine_multiplies_identity() {
        let point = [3.0, 4.0];
        let moved = Affine2D::translate(1.0, 2.0).multiply(Affine2D::identity()).transform_point(point);
        assert_eq!(moved, [4.0, 6.0]);
    }
}
// #endregion 🔖️Tests
