//! 🧪️ Aggregated unit tests of the `commands` leaves, one module per leaf.

mod apply_directory_event_page {
use crate::SHomeSnapshot;
use crate::editor::home::config::HomeConfig;
use crate::{editor::home::transient::DirectoryProjectionReceiptV1, editor::home::transient::HomeTransient};
use semio_framework_plugin::{ArtifactView, ConfigView, Fault};
use super::super::apply_directory_event_page::*;
use protocol::Mutation as _;

fn seal(mut page: store::os_directory::DirectoryEventPageV1) -> store::os_directory::DirectoryEventPageV1 {
    page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    page
}

/// 📬️ The page route's answer against `transient`, with its one item (if any) folded the way the transient lane folds it.
fn answer(transient: &HomeTransient, page: &store::os_directory::DirectoryEventPageV1) -> Result<(DirectoryProjectionReceiptV1, Option<HomeTransient>), Fault> {
    let answer = directory_page_answer(&semio_framework_pack_json::to_json_string(page), transient.directory())?;
    let next = answer.item.map(|item| protocol::apply_diff_cold(item.diff(transient).diff(), transient).expect("the page diff applies"));
    Ok((answer.receipt, next))
}

#[semio_framework_async_macros::async_test]
async fn sealed_page_replaces_projection_once_and_rejects_races() {
    let binding = "a".repeat(64);
    let first = seal(store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: binding.clone(),
        authorization_generation: 7,
        after_seq_exclusive: 0,
        through_seq_inclusive: 5,
        has_more: true,
        events: Vec::new(),
        receipt_sha256: String::new(),
    });
    let initial = HomeTransient::default();
    let (receipt, next) = answer(&initial, &first).expect("first sealed page");
    let current = next.expect("the first page is one transient item");
    assert_eq!(current.directory().cursor(), 5, "invisible raw holes advance the resume frontier");
    assert_eq!(receipt, current.directory().receipt().expect("current receipt"));
    let (duplicate_receipt, duplicate) = answer(&current, &first).expect("idempotent replay");
    assert!(duplicate.is_none(), "an already-held frontier folds nothing");
    assert_eq!(duplicate_receipt, current.directory().receipt().expect("current receipt"), "an already-held frontier returns the same terminal receipt");

    let raced = seal(store::os_directory::DirectoryEventPageV1 { after_seq_exclusive: 3, through_seq_inclusive: 6, receipt_sha256: String::new(), ..first.clone() });
    assert!(answer(&current, &raced).is_err(), "a same-authority page that does not continue the held frontier cannot replace the projection");

    let event = store::os_directory::DirectoryEvent {
        seq: 7,
        id: "event-7".into(),
        hlc: store::os_directory::Hlc { physical_ms: 7, logical: 0 },
        actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "user:u1#s1".into() },
        space_id: Some("space-1".into()),
        user_id: None,
        body: store::os_directory::DirectoryEventBody::SpaceCreated {
            space_id: "space-1".into(),
            name: "Werkstatt".into(),
            space_kind: store::os_directory::DirectorySpaceKind::Studio,
            visibility: store::os_directory::DirectorySpaceVisibility::Private,
            owner_user_id: "u1".into(),
        },
        recorded_at_ms: 7,
    };
    let second = seal(store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: binding,
        authorization_generation: 7,
        after_seq_exclusive: 5,
        through_seq_inclusive: 7,
        has_more: false,
        events: vec![event],
        receipt_sha256: String::new(),
    });
    let (receipt, advanced) = answer(&current, &second).expect("ordered successor page");
    let advanced = advanced.expect("an ordered successor is one transient item");
    assert_eq!(receipt.schema, DirectoryProjectionReceiptV1::SCHEMA);
    assert_eq!(receipt.through_seq_inclusive, 7);
    assert_eq!(receipt.receipt_sha256, second.receipt_sha256);
    assert_eq!(advanced.directory().cursor(), 7);
    assert!(advanced.directory().space("space-1").is_some());
    assert_eq!(current.directory().space_count(), 0, "the captured root the successor was folded from never changes");

    let replacement = seal(store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: "b".repeat(64),
        authorization_generation: 8,
        after_seq_exclusive: 0,
        through_seq_inclusive: 0,
        has_more: false,
        events: Vec::new(),
        receipt_sha256: String::new(),
    });
    let replaced = answer(&advanced, &replacement).expect("new authority rebootstrap").1.expect("a new authority rebuilds");
    assert_eq!(replaced.directory().cursor(), 0);
    assert_eq!(replaced.directory().space_count(), 0);

    let origin_replay = seal(store::os_directory::DirectoryEventPageV1 { after_seq_exclusive: 0, through_seq_inclusive: 4, has_more: true, events: Vec::new(), receipt_sha256: String::new(), ..second.clone() });
    let rebuilt = answer(&advanced, &origin_replay).expect("a same-authority replay from the origin rebuilds the projection").1.expect("an origin replay is one item");
    assert_eq!(rebuilt.directory().cursor(), 4, "the rebuild restarts at the replayed page's frontier");
    assert_eq!(rebuilt.directory().space_count(), 0, "nothing folded before the origin replay survives it");

    let mut forged = second;
    forged.receipt_sha256 = "c".repeat(64);
    assert!(answer(&current, &forged).is_err(), "forged receipt is terminally rejected");
}

