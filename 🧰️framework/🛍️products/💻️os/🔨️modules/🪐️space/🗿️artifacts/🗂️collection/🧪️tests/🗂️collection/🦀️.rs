//! 🧪️ Collection package identity, codec, mutation, and hierarchy laws.

use crate::*;
use protocol::Mutation as _;
use store::ArtifactDsl as _;

fn demo_collection() -> CollectionSnapshot {
    let mut collection = empty_collection_snapshot("Main");
    collection.folders.push(CollectionFolder { id: "f1".into(), parent_id: None, name: "Renders".into() });
    collection.entries.push(CollectionEntry {
        id: "e1".into(),
        folder_id: Some("f1".into()),
        name: "sketch".into(),
        kind_id: "puzzle.2d".into(),
        body: Box::new(ArtifactBody::Document { schema: "test.puzzle2d".into(), document_id: "doc-e1".into() }),
    });
    collection
}

#[test]
fn package_declaration_matches_serde_json_oracle() {
    let ours = package_descriptor().expect("first-party package parser");
    let oracle: serde_json::Value = serde_json::from_str(COLLECTION_ARTIFACT_DEFINITION_SCHEMA).expect("third-party package parser");
    assert_eq!(ours.id, oracle["id"].as_str().expect("oracle id"));
    assert_eq!(ours.rust_package, oracle["rust_package"].as_str().expect("oracle package"));
    let invalid = COLLECTION_ARTIFACT_DEFINITION_SCHEMA.replace("\"os.collection\"", "\"os.space\"");
    assert!(collection_package_from_schema(&invalid).is_err());
    assert!(serde_json::from_str::<serde_json::Value>(&invalid).is_ok());
}

#[test]
fn collection_document_example_matches_codec_and_pack_contract() {
    let parsed = CollectionSnapshot::parse_dsl(include_str!("../../📚️examples/🎬️demo.collection")).expect("parse collection example");
    assert_eq!(CollectionSnapshot::envelope_id(), S_COLLECTION_SCHEMA);
    assert_eq!(parsed.schema, "s.collection");
    store::os_store::test_support::assert_dsl_round_trip(&parsed);
    store::os_store::test_support::assert_dsl_pack_equivalence(&demo_collection());
}

#[semio_framework_async_macros::async_test]
async fn collection_mutation_inventory_and_round_trip_are_owned_by_the_package() {
    assert_eq!(CollectionMutation::DESCRIPTORS.len(), 10);
    for descriptor in CollectionMutation::DESCRIPTORS {
        assert!(descriptor.validate().is_ok());
        assert!(descriptor.owner.starts_with("🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/"));
    }
    let rename = CollectionMutation::RenameEntry { entry_id: "e1".into(), new_name: "sketch 2".into() };
    store::os_store::test_support::assert_op_line_round_trip(&rename);
    store::os_store::test_support::assert_operation_round_trip(&demo_collection(), rename).await;
}

#[test]
fn hierarchy_reconciliation_and_path_resolution_stay_deterministic() {
    let mut collection = demo_collection();
    collection.folders.push(CollectionFolder { id: "orphan".into(), parent_id: Some("missing".into()), name: "Loose".into() });
    collection.entries.push(CollectionEntry {
        id: "e2".into(),
        folder_id: Some("missing".into()),
        name: "orphan.txt".into(),
        kind_id: "file.blob".into(),
        body: Box::new(ArtifactBody::Blob { blob: store::BlobRef { hash: "h".into(), size: 1, media_type: "text/plain".into() } }),
    });
    let (reconciled, messages) = reconcile_collection_integrity(collection);
    assert_eq!(reconciled.folders.iter().find(|folder| folder.id == "orphan").and_then(|folder| folder.parent_id.as_deref()), None);
    assert_eq!(reconciled.entries.iter().find(|entry| entry.id == "e2").and_then(|entry| entry.folder_id.as_deref()), None);
    assert_eq!(entry_path(&reconciled, "e1").as_deref(), Some("Renders/sketch"));
    assert_eq!(resolve_entry_by_path(&reconciled, "Renders/sketch").map(|entry| entry.id.as_str()), Some("e1"));
    assert_eq!(messages.len(), 2);
}
