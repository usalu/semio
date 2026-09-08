
use super::*;
use crate::document_dsl as dsl;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = dsl::parse_dsl(dsl::DAG_EXAMPLE_TEXT).expect("parse default fixture");
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `DagMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing dsl/pack round-trip law (same pattern as `mathematical`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::DAG_DOCUMENT_SCHEMA;
    use crate::op::DagMutation;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, ArtifactStore, create_document_envelope};

    let document = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), content: crate::dag_content_child_with_owner(Vec::new(), Vec::new()) };
    let mut store: ArtifactStore<DagSnapshot, DagMutation> = ArtifactStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag-demo", document, None)).await.expect("valid artifact store fixture");
    let node = crate::schema::default_node_for_kind("note", "node-1", 0.0, 0.0);
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::mutations::create_node(node)], description: None }).await.expect("apply");
    let edit: &Edit<DagMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<DagSnapshot, DagMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
