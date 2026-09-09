use super::*;

/// 🌱 Reuses `demo_image_snapshot()` (single source of truth, also feeds the shipped fixtures
/// and `🎹️composer/🦀️.rs`'s conformance-law tests) rather than an independent copy.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioImageSnapshot {
    demo_image_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioImageSnapshot::default();
    let bytes = <SemioImageSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioImageSnapshot::default();
    let text = <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
/// on a fully-populated snapshot (frames/icc/metadata all non-empty), not just the default.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = populated();
    let bytes = <SemioImageSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}
