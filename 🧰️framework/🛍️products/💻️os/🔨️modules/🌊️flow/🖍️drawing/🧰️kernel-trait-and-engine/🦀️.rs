//! 🧰️ Kernel trait and engine of the drawing kernel: the `DrawingKernel` operations, the content-addressed drawing engine, its cache helpers and the stored node. See [`super`].

use super::*;

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
    // #endregion Export

    // #region Core
    fn kind(&self, handle: &DrawingHandle) -> Result<DrawingKind, semio_framework_2d::DrawingError>;
    fn dispose(&mut self, handle: &DrawingHandle);
    fn retain_sync(&mut self, live: &HashSet<String>);
    // #endregion Core
}
// #endregion 🔖️KernelTrait

// #region 🔖️Engine
/// ⚙️ Content-addressed drawing node engine (`ENGINE_ID = "framework.2d.drawing"`).
pub struct DrawingEngine;

impl semio_framework_2d_compute::Engine for DrawingEngine {
    const ENGINE_ID: &'static str = "framework.2d.drawing";

    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, semio_framework_2d_compute::EngineFault> {
        let node = StoredNode::decode_pack(input).map_err(semio_framework_2d_compute::EngineFault::InvalidInput)?;
        StoredNode::encode_pack(&node).map_err(semio_framework_2d_compute::EngineFault::InvalidInput)
    }
}

pub(super) const ENGINE_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;

pub(super) fn map_engine_fault(fault: semio_framework_2d_compute::EngineFault) -> semio_framework_2d::DrawingError {
    match fault {
        semio_framework_2d_compute::EngineFault::Evicted => semio_framework_2d::DrawingError::MissingHandle("engine cache evicted handle".into()),
        semio_framework_2d_compute::EngineFault::InvalidInput(message) => semio_framework_2d::DrawingError::InvalidInput(message),
        semio_framework_2d_compute::EngineFault::Compute(message) => semio_framework_2d::DrawingError::Operation(message),
        semio_framework_2d_compute::EngineFault::UnknownEngine(message) => semio_framework_2d::DrawingError::Operation(message),
    }
}

pub(super) fn drawing_handle_from_key(key: semio_framework_2d_compute::EngineKey) -> DrawingHandle {
    DrawingHandle(hex_encode(&key.0))
}

pub(super) fn compute_handle_for_drawing(handle: &DrawingHandle) -> Result<semio_framework_2d_compute::EngineHandle, semio_framework_2d::DrawingError> {
    let bytes = hex_decode(handle.as_str()).map_err(|_| semio_framework_2d::DrawingError::InvalidInput("invalid drawing handle hex".into()))?;
    if bytes.len() != 32 {
        return Err(semio_framework_2d::DrawingError::InvalidInput("drawing handle length".into()));
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes);
    Ok(semio_framework_2d_compute::EngineHandle { key: semio_framework_2d_compute::EngineKey(key), engine_id: DrawingEngine::ENGINE_ID.to_string() })
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
pub(super) struct StoredNode {
    pub(super) kind: DrawingKind,
    pub(super) node: DrawingNode,
    pub(super) transform: Affine2D,
    pub fill: Option<FillStyle>,
    pub(super) stroke: Option<StrokeStyle>,
    #[serde(default, with = "optional_path_segments")]
    pub(super) clip: Option<Vec<semio_framework_2d::PathSegment>>,
    pub(super) opacity: f64,
}

impl StoredNode {
    pub(super) fn encode_pack(node: &StoredNode) -> Result<Vec<u8>, String> {
        serde_json::to_vec(node).map_err(|error| error.to_string())
    }

    pub(super) fn decode_pack(bytes: &[u8]) -> Result<StoredNode, String> {
        serde_json::from_slice(bytes).map_err(|error| error.to_string())
    }
}
// #endregion 🔖️Store
