
use super::*;
use crate::{NOTE_DOCUMENT_SCHEMA, NoteBlockNode, NoteImageAsset, NoteTableCell, NoteTextParagraph, NoteTextRun};
use std::collections::BTreeMap;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::io::snapshot::text::SEMIO_NOTE_EXAMPLE_TEXT).expect("parse semio example");
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_representative_document() {
    let mut assets = BTreeMap::new();
    assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into(), width: Some(10.0), height: Some(20.0) });
    let document = NoteSnapshot {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "doc-1".into(),
        title: Some("Representative \"Doc\"".into()),
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: None,
        grid_opacity: Some(0.35),
        snap_enabled: None,
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: None,
        assets,
        linked_artifact: None,
        blocks: vec![
            NoteBlockNode::Text {
                content: crate::note_text_child_record("text-1", &[NoteTextParagraph { runs: vec![NoteTextRun { text: "plain".into(), bold: None, italic: None, underline: None, link: None }] }]),
                id: "text-1".into(),
                name: "Text".into(),
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 80.0,
                rotation: 0.0,
                visible: true,
                locked: false,
                font_size: 16.0,
                font_weight: "bold".into(),
                align: "center".into(),
            },
            NoteBlockNode::Table {
                id: "table-1".into(),
                name: "Table".into(),
                x: 20.0,
                y: 20.0,
                width: 320.0,
                height: 120.0,
                rotation: 0.0,
                visible: true,
                locked: false,
                columns: vec!["A".into(), "B".into()],
                rows: vec![vec![NoteTableCell { content: "a1".into() }, NoteTableCell { content: "b1".into() }]],
            },
        ],
    };
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `NoteMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip laws (same pattern as `mathematical_pack`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::io::mutations::text::NoteMutation;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, ArtifactStore, create_document_envelope};

    let initial = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::io::snapshot::text::SEMIO_NOTE_EXAMPLE_TEXT).expect("parse semio example");
    let envelope = create_document_envelope::<NoteSnapshot, NoteMutation>(NOTE_DOCUMENT_SCHEMA, "note-command-envelope-demo", initial, None);
    let mut store = ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::schema::mutations::change_grid_visible(Some(false))], description: None }).await.expect("apply");
    let edit: &Edit<NoteMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<NoteSnapshot, NoteMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
