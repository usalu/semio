use super::*;
use crate::NoteBlockNode;

/// 🧪️ `linked_artifact` (the new `R:any` forward reference slot) and a text block's composed
/// `content` child handle must both survive the hand-rolled text/binary codecs — codec
/// completeness is not caught by `cargo check`, only a real round trip proves it.
#[semio_framework_async_macros::async_test]
async fn linked_artifact_and_text_content_round_trip_through_text_and_binary() {
    let mut snapshot = NoteSnapshot::default();
    snapshot.id = "doc-composed".into();
    snapshot.linked_artifact = Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("doc-2!s.writer.writer@1/any").expect("valid link ref uri"), pin: store::LinkPin::Head, role: "any".into() });
    snapshot.blocks.push(NoteBlockNode::Text {
        id: "text-1".into(),
        name: "Text".into(),
        x: 0.0,
        y: 0.0,
        width: 200.0,
        height: 80.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        content: crate::note_text_child_handle("text-1", &[]),
        font_size: 16.0,
        font_weight: "normal".into(),
        align: "left".into(),
    });

    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let from_text = <NoteSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse round-tripped text");
    assert_eq!(from_text, snapshot);

    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let from_binary = <NoteSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode round-tripped binary");
    assert_eq!(from_binary, snapshot);
}

#[semio_framework_async_macros::async_test]
async fn absent_linked_artifact_round_trips_as_none() {
    let snapshot = NoteSnapshot::default();
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    assert_eq!(<NoteSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snapshot);
    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    assert_eq!(<NoteSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snapshot);
}
