
use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::{STDIO_SEMIOTEXT_DOCUMENT_SCHEMA, SemioTextSnapshot};

fn text_child(owner: Option<std::sync::Arc<SemioTextSnapshot>>, child_id: String) -> crate::NoteTextChild {
    let target = store::os_io::ArtifactRef { artifact_id: "note-text-artifact".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "text".into() } };
    let handle = match owner {
        Some(owner) => store::ArtifactChild::new(child_id, target).with_local_owner(owner),
        None => store::ArtifactChild::new(child_id, target),
    };
    crate::NoteTextChild { handle, paragraphs: Vec::new() }
}

fn materialize(source: &crate::NoteTextChild) -> store::ArtifactChild<SemioTextSnapshot> {
    let mut cursor = NoteTextChildMaterializationCursor::new(source);
    for _ in 0..64 {
        if let Some(child) = cursor.step(source).expect("bounded child metadata copy") {
            return child;
        }
    }
    panic!("Note text-child materializer did not terminate")
}

#[semio_framework_async_macros::async_test]
async fn text_child_materialization_preserves_present_typed_owner() {
    let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
    let source = text_child(Some(owner.clone()), "child-present".into());
    let materialized = materialize(&source);
    let retained = materialized.local_owner::<SemioTextSnapshot>().expect("typed owner retained");
    assert!(std::sync::Arc::ptr_eq(&owner, &retained));
    assert_eq!(materialized.child_id, source.handle.child_id);
    assert_eq!(materialized.target, source.handle.target);
}

#[semio_framework_async_macros::async_test]
async fn text_child_materialization_preserves_absent_owner() {
    let source = text_child(None, "child-absent".into());
    let materialized = materialize(&source);
    assert!(materialized.local_owner::<SemioTextSnapshot>().is_none());
    assert_eq!(materialized.child_id, source.handle.child_id);
    assert_eq!(materialized.target, source.handle.target);
}

#[semio_framework_async_macros::async_test]
async fn text_child_materialization_cancellation_retires_partial_metadata() {
    let source = text_child(None, "x".repeat(NOTE_MATERIALIZATION_STRING_CHUNK_BYTES * 3));
    let mut cursor = NoteTextChildMaterializationCursor::new(&source);
    assert!(cursor.step(&source).expect("first bounded metadata chunk").is_none());
    cursor.begin_close();
    for _ in 0..8 {
        if cursor.close_step(1, NOTE_MATERIALIZATION_STRING_CHUNK_BYTES) == InteractiveJobCloseStep::Complete {
            break;
        }
    }
    assert!(cursor.terminal_is_empty());
}