/// 🚫️ The direct lane never lands a page: no transient item, no receipt, a named refusal.
#[test]
fn the_direct_lane_refuses_by_name() {
    let history = semio_framework_plugin::HistoryView::empty();
    let document = SHomeSnapshot::default();
    let view = ArtifactView::new(&document, &history);
    let config = HomeConfig::default();
    let refused = handle(&ApplyDirectoryEventPage { page_json: String::new() }, &view, &ConfigView { snapshot: &config, window: None });
    assert!(matches!(refused, Err(fault) if fault.code.0.as_str() == "s.home.directory-event-page.requires-retained-job"));
}
}

mod create_studio {
use crate::SHomeSnapshot;
use crate::editor::home::config::HomeConfig;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect};
use super::super::create_studio::*;
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
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut home: VcsArtifactApp<EditorApp<crate::editor::home::HomeApp>> = VcsArtifactApp::with_registry(EditorApp::<crate::editor::home::HomeApp>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    let routes = [
        crate::editor::home::HomeCommand::CreateStudio(CreateStudio { name: "Test Studio".into(), kind: "catalog".into(), folder_path: None }),
        crate::editor::home::HomeCommand::ApplyDirectoryEventPage(crate::editor::home::commands::apply_directory_event_page::ApplyDirectoryEventPage { page_json: "{}".into() }),
    ];
    for command in routes {
        let id = command.command_id();
        let fault = home.dispatch_typed(command, &artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.expect_err("a registry fixture owns no live instance");
        assert_ne!(fault.code.0.as_str(), "interactive-job.not-ui-safe", "{id} must be past the classification gate");
        assert_eq!(fault.code.0.as_str(), "interactive-job.live-instance", "{id} must fail only for the fixture's missing live instance");
    }
    std::mem::forget(home);
}

#[semio_framework_async_macros::async_test]
async fn temporary_studio_uses_ephemeral_registry_not_catalog() {
    let mut authoring_observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
    let mut authoring = store::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(44, &mut authoring_observer).expect("finite studio fixture admission");
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let Err(fault) = handle(&CreateStudio { name: "Missing Identity".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg) else {
        panic!("a temporary studio minted without a view session identity must be rejected");
    };
    assert_eq!((fault.code.0.as_str(), fault.message.as_str()), ("app.message", "s.home.create-studio.effect-authority-required"));
    let identity = semio_framework_plugin::ViewSessionIdentity { user_id: "u1".into(), display_name: "Ada".into() };
    let emit = handle_with_identity(&CreateStudio { name: "Temp Studio".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg, &identity, &mut authoring).expect("handle");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::Navigate { .. })));
    assert!(!emit.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })), "ephemeral create must not download");
    let persistent = list_os_space_catalog_entries(&semio_s_space_core::catalog_port().await).expect("list");
    assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
    let ephemeral_catalog = list_os_space_catalog_entries(&semio_s_space_core::temp_catalog_port().await).unwrap_or_default();
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
    let document = semio_s_space_core::resolve_studio_document(space_id).await.expect("ephemeral studio");
    assert_eq!(document.name, "Temp Studio");
    assert!(document.backbone.is_none());
    assert!(document.vcs.genesis.facts().snapshot().collections.is_empty());
}
}

