
use super::*;

/// 🌱 Reuses `demo_audio_snapshot()` (single source of truth, also feeds the shipped fixtures
/// and `🎹️composer/🦀️.rs`'s conformance-law tests) rather than an independent copy.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_snapshot() -> SemioAudioSnapshot {
    demo_audio_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = sample_snapshot();
    let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = sample_snapshot();
    let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_no_channels_or_tags() {
    let snap = SemioAudioSnapshot::default();
    assert!(snap.channels.is_empty());
    assert!(snap.tags.is_empty());
    assert_eq!(snap.format, SemioAudioFormat::Pcm16);
}

/// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
/// on a fully-populated snapshot (channels/tags non-empty), not just the default.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = sample_snapshot();
    let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioAudioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioAudioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}
