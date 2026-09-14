//! 📋️ `register-brush-mesh` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::derive_brush_mesh;

/// 🧊️ Real GLB geometry the browser round-tripped for one mesh url — derived into puzzle 3d's process-wide mesh
/// store, which the brush suggestions and fill runs read. Aborts (emitting nothing at all) because that store is
/// neither document nor config state.
pub fn register_brush_mesh(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let (Some(url), Some(positions), Some(indices)) = (args.and_then(|v| v.get("url")).and_then(Value::as_str), args.and_then(|v| v.get("positions")).and_then(Value::as_array), args.and_then(|v| v.get("indices")).and_then(Value::as_array)) {
        let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
        let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().and_then(|n| u32::try_from(n).ok())).collect();
        let _ = derive_brush_mesh(url, &positions, &indices);
    }
    ctx.abort = true;
}
