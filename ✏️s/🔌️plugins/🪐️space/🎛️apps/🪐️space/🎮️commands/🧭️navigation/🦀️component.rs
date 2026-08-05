//! 🧭️ S Studio app — shell navigation + app-registry push commands.

use crate::apps::space::config::{SpaceConfig, SpaceConfigOperation};
use semio_framework_os::{register_app_io, AppDefinition, WorkflowDocument, WorkflowOperation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};
use serde::Deserialize;

//#region 🔖️SetActivePanelTab
pub mod set_active_panel_tab {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-panel-tab")]
    pub struct SetActivePanelTab {
        pub tab_id: String,
    }

    pub fn handle(payload: &SetActivePanelTab, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        Ok(Emit::config(vec![SpaceConfigOperation::SetActivePanelTab { tab_id: payload.tab_id.clone() }]))
    }
}
//#endregion 🔖️SetActivePanelTab

//#region 🔖️GoHome
pub mod go_home {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "go-home")]
    pub struct GoHome {}

    pub fn handle(_payload: &GoHome, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        Ok(Emit::effect(HostEffect::Navigate { uri: "/".into() }))
    }
}
//#endregion 🔖️GoHome

//#region 🔖️NavigateVirtualFileSystemNode
pub mod navigate_virtual_file_system_node {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "navigate-vfs-node")]
    pub struct NavigateVirtualFileSystemNode {
        pub space_id: String,
    }

    pub fn handle(payload: &NavigateVirtualFileSystemNode, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
    }
}
//#endregion 🔖️NavigateVirtualFileSystemNode

//#region 🔖️SetAppRegistrations
/// 🪐️ One `appRegistrationsJson` entry — the wire shape `os-shell.tsx`'s `SetAppRegistrations` push
/// builds from `loadedPlugins.flatMap(entry => entry.manifest.apps.map(app => ({pluginId, app})))`.
/// `app` deserializes straight off `AppDefinition`'s own `Deserialize` impl since it's the literal
/// manifest-JSON `AppDefinition` object, unmodified across the wasm boundary.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppRegistrationWireEntry {
    plugin_id: String,
    app: AppDefinition,
}

/// 🪐️ Registers every `{pluginId, app}` entry `json` carries into this wasm instance's OWN
/// `semio_framework_os::APP_REGISTRATIONS` copy — the space app is its own wasm component, so its
/// statically-linked copy of os-core's `APP_REGISTRATIONS` never sees what native/test hosts populate
/// via `PluginHost::load_plugin`/`hot_swap_plugin`; this is how it gets populated in a real
/// browser/wasm host instead. Malformed/empty `json` degrades to a silent no-op — this is a
/// best-effort host hint push, not a user-facing operation with error surfacing.
fn apply_app_registrations(json: &str) {
    let Ok(entries) = serde_json::from_str::<Vec<AppRegistrationWireEntry>>(json) else { return };
    for entry in entries {
        register_app_io(&entry.plugin_id, &entry.app);
    }
}

pub mod set_app_registrations {
    use super::*;
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-app-registrations")]
    pub struct SetAppRegistrations {
        pub json: String,
    }

    /// 🪐️ Pure host-hint side effect; no document/config mutation, so the default full-refresh `Emit`
    /// is enough to pick up the newly-registered apps on the next catalogue render.
    pub fn handle(payload: &SetAppRegistrations, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        super::apply_app_registrations(&payload.json);
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetAppRegistrations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&set_active_panel_tab::SetActivePanelTab { tab_id: "s-play-catalogue".into() });
        store::test_support::assert_op_line_round_trip(&go_home::GoHome {});
        store::test_support::assert_op_line_round_trip(&navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { space_id: "demo".into() });
        store::test_support::assert_op_line_round_trip(&set_app_registrations::SetAppRegistrations { json: "[]".into() });
    }

    /// 🪐️ End-to-end proof of the catalogue-empty bugfix: registers an app with an EMPTY `document`
    /// breadcrumb purely through this command, then asserts the registry, `workflow_palette()`, and
    /// `build_catalogue_tree` all pick it up.
    #[test]
    fn set_app_registrations_command_registers_app_and_surfaces_empty_document_apps_in_catalogue() {
        use crate::apps::space::testkit::studio_emit;
        use crate::apps::space::SpaceCommand;
        use semio_framework_os::{empty_workflow_document, os_app_registration, workflow_palette, ArtifactPresentation, MediaClass, MediaForm, MediaType};
        use semio_framework_plugin::{App, AppIo, LocalizedLabel, SurfaceKind};
        use serde_json::json;
        // 🌉️ `AppBuilder::build_definition` itself hard-asserts a non-empty `document` — so the
        // empty-breadcrumb case can only ever reach `register_app_io` via a wire-decoded `AppDefinition`
        // that bypassed the builder entirely. Simulate that faithfully: build a normal, valid
        // definition, then blank `document` out at the JSON level before it's pushed.
        let definition = App::builder("root-tool", LocalizedLabel::data("Root Tool"))
            .document(["root-tool".to_string()])
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), "root-tool.main", SurfaceKind::Canvas2d, "square-pen")
            .io(AppIo::from_document("root-tool.document", MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: "root-tool".into(), name: "Root Tool".into(), dimension: String::new(), component_kind: "root-tool".into() }))
            .build_definition();
        let mut app_json = serde_json::to_value(&definition).expect("serialize AppDefinition");
        app_json["document"] = json!([]);
        let wire = json!([{ "pluginId": "root", "app": app_json }]).to_string();
        let projection = empty_workflow_document();
        let config = SpaceConfig::default();
        studio_emit(&projection, &config, SpaceCommand::SetAppRegistrations(set_app_registrations::SetAppRegistrations { json: wire })).expect("handle");
        assert!(os_app_registration("root", "root-tool").is_some(), "SetAppRegistrations must populate this wasm instance's own registry");
        assert!(workflow_palette().iter().any(|entry| entry.plugin_id == "root" && entry.app_id == "root-tool"), "workflow_palette must surface the pushed app");
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::apps::space::terminology::SStudioLabels>(&config.locale);
        let tree = crate::apps::space::panels::catalogue::build_catalogue_tree(labels, semio_framework_plugin::locale_from_str(&config.locale));
        let json_tree = serde_json::to_string(&tree).unwrap();
        assert!(json_tree.contains("s-play-catalogue.document.root-tool"), "an empty-document app must still surface as a top-level catalogue leaf, json={json_tree}");
    }
}
//#endregion 🧪️Tests
