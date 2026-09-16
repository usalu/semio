//! 📄 `set-panel-page` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework::kernel::UiDirtyScope;
use serde_json::Value;

/// 📄 Advances one virtualised panel section to an explicit page index (window transient state).
pub fn set_panel_page(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let section = args.and_then(|value| value.get("section")).and_then(Value::as_str).unwrap_or("").to_string();
    let page = args.and_then(|value| value.get("page")).and_then(Value::as_u64).unwrap_or(0) as u32;
    if section.is_empty() {
        *ctx.ui_scope = UiDirtyScope::None;
        return;
    }
    ctx.scene.runtime.panel_pages.insert(section, page);
    *ctx.ui_scope = UiDirtyScope::Partial {
        window_bodies: Vec::new(),
        panel_bodies: vec![crate::editor::puzzle2d::panels::artifact::PUZZLE2D_PLAY_BODY_LAYERS.into(), crate::editor::puzzle2d::panels::catalogue::PUZZLE2D_PLAY_BODY_CATALOGUE.into(), crate::editor::puzzle2d::panels::inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.into()],
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    };
}
