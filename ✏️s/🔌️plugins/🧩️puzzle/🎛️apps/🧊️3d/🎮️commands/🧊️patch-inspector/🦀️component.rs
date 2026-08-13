//! 🧊️ `patch-inspector` command.

use crate::apps::puzzle3d::panels::inspection;
use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use std::collections::HashSet;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::apply_puzzle3d_inspector_patch;
use crate::apps::puzzle3d::resolve_puzzle3d_attractions;

pub fn patch_inspector(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str()).unwrap_or("");
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let ids = args
        .and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok())
        .filter(|ids| !ids.is_empty())
        .unwrap_or_else(|| inspection::target_ids(entity, &ctx.scene.runtime.selection));
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    apply_puzzle3d_inspector_patch(&mut ctx.scene.fixture, entity, &ids, field, value, delta);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
