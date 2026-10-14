use super::*;

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = demo_semio_document_snapshot();
    let bytes = <SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    let back = <SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = demo_semio_document_snapshot();
    let text = <SemioDocumentSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioDocumentSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_binary_round_trips() {
    let snap = SemioDocumentSnapshot::default();
    let bytes = <SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    let back = <SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}
