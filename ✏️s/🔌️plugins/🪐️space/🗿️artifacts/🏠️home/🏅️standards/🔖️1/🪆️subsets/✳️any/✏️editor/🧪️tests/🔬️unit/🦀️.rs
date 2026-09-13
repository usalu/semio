
use super::*;

//#region 🧪️RetainedCommandEnvelope
#[test]
fn retained_command_fixture_matches_exact_routes_and_serde_json_boundaries() {
    let fixture: pack::JsonValue = pack::parse_json(include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json")).expect("language-neutral retained fixture");
    let migrated: Vec<&str> = fixture["routes"].as_array().expect("routes").iter().filter(|row| row["disposition"] == "Migrated").map(|row| row["id"].as_str().expect("route id")).collect();
    assert_eq!(migrated, HOME_RETAINED_TOOL_IDS);
    assert_eq!(HOME_RETAINED_PUBLICATION_CONTRACTS.len(), migrated.len());
    assert_eq!(fixture["controller"].as_str(), Some(S_HOME_CONTROLLER_ID));
    assert_eq!(fixture["limits"]["scalarBytes"].as_u64(), Some(HOME_RETAINED_SCALAR_BYTES as u64));
    assert_eq!(fixture["limits"]["storeStepBytes"].as_u64(), Some(HOME_CONFIG_STEP_BYTES as u64));
    for case in fixture["boundaryCases"].as_array().expect("boundary cases") {
        let value = "x".repeat(case["bytes"].as_u64().expect("byte count") as usize);
        let command = HomeCommand::OpenSpace(open_space::OpenSpace { space_id: value });
        let first_party = pack::json_from_dsl_value(&dsl::ToValue::to_value(&command));
        let oracle: serde_json::Value = serde_json::from_str(&pack::json_to_string(&first_party)).expect("third-party JSON decode");
        let oracle_wire = serde_json::to_string(&oracle).expect("third-party JSON encode");
        assert_eq!(pack::parse_json(&oracle_wire).expect("first-party JSON decode"), first_party);
        let decoded: HomeCommand = dsl::from_dsl_value(pack::json_to_dsl_value(&first_party)).expect("command value decode");
        assert_eq!(decoded, command);
        assert_eq!(home_retained_extent(&decoded, &SHomeSnapshot::default(), &protocol::InteractionState::default()).is_some(), case["accepted"].as_bool().expect("admission oracle"));
    }
}

#[test]
fn every_migrated_home_route_has_an_exact_scalar_boundary() {
    let scalar = |bytes: usize| "x".repeat(bytes);
    let pairs = [
        (HomeCommand::OpenSpace(open_space::OpenSpace { space_id: scalar(4096) }), HomeCommand::OpenSpace(open_space::OpenSpace { space_id: scalar(4097) })),
        (HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { node_id: scalar(4096) }), HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { node_id: scalar(4097) })),
        (HomeCommand::CreateSpace(create_space::CreateSpace { name: scalar(4094), kind: "k".into(), visibility: "v".into() }), HomeCommand::CreateSpace(create_space::CreateSpace { name: scalar(4095), kind: "k".into(), visibility: "v".into() })),
        (HomeCommand::DeleteSpace(delete_space::DeleteSpace { space_id: scalar(4096), confirmed: true }), HomeCommand::DeleteSpace(delete_space::DeleteSpace { space_id: scalar(4097), confirmed: true })),
        (HomeCommand::ShareSpace(share_space::ShareSpace { space_id: scalar(4094), email: "e".into(), role: "r".into() }), HomeCommand::ShareSpace(share_space::ShareSpace { space_id: scalar(4095), email: "e".into(), role: "r".into() })),
        (HomeCommand::ManageSpace(manage_space::ManageSpace { space_id: scalar(4096) }), HomeCommand::ManageSpace(manage_space::ManageSpace { space_id: scalar(4097) })),
        (HomeCommand::CopyInviteLink(copy_invite_link::CopyInviteLink { space_id: scalar(4095), role: "r".into(), ttl_secs: u64::MAX }), HomeCommand::CopyInviteLink(copy_invite_link::CopyInviteLink { space_id: scalar(4096), role: "r".into(), ttl_secs: u64::MAX })),
    ];
    let snapshot = SHomeSnapshot::default();
    let interaction = protocol::InteractionState::default();
    for (at_limit, overflow) in pairs {
        assert_eq!(home_retained_extent(&at_limit, &snapshot, &interaction), Some(1), "{} must admit the exact scalar limit", at_limit.command_id());
        assert_eq!(home_retained_extent(&overflow, &snapshot, &interaction), None, "{} must reject one byte beyond the scalar limit", overflow.command_id());
    }
    for zero in [HomeCommand::GoHome(go_home::GoHome {}), HomeCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat {})] {
        assert_eq!(home_retained_extent(&zero, &snapshot, &interaction), Some(1), "{} carries no scalar payload", zero.command_id());
    }
}

