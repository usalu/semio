//! ⚖️ S Home launcher artifact — binary command protocol surface + laws (constitutional: spr).

use crate::artifacts::home::op::SHomeOperation;
use protocol::OpBinary;

/// 📦️ Encodes an `SHomeOperation` to its binary command form.
pub fn encode_op(operation: &SHomeOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `SHomeOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SHomeOperation, protocol::ProtocolError> {
    SHomeOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SHomeOperation::SetCatalogGeneration { value: 7 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn home_document_text_round_trips_through_the_store() {
        use crate::artifacts::home::SHomeDocument;
        let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let envelope = store::create_document_envelope::<SHomeDocument, SHomeOperation>("s.home", "home", projection, None);
        let mut store: store::DocumentStore<SHomeDocument, SHomeOperation> = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![SHomeOperation::SetCatalogGeneration { value: 3 }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
