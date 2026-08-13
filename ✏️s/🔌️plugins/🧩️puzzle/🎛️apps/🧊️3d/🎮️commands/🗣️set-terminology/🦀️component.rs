//! 🗣️ `set-terminology` command.

use crate::apps::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub fn set_terminology(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        ctx.scene.runtime.terminology = value.into();
    }
}