mod promote_to_hub_space {
use crate::SHomeSnapshot;
use crate::editor::home::config::HomeConfig;
use super::super::promote_to_hub_space::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

#[test]
fn promote_relays_create_space_with_source_id() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PromoteToHubSpace { space_id: "draft-1".into(), name: "Promoted".into() }, &doc, &cfg).expect("handle");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { action_id, .. } if action_id == "os.directory.create-space")));
}
}

mod persist_locally {
use crate::editor::home::config::HomeConfig;
use crate::SHomeSnapshot;
use super::super::persist_locally::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

#[test]
fn empty_folder_opens_persist_dialog() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    for folder_path in [None, Some("  ".to_owned())] {
        let emit = handle(&PersistLocally { space_id: "draft-1".into(), folder_path }, &doc, &cfg).expect("handle");
        assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "persistLocally")));
        assert!(emit.artifact_mutations.is_empty());
    }
}

/// 🚫️ The direct lane never writes: a folder-bound persist runs only as the retained job, and the job's validate stage
/// refuses by name before anything is written.
#[test]
fn a_folder_persist_runs_only_as_the_retained_job_and_validates_by_name() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let direct = handle(&PersistLocally { space_id: "draft-1".into(), folder_path: Some("/tmp/sh2-persist".into()) }, &doc, &cfg);
    assert!(matches!(direct, Err(fault) if fault.code.0.as_str() == "s.home.persist-locally.requires-retained-job"));
    for (space_id, folder_path, code) in [
        ("", "/tmp/sh2-persist", "s.home.persist-locally.studio-invalid"),
        ("draft-1", " ", "s.home.persist-locally.path-invalid"),
        ("sh2-no-such-draft", "/tmp/sh2-persist", "s.home.persist-locally.unknown-studio"),
    ] {
        assert!(matches!(validate(space_id, folder_path), Err(fault) if fault.code.0.as_str() == code), "{code}");
    }
}
}

mod navigate_virtual_file_system_node {
use super::super::navigate_virtual_file_system_node::*;

#[semio_framework_async_macros::async_test]
async fn home_command_op_text_round_trips_every_variant() {
    use crate::editor::home::HomeCommand;
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::NavigateVirtualFileSystemNode(NavigateVirtualFileSystemNode { node_id: "studio:s1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::DeleteVirtualFileSystemNode(crate::editor::home::commands::delete_virtual_file_system_node::DeleteVirtualFileSystemNode { node_id: "studio:s1".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::GoHome(crate::editor::home::commands::go_home::GoHome {}));
}
}

mod copy_invite_link {
use crate::SHomeSnapshot;
use crate::editor::home::config::HomeConfig;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect};
use super::super::copy_invite_link::*;

#[semio_framework_async_macros::async_test]
async fn copy_invite_link_relays_share_link_with_default_ttl() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&CopyInviteLink { space_id: "sp-1".into(), role: "spectator".into(), ttl_secs: 0 }, &doc, &cfg).expect("handle");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.share-link");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["ttlSecs"], 3600);
    assert_eq!(args_value["spaceId"], "sp-1");
}

#[semio_framework_async_macros::async_test]
async fn explicit_ttl_is_respected() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&CopyInviteLink { space_id: "sp-1".into(), role: "author".into(), ttl_secs: 60 }, &doc, &cfg).expect("handle");
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args);
    assert_eq!(args_value["ttlSecs"], 60);
    assert_eq!(args_value["role"], "author");
}
}

mod create_space {
use crate::SHomeSnapshot;
use crate::editor::home::config::HomeConfig;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect};
use super::super::create_space::*;

async fn view<'a>(history: &'a semio_framework_plugin::HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

#[semio_framework_async_macros::async_test]
async fn empty_name_opens_the_dialog_instead_of_relaying() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&CreateSpace { name: String::new(), kind: "atelier".into(), visibility: "private".into() }, &doc, &cfg).expect("handle");
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, args: None, .. }] if dialog_id == "createSpace"), "empty name must open the dialog, not relay: {:?}", emit.effects);
}

#[semio_framework_async_macros::async_test]
async fn valid_name_emits_the_replay_shell_command_with_the_right_action_id_and_args() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&CreateSpace { name: "Atelier".into(), kind: "atelier".into(), visibility: "private".into() }, &doc, &cfg).expect("handle");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.create-space");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("args present"));
    assert_eq!(args_value["name"], "Atelier");
    assert_eq!(args_value["spaceKind"], "atelier");
    assert_eq!(args_value["visibility"], "private");
}

