
use super::*;

#[semio_framework_async_macros::async_test]
async fn component_protocol_semio_is_protocol_dialect() {
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
    assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
    assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
    let _ = COMPONENT_PROTOCOL_PATH;
}

#[semio_framework_async_macros::async_test]
async fn verify_protocol_bytes_against_encoded_pack() {
    let document = crate::document_dsl::parse_dsl(crate::document_dsl::DAG_EXAMPLE_TEXT).expect("parse fixture");
    let bytes = encode(&document);
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
    ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
}
