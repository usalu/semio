//! 📦️ DAG app — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack for DagDocument` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`); see `s/plugin/dag/app/rs/lib.rs` for why. This crate only adds
//! the thin app-facing `encode`/`decode` wrappers plus the pack↔dsl equivalence law.

use dag::DagDocument;
use store::PackError;

/// 📦️ Encodes a `DagDocument` to its binary pack form.
pub fn encode(document: &DagDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `DagDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<DagDocument, PackError> {
    <DagDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dag_dsl::parse_dsl(dag_dsl::DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `DagOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing dsl/pack round-trip law (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use dag::DAG_DOCUMENT_SCHEMA;
        use dag_op::DagOperation;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let document = DagDocument { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() };
        let mut store: DocumentStore<DagDocument, DagOperation> = DocumentStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag-demo", document, None));
        store.dispatch(DocumentCommand::Apply { operations: vec![DagOperation::SetNodes { nodes: Vec::new() }], description: None }).expect("apply");
        let edit: &Edit<DagOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<DagDocument, DagOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