#[semio_framework_async_macros::async_test]
async fn blank_kind_and_visibility_default_to_atelier_and_private() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&CreateSpace { name: "Studio".into(), kind: String::new(), visibility: String::new() }, &doc, &cfg).expect("handle");
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args present");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args);
    assert_eq!(args_value["spaceKind"], "atelier");
    assert_eq!(args_value["visibility"], "private");
}
}

mod delete_space {
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use crate::{editor::home::config::HomeConfig, editor::home::config::HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit};
use super::super::delete_space::*;

async fn dispatch(payload: DeleteSpace) -> Emit<SHomeMutation, HomeConfigMutation> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    handle(&payload, &doc, &cfg).expect("handle")
}

#[semio_framework_async_macros::async_test]
async fn unconfirmed_delete_emits_the_confirm_dialog_and_never_the_command() {
    let emit = dispatch(DeleteSpace { space_id: "sp-1".into(), confirmed: false }).await;
    assert_eq!(emit.effects.len(), 1);
    let (dialog_id, args) = match &emit.effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => (dialog_id.clone(), args.clone()),
        other => panic!("expected OpenDialog, got {other:?}"),
    };
    assert_eq!(dialog_id, "deleteSpace");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("pre-seeded args"));
    assert_eq!(args_value["spaceId"], "sp-1");
    assert_eq!(args_value["confirmed"], true);
    assert!(!emit.effects.iter().any(|e| matches!(e, Effect::ReplayShellCommand { .. })), "the confirm dialog must be emitted BEFORE any command");
}

#[semio_framework_async_macros::async_test]
async fn confirmed_delete_emits_the_replay_shell_command() {
    let emit = dispatch(DeleteSpace { space_id: "sp-1".into(), confirmed: true }).await;
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.delete-space");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["spaceId"], "sp-1");
    assert!(!emit.effects.iter().any(|e| matches!(e, Effect::OpenDialog { .. })), "a confirmed dispatch never re-opens the dialog");
}
}

mod presence_heartbeat {
use crate::SHomeSnapshot;
use crate::editor::home::config::HomeConfig;
use semio_framework_plugin::{ArtifactView, ConfigView};
use super::super::presence_heartbeat::*;

#[semio_framework_async_macros::async_test]
async fn heartbeat_is_dispatchable_and_emits_nothing() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PresenceHeartbeat {}, &doc, &cfg).expect("handle");
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert!(emit.effects.is_empty());
}
}

mod rename_space {
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use crate::{editor::home::config::HomeConfig, editor::home::config::HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit};
use super::super::rename_space::*;

fn dispatch(payload: RenameSpace, row: Option<&store::os_directory::DirectorySpace>) -> Emit<SHomeMutation, HomeConfigMutation> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    handle_with_row(&payload, &doc, &cfg, row).expect("handle")
}

fn folded_row(name: &str) -> store::os_directory::DirectorySpace {
    let event_json = semio_framework_pack_json::json!({
        "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
        "body": {"kind": "space.created", "spaceId": "sp-1", "name": name, "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
        "recordedAtMs": 1000
    })
    .to_string();
    let event = semio_framework_pack_json::from_json_str::<store::os_directory::DirectoryEvent>(&event_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture directory event");
    store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event).spaces.remove("sp-1").expect("folded row")
}

#[semio_framework_async_macros::async_test]
async fn empty_name_opens_the_dialog_preseeded_with_the_current_name() {
    let row = folded_row("Old Name");
    let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: String::new() }, Some(&row));
    let (dialog_id, args) = match &emit.effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => (dialog_id.clone(), args.clone()),
        other => panic!("expected OpenDialog, got {other:?}"),
    };
    assert_eq!(dialog_id, "renameSpace");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["name"], "Old Name");
}

#[semio_framework_async_macros::async_test]
async fn non_empty_name_relays_the_rename() {
    let emit = dispatch(RenameSpace { space_id: "sp-1".into(), name: "New Name".into() }, None);
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.rename-space");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["spaceId"], "sp-1");
    assert_eq!(args_value["name"], "New Name");
}

