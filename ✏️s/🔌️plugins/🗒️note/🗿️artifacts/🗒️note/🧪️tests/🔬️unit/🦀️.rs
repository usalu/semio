use super::*;

/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` and `NOTE_DOCUMENT_SCHEMA` are deliberately the
/// same string here (note has no separate "fixture" store schema, unlike shooting) — pinned so a
/// future edit can't silently diverge them without noticing.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_schema_matches_the_store_schema() {
    assert_eq!(artifact_kind().schema, NOTE_DOCUMENT_SCHEMA);
}

//#region 🔖️TextBridgeTests
/// 🧪️ Real round trip for the paragraph <-> `SemioTextSnapshot` converter: multiple paragraphs,
/// multiple runs, every mark (bold/italic/link) the converter maps.
#[semio_framework_async_macros::async_test]
async fn text_bridge_round_trips_paragraphs_through_semio_text_snapshot() {
    let paragraphs = vec![
        NoteTextParagraph { runs: vec![NoteTextRun { text: "plain ".into(), bold: None, italic: None, underline: None, link: None }, NoteTextRun { text: "bold".into(), bold: Some(true), italic: None, underline: None, link: None }] },
        NoteTextParagraph { runs: vec![NoteTextRun { text: "second para".into(), bold: None, italic: Some(true), underline: None, link: Some("https://semio.tech".into()) }] },
    ];
    let snapshot = text_snapshot_from_paragraphs(&paragraphs);
    assert_eq!(snapshot.runs.len(), 4, "2 content runs + 1 paragraph separator, but bold/link split across marks not runs: {snapshot:?}");
    let restored = paragraphs_from_text_snapshot(&snapshot);
    assert_eq!(restored, paragraphs);
}

/// 🧪️ Documents the one honest lossy edge case: an empty paragraph list and a single paragraph
/// with zero runs both flatten to zero runs, and [`paragraphs_from_text_snapshot`] always emits a
/// trailing paragraph for whatever ran since the last separator (even if that's none) — so both
/// restore as the SAME single empty paragraph, never as an empty paragraph list.
#[semio_framework_async_macros::async_test]
async fn text_bridge_collapses_empty_paragraph_shapes() {
    let one_empty_paragraph = vec![NoteTextParagraph { runs: Vec::new() }];
    assert_eq!(paragraphs_from_text_snapshot(&text_snapshot_from_paragraphs(&[])), one_empty_paragraph);
    assert_eq!(paragraphs_from_text_snapshot(&text_snapshot_from_paragraphs(&one_empty_paragraph)), one_empty_paragraph);
}

/// 🧪️ `underline` has no equivalent mark in stdio's text subset and is honestly dropped, never
/// fabricated back on the way out.
#[semio_framework_async_macros::async_test]
async fn text_bridge_drops_underline_honestly() {
    let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: "u".into(), bold: None, italic: None, underline: Some(true), link: None }] }];
    let restored = paragraphs_from_text_snapshot(&text_snapshot_from_paragraphs(&paragraphs));
    assert_eq!(restored[0].runs[0].underline, None);
}

/// 🧪️ Each minted text-child record owns its paragraphs, and two distinct block ids never
/// collide even with identical paragraph content.
#[semio_framework_async_macros::async_test]
async fn text_child_records_are_owned_and_block_ids_never_collide() {
    let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: "hi".into(), bold: None, italic: None, underline: None, link: None }] }];
    let a = note_text_child_record("block-a", &paragraphs);
    let b = note_text_child_record("block-b", &paragraphs);
    assert_ne!(a.handle.child_id, b.handle.child_id, "identical content on distinct block ids must not share a child slot");
    assert_eq!(note_block_text(&a), paragraphs);
    assert_eq!(note_block_text(&b), paragraphs);
}

/// 🧪️ An explicitly empty durable child record reads back an empty paragraph list.
#[semio_framework_async_macros::async_test]
async fn note_block_text_reads_an_empty_owned_record() {
    let handle = note_text_child_handle("empty-owned-record", &[]);
    assert_eq!(note_block_text(&handle), Vec::<NoteTextParagraph>::new());
}
//#endregion 🔖️TextBridgeTests

#[semio_framework_async_macros::async_test]
async fn note_document_round_trips_assets_and_grid_settings() {
    let mut document = NoteSnapshot {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        title: None,
        blocks: Vec::new(),
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: Some(4.0),
        grid_opacity: Some(0.35),
        snap_enabled: Some(false),
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
        assets: BTreeMap::new(),
        linked_artifact: None,
    };
    document.assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc".into(), width: Some(10.0), height: Some(20.0) });
    document.grid_subdivisions = Some(6.0);
    document.grid_opacity = Some(0.5);
    let json_text = dsl::os_pack::to_json_string(&document);
    let parsed: NoteSnapshot = dsl::os_pack::from_json_str(&json_text).unwrap();
    assert_eq!(parsed.assets.get("asset-1").unwrap().mime, "image/png");
    assert_eq!(parsed.grid_subdivisions, Some(6.0));
    assert_eq!(parsed.grid_opacity, Some(0.5));
}
