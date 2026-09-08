//! 🧪️ Space package identity, codec, mutation, and collaboration laws.

use crate::*;
use protocol::Mutation as _;
use store::ArtifactDsl as _;

fn demo_user(id: &str, role: SpaceRole) -> SpaceUser {
    SpaceUser { id: id.into(), name: format!("User {id}"), avatar: None, role }
}

fn demo_space() -> SpaceSnapshot {
    let mut space = empty_space_snapshot("Atelier Demo", SpaceKind::Atelier, SpaceVisibility::Private);
    space.users.push(demo_user("u1", SpaceRole::Author));
    space.collections.push(CollectionRef { id: "c1".into(), name: "Main".into(), document_id: "doc-c1".into() });
    space
}

#[test]
fn package_declaration_matches_serde_json_oracle() {
    let ours = package_descriptor().expect("first-party package parser");
    let oracle: serde_json::Value = serde_json::from_str(SPACE_ARTIFACT_DEFINITION_SCHEMA).expect("third-party package parser");
    assert_eq!(ours.id, oracle["id"].as_str().expect("oracle id"));
    assert_eq!(ours.rust_package, oracle["rust_package"].as_str().expect("oracle package"));
    let invalid = SPACE_ARTIFACT_DEFINITION_SCHEMA.replace("\"os.space\"", "\"os.collection\"");
    assert!(space_package_from_schema(&invalid).is_err());
    assert!(serde_json::from_str::<serde_json::Value>(&invalid).is_ok());
}

#[test]
fn space_document_example_matches_codec_and_pack_contract() {
    let parsed = SpaceSnapshot::parse_dsl(include_str!("../../📚️examples/🪐️demo.space")).expect("parse space example");
    assert_eq!(SpaceSnapshot::envelope_id(), S_SPACE_SCHEMA);
    assert_eq!(parsed.schema, "s.space");
    store::os_store::test_support::assert_dsl_round_trip(&parsed);
    store::os_store::test_support::assert_dsl_pack_equivalence(&demo_space());
}

#[semio_framework_async_macros::async_test]
async fn space_mutation_inventory_and_round_trip_are_owned_by_the_package() {
    assert_eq!(SpaceMutation::DESCRIPTORS.len(), 13);
    for descriptor in SpaceMutation::DESCRIPTORS {
        assert!(descriptor.validate().is_ok());
        assert!(descriptor.owner.starts_with("🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/🧬️mutations/"));
    }
    let rename = SpaceMutation::SetName { name: "Renamed".into() };
    store::os_store::test_support::assert_op_line_round_trip(&rename);
    store::os_store::test_support::assert_operation_round_trip(&demo_space(), rename).await;
}

#[test]
fn atelier_reconciliation_is_deterministic() {
    let mut atelier = empty_space_snapshot("Atelier", SpaceKind::Atelier, SpaceVisibility::Private);
    atelier.users.push(demo_user("u2", SpaceRole::Author));
    atelier.users.push(demo_user("u1", SpaceRole::Author));
    let (reconciled, messages) = reconcile_space_atelier_invariant(atelier);
    assert_eq!(messages.len(), 1);
    assert_eq!(space_role_of(&reconciled, "u1"), Some(SpaceRole::Author));
    assert_eq!(space_role_of(&reconciled, "u2"), Some(SpaceRole::Spectator));
    assert!(can_write(&reconciled, "u1"));
    assert!(!can_write(&reconciled, "u2"));
}
