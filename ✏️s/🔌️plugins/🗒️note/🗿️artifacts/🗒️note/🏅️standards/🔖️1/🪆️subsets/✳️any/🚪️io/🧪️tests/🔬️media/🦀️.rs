
use super::*;
use crate::{NoteImageAsset, NoteTableCell};
use semio_s_artifact_stdio_dwg::{DwgColor, DwgEntity, DwgLayer};

/// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[semio_framework_async_macros::async_test]
async fn imports_dwg_polyline_and_text_into_note_blocks() {
    let drawing = DwgDrawing {
        layers: vec![DwgLayer::default()],
        entities: vec![
            DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], bulges: vec![0.0, 0.5, 0.0] } },
            DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [1.0, 2.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".into() } },
        ],
        extmin: [0.0, 0.0, 0.0],
        extmax: [10.0, 10.0, 0.0],
    };
    let value = note_document_json_from_dwg(&drawing).unwrap();
    let document: NoteSnapshot = dsl::os_pack::from_json_str(&value.to_string()).unwrap();
    assert_eq!(document.schema, crate::NOTE_DOCUMENT_SCHEMA);
    assert_eq!(document.blocks.len(), 2);
    let ink_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Ink { .. })).count();
    let text_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Text { .. })).count();
    assert_eq!(ink_count, 1);
    assert_eq!(text_count, 1);
    if let Some(NoteBlockNode::Ink { points, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Ink { .. })) {
        assert_eq!(points.len(), 4);
    } else {
        panic!("expected ink block");
    }
    if let Some(NoteBlockNode::Text { content, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Text { .. })) {
        let paragraphs = crate::note_block_text(content);
        assert_eq!(paragraphs[0].runs[0].text, "semio");
    } else {
        panic!("expected text block");
    }
}

/// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[semio_framework_async_macros::async_test]
async fn imports_empty_dwg_drawing_as_valid_empty_note_snapshot() {
    let drawing = DwgDrawing::default();
    let value = note_document_json_from_dwg(&drawing).unwrap();
    let document: NoteSnapshot = dsl::os_pack::from_json_str(&value.to_string()).unwrap();
    assert_eq!(document.schema, crate::NOTE_DOCUMENT_SCHEMA);
    assert!(document.blocks.is_empty());
}

/// 🧪️ Real end-to-end proof `note_document_to_svg` goes through `io_dispatch` onto stdio's
/// registered semio/drawing→svg composer (not a hand-rolled SVG string): text content, ink
/// strokes, and the Table catch-all outline all have to survive a real `SemioDrawingSnapshot`
/// build + a real svg-composer round trip to show up in the output. Relocated from the deleted
/// `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[semio_framework_async_macros::async_test]
async fn document_to_svg_dispatches_through_semio_drawing_bridge() {
    let mut document = crate::schema::empty_note_snapshot();
    document.blocks.push(NoteBlockNode::Text {
        content: crate::note_text_child_record("t1", &[NoteTextParagraph { runs: vec![NoteTextRun { text: "hello semio".into(), bold: None, italic: None, underline: None, link: None }] }]),
        id: "t1".into(),
        name: "Text".into(),
        x: 10.0,
        y: 20.0,
        width: 100.0,
        height: 30.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        font_size: 18.0,
        font_weight: "normal".into(),
        align: "left".into(),
    });
    document.blocks.push(NoteBlockNode::Ink {
        id: "i1".into(),
        name: "Ink".into(),
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        points: vec![[0.0, 0.0], [5.0, 5.0]],
        stroke_width: 2.0,
        color: [1.0, 0.0, 0.0, 1.0],
    });
    document.blocks.push(NoteBlockNode::Table {
        id: "tb1".into(),
        name: "Table".into(),
        x: 0.0,
        y: 0.0,
        width: 50.0,
        height: 40.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        columns: vec!["A".into()],
        rows: vec![vec![NoteTableCell { content: String::new() }]],
    });

    let (svg, width, height) = note_document_to_svg(&document).expect("svg export via io_dispatch");
    assert!(svg.starts_with("<svg"), "{svg}");
    assert!(svg.contains("hello semio"), "{svg}");
    assert!(width >= 1024 && height >= 1024);

    // Same pipeline through the JSON-wrapped entry point every io leaf/media handler actually calls.
    let json = serde_json::from_str::<Value>(&dsl::os_pack::to_json_string(&document)).unwrap();
    let (svg_via_json, _w, _h) = note_document_json_to_svg(&json).expect("json svg export via io_dispatch");
    assert_eq!(svg, svg_via_json);
}

/// 🧪️ Real proof an image block's asset bytes flow through `DrawNode::Image` (base64-decoded
/// from the `NoteImageAsset.data` uri) and back out as a data uri on the svg side. Relocated
/// from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[semio_framework_async_macros::async_test]
async fn document_to_svg_embeds_image_asset_bytes_as_data_uri() {
    let mut document = crate::schema::empty_note_snapshot();
    document.assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,AAECAw==".into(), width: Some(4.0), height: Some(4.0) });
    document.blocks.push(NoteBlockNode::Image { id: "im1".into(), name: "Image".into(), x: 0.0, y: 0.0, width: 4.0, height: 4.0, rotation: 0.0, visible: true, locked: false, image_key: "asset-1".into() });

    let (svg, _w, _h) = note_document_to_svg(&document).expect("svg export via io_dispatch");
    assert!(svg.contains("data:image/png;base64,"), "{svg}");
}

/// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[semio_framework_async_macros::async_test]
async fn note_document_to_drawing_snapshot_flattens_visible_blocks_into_one_layer() {
    let mut document = crate::schema::empty_note_snapshot();
    let mut ids = crate::schema::NoteIdOwner::new("io-test", 0);
    document.blocks.push(crate::schema::create_block_by_kind(&mut ids, "text", 5.0, 6.0));
    let mut hidden = crate::schema::create_block_by_kind(&mut ids, "text", 0.0, 0.0);
    if let NoteBlockNode::Text { visible, .. } = &mut hidden {
        *visible = false;
    }
    document.blocks.push(hidden);

    let drawing = note_document_to_drawing_snapshot(&document);
    assert_eq!(drawing.layers.len(), 1);
    let DrawNode::Group { children, .. } = &drawing.layers[0].root else { panic!("expected root Group") };
    assert_eq!(children.len(), 1, "hidden block must not be mapped into a DrawNode");
}
