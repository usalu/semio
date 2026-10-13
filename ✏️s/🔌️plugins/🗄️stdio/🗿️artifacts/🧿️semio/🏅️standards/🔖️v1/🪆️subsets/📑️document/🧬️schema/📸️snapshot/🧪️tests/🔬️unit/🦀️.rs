use super::*;

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = demo_semio_document_snapshot();
    let bytes = <SemioDocumentSnapshot as store::os_store::serialization::ArtifactPack>::encode_pack(&snap);
    let back = <SemioDocumentSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = demo_semio_document_snapshot();
    let text = <SemioDocumentSnapshot as store::os_store::serialization::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioDocumentSnapshot as store::os_store::serialization::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_binary_round_trips() {
    let snap = SemioDocumentSnapshot::default();
    let bytes = <SemioDocumentSnapshot as store::os_store::serialization::ArtifactPack>::encode_pack(&snap);
    let back = <SemioDocumentSnapshot as store::os_store::serialization::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}
