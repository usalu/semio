pub(crate) mod context {
    
    use super::super::*;
    use semio_framework_plugin::EditorApp;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app as framework_new_app};
    
    pub type SpaceIndexApp = semio_framework_plugin::VcsArtifactApp<EditorApp<SpaceIndexEditor>>;
    
    pub async fn new_app() -> SpaceIndexApp {
        framework_new_app::<EditorApp<SpaceIndexEditor>>().await
    }
    
    pub async fn new_app_with_artifact() -> (SpaceIndexApp, String) {
        use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow, empty_space_index_snapshot};
        use semio_framework_plugin::PluginApp;
        use store::ArtifactDsl;
        let mut app = new_app().await;
        let id = "artifact-1".to_string();
        let mut snapshot = empty_space_index_snapshot("space-1");
        snapshot.artifacts.push(SpaceArtifactRow {
            id: id.clone(),
            name: "First".into(),
            kind_id: "s.draw.draw".into(),
            schema: "s.draw.draw".into(),
            dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() },
            created_at_ms: 1,
            created_by: "user:1".into(),
            updated_at_ms: 1,
            updated_by: "user:1".into(),
        });
        app.load_document_text(&store::ArtifactTextFiles { dsl: snapshot.print_dsl(), ops: String::new() }).await.expect("load test artifact");
        (app, id)
    }
    
    pub async fn new_app_with_indexed_artifact() -> (SpaceIndexApp, String) {
        use crate::editor::space_index::commands::fold_directory_events::FoldDirectoryEvents;
        use crate::standards::v1::subsets::any::schema::snapshot::empty_space_index_snapshot;
        use semio_framework_os_kernel::os_directory::{
            ArtifactHash, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceVisibility, DocumentDescriptor, DocumentFrontier, DocumentIndexEntryV1, DocumentOwner, DocumentScope, Hlc,
        };
        use semio_framework_plugin::{ArtifactDialect, PluginApp};
        use store::ArtifactDsl;
    
        let mut app = new_app().await;
        let id = "artifact-0123456789abcdef0123456789abcdef".to_string();
        let snapshot = empty_space_index_snapshot("space-1");
        app.load_document_text(&store::ArtifactTextFiles { dsl: snapshot.print_dsl(), ops: String::new() }).await.expect("load empty Space index");
        let descriptor = DocumentDescriptor {
            space_id: "space-1".into(),
            document_id: id.clone(),
            artifact_kind: "s.draw.draw".into(),
            artifact_schema: "s.draw.draw".into(),
            owner: DocumentOwner { plugin_id: "draw".into(), package_id: "draw".into(), version: "1".into(), package_hash: "a".repeat(64) },
            pack_schema_hash: "b".repeat(64),
            bootstrap_version: 1,
            bootstrap_frontier: DocumentFrontier { head_seq: 1, commit_seq: 1, epoch: 1 },
            bootstrap_snapshot_hash: "c".repeat(64),
        };
        let event = |seq: u64, user_id: Option<&str>, body: DirectoryEventBody| DirectoryEvent {
            seq,
            id: format!("evt-{seq}"),
            hlc: Hlc { physical_ms: seq as i64, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
            space_id: Some("space-1".into()),
            user_id: user_id.map(Into::into),
            body,
            recorded_at_ms: seq as i64,
        };
        let events = vec![
            event(1, None, DirectoryEventBody::SpaceCreated { space_id: "space-1".into(), name: "Space 1".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private, owner_user_id: "u-1".into() }),
            event(2, None, DirectoryEventBody::DocumentAnnounced { descriptor }),
            event(
                3,
                Some("u-1"),
                DirectoryEventBody::DocumentIndexed {
                    scope: DocumentScope { space_id: "space-1".into(), document_id: id.clone() },
                    descriptor_digest_v1: ArtifactHash([7; 32]),
                    entry: DocumentIndexEntryV1 { name: "First".into(), dialect: ArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() } },
                },
            ),
        ];
        app.dispatch_typed(SpaceIndexCommand::FoldDirectoryEvents(FoldDirectoryEvents { events_json: pack::to_json_string(&events) }), &meta("local")).await.expect("fold indexed artifact");
        let files = app.config_pack().await.expect("indexed config pack");
        let config = store::parse_document_pack::<SpaceIndexConfig, SpaceIndexConfigMutation>(&files.pack, &files.spr).await.expect("indexed config projection").snapshot;
        assert_eq!(config.indexed_artifacts.len(), 1);
        (app, id)
    }
    
    #[allow(dead_code)]
    pub async fn dispatch(app: &mut SpaceIndexApp, command: SpaceIndexCommand) -> semio_framework_plugin::InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
}


use super::*;
use semio_framework_plugin::Effect;

