use super::*;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_default_subset() {
    let snap = SemioSnapshot::default();
    let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips_default_subset() {
    let snap = SemioSnapshot::default();
    let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ Real delegation, non-default nested payload: the demo (flow-wrapped) snapshot's
/// text/binary round trips must both hold, proving this isn't merely round-tripping an
/// all-zero stub.
#[semio_framework_async_macros::async_test]
async fn pack_and_dsl_round_trip_the_demo_snapshot() {
    let snap = demo_semio_snapshot();
    let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snap);
    assert_eq!(<SemioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snap);
    let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    assert_eq!(<SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snap);
}

/// 🧪️ Every one of the 18 subset tags real-round-trips through both facets — proves the
/// dispatch tables (text AND binary) are wired correctly for every wrapped subset, not just
/// the one exercised by [`demo_semio_snapshot`].
#[semio_framework_async_macros::async_test]
async fn all_eighteen_subset_tags_round_trip_text_and_binary() {
    let subsets: Vec<SemioSubsetSnapshot> = vec![
        SemioSubsetSnapshot::Brep(Default::default()),
        SemioSubsetSnapshot::Mesh(Default::default()),
        SemioSubsetSnapshot::Model(Default::default()),
        SemioSubsetSnapshot::Value(Default::default()),
        SemioSubsetSnapshot::Document(Default::default()),
        SemioSubsetSnapshot::Cad(Default::default()),
        SemioSubsetSnapshot::Drawing(Default::default()),
        SemioSubsetSnapshot::Image(Default::default()),
        SemioSubsetSnapshot::Video(Default::default()),
        SemioSubsetSnapshot::Audio(Default::default()),
        SemioSubsetSnapshot::Animation(Default::default()),
        SemioSubsetSnapshot::Presentation(Default::default()),
        SemioSubsetSnapshot::Flow(Default::default()),
        SemioSubsetSnapshot::Text(Default::default()),
        SemioSubsetSnapshot::Table(Default::default()),
        SemioSubsetSnapshot::Graph(Default::default()),
        SemioSubsetSnapshot::Object(Default::default()),
        SemioSubsetSnapshot::Kit(Default::default()),
    ];
    for subset in subsets {
        let snap = SemioSnapshot { schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset };
        let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert_eq!(<SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap_or_else(|e| panic!("parse_dsl failed for {:?}: {e}", subset_tag(&snap.subset))), snap);
        let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snap);
        assert_eq!(<SemioSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap_or_else(|e| panic!("decode_pack failed for {:?}: {e}", subset_tag(&snap.subset))), snap);
    }
}
