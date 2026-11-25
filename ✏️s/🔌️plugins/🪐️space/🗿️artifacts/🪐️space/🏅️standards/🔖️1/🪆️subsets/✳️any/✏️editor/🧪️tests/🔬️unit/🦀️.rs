pub(crate) mod context {
    
    use super::super::*;
    use semio_plugin::EditorApp;
    use semio_plugin::artifact_app_laws::{meta, new_app_with_registry};

    /// 🛋️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the app's paid close.
pub struct SpaceIndexApp(pub semio_plugin::VcsArtifactApp<EditorApp<SpaceIndexEditor>>, pub semio_plugin::artifact_app_laws::OriginalFixtureScope);

impl std::ops::Deref for SpaceIndexApp {
    type Target = semio_plugin::VcsArtifactApp<EditorApp<SpaceIndexEditor>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SpaceIndexApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for SpaceIndexApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("the fixture app pays its original close");
        }
    }
}

    /// 🧩️ Adapts `create_space_index_editor`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `new_app_with_registry` expects.
    fn space_index_manifest_for_tests() -> semio_plugin::App {
        semio_plugin::App { definition: create_space_index_editor(), examples: Vec::new() }
    }

    semio_plugin::history_edit_acceptance_law!("space", SpaceIndexEditor, space_index_manifest_for_tests, "../../..");

    /// 🧪️ An app instance carrying the real `AppActionRegistry`. The registry-LESS
    /// `artifact_app_laws::new_app` is unusable here: `with_registry_on_bus` joins
    /// `EditorApp<SpaceIndexEditor>`'s `bounded_first_step_tool_proofs!` roster against the registry's
    /// `Migrated` tool ids (`AppActionRegistry::validate_tool_job_rows`), and an empty registry
    /// declares none of them — construction panics with `interactive-job.catalog-authority` …
    /// `generated_migrated=false`, `migrated={}`. A registry-less wrapper could not dispatch anything
    /// anyway (`admit_command_wire_with_proof` refuses every verb with no manifest declaration). The app is
    /// bound to live instance `1`, the instance `artifact_app_laws::meta` stamps on every dispatch — a typed
    /// command addressed to any other instance is refused as not belonging to the mounted app.
    pub async fn new_app() -> SpaceIndexApp {
        use semio_plugin::PluginApp;
        let scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = new_app_with_registry::<EditorApp<SpaceIndexEditor>>(space_index_manifest_for_tests, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
        app.bind_instance_id(1).await;
        SpaceIndexApp(app, scope)
    }
    
    pub async fn new_app_with_artifact() -> (SpaceIndexApp, String) {
        use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow, empty_space_index_snapshot};
        use semio_plugin::PluginApp;
        use pack::snapshot::ArtifactDsl;
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
        let booted = app.0.document_text().await.expect("booted document text");
        semio_plugin::artifact_app_laws::load_document_text(&mut app.0, &store::os_store::text_format::ArtifactTextFiles { dsl: snapshot.print_dsl(), ops: booted.ops }, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("load test artifact");
        (app, id)
    }
    
    pub async fn new_app_with_indexed_artifact() -> (SpaceIndexApp, String) {
        use crate::editor::space_index::commands::fold_directory_events::FoldDirectoryEvents;
        use crate::standards::v1::subsets::any::schema::snapshot::empty_space_index_snapshot;
        use semio_os_kernel::os_directory::{
            ArtifactHash, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceVisibility, DocumentDescriptor, DocumentFrontier, DocumentIndexEntryV1, DocumentOwner, DocumentScope, Hlc,
        };
        use {semio_framework_artifact_reference::ArtifactDialect,semio_plugin::PluginApp};
        use pack::snapshot::ArtifactDsl;
    
        let mut app = new_app().await;
        let id = "artifact-0123456789abcdef0123456789abcdef".to_string();
        let snapshot = empty_space_index_snapshot("space-1");
        let booted = app.0.document_text().await.expect("booted document text");
        semio_plugin::artifact_app_laws::load_document_text(&mut app.0, &store::os_store::text_format::ArtifactTextFiles { dsl: snapshot.print_dsl(), ops: booted.ops }, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("load empty Space index");
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
        dispatch(&mut app, SpaceIndexCommand::FoldDirectoryEvents(FoldDirectoryEvents { events_json: semio_framework_pack_json::to_json_string(&events) })).await.expect("fold indexed artifact");
        let files = app.config_pack().await.expect("indexed config pack");
        let config = store::os_store::text_format::parse_document_pack::<SpaceIndexConfig, SpaceIndexConfigMutation>(&files.pack, &files.spr, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("indexed config projection").into_snapshot();
        assert_eq!(config.indexed_artifacts.len(), 1);
        (app, id)
    }
    
    /// 🚦️ Dispatches one command as the interactive host does and settles its job ticket, so the receipt carries the effects and the revisions the typed operation actually published.
    pub async fn dispatch(app: &mut SpaceIndexApp, command: SpaceIndexCommand) -> Result<semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt, semio_framework_job::OriginalTurnFailure> {
        dispatch_as(app, command, "local").await
    }

    /// 📝️ Whether a settled operation published a document, draft or config row; a relay or dialog request publishes none.
    pub fn writes_the_document(receipt: &semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt) -> bool {
        use semio_plugin::app::TypedOperationResultLane;
        receipt.lanes.iter().any(|lane| matches!(lane, TypedOperationResultLane::Artifact | TypedOperationResultLane::Draft | TypedOperationResultLane::Config))
    }

    /// 🚦️ [`dispatch`] with an explicit action source.
    pub async fn dispatch_as(app: &mut SpaceIndexApp, command: SpaceIndexCommand, source: &str) -> Result<semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt, semio_framework_job::OriginalTurnFailure> {
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &meta(source), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await?;
        Ok(semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, 1, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await?)
    }
}


use super::*;
use semio_plugin::Effect;

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
        assert!(definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).any(|action| action.id == command), "registry declares {command}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_three_dialogs_are_registered_with_the_right_submit_actions() {
    let definition = create_space_index_editor();
    let own = definition.dialogs.iter().filter(|dialog| dialog.id != semio_framework::history_edit::HISTORY_EDIT_FINALIZE_DIALOG_ID).count();
    assert_eq!(own, 3);
    assert!(definition.dialogs.iter().any(|dialog| dialog.id == semio_framework::history_edit::HISTORY_EDIT_FINALIZE_DIALOG_ID), "the framework injects the history-edit finalize dialog");
    let by_id = |id: &str| definition.dialogs.iter().find(|dialog| dialog.id == id).unwrap_or_else(|| panic!("dialog {id} must be registered"));
    assert_eq!(by_id("createArtifact").submit_action, ActionRef::new("createArtifact"));
    assert_eq!(by_id("createArtifact").args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), ["name", "kindChoice"]);
    assert_eq!(by_id("deleteArtifact").submit_action, ActionRef::new("deleteArtifact"));
    assert_eq!(by_id("inviteMember").submit_action, ActionRef::new("inviteMember"));
    assert_eq!(by_id("inviteMember").args.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use semio_plugin::{ArtifactView, ConfigView, HistoryView};
    let snapshot = SSpaceSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(<SpaceIndexEditor as ArtifactEditor>::render("nope", &doc, &cfg, &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).expect("unknown body diagnostic tree")).expect("unknown body projection");
    assert!(json.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn the_members_panel_body_renders_through_the_editor_dispatch() {
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    use semio_plugin::{ArtifactView, ConfigView, HistoryView};
    let snapshot = SSpaceSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(<SpaceIndexEditor as ArtifactEditor>::render(members_panel::SPACE_INDEX_BODY_MEMBERS, &doc, &cfg, &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).expect("members panel tree")).expect("members panel projection");
    assert!(json.contains("s-space-invite"));
}

/// 🆔️ Lane 4-F: `command_from_action` was entirely missing before this — every one of this app's
/// own actions dispatched via a plain button click (`onAction`/`handleAction`) hit the default
/// trait impl's unconditional error. Covers every declared action id, mirroring the sibling
/// `command_from_action_covers_every_declared_action_and_rejects_unknown_ones` convention other
/// artifact editors already use (e.g. `🌍️gis/🗿️artifacts/🗺️gismap`).
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    let cases: Vec<(&str, semio_framework_pack_json::Value)> = vec![
        ("createArtifact", semio_framework_pack_json::json!({ "name": "First", "kindChoice": "{\"kindId\":\"s.gis.gismap\"}" })),
        ("deleteArtifact", semio_framework_pack_json::json!({ "id": "artifact-1" })),
        ("renameArtifact", semio_framework_pack_json::json!({ "id": "artifact-1", "newName": "Renamed" })),
        ("touchArtifact", semio_framework_pack_json::json!({ "id": "artifact-1", "nowMs": 2, "actor": "user:1" })),
        ("requestDeleteArtifact", semio_framework_pack_json::json!({ "id": "artifact-1" })),
        ("openArtifact", semio_framework_pack_json::json!({ "id": "artifact-1" })),
        ("openArtifactWith", semio_framework_pack_json::json!({ "id": "artifact-1", "role": "editor", "pluginId": "writer", "appId": "writer.editor" })),
        ("foldDirectoryEvents", semio_framework_pack_json::json!({ "eventsJson": "[]" })),
        ("presenceHeartbeat", semio_framework_pack_json::json!({ "artifactId": "artifact-1", "actorsCsv": "user:1" })),
        ("inviteMember", semio_framework_pack_json::json!({ "email": "a@example.com", "role": "author" })),
        ("removeMember", semio_framework_pack_json::json!({ "userId": "user:1" })),
        ("setVisibility", semio_framework_pack_json::json!({ "visibility": "public" })),
        ("copyInviteLink", semio_framework_pack_json::json!({ "role": "spectator", "ttlSecs": 604800u64 })),
        ("requestInviteMember", semio_framework_pack_json::json!({})),
    ];
    for (action, args) in cases {
        let args = semio_framework_pack_json::to_dsl_value(&args);
        let command = SpaceIndexEditor::command_from_action(action, &mut Some(args.clone()), &mut None).unwrap_or_else(|error| panic!("{action} must bridge: {error:?}"));
        assert_eq!(command.command_id(), action, "the bridged command's own id must round-trip");
    }
    assert!(SpaceIndexEditor::command_from_action("bogus", &mut None, &mut None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn create_artifact_dialog_submission_preserves_the_exact_catalog_choice_in_the_host_relay() {
    let kind_choice = "{\"kindId\":\"s.gis.gismap\",\"schema\":\"gis.map\"}";
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "name": " First map ", "kindChoice": kind_choice }));
    let SpaceIndexCommand::CreateArtifact(command) = SpaceIndexEditor::command_from_action("createArtifact", &mut Some(args.clone()), &mut None).expect("dialog submission must bridge") else {
        panic!("expected CreateArtifact");
    };
    assert_eq!(command.kind_choice, kind_choice);
    let mut app = context::new_app().await;
    let result = context::dispatch_as(&mut app, SpaceIndexCommand::CreateArtifact(command), "dialog").await.expect("dialog submission");
    assert!(app.snapshot().expect("projection").artifacts.is_empty());
    let [Effect::ReplayShellCommand { action_id, args }] = result.effects.as_slice() else {
        panic!("dialog submission must emit one ReplayShellCommand");
    };
    assert_eq!(action_id, "os.create-space-artifact");
    let args = semio_framework_pack_json::from_dsl_value(args.as_ref().expect("relay args"));
    assert_eq!(args, semio_framework_pack_json::json!({ "kindChoice": kind_choice, "name": "First map" }));
}

/// 🆔️ Lane 4-F: `#s-space-create-artifact`'s no-args click must bridge to an EMPTY `CreateArtifact`
/// payload (not error on missing fields) — its own handler treats empty `name`/`kindId` as "open
/// the dialog", mirroring Home's `createSpace`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_bridges_an_empty_create_artifact_click() {
    let SpaceIndexCommand::CreateArtifact(payload) = SpaceIndexEditor::command_from_action("createArtifact", &mut None, &mut None).expect("no-args click must bridge") else { panic!("expected CreateArtifact") };
    assert_eq!(payload.name, "");
    assert_eq!(payload.kind_choice, "");
}
