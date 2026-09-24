use super::*;
use crate::empty_playbook_snapshot;
use crate::PLAYBOOK_DOCUMENT_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_the_empty_snapshot() {
    let document = empty_playbook_snapshot();
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn facade_generator_example_pack_round_trips() {
    let document = super::super::text::parse_dsl(super::super::text::FACADE_GENERATOR_EXAMPLE_TEXT).expect("parse example");
    let decoded = decode(&encode(&document)).expect("decode");
    assert_eq!(decoded, document);
    assert!(!decoded.steps().is_empty(), "the steps the flow handle owns travel through pack");
    assert_eq!(decoded.steps(), document.steps(), "pack carries every step and block exactly");
}

#[semio_framework_async_macros::async_test]
async fn facade_generator_example_pack_agrees_with_dsl() {
    let document = empty_playbook_snapshot();
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    store::os_store::test_support::assert_pack_schema_identity(&document);
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::op::{change_title_operation, PlaybookMutation};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

    let mut store: ArtifactStore<PlaybookSnapshot, PlaybookMutation> = ArtifactStore::new(create_document_envelope(PLAYBOOK_DOCUMENT_SCHEMA, "playbook-demo", empty_playbook_snapshot(), None)).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<PlaybookSnapshot, PlaybookMutation>());
    store.dispatch(ArtifactCommand::Apply { mutations: vec![change_title_operation(Some("Recipe".into()))], description: None }).await.expect("apply");
    let edit: &Edit<PlaybookMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<PlaybookSnapshot, PlaybookMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("playbook store closes through its exact bounded owners");
    }
}
