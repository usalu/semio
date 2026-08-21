//! 🖌️ `register-brush-mesh` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

/// 🧊️ Real GLB geometry the browser round-tripped for one mesh url — installed into the collision
/// engine and remembered so `drive_precompute` never re-registers a box over it. Aborts (emitting
/// nothing at all) because the session cache is not document or config state — the pre-migration
/// `return Emit::default()`.
pub async fn register_brush_mesh(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let (Some(url), Some(positions), Some(indices)) = (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array())) {
        let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
        let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
        ctx.app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
        ctx.app.registered_mesh_urls.borrow_mut().insert(url.to_string());
    }
    ctx.abort = true;
}