#[semio_framework_async_macros::async_test]
async fn create_space_index_editor_builds_a_definition_for_this_dialect() {
    let definition = create_space_index_editor();
    assert_eq!(definition.dialect, SPACE_INDEX_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SpaceIndexEditor as ArtifactEditor>::DIALECT, SPACE_INDEX_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_mutation_action_is_registered() {
    let definition = create_space_index_editor();
    for command in [
        "createArtifact",
        "deleteArtifact",
        "renameArtifact",
        "touchArtifact",
        "requestDeleteArtifact",
        "openArtifact",
        "openArtifactWith",
        "inviteMember",
        "requestInviteMember",
        "removeMember",
        "setVisibility",
        "copyInviteLink",
        "foldDirectoryEvents",
        "presenceHeartbeat",
    ] {
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == command), "registry declares {command}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_three_dialogs_are_registered_with_the_right_submit_actions() {
    let definition = create_space_index_editor();
    assert_eq!(definition.dialogs.len(), 3);
    let by_id = |id: &str| definition.dialogs.iter().find(|dialog| dialog.id == id).unwrap_or_else(|| panic!("dialog {id} must be registered"));
    assert_eq!(by_id("createArtifact").submit_action, ActionRef::new("createArtifact"));
    assert_eq!(by_id("createArtifact").args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), ["name", "kindChoice"]);
    assert_eq!(by_id("deleteArtifact").submit_action, ActionRef::new("deleteArtifact"));
    assert_eq!(by_id("inviteMember").submit_action, ActionRef::new("inviteMember"));
    assert_eq!(by_id("inviteMember").args.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView};
    let snapshot = SSpaceSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(<SpaceIndexEditor as ArtifactEditor>::render("nope", &doc, &cfg, &semio_framework_plugin::ViewModel::default()).expect("unknown body diagnostic tree")).expect("unknown body projection");
    assert!(json.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn the_members_panel_body_renders_through_the_editor_dispatch() {
    use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView};
    let snapshot = SSpaceSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(<SpaceIndexEditor as ArtifactEditor>::render(members_panel::SPACE_INDEX_BODY_MEMBERS, &doc, &cfg, &semio_framework_plugin::ViewModel::default()).expect("members panel tree")).expect("members panel projection");
    assert!(json.contains("s-space-invite"));
}

/// 🆔️ Lane 4-F: `command_from_action` was entirely missing before this — every one of this app's
/// own actions dispatched via a plain button click (`onAction`/`handleAction`) hit the default
/// trait impl's unconditional error. Covers every declared action id, mirroring the sibling
/// `command_from_action_covers_every_declared_action_and_rejects_unknown_ones` convention other
/// artifact editors already use (e.g. `🌍️gis/🗿️artifacts/🗺️gismap`).
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    let cases: Vec<(&str, pack::JsonValue)> = vec![
        ("createArtifact", pack::json!({ "name": "First", "kindChoice": "{\"kindId\":\"s.gis.gismap\"}" })),
        ("deleteArtifact", pack::json!({ "id": "artifact-1" })),
        ("renameArtifact", pack::json!({ "id": "artifact-1", "newName": "Renamed" })),
        ("touchArtifact", pack::json!({ "id": "artifact-1", "nowMs": 2, "actor": "user:1" })),
        ("requestDeleteArtifact", pack::json!({ "id": "artifact-1" })),
        ("openArtifact", pack::json!({ "id": "artifact-1" })),
        ("openArtifactWith", pack::json!({ "id": "artifact-1", "role": "editor", "pluginId": "writer", "appId": "writer.editor" })),
        ("foldDirectoryEvents", pack::json!({ "eventsJson": "[]" })),
        ("presenceHeartbeat", pack::json!({ "artifactId": "artifact-1", "actorsCsv": "user:1" })),
        ("inviteMember", pack::json!({ "email": "a@example.com", "role": "author" })),
        ("removeMember", pack::json!({ "userId": "user:1" })),
        ("setVisibility", pack::json!({ "visibility": "public" })),
        ("copyInviteLink", pack::json!({ "role": "spectator", "ttlSecs": 604800u64 })),
        ("requestInviteMember", pack::json!({})),
    ];
    for (action, args) in cases {
        let args = pack::json_to_dsl_value(&args);
        let command = SpaceIndexEditor::command_from_action(action, Some(&args)).unwrap_or_else(|error| panic!("{action} must bridge: {error:?}"));
        assert_eq!(command.command_id(), action, "the bridged command's own id must round-trip");
    }
    assert!(SpaceIndexEditor::command_from_action("bogus", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn create_artifact_dialog_submission_preserves_the_exact_catalog_choice_in_the_host_relay() {
    let kind_choice = "{\"kindId\":\"s.gis.gismap\",\"schema\":\"gis.map\"}";
    let args = pack::json_to_dsl_value(&pack::json!({ "name": " First map ", "kindChoice": kind_choice }));
    let SpaceIndexCommand::CreateArtifact(command) = SpaceIndexEditor::command_from_action("createArtifact", Some(&args)).expect("dialog submission must bridge") else {
        panic!("expected CreateArtifact");
    };
    assert_eq!(command.kind_choice, kind_choice);
    let mut app = artifact_app_laws::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::CreateArtifact(command), &semio_framework_plugin::artifact_app_laws::meta("dialog")).await.expect("dialog submission");
    assert!(app.snapshot().expect("projection").artifacts.is_empty());
    let [Effect::ReplayShellCommand { action_id, args }] = result.requested_effects.as_slice() else {
        panic!("dialog submission must emit one ReplayShellCommand");
    };
    assert_eq!(action_id, "os.create-space-artifact");
    let args = pack::json_from_dsl_value(args.as_ref().expect("relay args"));
    assert_eq!(args, pack::json!({ "kindChoice": kind_choice, "name": "First map" }));
}

/// 🆔️ Lane 4-F: `#s-space-create-artifact`'s no-args click must bridge to an EMPTY `CreateArtifact`
/// payload (not error on missing fields) — its own handler treats empty `name`/`kindId` as "open
/// the dialog", mirroring Home's `createSpace`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_bridges_an_empty_create_artifact_click() {
    let SpaceIndexCommand::CreateArtifact(payload) = SpaceIndexEditor::command_from_action("createArtifact", None).expect("no-args click must bridge") else { panic!("expected CreateArtifact") };
    assert_eq!(payload.name, "");
    assert_eq!(payload.kind_choice, "");
}
