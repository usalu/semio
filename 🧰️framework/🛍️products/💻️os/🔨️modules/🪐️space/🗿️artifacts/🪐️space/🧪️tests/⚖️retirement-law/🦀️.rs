//! ⚖️ Space manifest owners retire under quoted grants: every space mutation passes the typed-command input lane, and a manifest edit applies, undoes and redoes through the bounded catalog the OS host installs on its space store.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../../../../../../../../../🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs"]
mod typed_input_law;

use semio_framework_artifact_space_space::{empty_space_snapshot, CollectionRef, SpaceKind, SpaceMutation, SpaceRole, SpaceSnapshot, SpaceUser, SpaceVisibility, S_SPACE_SCHEMA};
use semio_framework_os_kernel::{self as store, os_store::test_support::{close_plain_test_store, dispatch_test_command}};
use typed_input_law::{typed_command_retirement_law, typed_input_retirement_law};

fn user(id: &str) -> SpaceUser {
    SpaceUser { id: id.into(), name: format!("User {id}"), avatar: Some("avatar.png".into()), role: SpaceRole::Author }
}

fn demo_space() -> SpaceSnapshot {
    let mut space = empty_space_snapshot("Atelier Demo", SpaceKind::Atelier, SpaceVisibility::Private);
    space.users.push(user("u1"));
    space.collections.push(CollectionRef { id: "c1".into(), name: "Main".into(), document_id: "doc-c1".into() });
    space
}

fn mutations() -> Vec<(&'static str, SpaceMutation)> {
    vec![
        ("set-name", SpaceMutation::SetName { name: "Renamed Atelier".into() }),
        ("upsert-user", SpaceMutation::UpsertUser { user: user("u2"), index: Some(1) }),
        ("remove-user", SpaceMutation::RemoveUser { user_id: "u1".into() }),
        ("add-collection", SpaceMutation::AddCollection { collection: CollectionRef { id: "c2".into(), name: "Second".into(), document_id: "doc-c2".into() }, index: None }),
        ("rename-collection", SpaceMutation::RenameCollection { collection_id: "c1".into(), name: "Primary".into() }),
        ("install-program", SpaceMutation::InstallProgram { plugin_id: "plugin.law".into(), index: None }),
    ]
}

/// 🎟️ Every space mutation and the manifest itself retire through the typed-command input lane.
#[test]
fn space_mutations_and_manifest_retire_through_the_typed_command_lane() {
    for (label, mutation) in mutations() {
        let law = typed_command_retirement_law(label, mutation);
        assert!(law.turns > 0 && law.declared_births > 0, "{label}: retiring costs paid turns and births");
    }
    let manifest = typed_input_retirement_law("space manifest", demo_space());
    assert!(manifest.turns > 0 && manifest.declared_births > 0, "retiring the manifest costs paid turns and births");
}

/// 📄️ Each mutation applies, undoes and redoes through the bounded document catalog the OS host installs, and the store closes to its terminal-empty witness.
#[semio_framework_async_macros::async_test]
async fn a_space_manifest_edit_applies_undoes_and_redoes_through_its_real_catalog() {
    for (label, mutation) in mutations() {
        let owners = store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<SpaceSnapshot, SpaceMutation>().unwrap_or_else(|error| panic!("{label}: the bounded catalog is funded: {error}"));
        let initial = demo_space();
        let envelope = store::os_store::document_materialize::create_document_envelope::<SpaceSnapshot, SpaceMutation>(S_SPACE_SCHEMA, "space-retirement-law", initial.clone(), None);
        let mut space = store::ArtifactStore::new(envelope, store::os_spr::ActorId(store::os_spr::LOCAL_ACTOR_ID.into())).await.expect("the law space store");
        store::install_unscheduled_catalog(&mut space, Ok(owners)).unwrap_or_else(|_| panic!("{label}: the bounded catalog installs"));
        dispatch_test_command(&mut space, store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }).await.unwrap_or_else(|error| panic!("{label}: apply: {error}"));
        let applied = space.snapshot().expect("applied snapshot");
        assert_ne!(applied, initial, "{label}: the mutation changes the manifest");
        dispatch_test_command(&mut space, store::os_store::store_schema::ArtifactCommand::Undo).await.unwrap_or_else(|error| panic!("{label}: undo: {error}"));
        assert_eq!(space.snapshot().expect("undone snapshot"), initial, "{label}: undo restores the manifest");
        dispatch_test_command(&mut space, store::os_store::store_schema::ArtifactCommand::Redo).await.unwrap_or_else(|error| panic!("{label}: redo: {error}"));
        assert_eq!(space.snapshot().expect("redone snapshot"), applied, "{label}: redo restores the post manifest");
        close_plain_test_store(&mut space);
    }
}
