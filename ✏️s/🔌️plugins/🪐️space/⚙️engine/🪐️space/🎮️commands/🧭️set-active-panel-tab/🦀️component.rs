//! 🧭️ 🧭️ S Studio app command — `set-active-panel-tab`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-panel-tab")]
pub struct SetActivePanelTab {
    pub tab_id: String,
}

pub async fn handle(payload: &SetActivePanelTab, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(vec![SpaceConfigMutation::SetActivePanelTab { tab_id: payload.tab_id.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn space_command_op_text_round_trips_every_variant() {
        use crate::engine::space::SpaceCommand;
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SetActivePanelTab(SetActivePanelTab { tab_id: "s-play-catalogue".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::GoHome(crate::engine::space::commands::go_home::GoHome {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NavigateVirtualFileSystemNode(crate::engine::space::commands::navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { space_id: "demo".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SetAppRegistrations(crate::engine::space::commands::set_app_registrations::SetAppRegistrations { json: "[]".into() }));
    }

    /// 🪐️ End-to-end proof of the catalogue-empty bugfix: registers an app with an EMPTY `document`
    /// breadcrumb purely through this command, then asserts the registry, `workflow_palette()`, and
    /// `build_catalogue_tree` all pick it up.
    #[semio_framework_async_macros::async_test]
    async fn set_app_registrations_command_registers_app_and_surfaces_empty_document_apps_in_catalogue() {
        use crate::engine::space::testkit::studio_emit;
        use crate::engine::space::SpaceCommand;
        use semio_framework_os::{empty_workflow_snapshot, os_app_registration, workflow_palette, ArtifactPresentation, MediaClass, MediaForm, MediaType};
        use semio_framework_plugin::{App, AppIo, LocalizedLabel, SurfaceKind};
        use serde_json::json;
        // 🌉️ `AppBuilder::build_definition` itself hard-asserts a non-empty `document` — so the
        // empty-breadcrumb case can only ever reach `register_app_io` via a wire-decoded `AppDefinition`
        // that bypassed the builder entirely. Simulate that faithfully: build a normal, valid
        // definition, then blank `document` out at the JSON level before it's pushed.
        // 🪪️ `App::builder`'s id must parse via `semio_framework::parse_surface_app_id` (ticket
        // 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1) — mirror `testkit::test_surface_id`'s
        // synthetic-dialect convention here since this test builds its `AppDefinition` by hand instead
        // of through `seed_app`.
        let root_tool_id = crate::engine::space::testkit::test_surface_id("root-tool");
        let definition = App::builder(root_tool_id.clone(), LocalizedLabel::data("Root Tool"))
            .document(["root-tool".to_string()])
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), "root-tool.main", SurfaceKind::Canvas2d, "square-pen")
            .io(AppIo::from_document(
                "root-tool.document",
                MediaType { class: MediaClass::Data, form: MediaForm::Value },
                ArtifactPresentation { id: "root-tool".into(), name: "Root Tool".into(), dimension: String::new(), component_kind: "root-tool".into() },
            ))
            .build_definition();
        let mut app_json = serde_json::to_value(&definition).expect("serialize AppDefinition");
        // 🩹️ `AppDefinition`'s wire field is `breadcrumb` (`AppBuilder::document(...)` is the builder
        // METHOD name that sets it, not the serialized field name — `#[serde(rename_all =
        // "camelCase")]` leaves the single-word `breadcrumb` unchanged). Blanking `"document"` here was
        // a no-op that silently left the real `"breadcrumb": ["root-tool"]` untouched, so this test's
        // "empty breadcrumb" simulation never actually happened — masked until now by the canonical-id
        // panic this lane fixed, which never let execution reach this far before.
        app_json["breadcrumb"] = json!([]);
        let wire = json!([{ "pluginId": "root", "app": app_json }]).to_string();
        let projection = empty_workflow_snapshot();
        let config = SpaceConfig::default();
        studio_emit(&projection, &config, &SpaceCommand::SetAppRegistrations(crate::engine::space::commands::set_app_registrations::SetAppRegistrations { json: wire })).expect("handle");
        assert!(os_app_registration("root", &root_tool_id).is_some(), "SetAppRegistrations must populate this wasm instance's own registry");
        assert!(workflow_palette().iter().any(|entry| entry.plugin_id == "root" && entry.app_id == root_tool_id), "workflow_palette must surface the pushed app");
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::engine::space::terminology::SStudioLabels>(&config.locale);
        let tree = crate::engine::space::panels::catalogue::build_catalogue_tree(labels, semio_framework_plugin::locale_from_str(&config.locale));
        let json_tree = serde_json::to_string(&tree).unwrap();
        assert!(json_tree.contains(&format!("s-play-catalogue.document.{root_tool_id}")), "an empty-document app must still surface as a top-level catalogue leaf, json={json_tree}");
    }
}
//#endregion 🧪️Tests
