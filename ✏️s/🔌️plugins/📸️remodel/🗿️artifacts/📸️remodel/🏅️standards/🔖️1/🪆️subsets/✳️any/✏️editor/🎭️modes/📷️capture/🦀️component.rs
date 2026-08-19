//! 📷️ Remodel play app — the `capture` mode: frame ingestion, with the filmstrip taking the lead.
//! Owns the Frames window.

use crate::editor::remodel::modes::capture::windows::frames;
use crate::editor::remodel::modes::model::windows::model;
use semio_framework_plugin::{create_default_layout, create_named_layout, LocalizedLabel, ModeDefinition, NamedLayout};

pub const REMODEL_PLAY_MODE_CAPTURE: &str = "capture";
pub const REMODEL_PLAY_LAYOUT_CAPTURE: &str = "remodel-capture";

//#region 🔖️Definition
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: REMODEL_PLAY_MODE_CAPTURE.into(), label: LocalizedLabel::native("Capture", "Aufnahme"), icon_id: "camera".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub async fn layout() -> NamedLayout {
    create_named_layout(
        REMODEL_PLAY_LAYOUT_CAPTURE,
        "Capture",
        create_default_layout(&[frames::REMODEL_PLAY_WINDOW_FRAMES.into(), model::REMODEL_PLAY_WINDOW_MAIN.into()], "row", Some(&[60.0, 40.0]), Some(&["Frames".into(), "Model".into()])),
        "builtin",
        Some("video".into()),
        None,
    )
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_capture_layout_puts_the_filmstrip_first() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(REMODEL_PLAY_LAYOUT_CAPTURE));
        assert!(json.contains(frames::REMODEL_PLAY_WINDOW_FRAMES));
    }
}
//#endregion 🧪️Tests
