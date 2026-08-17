//! 🖌️ `register-brush-mesh` command.

use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;

/// 🧊️ Real GLB geometry the browser round-tripped for one mesh url — installed into the collision
/// engine and remembered for the mesh exporters.
pub fn register_brush_mesh(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let (Some(url), Some(positions), Some(indices)) =
        (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
    else {
        return;
    };
    let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
    let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
    ctx.app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
    if let Ok(mut registry) = crate::editor::puzzle3d::PUZZLE3D_MESH_REGISTRY.lock() {
        registry.insert(url.to_string(), (positions, indices));
    }
}
