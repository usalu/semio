//! 🧪️ Collection package identity, codec, mutation, and hierarchy laws.

use crate::*;
use protocol::Mutation as _;
use store::ArtifactDsl as _;

#[test]
fn collection_native_schema_factories_admit_both_directions_before_copies(){
    let oracle:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏭️native-schema/🔣️.json")).unwrap();
    let mut accept=|_|true;
    let mut decoding=semio_framework_value::NativeDecodeControl::new(oracle["maximumBytes"].as_u64().unwrap() as usize,&mut accept);
    let variants=<ArtifactBody as semio_framework_dsl_record::DslVariants>::variants_controlled(&mut decoding).unwrap();
    let mut output=Vec::new();
    for (keyword,producer) in &variants{
        let decoded=producer.decode(&mut decoding).unwrap();
        let mut accept=|_|true;
        let mut encoding=semio_framework_value::NativeEncodeControl::new(oracle["maximumBytes"].as_u64().unwrap() as usize,&mut accept);
        let encoded=producer.encode(&mut encoding).unwrap();
        let summarize=|spec:&semio_framework_dsl_record::RecordSpec|serde_json::json!({"keyword":spec.keyword,"fields":spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"shape":match field.shape{semio_framework_dsl_record::Shape::Text=>"text",semio_framework_dsl_record::Shape::UInt=>"uint",_=>panic!("unexpected collection field shape")}})).collect::<Vec<_>>()});
        assert_eq!(decoded.keyword.as_deref(),Some(keyword.as_str()));
        assert!(decoded.fields.iter().all(|field|!field.optional&&!field.flatten&&field.position.is_none()&&field.defines.is_none()&&!field.is_call_name));
        assert_eq!(summarize(&decoded),summarize(&encoded));
        output.push(summarize(&decoded));
        let mut refuse=|_|false;
        assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(65536,&mut refuse)).is_err());
        let mut refuse=|_|false;
        assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(65536,&mut refuse)).is_err());
        let mut accept=|_|true;
        assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(1,&mut accept)).is_err());
        let mut accept=|_|true;
        assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(1,&mut accept)).is_err());
    }
    assert_eq!(serde_json::Value::Array(output),oracle["variants"]);
    let mut refuse=|_|false;
    let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut refuse);
    assert!(<ArtifactBody as semio_framework_dsl_record::DslVariants>::variants_controlled(&mut decoding).is_err());
    let mut refuse=|_|false;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(65536,&mut refuse);
    assert!(<ArtifactBody as semio_framework_dsl_record::DslVariants>::variants_controlled(&mut encoding).is_err());
    let mut accept=|_|true;
    let mut decoding=semio_framework_value::NativeDecodeControl::new(oracle["refusedBytes"].as_u64().unwrap() as usize,&mut accept);
    assert!(<ArtifactBody as semio_framework_dsl_record::DslVariants>::variants_controlled(&mut decoding).is_err());
    let mut accept=|_|true;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(oracle["refusedBytes"].as_u64().unwrap() as usize,&mut accept);
    assert!(<ArtifactBody as semio_framework_dsl_record::DslVariants>::variants_controlled(&mut encoding).is_err());
}

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
