//! 🎒️ Imperative artifact — binary document surface + laws (constitutional: pack).
//!
//! `ImperativeDocument` carries a manual `store::DocumentDsl` impl (see `🗣️dsl`, not
//! `#[derive(dsl::DslDocument)]` directly), so it did NOT automatically gain `store::DocumentPack` from
//! `dsl_derive`'s expansion. This mirrors the derive-emitted shape exactly, substituting
//! `dsl::ImperativeDocumentDsl`'s `__dsl_spec`/`__dsl_to_record`/`__dsl_from_record` trio for `Self`'s
//! (unavailable here) and routing through the same mirror-struct conversion `🗣️dsl::parse_dsl`/
//! `print_dsl` already use.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::imperative::dsl::{self, ImperativeDocumentDsl};
use crate::artifacts::imperative::ImperativeDocument;
use store::PackError;

impl store::DocumentPack for ImperativeDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let mirror = dsl::document_to_document_dsl(self);
        store::pack_rt::encode_document(&ImperativeDocumentDsl::__dsl_spec(), &mirror.__dsl_to_record(), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (record, _report) = store::pack_rt::decode_document(bytes, &ImperativeDocumentDsl::__dsl_spec(), options)?;
        let parsed = ImperativeDocumentDsl::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        Ok(dsl::document_dsl_to_document(parsed))
    }
}

//#region 🔖️Api
/// 📦️ Encodes an `ImperativeDocument` to its binary pack form.
pub fn encode(document: &ImperativeDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes an `ImperativeDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<ImperativeDocument, PackError> {
    <ImperativeDocument as store::DocumentPack>::decode_pack(bytes)
}
//#endregion 🔖️Api

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_representative_document_with_nested_control_body() {
        use crate::artifacts::imperative::{Dictionary, Path, Step};
        use std::collections::BTreeMap;

        let mut document = dsl::parse_dsl(dsl::IMPERATIVE_EXAMPLE_TEXT).expect("parse 📜️default.imperative");
        let inner = Step { id: "step-inner".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let mut owner = Step { id: "step-if".into(), kind: "control.if".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
        document.path.steps = vec![owner];

        store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪️Tests
