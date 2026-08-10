//! ⚖️ S Home launcher artifact — binary command protocol surface + laws (constitutional: spr).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::home::schema::mutations::text::SHomeMutation;
use protocol::OpBinary;

/// 📦️ Encodes an `SHomeMutation` to its binary command form.
pub fn encode_op(operation: &SHomeMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `SHomeMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SHomeMutation, protocol::ProtocolError> {
    SHomeMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SHomeMutation::SetCatalogGeneration { value: 7 };
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn home_document_text_round_trips_through_the_store() {
        use crate::artifacts::home::SHomeSnapshot;
        let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let envelope = store::create_document_envelope::<SHomeSnapshot, SHomeMutation>("s.home", "home", projection, None);
        let mut store: store::DocumentStore<SHomeSnapshot, SHomeMutation> = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![SHomeMutation::SetCatalogGeneration { value: 3 }], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
