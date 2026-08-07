//! 🖍️ Flow 2D drawing kernel JSON bridge.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use crate::dag;
use crate::dag::{
    computation_node_height, computation_node_width, dag_fixture_execution_rows, dag_fixture_to_wire_literal, fit_node_size, image_widget_size, io_widget_height, io_widget_width, normalize_node_display, note_widget_size, preview_widget_size,
    slider_widget_height, slider_widget_width, would_create_cycle, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, EdgeRouteStyle, IoPortSpec,
};
use crate::canvas;
use crate::neural::{
    channel_output, cluster_operator_info, compute_dirty_set, Atom, BudgetedEval, ChannelSpec, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorImpl, OperatorInfo, Synapse, Tree, TreeSnapshot, Value as NeuralValue, CLUSTER_KIND,
    INPUT_KIND, OUTPUT_KIND,
};
use crate::neural;
use math::graph::manifest::{PropertyBag, PropertyValue};
use flow_extension_sdk::FlowExtensionManifest;
use serde::{Deserialize, Serialize};

use crate::document::*;
use crate::catalogue::*;
use crate::registry::*;
use crate::bridge::*;
use crate::host::*;
use crate::wasm_session::*;
use crate::vcs::*;
use crate::brep_geometry::{dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry};


// #region 🖍️DrawingKernel
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use semio_s_2d::{block_on as drawing_block_on, DrawingHandle, DrawingStore};

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
#[derive(Debug, thiserror::Error)]
enum DrawingKernelError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Invalid(String),
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
            match drawing_block_on(store.flatten_scene(&drawing)) {
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
            match drawing_block_on(store.export_svg(&drawing)) {
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
            match drawing_block_on(store.export_pdf(&drawing)) {
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
            match drawing_block_on(store.export_dwg(&drawing)) {
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
        .map(|mut store| match drawing_block_on(store.import_dwg(&bytes)) {
            Ok(handle) => serde_json::json!({ "handle": handle.as_str() }).to_string(),
            Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 🗑️ Disposes a drawing handle owned by the in-process draw kernel.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn dispose_drawing(handle: &str) {
    if let Ok(mut store) = drawing_kernel().lock() {
        drawing_block_on(store.dispose(&DrawingHandle(handle.to_string())));
    }
}

/// 🔍️ Autotraces a bitmap mask into path segments JSON.
pub fn trace_bitmap_json(width: u32, height: u32, mask: &[u8], threshold: f64, simplify_epsilon: f64) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .and_then(|mut store| match drawing_block_on(store.trace_bitmap(width, height, mask, threshold, simplify_epsilon)) {
            Ok(handle) => match drawing_block_on(store.flatten_scene(&handle)) {
                Ok(scene) => {
                    let segments = scene.nodes.into_iter().find_map(|node| if let semio_s_2d::DrawingNode::Path { segments } = node.node { Some(segments) } else { None });
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
    let parse = |json: &str| -> Result<Vec<semio_s_2d::PathSegment>, DrawingKernelError> {
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
            (Ok(a), Ok(b)) => match drawing_block_on(store.boolean_segments(&a, &b, operation)) {
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
