
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
    use crate::engine::space::SpaceCommand;
    use crate::engine::space::testkit::studio_emit;
    use semio_framework_os::{ArtifactPresentation, MediaClass, MediaForm, MediaType, empty_workflow_snapshot, os_app_registration, workflow_palette};
    use semio_framework_plugin::{App, AppIo, LocalizedLabel};
    // 🌉️ `AppBuilder::build_definition` itself hard-asserts a non-empty `document` — so the
    // empty-breadcrumb case can only ever reach `register_app_io` via a wire-decoded `AppDefinition`
    // that bypassed the builder entirely. Simulate that faithfully: build a normal, valid
    // definition, then blank `document` out at the JSON level before it's pushed.
    // 🪪️ `App::builder`'s id must parse via `semio_framework::parse_surface_app_id` (ticket
    // 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1) — mirror `testkit::test_surface_id`'s
    // synthetic-dialect convention here since this test builds its `AppDefinition` by hand instead
    // of through `seed_app`.
    let root_tool_id = crate::engine::space::testkit::test_surface_id("root-tool").await;
    let definition = App::builder(root_tool_id.clone(), LocalizedLabel::data("Root Tool"))
        .await
        .document(["root-tool".to_string()])
        .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), "root-tool.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, "square-pen")
        .await
        .io(AppIo::from_document(
            "root-tool.document",
            MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ArtifactPresentation { id: "root-tool".into(), name: "Root Tool".into(), dimension: String::new(), component_kind: "root-tool".into() },
        )
        .await)
        .await
        .build_definition();
    let mut app_json = pack::json_from_dsl_value(&dsl::to_dsl_value(&definition).expect("serialize AppDefinition"));
    // 🩹️ `AppDefinition`'s wire field is `breadcrumb` (`AppBuilder::document(...)` is the builder
    // METHOD name that sets it, not the serialized field name — `#[serde(rename_all =
    // "camelCase")]` leaves the single-word `breadcrumb` unchanged). Blanking `"document"` here was
    // a no-op that silently left the real `"breadcrumb": ["root-tool"]` untouched, so this test's
    // "empty breadcrumb" simulation never actually happened — masked until now by the canonical-id
    // panic this lane fixed, which never let execution reach this far before.
    if let pack::JsonValue::Object(object) = &mut app_json {
        object.insert("breadcrumb", pack::json!([]));
    }
    let wire = pack::json!([{ "pluginId": "root", "app": app_json }]).to_string();
    let projection = empty_workflow_snapshot().await;
    let config = SpaceConfig::default();
    studio_emit(&projection, &config, &SpaceCommand::SetAppRegistrations(crate::engine::space::commands::set_app_registrations::SetAppRegistrations { json: wire })).await.expect("handle");
    assert!(os_app_registration("root", &root_tool_id).is_some(), "SetAppRegistrations must populate this wasm instance's own registry");
    assert!(workflow_palette().iter().any(|entry| entry.plugin_id == "root" && entry.app_id == root_tool_id), "workflow_palette must surface the pushed app");
    let labels = semio_framework_plugin::resolve_labels::<crate::engine::space::terminology::SStudioLabels>(&semio_framework_plugin::ViewModel::default());
    let tree = crate::engine::space::panels::catalogue::build_catalogue_tree(labels, semio_framework_plugin::Locale::En).await.expect("catalogue tree");
    let json_tree = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("catalogue projection");
    assert!(json_tree.contains(&format!("s-play-catalogue.document.{root_tool_id}")), "an empty-document app must still surface as a top-level catalogue leaf, json={json_tree}");
}
