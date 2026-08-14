//! 🔄️ `scale-selection` command.

use serde_json::{json, Value};
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use crate::apps::puzzle5d::mesh_selection_ids;
use crate::apps::puzzle5d::part_scale_json;

pub fn scale_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.selected_part_ids());
    let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    for part in &mut ctx.scene.document.parts {
        if ids.contains(&part.id) {
            let current = part_scale_json(part);
            part.part_3d.scale = Some(json!([current[0] * sx, current[1] * sy, current[2] * sz]));
        }
    }
}
