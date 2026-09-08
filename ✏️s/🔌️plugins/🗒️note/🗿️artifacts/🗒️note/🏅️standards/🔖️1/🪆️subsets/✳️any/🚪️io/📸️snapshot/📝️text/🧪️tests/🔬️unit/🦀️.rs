
use super::*;
use crate::{NOTE_DOCUMENT_SCHEMA, NoteBlockNode, NoteImageAsset, NoteTableCell, NoteTextParagraph, NoteTextRun};
use std::collections::BTreeMap;

#[semio_framework_async_macros::async_test]
async fn semio_example_dsl_round_trips() {
    let document = parse_dsl(SEMIO_NOTE_EXAMPLE_TEXT).expect("parse semio example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn note_dsl_round_trips_representative_document() {
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
                content: crate::note_text_child_record(
                    "text-1",
                    &[NoteTextParagraph {
                        runs: vec![
                            NoteTextRun { text: "plain".into(), bold: None, italic: None, underline: None, link: None },
                            NoteTextRun { text: "bold+link".into(), bold: Some(true), italic: Some(false), underline: Some(true), link: Some("https://semio.io".into()) },
                        ],
                    }],
                ),
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
            NoteBlockNode::Image { id: "image-1".into(), name: "Image".into(), x: 10.0, y: 10.0, width: 240.0, height: 160.0, rotation: 15.0, visible: false, locked: true, image_key: "asset-1".into() },
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
                rows: vec![vec![NoteTableCell { content: "a1".into() }, NoteTableCell { content: "b1".into() }], vec![NoteTableCell { content: "a2".into() }, NoteTableCell { content: "with \"quotes\" and \\ backslash".into() }]],
            },
            NoteBlockNode::Math { id: "math-1".into(), name: "Math".into(), x: 30.0, y: 30.0, width: 200.0, height: 80.0, rotation: 0.0, visible: true, locked: false, tex: "\\int_0^1 x\\,dx".into(), display_mode: true },
            NoteBlockNode::Ink {
                id: "stroke-1".into(),
                name: "Ink".into(),
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
                rotation: 0.0,
                visible: true,
                locked: false,
                points: (0..20).map(|i| [i as f64, (i * 2) as f64]).collect(),
                stroke_width: 2.5,
                color: [0.1, 0.2, 0.3, 1.0],
            },
            NoteBlockNode::Group {
                id: "group-1".into(),
                name: "Group".into(),
                x: 40.0,
                y: 40.0,
                width: 280.0,
                height: 120.0,
                rotation: 0.0,
                visible: true,
                locked: false,
                children: vec![NoteBlockNode::Text {
                    content: crate::note_text_child_record("child-text-1", &[]),
                    id: "child-text-1".into(),
                    name: "Child".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 40.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    font_size: 12.0,
                    font_weight: "normal".into(),
                    align: "left".into(),
                }],
            },
        ],
    };
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
