use super::*;

/// 🧪️ codec_retention_law: decode(encode(x)) == x through both the pack (binary) and dsl
/// (text) envelopes, on a snapshot exercising every `AnimValue` variant and both `AnimTarget`
/// property kinds (incl. `Custom`).
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = demo_animation_snapshot();
    let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);

    let text = <SemioAnimationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_round_trips() {
    let snap = SemioAnimationSnapshot::default();
    let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}
