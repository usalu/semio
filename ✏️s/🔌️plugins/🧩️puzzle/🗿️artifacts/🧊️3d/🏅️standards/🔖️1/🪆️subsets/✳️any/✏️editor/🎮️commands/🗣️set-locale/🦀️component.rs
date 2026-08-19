//! 🗣️ `set-locale` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub async fn set_locale(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        ctx.scene.runtime.locale = value.into();
    }
}
