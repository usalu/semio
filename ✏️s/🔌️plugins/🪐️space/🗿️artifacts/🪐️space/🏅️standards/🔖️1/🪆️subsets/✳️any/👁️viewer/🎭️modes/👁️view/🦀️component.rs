//! 👁️ SpaceIndexViewer — the `view` mode: a single full-pane read-only artifact table.

use crate::viewer::space_index::modes::view::windows::main;
use semio_framework_plugin::{ModeDefinition, WindowLayout};

pub const SPACE_INDEX_MODE_VIEW: &str = "view";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: SPACE_INDEX_MODE_VIEW.into(), label: semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), icon_id: "table".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub fn layout() -> WindowLayout {
    semio_framework_plugin::create_default_layout(&[main::WINDOW_KIND_ID.into()], "row", Some(&[100.0]), Some(&["Artifacts".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mode_is_the_viewers_default() {
        assert_eq!(definition().id, SPACE_INDEX_MODE_VIEW);
    }
}
//#endregion 🧪️Tests
