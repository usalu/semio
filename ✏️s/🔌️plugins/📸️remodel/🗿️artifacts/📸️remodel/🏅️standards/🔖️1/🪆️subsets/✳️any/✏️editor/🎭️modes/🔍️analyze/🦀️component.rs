//! 🔍️ Remodel play app — the `analyze` mode: the reconstruction result beside the tabular report.
//! Owns the Report window.

use crate::editor::remodel::modes::analyze::windows::report;
use crate::editor::remodel::modes::model::windows::model;
use semio_framework_plugin::{create_default_layout, create_named_layout, LocalizedLabel, ModeDefinition, NamedLayout};

pub const REMODEL_PLAY_MODE_ANALYZE: &str = "analyze";
pub const REMODEL_PLAY_LAYOUT_ANALYZE: &str = "remodel-analyze";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: REMODEL_PLAY_MODE_ANALYZE.into(), label: LocalizedLabel::native("Analyze", "Analyse"), icon_id: "search".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

pub fn layout() -> NamedLayout {
    create_named_layout(
        REMODEL_PLAY_LAYOUT_ANALYZE,
        "Analyze",
        create_default_layout(&[model::REMODEL_PLAY_WINDOW_MAIN.into(), report::REMODEL_PLAY_WINDOW_REPORT.into()], "row", Some(&[60.0, 40.0]), Some(&["Model".into(), "Report".into()])),
        "builtin",
        Some("table-2".into()),
        None,
    )
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_analyze_layout_pairs_the_model_window_with_the_report() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(REMODEL_PLAY_LAYOUT_ANALYZE));
        assert!(json.contains(report::REMODEL_PLAY_WINDOW_REPORT));
    }
}
//#endregion 🧪️Tests
