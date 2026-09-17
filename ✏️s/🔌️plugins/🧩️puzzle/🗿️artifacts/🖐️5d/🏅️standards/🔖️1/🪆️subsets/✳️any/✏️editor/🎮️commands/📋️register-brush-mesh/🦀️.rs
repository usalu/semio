//! 📋️ `register-brush-mesh` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::derive_brush_mesh;

/// 🧊️ The ONE install: real GLB geometry the browser round-tripped for one mesh url, derived into puzzle
/// 3d's process-wide mesh store, which the brush suggestions and fill runs read. Answers whether the store
/// accepted it, so a retained caller can surface a refusal instead of completing silently.
pub fn puzzle5d_install_brush_mesh(url: &str, positions: &[f32], indices: &[u32]) -> bool {
    !url.is_empty() && !positions.is_empty() && !indices.is_empty() && derive_brush_mesh(url, positions, indices).is_some()
}

/// 🧊️ The synchronous arm behind `ArtifactEditor::handle`. Aborts (emitting nothing at all) because that
/// store is neither document nor config state — the retained route is `Puzzle5dRegisterBrushMeshWork`,
/// which pages the two number arrays in and calls the same [`puzzle5d_install_brush_mesh`].
pub fn register_brush_mesh(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let (Some(url), Some(positions), Some(indices)) = (args.and_then(|v| v.get("url")).and_then(Value::as_str), args.and_then(|v| v.get("positions")).and_then(Value::as_array), args.and_then(|v| v.get("indices")).and_then(Value::as_array)) {
        let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
        let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().and_then(|n| u32::try_from(n).ok())).collect();
        if !puzzle5d_install_brush_mesh(url, &positions, &indices) {
            ctx.notice(|labels| labels.brush_reason_pose_unavailable.as_str());
        }
    }
    ctx.abort = true;
}
