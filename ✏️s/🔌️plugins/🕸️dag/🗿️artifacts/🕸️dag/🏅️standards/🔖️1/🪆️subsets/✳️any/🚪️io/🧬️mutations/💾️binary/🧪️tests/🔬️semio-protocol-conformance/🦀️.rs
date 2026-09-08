
use super::*;

#[semio_framework_async_macros::async_test]
async fn component_protocol_semio_is_protocol_dialect() {
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
    assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
    assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
    let _ = COMPONENT_PROTOCOL_PATH;
}

#[semio_framework_async_macros::async_test]
async fn verify_protocol_bytes_against_encoded_spr() {
    let operation = crate::mutations::delete_node("node-1".into());
    let bytes = encode_op(&operation).expect("encode op");
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
    ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
}
