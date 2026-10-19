
use semio_hub_space_commands_media::set_active_panel_tab::SetActivePanelTab;
use semio_hub_space_foundation::config::SpaceConfig;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use semio_hub_space_app::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SetActivePanelTab(SetActivePanelTab { tab_id: "s-play-catalogue".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::GoHome(semio_hub_space_commands_media::go_home::GoHome {}));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NavigateVirtualFileSystemNode(semio_hub_space_commands_media::navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { space_id: "demo".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SetAppRegistrations(semio_hub_space_commands_media::set_app_registrations::SetAppRegistrations { json: "[]".into() }));
}

/// 🪐️ End-to-end proof of the catalogue-empty bugfix: registers an app with an EMPTY `document`
/// breadcrumb purely through this command, then asserts the registry, `workflow_palette()`, and
/// `build_catalogue_tree` all pick it up.
#[semio_framework_async_macros::async_test]
async fn set_app_registrations_command_registers_app_and_surfaces_empty_document_apps_in_catalogue() {
    use semio_hub_space_app::SpaceCommand;
    use semio_hub_space_app::unit_tests::context::studio_emit;
    use semio_framework_os::{ArtifactPresentation, MediaClass, MediaForm, MediaType, empty_workflow_snapshot, os_app_registration, workflow_palette};
    use semio_framework_ui_locale::{LocalizedLabel};
use semio_framework_plugin::{App, AppIo};
    let root_tool_id = semio_hub_space_app::unit_tests::context::test_surface_id("root-tool").await;
    let definition = App::builder(root_tool_id.clone(), LocalizedLabel::data("Root Tool"))
        .await
        .document(["root-tool".to_string()])
        .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), "root-tool.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, "square-pen")
        .await
        .io(AppIo::from_artifact(
            "root-tool.document",
            MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ArtifactPresentation { id: "root-tool".into(), name: "Root Tool".into(), dimension: String::new(), component_kind: "root-tool".into() },
        )
        .await)
        .await
        .build_definition();
    let mut app_json = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&definition));
    if let semio_framework_pack_json::Value::Object(object) = &mut app_json {
        object.insert("breadcrumb", semio_framework_pack_json::json!([]));
    }
    let wire = semio_framework_pack_json::json!([{ "pluginId": "root", "app": app_json }]).to_string();
    let projection = empty_workflow_snapshot().await;
    let config = SpaceConfig::default();
    studio_emit(&projection, &config, &SpaceCommand::SetAppRegistrations(semio_hub_space_commands_media::set_app_registrations::SetAppRegistrations { json: wire })).await.expect("handle");
    assert!(os_app_registration("root", &root_tool_id).is_some(), "SetAppRegistrations must populate this wasm instance's own registry");
    assert!(workflow_palette().iter().any(|entry| entry.plugin_id == "root" && entry.app_id == root_tool_id), "workflow_palette must surface the pushed app");
    let labels = semio_framework_plugin::resolve_labels::<semio_hub_space_foundation::terminology::SStudioLabels>(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let tree = semio_hub_space_app::panels::catalogue::build_catalogue_tree(labels, semio_framework_ui_locale::Locale::En, &semio_framework_plugin::TreeWindows::unhosted()).await.expect("catalogue tree");
    let json_tree = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("catalogue projection");
    assert!(json_tree.contains(&format!("s-play-catalogue.document.{root_tool_id}")), "an empty-document app must still surface as a top-level catalogue leaf, json={json_tree}");
}
