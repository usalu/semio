//! 🤝️ Puzzle 5d play app commands — the per-window engagement command line: keystroke echo, token
//! submission (which doubles as a programmatic utility switch) and abort.

use crate::apps::puzzle5d::config::puzzle5d_clear_selection;
use crate::apps::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::apps::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_PLAY_WINDOWS};
use serde_json::Value;

pub fn engagement_input(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(board2d::WINDOW_KIND_ID);
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
    if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
        ctx.scene.runtime.engagement_input_by_window.insert(window.to_string(), value.to_string());
    }
}

/// ⌨️ One submitted token: `select`/`brush`/`fill` switch the utility (the 3D window's `select` lands
/// on the `move` gumball instead), `clear` drops the selection, `rectangle`/`lasso` set the marquee.
pub fn engagement_submit(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(board2d::WINDOW_KIND_ID).to_string();
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map_or("", str::trim).to_lowercase();
    match value.as_str() {
        "select" if window == world3d::WINDOW_KIND_ID => ctx.scene.active_utility = "move".into(),
        "select" | "brush" | "fill" => {
            ctx.scene.active_utility = if value == "select" { "select".into() } else { value };
            if ctx.scene.active_utility != "select" {
                ctx.app.drive_precompute(ctx.scene);
            }
        }
        "clear" => puzzle5d_clear_selection(&mut ctx.scene.runtime.selection),
        "rectangle" | "lasso" => ctx.scene.runtime.selection_method = value,
        _ => {}
    }
    if PUZZLE5D_PLAY_WINDOWS.contains(&window.as_str()) {
        ctx.scene.runtime.engagement_input_by_window.insert(window, String::new());
    }
}

pub fn engagement_abort(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(window) = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()) {
        if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
            ctx.scene.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
        }
    }
    let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(board2d::WINDOW_KIND_ID);
    ctx.scene.active_utility = if window == world3d::WINDOW_KIND_ID { "move".into() } else { "select".into() };
}