#[test]
fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
    use std::io::Write as _;
    use store::ArtifactStoreOneItemPreparation as _;
    let config = HomeConfig::default();
    let mut preparation = HomeConfigPreparation {
        base: None,
        mutation: Some(HomeConfigMutation::ReplaceDirectoryProjection {
            directory_json: config.directory_json,
            session_binding_sha256: "a".repeat(64),
            authorization_generation: 1,
            receipt_sha256: "b".repeat(64),
        }),
        description: None,
        authority: None,
        candidate: None,
        sealed_candidate: None,
        serialized_bytes: None,
        prepared: None,
        checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
        cancelled: false,
        closing: false,
    };
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: HOME_CONFIG_STEP_BYTES };
    preparation.cancel();
    assert!(matches!(preparation.advance(grant).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
    preparation.begin_close();
    assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).expect("undersized close"), store::SnapshotRetirementStep::Blocked));
    assert!(matches!(preparation.close_step(grant).expect("bounded close"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes } if released_bytes == HOME_CONFIG_STEP_BYTES));
    assert!(matches!(preparation.close_step(grant).expect("terminal close"), store::SnapshotRetirementStep::Complete));
    assert!(preparation.terminal_is_empty());
    let mut counter = HomeConfigByteCounter { bytes: 0 };
    let maximum = vec![0; HOME_CONFIG_STEP_BYTES];
    assert_eq!(counter.write(&maximum).expect("maximum serialized envelope"), HOME_CONFIG_STEP_BYTES);
    assert!(counter.write(&[0]).is_err());
}
//#endregion 🧪️RetainedCommandEnvelope

use semio_framework_artifact_space_space::{S_SPACE_SCHEMA, SpaceKind, SpaceVisibility, empty_space_snapshot};
use semio_framework_os::{LocalStorageBackbonePort, OsBackbonePorts, OsSpaceDocument, create_backbone_document, load_os_space_document, seed_os_space_catalog_if_empty};
use std::sync::Arc;

fn empty_history() -> semio_framework_plugin::HistoryView {
    semio_framework_plugin::HistoryView::empty()
}

fn home_view(user_id: &str, locale: semio_framework_plugin::Locale) -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        locale,
        session_identity: Some(semio_framework_plugin::ViewSessionIdentity { user_id: user_id.into(), display_name: "Ada".into() }),
        ..Default::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn home_manifest_derives_the_canonical_surface_id() {
    let definition = create_home_app().await;
    assert_eq!(definition.id, semio_framework::surface_app_id(&HomeApp::DIALECT.into(), semio_framework::AppRole::Editor));
    assert_eq!(definition.controller_id, "s.space.home@1/*#editor");
}

#[semio_framework_async_macros::async_test]
async fn home_declares_create_space_action() {
    let definition = create_home_app().await;
    let main = definition.window_kinds.iter().find(|window| window.id == crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW).expect("home main window");
    assert!(main.actions.iter().any(|action| action.id == "createStudio"));
}

