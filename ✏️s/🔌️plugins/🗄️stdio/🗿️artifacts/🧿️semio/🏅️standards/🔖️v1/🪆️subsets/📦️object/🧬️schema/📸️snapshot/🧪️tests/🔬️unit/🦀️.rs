
use super::*;

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioObjectSnapshot::default();
    let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioObjectSnapshot::default();
    let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law on a fully-populated snapshot (all 3 child handles present, non-
/// identity transform), not just the default.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = demo_object_snapshot();
    let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

/// 🧪️ A parent snapshot NEVER embeds child content — only the handle's two strings. Proven by
/// asserting the printed DSL contains the child's `child_id`/target URI but never a byte
/// sequence that could only come from parsing the CHILD's own snapshot type.
#[semio_framework_async_macros::async_test]
async fn parent_snapshot_stores_only_child_handles_never_content() {
    let snap = demo_object_snapshot();
    let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    assert!(text.contains(&enc_str("brep-01")), "hex-encoded child_id must be present");
    assert!(!text.to_lowercase().contains("vertices") && !text.to_lowercase().contains("faces"), "must never embed brep/mesh field names — only the handle");
}
