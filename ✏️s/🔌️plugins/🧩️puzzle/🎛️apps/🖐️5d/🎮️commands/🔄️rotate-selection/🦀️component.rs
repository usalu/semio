//! 🔄️ `rotate-selection` command.

use serde_json::{json, Value};
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use crate::apps::puzzle5d::mesh_selection_ids;
use crate::apps::puzzle5d::quat_from_axis_angle;
use crate::apps::puzzle5d::quat_mul;

pub fn rotate_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.selected_part_ids());
    let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let delta = quat_from_axis_angle(ax, ay, az, angle);
    for part in &mut ctx.scene.document.parts {
        if ids.contains(&part.id) {
            let current = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            part.part_3d.orientation = Some(quat_mul(delta, current));
        }
    }
}
