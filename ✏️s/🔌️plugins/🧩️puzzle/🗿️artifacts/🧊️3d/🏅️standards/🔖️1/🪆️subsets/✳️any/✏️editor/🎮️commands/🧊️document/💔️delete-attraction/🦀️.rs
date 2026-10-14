//! 🔗️ `delete-attraction` command.

use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices;
use semio_framework_pack_json::Value;

/// 💔️ One `disconnect-vortices` for the named attraction, when the base holds it.
pub fn delete_attraction(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).filter(|id| ctx.base.attractions.iter().any(|attraction| attraction.id == *id)) {
        ctx.artifact_mutations.push(disconnect_vortices(id.to_string()));
    }
}
