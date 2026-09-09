use super::*;

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioKitSnapshot::default();
    let bytes = <SemioKitSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioKitSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioKitSnapshot::default();
    let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = demo_kit_snapshot();
    let bytes = <SemioKitSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioKitSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

/// 🧪️ A parent snapshot NEVER embeds owned-child content — only handles. Links are references
/// by design (never owned), so this proves both composition primitives stay handle-only.
#[semio_framework_async_macros::async_test]
async fn parent_snapshot_stores_only_handles_never_child_content() {
    let snap = demo_kit_snapshot();
    let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    assert!(text.contains(&enc_str("obj-01")));
    assert!(!text.to_lowercase().contains("primitives") && !text.to_lowercase().contains("elements"), "must never embed object/model field names — only the handle");
}
