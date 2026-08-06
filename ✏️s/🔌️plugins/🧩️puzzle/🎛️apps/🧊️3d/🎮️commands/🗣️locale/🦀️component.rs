//! 🗣️ Puzzle 3d play app commands — locale and terminology. B1: both used to be host-pushed
//! `ViewModel` fields with no app-level action of their own; now that `ViewModel` is gone from the
//! app-facing surface they are real config edits.

use crate::apps::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub fn set_locale(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        ctx.scene.runtime.locale = value.into();
    }
}

pub fn set_terminology(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        ctx.scene.runtime.terminology = value.into();
    }
}
