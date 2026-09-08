
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioTextSnapshot {
    demo_text_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioTextSnapshot::default();
    let bytes = <SemioTextSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioTextSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioTextSnapshot::default();
    let text = <SemioTextSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
/// on a fully-populated snapshot (runs/marks non-empty), not just the default.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = populated();
    let bytes = <SemioTextSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioTextSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioTextSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}