fn hostile_snapshot(owner: std::sync::Arc<SemioTextSnapshot>) -> NoteSnapshot {
    let text = crate::NoteBlockNode::Text {
        id: "text-1".into(),
        name: "Text".into(),
        x: 1.0,
        y: 2.0,
        width: 3.0,
        height: 4.0,
        rotation: 5.0,
        visible: true,
        locked: false,
        content: crate::NoteTextChild {
            handle: text_child(Some(owner), "child-root".into()).handle,
            paragraphs: vec![crate::NoteTextParagraph { runs: vec![crate::NoteTextRun { text: "root text".into(), bold: Some(true), italic: Some(false), underline: None, link: Some("https://semio.tech".into()) }] }],
        },
        font_size: 16.0,
        font_weight: "600".into(),
        align: "center".into(),
    };
    let table = crate::NoteBlockNode::Table {
        id: "table-1".into(),
        name: "Table".into(),
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        columns: vec!["a".into(), "b".into()],
        rows: vec![vec![crate::NoteTableCell { content: "one".into() }, crate::NoteTableCell { content: "two".into() }]],
    };
    let group = crate::NoteBlockNode::Group {
        id: "group-1".into(),
        name: "Group".into(),
        x: 0.0,
        y: 0.0,
        width: 20.0,
        height: 20.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        children: vec![
            crate::NoteBlockNode::Image { id: "image-1".into(), name: "Image".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0, visible: true, locked: false, image_key: "asset-1".into() },
            crate::NoteBlockNode::Math { id: "math-1".into(), name: "Math".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0, visible: true, locked: false, tex: "x^2".into(), display_mode: true },
            crate::NoteBlockNode::Ink {
                id: "ink-1".into(),
                name: "Ink".into(),
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
                rotation: 0.0,
                visible: true,
                locked: false,
                points: vec![[0.0, 1.0], [2.0, 3.0]],
                stroke_width: 2.0,
                color: [0.1, 0.2, 0.3, 1.0],
            },
        ],
    };
    let mut assets = std::collections::BTreeMap::new();
    assets.insert("asset-1".into(), crate::NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc".into(), width: Some(10.0), height: Some(20.0) });
    NoteSnapshot {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "root".into(),
        title: Some("Hostile root".into()),
        blocks: vec![text, table, group],
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: Some(4.0),
        grid_opacity: Some(0.35),
        snap_enabled: Some(true),
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
        assets,
        linked_artifact: Some(store::ArtifactLink {
            target: store::os_io::ArtifactRef { artifact_id: "linked".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.any".into(), standard: "1".into(), subset: "any".into() } },
            pin: store::LinkPin::Snapshot { blob: store::BlobRef { hash: "blob-hash".into(), size: 9, media_type: "application/octet-stream".into() } },
            role: "any".into(),
        }),
    }
}

#[semio_framework_async_macros::async_test]
async fn snapshot_materialization_copies_every_nested_owner_and_preserves_typed_text_arc() {
    let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
    let source = hostile_snapshot(owner.clone());
    let operation = semio_framework_job::OperationId(31);
    let generation = semio_framework_job::Generation(7);
    let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
    let materialized = (0..4_096).find_map(|_| cursor.step(operation, generation, &source).expect("bounded nested snapshot copy")).expect("nested snapshot cursor terminates");
    assert_eq!(materialized, source);
    assert_eq!(cursor.progress().phase, "complete");
    let crate::NoteBlockNode::Text { content, .. } = &materialized.blocks[0] else { panic!("first hostile block must remain text") };
    let retained = content.handle.local_owner::<SemioTextSnapshot>().expect("nested typed owner retained");
    assert!(std::sync::Arc::ptr_eq(&owner, &retained));
}

#[semio_framework_async_macros::async_test]
async fn snapshot_materialization_preserves_absent_typed_owner() {
    let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
    let mut source = hostile_snapshot(owner);
    let crate::NoteBlockNode::Text { content, .. } = &mut source.blocks[0] else { panic!("first hostile block must remain text") };
    *content = text_child(None, "child-wire-only".into());
    let operation = semio_framework_job::OperationId(32);
    let generation = semio_framework_job::Generation(8);
    let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
    let materialized = (0..4_096).find_map(|_| cursor.step(operation, generation, &source).expect("bounded wire-only snapshot copy")).expect("wire-only snapshot cursor terminates");
    let crate::NoteBlockNode::Text { content, .. } = &materialized.blocks[0] else { panic!("first hostile block must remain text") };
    assert!(content.handle.local_owner::<SemioTextSnapshot>().is_none());
}

#[semio_framework_async_macros::async_test]
async fn snapshot_materialization_cancellation_during_nested_metadata_reaches_terminal_emptiness() {
    let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
    let mut source = hostile_snapshot(owner);
    let crate::NoteBlockNode::Text { content, .. } = &mut source.blocks[0] else { panic!("first hostile block must remain text") };
    content.handle.child_id = "metadata".repeat(NOTE_MATERIALIZATION_STRING_CHUNK_BYTES);
    let operation = semio_framework_job::OperationId(33);
    let generation = semio_framework_job::Generation(9);
    let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
    for _ in 0..12 {
        assert!(cursor.step(operation, generation, &source).expect("bounded metadata advance").is_none());
    }
    cursor.begin_close();
    for _ in 0..32_768 {
        if cursor.close_step(1, NOTE_MATERIALIZATION_STRING_CHUNK_BYTES) == InteractiveJobCloseStep::Complete {
            break;
        }
    }
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn snapshot_materialization_rejects_stale_operation_authority_and_retires() {
    let owner = std::sync::Arc::new(SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: Vec::new() });
    let source = hostile_snapshot(owner);
    let operation = semio_framework_job::OperationId(34);
    let generation = semio_framework_job::Generation(10);
    let mut cursor = NoteSnapshotMaterializationCursor::new(operation, generation, &source);
    assert!(cursor.step(semio_framework_job::OperationId(35), generation, &source).is_err());
    cursor.begin_close();
    for _ in 0..32_768 {
        if cursor.close_step(1, NOTE_MATERIALIZATION_STRING_CHUNK_BYTES) == InteractiveJobCloseStep::Complete {
            break;
        }
    }
    assert!(cursor.terminal_is_empty());
}

#[test]
fn root_scalar_preflight_admits_only_exact_valid_document_mutations() {
    let factory = NoteRootScalarPreparationFactory;
    let visible = crate::op::NoteMutation::ChangeGridVisible(crate::schema::mutations::ChangeGridVisible { new_visible: Some(false) });
    let spacing = crate::op::NoteMutation::ChangeGridSpacing(crate::schema::mutations::ChangeGridSpacing { new_spacing: Some(16.0) });
    let invalid_spacing = crate::op::NoteMutation::ChangeGridSpacing(crate::schema::mutations::ChangeGridSpacing { new_spacing: Some(f64::INFINITY) });
    let foreign = crate::op::NoteMutation::RenameNote(crate::schema::mutations::RenameNote { new_title: Some("foreign".into()) });
    assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &visible, None, store::HistoryLane::Document).is_ok());
    assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &spacing, None, store::HistoryLane::Document).is_ok());
    assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &invalid_spacing, None, store::HistoryLane::Document).is_err());
    assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &visible, None, store::HistoryLane::Interaction).is_err());
    assert!(store::ArtifactStoreOneItemPreparationFactory::preflight(&factory, &foreign, None, store::HistoryLane::Document).is_err());
}
