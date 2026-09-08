//! 🔍️ Imperative play app panel — inspection: read-only summary of the document.

use crate::ProcedureSnapshot;
use crate::editor::procedure::terminology::ImperativeLabels;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const IMPERATIVE_PLAY_BODY_INSPECTOR: &str = "imperative.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(IMPERATIVE_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ⚠️ Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-selected-step field group
/// (id/kind/params, resolved from `ImperativeConfig::selected_step_ids`) this panel used to build is
/// deleted along with that field — selection is framework-owned state now and
/// `ArtifactApp::render(body_key, doc, cfg)` is never given an `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` are). Documented reduced-fidelity gap, same shape as
/// `🖍️draw`'s `📌️panels/🔍️properties/🦀️.rs`: falls through to a step-count summary until a
/// resolved-selection render path exists.
pub fn render(document: &ProcedureSnapshot, labels: &ImperativeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let path = crate::procedure_working_scene(document).path;
    let field = tree_item_desc("imperative-play-inspector.steps", labels.inspector_steps.as_str(), Some(path.steps.len().to_string()))?;
    let mut fields = semio_framework_plugin::UiFixedList::default();
    fields
        .try_push(field)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed inspector field admission failed"))?;
    PanelTreeBuilder::new("imperative-play-inspector")?.section("imperative-play-inspector.summary", Some(crate::editor::procedure::ui_label(labels.inspection_title.as_str())?), true, fields)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedure::testkit::{imperative_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn inspection_shows_step_count_summary() {
        let mut app = imperative_app().await;
        assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_INSPECTOR).await.contains("imperative-play-inspector.steps"));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semantic_contract {
    use super::*;

    fn project(node: semio_framework_plugin::BuiltNode) -> serde_json::Value {
        let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
        serde_json::from_str(&text).expect("independent UI oracle")
    }

    #[test]
    fn imperative_semantic_panels_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️panels.json")).expect("neutral UI vectors");
        let document = ProcedureSnapshot::default();
        for row in vectors["cases"].as_array().expect("locales") {
            let labels = semio_framework_plugin::resolve_labels_for_locale::<ImperativeLabels>(row["locale"].as_str().expect("locale"));
            let tree = project(crate::editor::procedure::panels::document::render(&document, labels).expect("document"));
            assert_eq!(tree["children"][0]["component"]["label"], row["document"]);
            let catalogue = crate::editor::procedure::panels::catalogue::render(labels).expect("catalogue");
            let actions = catalogue.children[0].children.iter().map(|node| serde_json::to_value(&node.bindings[0].args).expect("independent action oracle")).collect::<Vec<_>>();
            let tree = project(catalogue);
            assert_eq!(tree["children"][0]["component"]["label"], row["catalogue"]);
            for (index, kind) in vectors["actionKinds"].as_array().expect("actions").iter().enumerate() {
                assert_eq!(actions[index]["kind"], *kind);
            }
            let tree = project(render(&document, labels).expect("inspector"));
            assert_eq!(tree["children"][0]["component"]["label"], row["inspection"]);
            assert_eq!(tree["children"][0]["children"][0]["component"]["label"], row["steps"]);
            let node = crate::editor::procedure::modes::edit::windows::main::render(&document, &vectors["runOutput"].to_string(), labels).expect("table");
            let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("table surface") };
            let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(props).expect("packed table");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.columns_json).expect("columns oracle"), row["columns"]);
            assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.rows_json).expect("rows oracle"), vectors["rows"]);
            project(node);
        }
        for node in [
            crate::editor::procedure::modes::edit::windows::script::render(&document).expect("editor text"),
            crate::viewer::procedure::modes::view::windows::script::render(&document).expect("viewer text"),
        ] {
            let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("text surface") };
            let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("packed text");
            assert_eq!(scene.language.as_deref(), Some("imperative"));
            assert_eq!(serde_json::from_str::<serde_json::Value>(scene.settings_json.as_deref().expect("settings")).expect("settings oracle"), vectors["settings"]);
            project(node);
        }
    }
}
