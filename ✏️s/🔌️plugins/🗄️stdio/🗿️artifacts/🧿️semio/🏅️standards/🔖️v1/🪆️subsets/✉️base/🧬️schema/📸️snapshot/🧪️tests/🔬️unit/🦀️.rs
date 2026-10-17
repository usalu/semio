#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
use crate::schema::snapshot::subset_tag;
use crate::schema::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_default_subset() {
    let snap = SemioSnapshot::default();
    let bytes = <SemioSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    let back = <SemioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips_default_subset() {
    let snap = SemioSnapshot::default();
    let text = <SemioSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ Real delegation, non-default nested payload: the demo (flow-wrapped) snapshot's
/// text/binary round trips must both hold, proving this isn't merely round-tripping an
/// all-zero stub.
#[semio_framework_async_macros::async_test]
async fn pack_and_dsl_round_trip_the_demo_snapshot() {
    let snap = demo_semio_snapshot();
    let bytes = <SemioSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    assert_eq!(<SemioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode"), snap);
    let text = <SemioSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
    assert_eq!(<SemioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("parse"), snap);
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
        let text = <SemioSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
        assert_eq!(<SemioSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).unwrap_or_else(|e| panic!("parse_dsl failed for {:?}: {e}", subset_tag(&snap.subset))), snap);
        let bytes = <SemioSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
        assert_eq!(<SemioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).unwrap_or_else(|e| panic!("decode_pack failed for {:?}: {e}", subset_tag(&snap.subset))), snap);
    }
}
