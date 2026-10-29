//! 🧪️ Aggregated unit tests of the `commands` leaves, one module per leaf.

mod copy_invite_link {
use semio_framework_plugin::kernel::Effect;
use super::super::copy_invite_link::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn copy_invite_link_relays_share_link() {
    let mut app = context::new_app().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::CopyInviteLink(CopyInviteLink { role: "spectator".into(), ttl_secs: 3600 }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("copy link");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.share-link");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("spectator"));
            assert_eq!(args.get("ttlSecs").and_then(|v| v.as_f64()), Some(3600.0));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
}

mod create_artifact {
use semio_framework_plugin::kernel::Effect;
use super::super::create_artifact::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn create_artifact_relays_only_the_catalog_choice_and_name_without_local_publication() {
    let mut app = context::new_app().await;
    let kind_choice = "{\"kindId\":\"s.gis.gismap\",\"schema\":\"gis.map\"}";
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::CreateArtifact(CreateArtifact { name: " First ".into(), kind_choice: kind_choice.into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("create artifact");
    let snapshot = app.snapshot().expect("projection");
    assert!(snapshot.artifacts.is_empty(), "the guest must not mint or publish a document identity");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.create-space-artifact");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().expect("args"));
            assert_eq!(args.get("kindChoice").and_then(|v| v.as_str()), Some(kind_choice));
            assert_eq!(args.get("name").and_then(|v| v.as_str()), Some("First"));
            assert_eq!(args.as_object().map(|value| value.len()), Some(2));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_name_and_kind_open_the_dialog_instead_of_failing() {
    let mut app = context::new_app().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::CreateArtifact(CreateArtifact { name: String::new(), kind_choice: String::new() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("empty args must open the dialog, not fail");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => {
            assert_eq!(dialog_id, "createArtifact");
            assert!(args.is_none());
        }
        other => panic!("expected OpenDialog, got {other:?}"),
    }
    let snapshot = app.snapshot().expect("projection");
    assert!(snapshot.artifacts.is_empty(), "opening the dialog must not create a row");
}
}

mod delete_artifact {
use super::super::delete_artifact::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn delete_artifact_removes_the_row() {
    let (mut app, id) = context::new_app_with_artifact().await;
    semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::DeleteArtifact(DeleteArtifact { id: id.clone() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("delete artifact");
    let snapshot = app.snapshot().expect("projection");
    assert!(snapshot.artifacts.iter().all(|row| row.id != id));
}
}

mod fold_directory_events {
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig};
use semio_framework_os_kernel::os_directory::{DirectoryEvent, DirectorySpaceRole, DirectorySpaceVisibility};
use semio_framework_plugin::{ConfigView};
use super::super::fold_directory_events::*;
use protocol::Mutation as _;
use semio_framework_os_kernel::os_directory::{ArtifactHash, DirectoryActor, DirectoryActorKind, DirectoryEventBody, DirectorySpaceKind, DocumentDescriptor, DocumentFrontier, DocumentIndexEntryV1, DocumentOwner, DocumentScope, Hlc};
use {semio_framework_artifact_reference::ArtifactDialect};
use semio_framework_plugin::{ArtifactView, HistoryView};

fn event(seq: u64, body: DirectoryEventBody, space_id: Option<&str>) -> DirectoryEvent {
    DirectoryEvent {
        seq,
        id: format!("evt-{seq}"),
        hlc: Hlc { physical_ms: seq as i64, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
        space_id: space_id.map(Into::into),
        user_id: None,
        body,
        recorded_at_ms: seq as i64,
    }
}

fn view_for(space_id: &str) -> SSpaceSnapshot {
    SSpaceSnapshot { space_id: space_id.into(), ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn folds_visibility_and_members_for_this_space_into_config() {
    let snapshot = view_for("space-1");
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &config_snapshot, window: None };
    let events = vec![
        event(1, DirectoryEventBody::UserCreated { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into() }, None),
        event(2, DirectoryEventBody::SpaceCreated { space_id: "space-1".into(), name: "Space 1".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Public, owner_user_id: "u-1".into() }, Some("space-1")),
        event(3, DirectoryEventBody::MemberUpserted { space_id: "space-1".into(), user_id: "u-1".into(), role: DirectorySpaceRole::Author }, Some("space-1")),
    ];
    let events_json = semio_framework_pack_json::to_json_string(&events);
    let result = handle(&FoldDirectoryEvents { events_json }, &doc, &cfg).expect("fold");
    assert_eq!(result.config_mutations.len(), 1);
    let config = protocol::apply_diff_cold(result.config_mutations[0].diff(&config_snapshot).diff(), &config_snapshot).expect("the fold applies");
    assert_eq!(config.visibility, "public");
    assert_eq!(config.members.len(), 1);
    assert_eq!(config.members[0].email, "a@example.com");
    assert_eq!(config.members[0].role, "author");
}

#[semio_framework_async_macros::async_test]
async fn folds_directory_indexed_documents_into_read_only_space_rows() {
    let snapshot = view_for("space-1");
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &config_snapshot, window: None };
    let descriptor = DocumentDescriptor {
        space_id: "space-1".into(),
        document_id: "artifact-0123456789abcdef0123456789abcdef".into(),
        artifact_kind: "s.gis.map".into(),
        artifact_schema: "s.gis.map".into(),
        owner: DocumentOwner { plugin_id: "gis".into(), package_id: "gis-map".into(), version: "1".into(), package_hash: "a".repeat(64) },
        pack_schema_hash: "b".repeat(64),
        bootstrap_version: 1,
        bootstrap_frontier: DocumentFrontier { head_seq: 1, commit_seq: 1, epoch: 1 },
        bootstrap_snapshot_hash: "c".repeat(64),
    };
    let mut indexed = event(
        4,
        DirectoryEventBody::DocumentIndexed {
            scope: DocumentScope { space_id: "space-1".into(), document_id: descriptor.document_id.clone() },
            descriptor_digest_v1: ArtifactHash([7; 32]),
            entry: DocumentIndexEntryV1 { name: "Shared Map".into(), dialect: ArtifactDialect { artifact_kind: "s.gis.map".into(), standard: "1".into(), subset: "*".into() } },
        },
        Some("space-1"),
    );
    indexed.user_id = Some("u-1".into());
    let events = vec![
        event(1, DirectoryEventBody::SpaceCreated { space_id: "space-1".into(), name: "Space 1".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Public, owner_user_id: "u-1".into() }, Some("space-1")),
        event(2, DirectoryEventBody::DocumentAnnounced { descriptor: descriptor.clone() }, Some("space-1")),
        indexed,
    ];
    let result = handle(&FoldDirectoryEvents { events_json: semio_framework_pack_json::to_json_string(&events) }, &doc, &cfg).expect("fold indexed document");
    let config = protocol::apply_diff_cold(result.config_mutations[0].diff(&config_snapshot).diff(), &config_snapshot).expect("the fold applies");
    assert_eq!(config.indexed_artifacts.len(), 1);
    assert_eq!(config.indexed_artifacts[0].id, descriptor.document_id);
    assert_eq!(config.indexed_artifacts[0].name, "Shared Map");
    assert_eq!(config.indexed_artifacts[0].dialect.artifact_kind, "s.gis.map");
    assert_eq!(config.indexed_artifacts[0].created_by, "u-1");
    assert!(snapshot.artifacts.is_empty(), "Directory rows never mutate the legacy whole-vector snapshot");
}

#[semio_framework_async_macros::async_test]
async fn folding_events_for_a_different_space_is_a_no_op() {
    let snapshot = view_for("space-1");
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &config_snapshot, window: None };
    let events =
        vec![event(1, DirectoryEventBody::SpaceCreated { space_id: "space-2".into(), name: "Other".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Public, owner_user_id: "u-1".into() }, Some("space-2"))];
    let events_json = semio_framework_pack_json::to_json_string(&events);
    let result = handle(&FoldDirectoryEvents { events_json }, &doc, &cfg).expect("fold");
    assert!(result.config_mutations.is_empty(), "unrelated-space events never touch this space's config");
    assert!(result.artifact_mutations.is_empty());
}

/// 🪐️ An index opened in the browser is a local genesis document whose `space_id` is still empty; the shell
/// hands it the history of the one space it indexes, and that space's members and visibility must land.
#[semio_framework_async_macros::async_test]
async fn an_index_without_a_space_id_folds_the_one_space_its_history_names() {
    let snapshot = view_for("");
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &config_snapshot, window: None };
    let events = vec![
        event(1, DirectoryEventBody::UserCreated { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into() }, None),
        event(2, DirectoryEventBody::SpaceCreated { space_id: "space-1".into(), name: "Space 1".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private, owner_user_id: "u-1".into() }, Some("space-1")),
        event(3, DirectoryEventBody::MemberUpserted { space_id: "space-1".into(), user_id: "u-1".into(), role: DirectorySpaceRole::Author }, Some("space-1")),
    ];
    let result = handle(&FoldDirectoryEvents { events_json: semio_framework_pack_json::to_json_string(&events) }, &doc, &cfg).expect("fold");
    let [mutation] = result.config_mutations.as_slice() else { panic!("one config mutation, got {:?}", result.config_mutations) };
    let config = protocol::apply_diff_cold(mutation.diff(&config_snapshot).diff(), &config_snapshot).expect("the fold applies");
    assert_eq!(config.visibility, "private");
    assert_eq!(config.members.iter().map(|member| member.email.as_str()).collect::<Vec<_>>(), ["a@example.com"]);
}

/// 🪐️ With no `space_id` of its own, an index handed a history naming two spaces cannot know which one it
/// indexes and must leave its config alone.
#[semio_framework_async_macros::async_test]
async fn an_index_without_a_space_id_never_guesses_between_two_spaces() {
    let snapshot = view_for("");
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &config_snapshot, window: None };
    let events = vec![
        event(1, DirectoryEventBody::SpaceCreated { space_id: "space-1".into(), name: "Space 1".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Public, owner_user_id: "u-1".into() }, Some("space-1")),
        event(2, DirectoryEventBody::SpaceCreated { space_id: "space-2".into(), name: "Space 2".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Public, owner_user_id: "u-1".into() }, Some("space-2")),
    ];
    let result = handle(&FoldDirectoryEvents { events_json: semio_framework_pack_json::to_json_string(&events) }, &doc, &cfg).expect("fold");
    assert!(result.config_mutations.is_empty(), "an ambiguous history never selects a space");
}
}

mod invite_member {
use semio_framework_plugin::kernel::Effect;
use super::super::invite_member::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn invite_member_relays_upsert_member() {
    let mut app = context::new_app().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::InviteMember(InviteMember { email: "a@example.com".into(), role: "author".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("invite");
    assert!(result.mutations.is_empty());
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.upsert-member");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("email").and_then(|v| v.as_str()), Some("a@example.com"));
            assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("author"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
}

mod open_artifact {
use semio_framework_plugin::kernel::Effect;
use super::super::open_artifact::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn open_artifact_relays_with_document_and_space_ids() {
    let (mut app, id) = context::new_app_with_indexed_artifact().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::OpenArtifact(OpenArtifact { id: id.clone() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("open indexed artifact");
    assert!(result.mutations.is_empty());
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.open-artifact");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("documentId").and_then(|v| v.as_str()), Some(id.as_str()));
            assert_eq!(args.get("schema").and_then(|v| v.as_str()), Some("s.draw.draw"));
            assert!(args.get("role").is_none(), "role is omitted so the shell resolves OpeningPreferences");
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn open_artifact_of_a_missing_row_faults() {
    let mut app = context::new_app().await;
    let error = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::OpenArtifact(OpenArtifact { id: "ghost".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect_err("missing row must fault");
    assert_eq!(error.code.0, "s.space.index.target-missing");
}
}

mod open_artifact_with {
use semio_framework_plugin::kernel::Effect;
use super::super::open_artifact_with::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn open_artifact_with_relays_the_explicit_choice() {
    let (mut app, id) = context::new_app_with_indexed_artifact().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::OpenArtifactWith(OpenArtifactWith { id: id.clone(), role: "viewer".into(), plugin_id: "draw".into(), app_id: "draw-play".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())
        .await
        .expect("open with");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.open-artifact-with");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("role").and_then(|v| v.as_str()), Some("viewer"));
            assert_eq!(args.get("pluginId").and_then(|v| v.as_str()), Some("draw"));
            assert_eq!(args.get("appId").and_then(|v| v.as_str()), Some("draw-play"));
            assert_eq!(args.get("schema").and_then(|v| v.as_str()), Some("s.draw.draw"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
}

mod presence_heartbeat {
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView};
use super::super::presence_heartbeat::*;
use protocol::Mutation as _;

fn empty_doc() -> (SSpaceSnapshot, semio_framework_plugin::HistoryView) {
    (SSpaceSnapshot::default(), semio_framework_plugin::HistoryView::empty())
}

#[semio_framework_async_macros::async_test]
async fn heartbeat_sets_presence_for_a_new_artifact() {
    let (snapshot, history) = empty_doc();
    let doc = ArtifactView::new(&snapshot, &history);
    let config_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &config_snapshot, window: None };
    let result = handle(&PresenceHeartbeat { artifact_id: "artifact-1".into(), actors_csv: "user:1,user:2".into() }, &doc, &cfg).expect("heartbeat");
    assert_eq!(result.config_mutations.len(), 1);
    assert_eq!(result.config_mutations[0], SpaceIndexConfigMutation::SetArtifactPresence(crate::editor::space_index::config::ArtifactPresenceSetting { artifact_id: "artifact-1".into(), actors_csv: "user:1,user:2".into() }));
    let after = protocol::apply_diff_cold(result.config_mutations[0].diff(&config_snapshot).diff(), &config_snapshot).expect("the heartbeat applies");
    assert_eq!(after.presence_for("artifact-1"), vec!["user:1", "user:2"]);
}

#[semio_framework_async_macros::async_test]
async fn heartbeat_replaces_an_existing_artifacts_presence() {
    let (snapshot, history) = empty_doc();
    let doc = ArtifactView::new(&snapshot, &history);
    let seeded = SpaceIndexConfig { presence: vec![SpaceIndexArtifactPresence { artifact_id: "artifact-1".into(), actors_csv: "user:1".into() }], ..Default::default() };
    let cfg = ConfigView { snapshot: &seeded, window: None };
    let result = handle(&PresenceHeartbeat { artifact_id: "artifact-1".into(), actors_csv: "user:2".into() }, &doc, &cfg).expect("heartbeat");
    let config = protocol::apply_diff_cold(result.config_mutations[0].diff(&seeded).diff(), &seeded).expect("the heartbeat applies");
    assert_eq!(config.presence.len(), 1, "replaces, does not append, an existing artifact's row");
    assert_eq!(config.presence_for("artifact-1"), vec!["user:2"]);
}
}

mod remove_member {
use semio_framework_plugin::kernel::Effect;
use super::super::remove_member::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn remove_member_relays_remove_member() {
    let mut app = context::new_app().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::RemoveMember(RemoveMember { user_id: "u-1".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("remove");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.remove-member");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("userId").and_then(|v| v.as_str()), Some("u-1"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
}

mod rename_artifact {
use super::super::rename_artifact::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn rename_artifact_updates_the_name() {
    let (mut app, id) = context::new_app_with_artifact().await;
    semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::RenameArtifact(RenameArtifact { id: id.clone(), new_name: "Renamed".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("rename artifact");
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.artifacts.iter().find(|row| row.id == id).map(|row| row.name.clone()), Some("Renamed".into()));
}
}

mod request_delete_artifact {
use semio_framework_plugin::kernel::Effect;
use super::super::request_delete_artifact::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn request_delete_opens_the_confirm_dialog_without_mutating() {
    let (mut app, id) = context::new_app_with_artifact().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::RequestDeleteArtifact(RequestDeleteArtifact { id: id.clone() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("request delete");
    assert!(result.mutations.is_empty(), "requesting delete never mutates the document directly");
    assert_eq!(app.snapshot().unwrap().artifacts.len(), 1, "the row survives until the dialog is confirmed");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::OpenDialog { dialog_id, args, .. } => {
            assert_eq!(dialog_id, "deleteArtifact");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("id").and_then(|v| v.as_str()), Some(id.as_str()));
        }
        other => panic!("expected OpenDialog, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn request_delete_of_a_missing_row_faults() {
    let mut app = context::new_app().await;
    let error = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::RequestDeleteArtifact(RequestDeleteArtifact { id: "ghost".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect_err("missing row must fault");
    assert_eq!(error.code.0, "s.space.mutation.target-missing");
}
}

mod request_invite_member {
use semio_framework_plugin::kernel::Effect;
use super::super::request_invite_member::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn request_invite_member_opens_the_dialog() {
    let mut app = context::new_app().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::RequestInviteMember(RequestInviteMember {}), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("request invite");
    assert!(result.mutations.is_empty());
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::OpenDialog { dialog_id, .. } => assert_eq!(dialog_id, "inviteMember"),
        other => panic!("expected OpenDialog, got {other:?}"),
    }
}
}

mod set_visibility {
use semio_framework_plugin::kernel::Effect;
use super::super::set_visibility::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn set_visibility_relays_the_directory_command() {
    let mut app = context::new_app().await;
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::SetVisibility(SetVisibility { visibility: "public".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("set visibility");
    assert_eq!(result.requested_effects.len(), 1);
    match &result.requested_effects[0] {
        Effect::ReplayShellCommand { action_id, args } => {
            assert_eq!(action_id, "os.directory.set-visibility");
            let args = semio_framework_pack_json::from_dsl_value(&args.clone().unwrap());
            assert_eq!(args.get("visibility").and_then(|v| v.as_str()), Some("public"));
        }
        other => panic!("expected ReplayShellCommand, got {other:?}"),
    }
}
}

mod touch_artifact {
use super::super::touch_artifact::*;
use crate::editor::space_index::{SpaceIndexCommand, unit_tests::context};

#[semio_framework_async_macros::async_test]
async fn touch_artifact_stamps_the_row() {
    let (mut app, id) = context::new_app_with_artifact().await;
    semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &SpaceIndexCommand::TouchArtifact(TouchArtifact { id: id.clone(), now_ms: 99, actor: "user:2".into() }), &semio_framework_plugin::artifact_app_laws::meta("local"), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("touch artifact");
    let snapshot = app.snapshot().expect("projection");
    let row = snapshot.artifacts.iter().find(|row| row.id == id).expect("row");
    assert_eq!(row.updated_at_ms, 99);
    assert_eq!(row.updated_by, "user:2");
}
}