/// 🚫️ Without the retained job's captured projection there is no current name to seed: the direct lane refuses by name.
#[test]
fn the_direct_lane_refuses_by_name() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = ArtifactView::new(&doc_snapshot, &history);
    let config = HomeConfig::default();
    let refused = handle(&RenameSpace { space_id: "sp-1".into(), name: String::new() }, &doc, &ConfigView { snapshot: &config, window: None });
    assert!(matches!(refused, Err(fault) if fault.code.0.as_str() == "s.home.rename-space.requires-retained-job"));
}
}

mod manage_space {
use crate::SHomeSnapshot;
use crate::editor::home::config::HomeConfig;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect};
use super::super::manage_space::*;

async fn doc_view<'a>(history: &'a semio_framework_plugin::HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

#[semio_framework_async_macros::async_test]
async fn a_space_id_relays_the_shell_administration_effect_without_a_local_mutation() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ManageSpace { space_id: "space-a".into() }, &doc, &cfg).expect("manage space relays");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.open-administration");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["spaceId"], "space-a");
}

#[semio_framework_async_macros::async_test]
async fn an_empty_space_id_is_a_fault_not_a_blank_pane() {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot).await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    assert!(handle(&ManageSpace { space_id: "  ".into() }, &doc, &cfg).is_err());
}
}

mod share_space {
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use crate::{editor::home::config::HomeConfig, editor::home::config::HomeConfigMutation};
use semio_framework_plugin::Emit;
use super::super::share_space::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

fn doc_view<'a>(history: &'a HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

fn hub_row(space_id: &str) -> store::os_directory::DirectorySpace {
    let event = store::os_directory::DirectoryEvent {
        seq: 1,
        id: "evt-1".into(),
        hlc: store::os_directory::Hlc { physical_ms: 0, logical: 0 },
        actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "u".into() },
        space_id: Some(space_id.into()),
        user_id: None,
        body: store::os_directory::DirectoryEventBody::SpaceCreated {
            space_id: space_id.into(),
            name: "Fabrication".into(),
            space_kind: store::os_directory::DirectorySpaceKind::Studio,
            visibility: store::os_directory::DirectorySpaceVisibility::Public,
            owner_user_id: "u1".into(),
        },
        recorded_at_ms: 1000,
    };
    store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event).spaces.remove(space_id).expect("folded hub row")
}

fn share(payload: ShareSpace, hub_row: Option<&store::os_directory::DirectorySpace>) -> Emit<SHomeMutation, HomeConfigMutation> {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = HomeConfig::default();
    handle_with_row(&payload, &doc, &ConfigView { snapshot: &config, window: None }, hub_row).expect("handle")
}

#[semio_framework_async_macros::async_test]
async fn empty_email_opens_the_share_dialog() {
    let row = hub_row("sp-1");
    let emit = share(ShareSpace { space_id: "sp-1".into(), email: String::new(), role: String::new() }, Some(&row));
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "shareSpace"));
}

#[semio_framework_async_macros::async_test]
async fn ephemeral_local_only_blocks_share_with_accessible_notice() {
    let emit = share(ShareSpace { space_id: "draft-1".into(), email: String::new(), role: String::new() }, None);
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "ephemeralShareBlocked"));
}

#[semio_framework_async_macros::async_test]
async fn email_and_role_relay_upsert_member() {
    let row = hub_row("sp-1");
    let emit = share(ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: "author".into() }, Some(&row));
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.upsert-member");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["email"], "ada@semio.dev");
    assert_eq!(args_value["role"], "author");
}

#[semio_framework_async_macros::async_test]
async fn blank_role_defaults_to_spectator() {
    let row = hub_row("sp-1");
    let emit = share(ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: String::new() }, Some(&row));
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args");
    let args_value: semio_framework_pack_json::Value = semio_framework_pack_json::from_dsl_value(&args);
    assert_eq!(args_value["role"], "spectator");
}

/// 🚫️ The direct lane cannot tell a hub space from a local one: it refuses by name instead of guessing.
#[test]
fn the_direct_lane_refuses_by_name() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = HomeConfig::default();
    let refused = handle(&ShareSpace { space_id: "sp-1".into(), email: String::new(), role: String::new() }, &doc, &ConfigView { snapshot: &config, window: None });
    assert!(matches!(refused, Err(fault) if fault.code.0.as_str() == "s.home.share-space.requires-retained-job"));
}
}
