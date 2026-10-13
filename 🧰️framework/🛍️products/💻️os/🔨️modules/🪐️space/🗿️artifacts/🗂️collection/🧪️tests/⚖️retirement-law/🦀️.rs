//! ⚖️ Collection owners retire under quoted grants: every collection mutation passes the typed-command input lane, and a tree edit applies, undoes and redoes through the bounded catalog the OS host installs on its collection store.

#[global_allocator]
static HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../../../../../../../../../🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs"]
mod typed_input_law;

use semio_framework_artifact_space_collection::{empty_collection_snapshot, CollectionFolder, CollectionMutation, CollectionSnapshot, S_COLLECTION_SCHEMA};
use semio_framework_os_kernel::{self as store, os_store::test_support::{close_plain_test_store, dispatch_test_command}};
use typed_input_law::{typed_command_retirement_law, typed_input_retirement_law};

fn demo_collection() -> CollectionSnapshot {
    empty_collection_snapshot("Main collection")
}

fn mutations() -> Vec<(&'static str, CollectionMutation)> {
    vec![
        ("rename-collection", CollectionMutation::RenameCollection { new_name: "Renamed collection".into() }),
        ("create-folder", CollectionMutation::CreateFolder { folder: CollectionFolder { id: "f1".into(), parent_id: None, name: "Folder".into() }, index: 0 }),
    ]
}

/// 🎟️ Every collection mutation and the collection itself retire through the typed-command input lane.
#[test]
fn collection_mutations_and_tree_retire_through_the_typed_command_lane() {
    for (label, mutation) in mutations() {
        let law = typed_command_retirement_law(label, mutation);
        assert!(law.turns > 0 && law.declared_births > 0, "{label}: retiring costs paid turns and births");
    }
    let tree = typed_input_retirement_law("collection tree", demo_collection());
    assert!(tree.turns > 0 && tree.declared_births > 0, "retiring the tree costs paid turns and births");
}

/// 📄️ Each mutation applies, undoes and redoes through the bounded document catalog the OS host installs, and the store closes to its terminal-empty witness.
#[semio_framework_async_macros::async_test]
async fn a_collection_edit_applies_undoes_and_redoes_through_its_real_catalog() {
    for (label, mutation) in mutations() {
        let owners = store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<CollectionSnapshot, CollectionMutation>().unwrap_or_else(|error| panic!("{label}: the bounded catalog is funded: {error}"));
        let initial = demo_collection();
        let envelope = store::os_store::document_materialize::create_document_envelope::<CollectionSnapshot, CollectionMutation>(S_COLLECTION_SCHEMA, "collection-retirement-law", initial.clone(), None);
        let mut collection = store::ArtifactStore::new(envelope, store::os_spr::ActorId(store::os_spr::LOCAL_ACTOR_ID.into())).await.expect("the law collection store");
        store::install_unscheduled_catalog(&mut collection, Ok(owners)).unwrap_or_else(|_| panic!("{label}: the bounded catalog installs"));
        dispatch_test_command(&mut collection, store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }).await.unwrap_or_else(|error| panic!("{label}: apply: {error}"));
        let applied = collection.snapshot().expect("applied snapshot");
        assert_ne!(applied, initial, "{label}: the mutation changes the collection");
        dispatch_test_command(&mut collection, store::os_store::store_schema::ArtifactCommand::Undo).await.unwrap_or_else(|error| panic!("{label}: undo: {error}"));
        assert_eq!(collection.snapshot().expect("undone snapshot"), initial, "{label}: undo restores the collection");
        dispatch_test_command(&mut collection, store::os_store::store_schema::ArtifactCommand::Redo).await.unwrap_or_else(|error| panic!("{label}: redo: {error}"));
        assert_eq!(collection.snapshot().expect("redone snapshot"), applied, "{label}: redo restores the post collection");
        close_plain_test_store(&mut collection);
    }
}
