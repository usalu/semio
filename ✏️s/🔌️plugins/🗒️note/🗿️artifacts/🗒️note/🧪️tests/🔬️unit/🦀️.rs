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

//#region 🚧️TemporaryFixtureRegeneration
/// 🚧️ [DEBUG] One-shot fixture regeneration (ticket 26/09/19): rewrites every committed
/// `🔺️diff/🔣️.json` through `NoteDiff`'s own decode→encode so its `f64` slots carry the canonical
/// `5.0` form instead of the stale `5`. Inert unless `SEMIO_REGENERATE_NOTE_DIFF_FIXTURES` is set.
#[semio_framework_async_macros::async_test]
async fn regenerate_committed_diff_fixtures() {
    if std::env::var("SEMIO_REGENERATE_NOTE_DIFF_FIXTURES").is_err() {
        return;
    }
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.file_name().is_some_and(|name| name == "🔣️.json") && path.parent().is_some_and(|parent| parent.file_name().is_some_and(|name| name == "🔺️diff")) {
                out.push(path);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let mut files = Vec::new();
    walk(&root, &mut files);
    files.sort();
    for path in files {
        let text = std::fs::read_to_string(&path).expect("committed diff reads");
        let decoded: crate::NoteDiff = dsl::os_pack::from_json_str(&text).expect("committed diff decodes");
        let canonical = dsl::os_pack::to_json_string(&decoded);
        if serde_json::from_str::<serde_json::Value>(&canonical).expect("canonical parses") != serde_json::from_str::<serde_json::Value>(&text).expect("committed parses") {
            eprintln!("[DEBUG] regenerating {}", path.display());
            std::fs::write(&path, canonical).expect("committed diff rewrites");
        }
    }
}
//#endregion 🚧️TemporaryFixtureRegeneration

//#region 🔖️GenesisLaw
/// 🌱️ `s.note.note`'s creation authority, exercised through the ONE producer every plugin's
/// `codec.genesis` export runs (`app::artifact_app_genesis_pair`, ticket 26/09/18 slice TC3b). GIS
/// has this law in `📇️native-codecs/🧪️tests`; note is the first package a hub links NO Rust codec
/// for, so for note this producer is not merely the guest's answer — it IS how the document gets
/// created, and nothing else can catch a fault in it before a hub traps on a live creation.
///
/// Measured 2026-09-21: `codec.genesis(s.note.note)` on the freshly built note component trapped
/// with `wasm trap: unreachable executed` during the trusted-catalog bootstrap, i.e. a guest panic.
/// This runs the identical producer natively, where a panic carries its message.
#[semio_framework_async_macros::async_test]
async fn note_genesis_produces_a_complete_zero_history_pair_at_a_server_minted_id() {
    let document_id = "artifact-11223344556677889900aabbccddeeff";
    let pair = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<crate::editor::note::NotePlayApp>>(document_id)
        .await
        .expect("note genesis pair");
    assert!(!pair.pack.is_empty(), "note genesis pack is empty");
    assert!(!pair.spr.is_empty(), "note genesis spr is empty");
    let history = store::os_spr::decode_history(&pair.spr, &store::os_spr::DecodeOptions::default()).await.expect("note genesis history");
    assert_eq!(history.doc_id, document_id);
    assert_eq!(history.schema, NOTE_DOCUMENT_SCHEMA);
    assert!(history.edits.is_empty() && history.transitions.is_empty(), "note genesis carries history");
}
//#endregion 🔖️GenesisLaw

//#region 🔖️ApplyOpsLaw
/// 🧩️ The NONEMPTY `codec.apply-ops` batch. Ticket 26/09/18 slice TC3d §2(b) rebuilt
/// `artifact_app_apply_ops` around a bounded close cursor because it used to build an
/// `ArtifactStore`, print from it and let it fall out of scope — and that store's `Drop` asserts an
/// exact terminal-empty shallow-shell witness, which on a `panic = "abort"` wasm32 guest is an
/// `unreachable` that kills the instance. TC3d's own law drove only the EMPTY batch, which returns
/// before a store is ever built (TC3d §6c), so the cursor it added was type-checked and never run.
/// This law is the one that runs it: a real `rename-note` op, encoded exactly the way the guest
/// receives it (`os_spr::encode_ops_vec` over `OpBinary::encode_op` blobs).
#[semio_framework_async_macros::async_test]
async fn note_apply_ops_reduces_a_nonempty_batch_and_closes_its_store() {
    let document_id = "artifact-11223344556677889900aabbccddeeff";
    let baseline = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<crate::editor::note::NotePlayApp>>(document_id).await.expect("note genesis pair");
    let op = <crate::schema::mutations::NoteMutation as protocol::OpBinary>::encode_op(&crate::schema::mutations::rename_note(Some("TC3e".into()))).expect("encode rename-note");
    let ops = store::os_spr::encode_ops_vec(&[op]);
    let applied = semio_framework_plugin::artifact_app_apply_ops::<semio_framework_plugin::EditorApp<crate::editor::note::NotePlayApp>>(&baseline.pack, &baseline.spr, &ops).await.expect("note apply-ops with a nonempty batch");
    assert!(!applied.pack.is_empty() && !applied.spr.is_empty(), "apply-ops produced an empty pair");
    let history = store::os_spr::decode_history(&applied.spr, &store::os_spr::DecodeOptions::default()).await.expect("applied history");
    assert_eq!(history.doc_id, document_id);
    assert_eq!(history.schema, NOTE_DOCUMENT_SCHEMA);
    assert_eq!(history.edits.len(), 1, "one op in the batch must land exactly one edit, got {:?}", history.edits.len());
}
//#endregion 🔖️ApplyOpsLaw

#[test]
fn temporary_regenerate_mutation_fixtures() {
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.file_name().is_some_and(|name| name == "🔣️.json") {
                out.push(path);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets");
    let mut files = Vec::new();
    walk(&root, &mut files);
    for path in files {
        let text = std::fs::read_to_string(&path).expect("fixture reads");
        let parent = path.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let canonical = if parent == "🔺️diff" {
            dsl::os_pack::from_json_str::<NoteDiff>(&text).ok().map(|value| dsl::os_pack::to_json_string(&value))
        } else if parent == "🦠️mutation" {
            dsl::os_pack::from_json_str::<crate::schema::mutations::NoteMutation>(&text).ok().map(|value| dsl::os_pack::to_json_string(&value))
        } else if parent == "⬅️before" || parent == "➡️after" {
            dsl::os_pack::from_json_str::<NoteSnapshot>(&text).ok().map(|value| dsl::os_pack::to_json_string(&value))
        } else {
            None
        };
        let Some(canonical) = canonical else { continue };
        let reparsed: serde_json::Value = serde_json::from_str(&canonical).expect("canonical reparses");
        let original: serde_json::Value = serde_json::from_str(&text).expect("original reparses");
        if reparsed != original {
            std::fs::write(&path, format!("{}\n", serde_json::to_string_pretty(&reparsed).expect("pretty"))).expect("fixture writes");
            println!("[DEBUG] rewrote {}", path.display());
        }
    }
}
