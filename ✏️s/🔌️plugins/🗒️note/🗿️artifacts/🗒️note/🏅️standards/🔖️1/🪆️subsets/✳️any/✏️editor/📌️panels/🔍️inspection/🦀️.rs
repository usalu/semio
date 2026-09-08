//! 🔍️ Note play app panel — the document-wide properties summary (schema, block count, active
//! utility, snap status).

use crate::schema::flatten_blocks;
use crate::NoteSnapshot;
use crate::editor::note::terminology::NotePlayLabels;
use crate::editor::note::ui_label;
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, PluginAssemblyError, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const NOTE_PLAY_BODY_PROPERTIES: &str = "note.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(NOTE_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactEditor::render` carries no
/// `InteractionView` (a known SDK gap — matches `gis2d`'s inspection panel precedent), so this panel
/// can no longer tell which blocks are selected — it always shows the document-wide summary now; the
/// per-selected-block detail branch (name/x/y/width/height/visible/locked, driven by `patchBlocks`)
/// that used to read `cfg.selected_block_ids` is gone with it.
pub fn render(document: &NoteSnapshot, active_utility_id: &str, labels: &NotePlayLabels) -> UiAssemblyResult<BuiltNode> {
    let mut section = semio_framework_ui_contract::section(ui_label(labels.inspection.as_str())?).default_open(true).try_id("note-inspector").map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note inspector id admission failed"))?;
    for (index, value) in [
        format!("{}: {}", labels.summary_schema.as_str(), document.schema),
        format!("{}: {}", labels.summary_blocks.as_str(), flatten_blocks(&document.blocks).len()),
        format!("{}: {active_utility_id}", labels.summary_utility.as_str()),
        format!("{}: {}", labels.summary_snap.as_str(), if document.snap_enabled.unwrap_or(false) { format!("{}px", document.snap_grid_spacing.unwrap_or(8.0)) } else { labels.summary_off.as_str().into() }),
    ].into_iter().enumerate() {
        let child = semio_framework_ui_contract::text(ui_label(value)?).try_id(format!("note-inspector.summary.{index}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note summary key admission failed"))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note summary text admission failed"))?;
        section = section.try_child(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note inspector child admission failed"))?;
    }
    section.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "note inspector node admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::note::testkit::{note_app, render as render_body};
    use crate::editor::note::NOTE_PLAY_BODY_PROPERTIES as BODY_PROPERTIES;

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = note_app().await;
        assert!(render_body(&mut app, "note.play.nope").await.contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_the_document_wide_summary() {
        let mut app = note_app().await;
        let json = render_body(&mut app, BODY_PROPERTIES).await;
        assert!(json.contains("Utility:"));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semantic_contract {
    use super::*;
    #[test]
    fn note_semantic_panels_match_the_json_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️summary.json")).expect("neutral UI vectors");
        let mut snapshot = crate::schema::empty_note_snapshot();
        snapshot.snap_enabled = Some(false);
        for row in fixture["cases"].as_array().expect("locale cases") {
            let config = crate::editor::note::config::NoteConfig { locale: row["locale"].as_str().expect("locale").into(), ..Default::default() };
            let labels = crate::editor::note::terminology::note_play_labels(&config);
            let inspector = render(&snapshot, fixture["utility"].as_str().expect("utility"), labels).expect("inspector");
            let projection = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(inspector)).expect("project and retire inspector");
            let actual: serde_json::Value = serde_json::from_str(&projection).expect("independent JSON oracle");
            assert_eq!(actual["component"]["label"], row["heading"]);
            let lines: Vec<_> = actual["children"].as_array().expect("summary").iter().map(|child| child["component"]["value"].clone()).collect();
            assert_eq!(lines, *row["lines"].as_array().expect("summary lines"));
            let catalogue = crate::editor::note::panels::catalogue::render(labels).expect("catalogue");
            let projection = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(catalogue)).expect("project and retire catalogue");
            let actual: serde_json::Value = serde_json::from_str(&projection).expect("catalogue JSON oracle");
            assert_eq!(actual["children"][0]["component"]["label"], row["catalogue"]);
            assert_eq!(actual["children"][0]["children"][0]["component"]["value"], row["firstKind"]);
        }
    }

    #[test]
    fn note_ink_canvas_payload_matches_the_json_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️summary.json")).expect("neutral canvas vectors");
        let snapshot = crate::schema::empty_note_snapshot();
        let camera = serde_json::from_value(fixture["camera"].clone()).expect("camera oracle");
        for mode in ["composite", "navigator"] {
            let node = crate::editor::note::modes::edit::windows::composite::render_canvas_scene(&snapshot, &camera, fixture["utility"].as_str().expect("utility"), "note.fixture.canvas", mode).expect("canvas");
            let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("semantic canvas surface") };
            let scene: semio_framework_ui_scene::InkCanvasScene = semio_framework_ui_scene::decode(props).expect("decode actual packed scene");
            let actual: serde_json::Value = serde_json::from_str(&scene.document_json).expect("independent canvas JSON oracle");
            assert_eq!(actual["schema"], fixture["schema"]);
            assert_eq!(actual["camera"], fixture["camera"]);
            assert_eq!(actual["blocks"], serde_json::json!([]));
            assert_eq!(scene.active_utility, fixture["utility"].as_str().expect("utility"));
            assert_eq!(scene.view_mode, mode);
            assert_eq!(scene.interactive, mode == "composite");
            semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire canvas");
        }
    }
}
