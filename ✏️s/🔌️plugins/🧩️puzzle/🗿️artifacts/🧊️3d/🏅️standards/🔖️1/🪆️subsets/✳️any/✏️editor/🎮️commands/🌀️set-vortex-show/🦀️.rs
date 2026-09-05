//! ⚙️ `set-vortex-show` command.

use crate::editor::puzzle3d::{Puzzle3dActionCtx, PUZZLE3D_VORTEX_SHOW_ALWAYS, PUZZLE3D_VORTEX_SHOW_SELECTED};
use dsl::os_pack::json::Value;

pub fn set_vortex_show(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        if mode == PUZZLE3D_VORTEX_SHOW_ALWAYS || mode == PUZZLE3D_VORTEX_SHOW_SELECTED {
            ctx.scene.runtime.vortex_show = mode.into();
        }
    }
}
