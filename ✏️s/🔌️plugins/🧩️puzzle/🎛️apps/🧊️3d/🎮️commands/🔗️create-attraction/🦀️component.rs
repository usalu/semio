//! 🔗️ `create-attraction` command.

use crate::apps::puzzle3d::{derive_attraction_params, puzzle3d_kinds_compatible, puzzle3d_local_vortex_geom, resolve_puzzle3d_attractions, resolve_vortex_kind, Puzzle3dActionCtx, Puzzle3dAttraction, PUZZLE3D_ID_COUNTER};
use serde_json::Value;
use std::sync::atomic::Ordering;

pub fn create_attraction(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let attracting = args.and_then(|value| value.get("attracting")).and_then(|value| value.as_str()).unwrap_or("");
    let attracted = args.and_then(|value| value.get("attracted")).and_then(|value| value.as_str()).unwrap_or("");
    if attracting.is_empty() || attracted.is_empty() || attracting == attracted {
        return;
    }
    let already_connected = ctx.scene.fixture.attractions.iter().any(|attraction| (attraction.attracting == attracting && attraction.attracted == attracted) || (attraction.attracting == attracted && attraction.attracted == attracting));
    let compatible = match (resolve_vortex_kind(&ctx.scene.fixture, attracting), resolve_vortex_kind(&ctx.scene.fixture, attracted)) {
        (Some(source_kind), Some(target_kind)) => puzzle3d_kinds_compatible(&ctx.scene.fixture, &source_kind, &target_kind),
        _ => false,
    };
    if already_connected || !compatible {
        return;
    }
    let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
    // 🌲️ Keep the drag gesture's direction (source = attracting) but derive params from the CURRENT
    // poses of both objects, so creating an attraction never moves either endpoint.
    let (gap, shift, rise, rotation, turn, tilt) = match (puzzle3d_local_vortex_geom(&ctx.scene.fixture, attracting), puzzle3d_local_vortex_geom(&ctx.scene.fixture, attracted)) {
        (Some((attracting_object_id, p_a, d_a)), Some((attracted_object_id, p_b, d_b))) => {
            let pose = |object_id: &str| ctx.scene.fixture.objects.iter().find(|object| object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
            match (pose(&attracting_object_id), pose(&attracted_object_id)) {
                (Some((t_a, q_a)), Some((t_b, q_b))) => derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b),
                _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            }
        }
        _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    };
    ctx.scene.fixture.attractions.push(Puzzle3dAttraction { id, attracting: attracting.into(), attracted: attracted.into(), gap, shift, rise, rotation, turn, tilt });
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
