//! 📄 `set-panel-page` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 📄 Advances one virtualised panel section to an explicit page index.
pub fn set_panel_page(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let section = args.and_then(|value| value.get("section")).and_then(|value| value.as_str()).unwrap_or("").to_string();
    let page = args.and_then(|value| value.get("page")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
    if section.is_empty() {
        return;
    }
    ctx.scene.runtime.panel_pages.insert(section, page);
}
