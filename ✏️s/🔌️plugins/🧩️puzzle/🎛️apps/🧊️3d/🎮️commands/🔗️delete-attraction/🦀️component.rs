//! 🔗️ `delete-attraction` command.

use crate::apps::puzzle3d::{derive_attraction_params, puzzle3d_kinds_compatible, puzzle3d_local_vortex_geom, resolve_puzzle3d_attractions, resolve_vortex_kind, Puzzle3dActionCtx, Puzzle3dAttraction, PUZZLE3D_ID_COUNTER};
use serde_json::Value;
use std::sync::atomic::Ordering;

pub fn delete_attraction(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
        ctx.scene.fixture.attractions.retain(|attraction| attraction.id != id);
    }
}
