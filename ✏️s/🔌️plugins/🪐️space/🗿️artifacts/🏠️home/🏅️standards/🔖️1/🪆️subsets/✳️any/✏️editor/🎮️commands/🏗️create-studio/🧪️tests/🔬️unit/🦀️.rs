
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

/// 🚦️ Both routes the shell actually needs from Home are past the classification gate. Ticket
/// 26/09/18 S4 measured the opposite live: `validate_ui_dispatch_classification` admits ONLY
/// `Migrated`, so a `BatchOnlyPendingRewrite` route answers `interactive-job.not-ui-safe` and is
/// hard-dead in every shell — `createStudio` was the local (hub-free) studio path and
/// `applyDirectoryEventPage` was the directory bootstrap's acknowledgement, so the signed-in `s`
/// host looped the hub's event page about once a second for ever. This law pins the gate, not the
/// execution: a registry fixture owns no live instance, so the remaining refusal is
/// `interactive-job.live-instance` — a DIFFERENT code, from a later stage, which is exactly the
/// evidence that the classification no longer refuses either route.
#[semio_framework_async_macros::async_test]
async fn registered_home_admits_its_retained_routes_at_interactive_dispatch() {
    let definition = crate::editor::home::create_home_app().await;
    let registry = AppActionRegistry::from_definition(&definition);
    let mut home: VcsArtifactApp<EditorApp<crate::editor::home::HomeApp>> = VcsArtifactApp::with_registry(EditorApp::<crate::editor::home::HomeApp>::default(), registry).await;
    let routes = [
        crate::editor::home::HomeCommand::CreateStudio(CreateStudio { name: "Test Studio".into(), kind: "catalog".into(), folder_path: None }),
        crate::editor::home::HomeCommand::ApplyDirectoryEventPage(crate::editor::home::commands::apply_directory_event_page::ApplyDirectoryEventPage { page_json: "{}".into() }),
    ];
    for command in routes {
        let id = command.command_id();
        let fault = home.dispatch_typed(command, &artifact_app_laws::meta("local")).await.expect_err("a registry fixture owns no live instance");
        assert_ne!(fault.code.0.as_str(), "interactive-job.not-ui-safe", "{id} must be past the classification gate");
        assert_eq!(fault.code.0.as_str(), "interactive-job.live-instance", "{id} must fail only for the fixture's missing live instance");
    }
    // ♻️ NOT closed through `close_registered_fixture_app`: Home's PRESENCE lane still has no
    // retirement factory, so the close ladder faults at that stage — the same gap the running shell
    // reports as `predecessor space/s.space.home@1/*#editor retirement failed` on every app switch
    // (measured by S4, ticket 26/09/18 §3). Home's document, config, draft and transient lanes are
    // declared above; the presence lane is a bespoke domain retirement (see `🌊️flow`'s) and is the
    // next owner's piece.
    std::mem::forget(home);
}

#[semio_framework_async_macros::async_test]
async fn temporary_studio_uses_ephemeral_registry_not_catalog() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    // 🚫️ `Emit` carries no `Debug`, so the rejection is destructured rather than `unwrap_err`ed.
    let Err(fault) = handle(&CreateStudio { name: "Missing Identity".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg) else {
        panic!("a temporary studio minted without a view session identity must be rejected");
    };
    // 🧯️ `Fault::from(&str)` mints `app.message` and carries the text as the MESSAGE — asserting the
    // code here read `app.message` and had never passed.
    assert_eq!((fault.code.0.as_str(), fault.message.as_str()), ("app.message", "s.home.session-identity-required"));
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
