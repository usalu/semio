//! 📷️ Remodeling play app — the `capture` mode: frame ingestion, with the filmstrip taking the lead.
//! Owns the Frames window.

use crate::editor::remodeling::modes::capture::windows::frames;
use crate::editor::remodeling::modes::model::windows::model;
use semio_framework_plugin::{create_default_layout, create_named_layout, LocalizedLabel, ModeDefinition, NamedLayout};

pub const REMODELING_PLAY_MODE_CAPTURE: &str = "capture";
pub const REMODELING_PLAY_LAYOUT_CAPTURE: &str = "remodeling-capture";

//#region 🔖️Definition
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: REMODELING_PLAY_MODE_CAPTURE.into(), label: LocalizedLabel::native("Capture", "Aufnahme"), icon_id: "camera".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub async fn layout() -> NamedLayout {
    create_named_layout(
        REMODELING_PLAY_LAYOUT_CAPTURE,
        "Capture",
        create_default_layout(&[frames::REMODELING_PLAY_WINDOW_FRAMES.into(), model::REMODELING_PLAY_WINDOW_MAIN.into()], "row", Some(&[60.0, 40.0]), Some(&["Frames".into(), "Model".into()])),
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
        assert!(json.contains(REMODELING_PLAY_LAYOUT_CAPTURE));
        assert!(json.contains(frames::REMODELING_PLAY_WINDOW_FRAMES));
    }
}
//#endregion 🧪️Tests
