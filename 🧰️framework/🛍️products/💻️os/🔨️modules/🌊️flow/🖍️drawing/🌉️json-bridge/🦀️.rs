//! 🌉️ JSON bridge of the drawing kernel: the shared kernel instance, the kernel error and the JSON entry points of the flow `draw` operator extension. See [`super`].

use super::*;

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

/// 🌉️ Single-key `{"error": message}` JSON wrapper, shared by every drawing-bridge JSON export.
fn json_error(message: impl fmt::Display) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(message.to_string()))]))
}

fn json_kernel_unavailable() -> String {
    json_error("draw kernel unavailable")
}

/// 🌉️ Single-key `{key: value}` string JSON wrapper, shared by every drawing-bridge JSON export.
fn json_field(key: &str, value: impl Into<String>) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([(key.to_string(), semio_framework_pack_json::Value::String(value.into()))]))
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
        .ok().map_or_else(json_kernel_unavailable, |store| {
            let drawing = DrawingHandle(handle.to_string());
            match store.flatten_scene(&drawing) {
                Ok(scene) => serde_json::to_string(&scene).unwrap_or_else(|_| "{}".into()),
                Err(error) => json_error(error),
            }
        })
}

/// 📄️ Exports a drawing handle as SVG JSON wrapper.
pub fn export_svg_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok().map_or_else(json_kernel_unavailable, |store| {
            let drawing = DrawingHandle(handle.to_string());
            match store.export_svg(&drawing) {
                Ok(svg) => json_field("svg", svg),
                Err(error) => json_error(error),
            }
        })
}

/// 📑️ Exports a drawing handle as base64 PDF JSON wrapper.
pub fn export_pdf_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok().map_or_else(json_kernel_unavailable, |store| {
            let drawing = DrawingHandle(handle.to_string());
            match store.export_pdf(&drawing) {
                Ok(pdf) => json_field("pdf", drawing_base64_encode(&pdf)),
                Err(error) => json_error(error),
            }
        })
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
                    segments.map(|segs| serde_json::json!({ "segments": segs.iter().map(PathSegmentJson::from).collect::<Vec<_>>() }).to_string())
                }
                Err(error) => Some(json_error(error)),
            },
            Err(error) => Some(json_error(error)),
        })
        .unwrap_or_else(json_kernel_unavailable)
}

/// 🔀️ Boolean-combines two path segment arrays.
pub fn boolean_segments_json(a_json: &str, b_json: &str, operation: &str) -> String {
    let parse = |json: &str| -> Result<Vec<semio_framework_2d::PathSegment>, DrawingKernelError> {
        let parsed: serde_json::Value = serde_json::from_str(json)?;
        if let Some(error) = parsed.get("error").and_then(|v| v.as_str()) {
            return Err(DrawingKernelError::Invalid(error.to_string()));
        }
        let segments_value = parsed.get("segments").cloned().ok_or_else(|| DrawingKernelError::Invalid("missing segments".to_string()))?;
        Ok(serde_json::from_value::<Vec<PathSegmentJson>>(segments_value)?.into_iter().map(Into::into).collect())
    };
    drawing_kernel()
        .lock()
        .ok().map_or_else(json_kernel_unavailable, |store| match (parse(a_json), parse(b_json)) {
            (Ok(a), Ok(b)) => match store.boolean_segments(&a, &b, operation) {
                Ok(segments) => serde_json::json!({ "segments": segments.iter().map(PathSegmentJson::from).collect::<Vec<_>>() }).to_string(),
                Err(error) => json_error(error),
            },
            (Err(error), _) | (_, Err(error)) => json_error(error),
        })
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
// #endregion 🖍️DrawingKernel
