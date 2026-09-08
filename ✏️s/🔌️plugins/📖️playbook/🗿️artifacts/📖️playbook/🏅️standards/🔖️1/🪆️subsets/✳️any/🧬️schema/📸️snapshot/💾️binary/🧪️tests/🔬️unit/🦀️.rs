
use super::*;
use crate::PLAYBOOK_DOCUMENT_SCHEMA;
use crate::empty_playbook_snapshot;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_the_empty_snapshot() {
    let document = empty_playbook_snapshot();
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn facade_generator_example_pack_round_trips() {
    let document = empty_playbook_snapshot();
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn facade_generator_example_pack_agrees_with_dsl() {
    let document = empty_playbook_snapshot();
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::op::{PlaybookMutation, change_title_operation};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, ArtifactStore, create_document_envelope};

    let mut store: ArtifactStore<PlaybookSnapshot, PlaybookMutation> = ArtifactStore::new(create_document_envelope(PLAYBOOK_DOCUMENT_SCHEMA, "playbook-demo", empty_playbook_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![change_title_operation(Some("Recipe".into()))], description: None }).await.expect("apply");
    let edit: &Edit<PlaybookMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<PlaybookSnapshot, PlaybookMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
