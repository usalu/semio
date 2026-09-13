
use super::*;
use semio_framework_os::list_os_space_catalog_entries;
use semio_framework_plugin::{AppActionRegistry, EditorApp, HistoryView, PluginApp, VcsArtifactApp, artifact_app_laws};

#[semio_framework_async_macros::async_test]
async fn home_command_op_text_round_trips_every_variant() {
    use crate::editor::home::HomeCommand;
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(CreateStudio { name: "Untitled".into(), kind: "catalog".into(), folder_path: None }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(CreateStudio { name: "Untitled".into(), kind: "folder".into(), folder_path: Some("/tmp/x".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::BindSpaceFile(crate::editor::home::commands::bind_space_file::BindSpaceFile { space_id: "s1".into(), file_path: "/tmp/x.os".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(crate::editor::home::commands::import_space::ImportSpace { dsl: Some("programs=[]".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(crate::editor::home::commands::import_space::ImportSpace { dsl: None }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::OpenSpace(crate::editor::home::commands::open_space::OpenSpace { space_id: "s1".into() }));
}

#[semio_framework_async_macros::async_test]
async fn registered_home_rejects_create_studio_until_its_retained_owner_exists() {
    let definition = crate::editor::home::create_home_app().await;
    let registry = AppActionRegistry::from_definition(&definition);
    let mut home: VcsArtifactApp<EditorApp<crate::editor::home::HomeApp>> = VcsArtifactApp::with_registry(EditorApp::<crate::editor::home::HomeApp>::default(), registry).await;
    let fault = home
        .dispatch_typed(crate::editor::home::HomeCommand::CreateStudio(CreateStudio { name: "Test Studio".into(), kind: "catalog".into(), folder_path: None }), &artifact_app_laws::meta("local"))
        .await
        .expect_err("batch-only createStudio must fail closed at interactive dispatch");
    assert_eq!(fault.code.as_str(), "interactive-job.not-ui-safe");
    artifact_app_laws::close_registered_fixture_app(&mut home);
}

#[semio_framework_async_macros::async_test]
async fn temporary_studio_uses_ephemeral_registry_not_catalog() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    assert_eq!(handle(&CreateStudio { name: "Missing Identity".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg).unwrap_err().code.as_str(), "s.home.session-identity-required");
    let identity = semio_framework_plugin::ViewSessionIdentity { user_id: "u1".into(), display_name: "Ada".into() };
    let emit = handle_with_identity(&CreateStudio { name: "Temp Studio".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg, &identity).expect("handle");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::Navigate { .. })));
    assert!(!emit.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })), "ephemeral create must not download");
    let persistent = list_os_space_catalog_entries(&crate::catalog_port().await).expect("list");
    assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
    let ephemeral_catalog = list_os_space_catalog_entries(&crate::temp_catalog_port().await).unwrap_or_default();
    assert!(!ephemeral_catalog.iter().any(|entry| entry.name == "Temp Studio"));
    let uri = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::Navigate { uri } => Some(uri.as_str()),
            _ => None,
        })
        .expect("navigate");
    let space_id = uri.trim_start_matches("/spaces/");
    let document = crate::resolve_studio_document(space_id).await.expect("ephemeral studio");
    assert_eq!(document.name, "Temp Studio");
    assert!(document.backbone.is_none());
    assert!(document.vcs.initial_snapshot.collections.is_empty());
}
