
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> WavSnapshot {
    WavSnapshot { data: WavData::Pcm16(vec![1, -1, 100, -100]), ..WavSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = sample_snapshot();
    let bytes = <WavSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <WavSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = sample_snapshot();
    let text = <WavSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <WavSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}
