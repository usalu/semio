
use super::*;

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioModelSnapshot::default();
    let bytes = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioModelSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioModelSnapshot::default();
    let text = <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law: decode(encode(x)) == x for a fully-populated snapshot (every
/// collection non-empty, every field set), through both the pack (binary) and DSL (text) codecs.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = demo_semio_model_snapshot();
    let packed = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let unpacked = <SemioModelSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode pack");
    assert_eq!(snap, unpacked);

    let text = <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let reparsed = <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse dsl");
    assert_eq!(snap, reparsed);
}