#[semio_framework_async_macros::async_test]
async fn space_document_persists_through_backbone_port() {
    // 🕳️ `parse_demo_space_document()` yields a `workflow::WorkflowSnapshot` (the demo fixture's own
    // artifact content), not a `space::SpaceSnapshot`-backed catalog entry
    // `seed_os_space_catalog_if_empty` expects. This test exercises the space-manifest persistence
    // path specifically, so it mints its own manifest instead.
    let port = Arc::new(OsBackbonePorts::Store(store::BackbonePorts::LocalStorage(LocalStorageBackbonePort::default())));
    let projection = empty_space_snapshot("Persist Test", SpaceKind::Atelier, SpaceVisibility::Private);
    let demo: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, "persist-test", "Persist Test", projection);
    let _ = seed_os_space_catalog_if_empty(demo, &port).expect("seed");
    let loaded = load_os_space_document("persist-test", &port).expect("load");
    assert_eq!(loaded.id, "persist-test");
    assert_eq!(loaded.name, "Persist Test");
}

/// 🧪️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the pre-ticket version of
/// these two tests asserted on the VFS scene's ALWAYS-present `emptyMessage` field, which happened
/// to make them incidentally immune to `crate::list_all_space_catalog_entries()`'s process-global
/// catalog singleton being polluted by other tests in this same test binary. The new table render
/// has no such structural field (`TableView` carries no message), so these are rewritten to fold a
/// KNOWN directory event (deterministic, independent of the global catalog) and assert on the
/// locale-correct COLUMN HEADERS instead — the real thing "labels resolve to the right locale" means
/// for a table.
async fn config_with_one_folded_space() -> HomeConfig {
    let event_json = pack::json!({
        "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
        "body": {"kind": "space.created", "spaceId": "sp-1", "name": "Fixture", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
        "recordedAtMs": 1000
    })
    .to_string();
    let base = HomeConfig::default();
    protocol::Mutation::diff(&HomeConfigMutation::FoldDirectoryEvent { event_json }, &base).diff().clone()
}

#[semio_framework_async_macros::async_test]
async fn home_labels_resolve_native_english_by_default() {
    let history = empty_history();
    let home_doc = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let home_view = ArtifactView::new(&home_doc, &history);
    let config = config_with_one_folded_space().await;
    let cfg = ConfigView { snapshot: &config, window: None };
    let view_state = home_view("u1", semio_framework_plugin::Locale::En);
    let home_node = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg, &view_state).expect("English Home assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(home_node).expect("English Home tree projection");
    assert!(json.contains("Updated"), "English column header must resolve: {json}");
    assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
}

#[semio_framework_async_macros::async_test]
async fn home_labels_resolve_native_german_locale() {
    let history = empty_history();
    let home_doc = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let home_view = ArtifactView::new(&home_doc, &history);
    let config = config_with_one_folded_space().await;
    let cfg = ConfigView { snapshot: &config, window: None };
    let view_state = home_view("u1", semio_framework_plugin::Locale::De);
    let home_node = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg, &view_state).expect("German Home assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(home_node).expect("German Home tree projection");
    assert!(json.contains("Aktualisiert"), "German column header must resolve: {json}");
    assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
}

#[semio_framework_async_macros::async_test]
async fn home_render_requires_current_host_identity_and_changes_roles_with_it() {
    let history = empty_history();
    let home_doc = SHomeSnapshot::default();
    let home = ArtifactView::new(&home_doc, &history);
    let config = config_with_one_folded_space().await;
    let cfg = ConfigView { snapshot: &config, window: None };
    let missing = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home, &cfg, &semio_framework_plugin::ViewModel::default()).unwrap_err();
    assert_eq!(missing.code, "s.home.session-identity-required");
    let author = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home, &cfg, &home_view("u1", semio_framework_plugin::Locale::En)).expect("author render");
    let author_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(author).expect("author projection");
    let foreign = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home, &cfg, &home_view("u2", semio_framework_plugin::Locale::En)).expect("foreign render");
    let foreign_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(foreign).expect("foreign projection");
    assert!(author_json.contains("manageSpace"));
    assert!(!foreign_json.contains("manageSpace"));
}
