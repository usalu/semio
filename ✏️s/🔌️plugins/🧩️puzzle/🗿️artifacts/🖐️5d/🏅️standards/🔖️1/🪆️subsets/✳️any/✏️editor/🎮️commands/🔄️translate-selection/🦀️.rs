//! 🔄️ `translate-selection` command.

use crate::editor::puzzle5d::mesh_selection_ids;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn translate_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.selected_part_ids());
    let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    for part in &mut ctx.scene.document.parts {
        if ids.contains(&part.id) {
            part.part_3d.origin[0] += dx;
            part.part_3d.origin[1] += dy;
            part.part_3d.origin[2] += dz;
        }
    }
}
