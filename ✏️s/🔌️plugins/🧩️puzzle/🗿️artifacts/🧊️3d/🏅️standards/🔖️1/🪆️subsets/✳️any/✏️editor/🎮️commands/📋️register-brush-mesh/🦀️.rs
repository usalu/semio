//! 🖌️ `register-brush-mesh` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 🥽️ Real GLB geometry for one mesh id. Two admissible forms, and the id-only one is the normal case:
///
/// - `{url}` alone — the geometry is derived from the process-wide content-addressed mesh store, so a
///   mesh any document already uploaded costs nothing on the wire ever again.
/// - `{url, positions, indices}` — a first upload, bounded by what one retained command can actually
///   carry (see [`MAX_POSITIONS`]).
///
/// The decode itself is the `"puzzle3d.mesh-decode"` engine kernel
/// (`✏️editor/⏳️precompute/🦀️.rs`), keyed by `(engine id, url, geometry)`, so one identity decodes once
/// per process and every open document reads the identical derived page.
pub fn register_brush_mesh(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(url) = args.and_then(|v| v.get("url")).and_then(|v| v.as_str()) else {
        return;
    };
    if url.len() > MAX_LEAF_BYTES {
        return;
    }
    let (Some(positions), Some(indices)) = (args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array())) else {
        ctx.app.precompute.borrow_mut().adopt_shared_mesh(url);
        return;
    };
    if positions.len() > MAX_POSITIONS || indices.len() > MAX_INDICES {
        return;
    }
    let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
    let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
    ctx.app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
}

const MAX_LEAF_BYTES: usize = 4 * 1024;
/// 📏️ What one `registerBrushMesh` upload can honestly carry: the shared retained-command contract
/// decodes at most `PUZZLE_COMMAND_DECODED_ITEMS` array items per command, and
/// `Puzzle3dPrecomputeCommandWork::extent` already refuses anything above it. The previous 196,608
/// promised two orders of magnitude more than the 8,192-byte wire could ever deliver; geometry larger
/// than this reaches the engine by id through the derived-mesh store instead of by upload.
const MAX_POSITIONS: usize = crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS;
const MAX_INDICES: usize = crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS;
