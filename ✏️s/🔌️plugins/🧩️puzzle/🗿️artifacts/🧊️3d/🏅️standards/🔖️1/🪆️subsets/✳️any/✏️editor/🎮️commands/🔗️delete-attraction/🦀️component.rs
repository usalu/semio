//! 🔗️ `delete-attraction` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub async fn delete_attraction(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
        ctx.scene.fixture.attractions.retain(|attraction| attraction.id != id);
    }
}
