//! 📦️ DAG artifact — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack for DagDocument` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`); see `crate::artifacts::dag::op`'s doc for why. This module only
//! adds the thin artifact-facing `encode`/`decode` wrappers plus the pack↔dsl equivalence law.

use crate::artifacts::dag::DagDocument;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


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
    use crate::artifacts::dag::dsl;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `DagMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing dsl/pack round-trip law (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::dag::op::DagMutation;
        use crate::artifacts::dag::DAG_DOCUMENT_SCHEMA;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let document = DagDocument { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() };
        let mut store: DocumentStore<DagDocument, DagMutation> = DocumentStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag-demo", document, None));
        store.dispatch(DocumentCommand::Apply { mutations: vec![DagMutation::SetNodes { nodes: Vec::new() }], description: None }).expect("apply");
        let edit: &Edit<DagMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<DagDocument, DagMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::dag::dsl::parse_dsl(crate::artifacts::dag::dsl::DAG_EXAMPLE_TEXT)
            .expect("parse fixture");
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
}

