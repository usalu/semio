//! 👁️ Puzzle 5D viewer — the `view` mode: the read-only counterpart of the editor's paired-window
//! `edit` mode, and paired the same way — a `◻️2d` board pane beside a `🧊️3d` world pane, so a
//! read-only 5d document shows BOTH projections instead of the world one alone.

use crate::viewer::puzzle5d::modes::view::windows::{board2d, world3d};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PUZZLE5D_VIEW_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::puzzle5d::create_puzzle5d_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PUZZLE5D_VIEW_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The same paired 40/60 split the editor's `edit` mode opens with — board pane on the left,
/// world pane on the right — so a read-only 5d document reads the way the edited one looks.
pub fn layout() -> WindowLayout {
    create_default_layout(&[board2d::WINDOW_KIND_ID.into(), world3d::WINDOW_KIND_ID.into()], "row", Some(&[40.0, 60.0]), Some(&["Puzzle 2D".into(), "Puzzle 3D".into()]))
}
//#endregion 🔖️Definition
