
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioTableSnapshot {
    demo_table_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioTableSnapshot::default();
    let bytes = <SemioTableSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioTableSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioTableSnapshot::default();
    let text = <SemioTableSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
/// on a fully-populated snapshot (columns/rows non-empty), not just the default. Also asserts
/// the CRITICAL row/column alignment invariant survives a round trip untouched.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = populated();
    let bytes = <SemioTableSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioTableSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    for row in &back.rows {
        assert_eq!(row.cells.len(), back.columns.len(), "row/column alignment must survive a pack round trip");
    }
    let text = <SemioTableSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioTableSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}
