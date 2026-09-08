
use crate::os_dsl::{parse_protocol, verify_protocol_source, walk_protocol};
use crate::os_store::semio_format::{Component, SemioEnvelope, wrap_binary};

const PROTOCOL: &str = include_str!("../../../../🧬️semio/📡️protocol/📡️.protocol.semio");

#[semio_framework_async_macros::async_test]
async fn semio_envelope_protocol_parses_under_the_real_dialect() {
    let spec = parse_protocol(PROTOCOL).expect("semio envelope protocol.semio must parse under dsl_grammar's real parser");
    assert_eq!(spec.id, "semio.envelope");
    assert_eq!(spec.schema, "semio.envelope");
}

#[semio_framework_async_macros::async_test]
async fn semio_envelope_protocol_walks_a_real_wrap_binary_payload() {
    let envelope = SemioEnvelope::from_envelope_id("stdio.gif", Component::Pack, 1).expect("valid envelope id");
    let payload = b"real gif89a pack payload bytes, not a fabricated placeholder".to_vec();
    let wrapped = wrap_binary(&envelope, &payload);

    verify_protocol_source(PROTOCOL, &wrapped).expect("verify_protocol_source must accept a real wrap_binary envelope");
    let spec = parse_protocol(PROTOCOL).expect("parse_protocol");
    let trace = walk_protocol(&spec, &wrapped).expect("walk_protocol must succeed on a real wrap_binary envelope");
    assert_eq!(trace.consumed, wrapped.len(), "walk_protocol must consume every byte of the envelope + payload, consumed == len");
}

#[semio_framework_async_macros::async_test]
async fn semio_envelope_protocol_walks_a_different_token_length_and_an_empty_payload() {
    // A different plugin/artifact/component/version -> a different token length (proves the
    // length-prefixed `token` segment genuinely reads `token_len`, not a hardcoded width), and
    // a genuinely empty inner payload (proves `chain bytes` tolerates zero trailing bytes).
    let envelope = SemioEnvelope::from_envelope_id("stdio.gif", Component::Spr, 3).expect("valid envelope id");
    let wrapped = wrap_binary(&envelope, &[]);

    let spec = parse_protocol(PROTOCOL).expect("parse_protocol");
    let trace = walk_protocol(&spec, &wrapped).expect("walk_protocol must succeed on an empty-payload envelope");
    assert_eq!(trace.consumed, wrapped.len());
}
