//! 📤️ The wgpu shell's half of the file-open import law, driven by the SAME language-agnostic fixture
//! the kernel and the TypeScript twin drive (`🎠️kernel/🧫️fixtures/📤️file-open-import/🔣️.json`).
//!
//! Three defects are pinned here, all measured on 6118 for ticket 26/09/09/PROCEDURAL-3D-END-TO-END
//! (`📓️wgpu-end-to-end-verification-2026-09-14.md` §C):
//!   1. `queue_host_effects` had no `RequestFileOpen` arm, and the only `requestFileOpen` reader this
//!      shell had read it out of a document MUTATION payload no producer in the repo emits;
//!   2. the wasm32 `request_file_open` was a stub returning an empty list, with a comment claiming
//!      "the browser shell handles `RequestFileOpen` itself" — on this renderer the wasm IS the shell;
//!   3. `dispatch_action` folded `requested_effects` with its OWN partial match ending in `_ => {}`,
//!      so every effect it did not name — `DownloadMediaExport` included — was dropped without even
//!      the funnel's `effect dropped` line. That is why `Export Document…` reached the guest, ran, and
//!      handed its bytes to nobody.
//!
//! A real picker and a real download both end on a browser gesture no headless test can present — so
//! the law drives the action-building contract both targets call, and reads the call sites out of this
//! module's own source so a future edit cannot quietly drop them again.

use super::*;

const WGPU_SHELL_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");

#[derive(serde::Deserialize)]
struct ArgumentChunk {
    payload: String,
    chunk: usize,
    #[serde(rename = "chunkCount")]
    chunk_count: usize,
}

#[derive(serde::Deserialize)]
struct FanOut {
    index: usize,
    total: usize,
}

#[derive(serde::Deserialize)]
struct ArgumentCase {
    id: String,
    name: String,
    chunk: ArgumentChunk,
    #[serde(rename = "fanOut")]
    fan_out: Option<FanOut>,
    arguments: serde_json::Map<String, serde_json::Value>,
}

#[derive(serde::Deserialize)]
struct FileOpenImportFixture {
    #[serde(rename = "importChunkBytes")]
    import_chunk_bytes: usize,
    #[serde(rename = "argumentCases")]
    argument_cases: Vec<ArgumentCase>,
}

fn fixture() -> FileOpenImportFixture {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/📤️file-open-import/🔣️.json")).expect("file-open-import fixture JSON")
}

/// 📥️ One picked file becomes exactly the actions the fixture declares — one per chunk, in order,
/// each carrying the effect contract's own argument envelope.
#[test]
fn one_picked_file_becomes_the_fixture_import_actions() {
    let fixture = fixture();
    assert_eq!(fixture.import_chunk_bytes, semio_framework::kernel::IMPORT_CHUNK_BYTES);
    for case in &fixture.argument_cases {
        let opened = vec![OpenedFile { name: case.name.clone(), contents: case.chunk.payload.clone() }];
        let multiple = case.fan_out.is_some();
        let actions = file_open_import_actions("procedural", "importDocument", opened, multiple);
        assert_eq!(actions.len(), 1, "{}: a payload under the extent is one chunk", case.id);
        let action = &actions[0];
        assert_eq!(action.controller_id, "procedural", "{}", case.id);
        assert_eq!(action.action, "importDocument", "{}", case.id);
        let Some(DslValue::Object(entries)) = action.args.as_ref() else { panic!("{} must carry object args", case.id) };
        for key in [semio_framework::kernel::IMPORT_ARGUMENT_PAYLOAD, semio_framework::kernel::IMPORT_ARGUMENT_NAME, semio_framework::kernel::IMPORT_ARGUMENT_CHUNK, semio_framework::kernel::IMPORT_ARGUMENT_CHUNK_COUNT] {
            assert!(entries.iter().any(|(name, _)| name == key), "{} is missing {key}", case.id);
        }
        assert_eq!(entries.iter().any(|(name, _)| name == semio_framework::kernel::IMPORT_ARGUMENT_INDEX), multiple, "{}: fan-out presence", case.id);
    }
}

/// 🧊️ A file above the extent fans out into several chunks, in order, and no chunk asks the guest for
/// a contiguous block above its own ceiling. The wgpu shell used to send the WHOLE file in one
/// invocation carrying `{json, payload}`.
#[test]
fn a_large_file_fans_out_into_ordered_bounded_chunks() {
    let payload = "s".repeat(semio_framework::kernel::IMPORT_CHUNK_BYTES * 2 + 11);
    let actions = file_open_import_actions("procedural", "importDocument", vec![OpenedFile { name: "big.stl".into(), contents: payload.clone() }], false);
    assert_eq!(actions.len(), 3, "two full chunks and a remainder");
    let mut reassembled = String::new();
    for (position, action) in actions.iter().enumerate() {
        let Some(DslValue::Object(entries)) = action.args.as_ref() else { panic!("object args") };
        let read = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, value)| value.clone()).unwrap_or(DslValue::Null);
        let DslValue::String(chunk_payload) = read(semio_framework::kernel::IMPORT_ARGUMENT_PAYLOAD) else { panic!("payload") };
        assert!(chunk_payload.len() <= semio_framework::kernel::IMPORT_CHUNK_BYTES, "chunk {position} is {} B", chunk_payload.len());
        assert_eq!(read(semio_framework::kernel::IMPORT_ARGUMENT_CHUNK), DslValue::Number(dsl::os_dsl::schema::Number::UInt(position as u64)), "chunk {position} names its position");
        assert_eq!(read(semio_framework::kernel::IMPORT_ARGUMENT_CHUNK_COUNT), DslValue::Number(dsl::os_dsl::schema::Number::UInt(3)), "chunk {position} names its run length");
        assert_eq!(read(semio_framework::kernel::IMPORT_ARGUMENT_NAME), DslValue::String("big.stl".into()), "chunk {position} carries the file name the format is resolved from");
        reassembled.push_str(&chunk_payload);
    }
    assert_eq!(reassembled, payload, "the run must reassemble");
}

/// 🗂️ A multi-file pick fans out per file AND per chunk, each action naming its file's position.
#[test]
fn a_multi_file_pick_fans_out_per_file_with_its_position() {
    let opened = vec![OpenedFile { name: "a.stl".into(), contents: "solid a\n".into() }, OpenedFile { name: "b.stl".into(), contents: "solid b\n".into() }];
    let actions = file_open_import_actions("procedural", "importDocument", opened, true);
    assert_eq!(actions.len(), 2);
    for (position, action) in actions.iter().enumerate() {
        let Some(DslValue::Object(entries)) = action.args.as_ref() else { panic!("object args") };
        let read = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, value)| value.clone()).unwrap_or(DslValue::Null);
        assert_eq!(read(semio_framework::kernel::IMPORT_ARGUMENT_INDEX), DslValue::Number(dsl::os_dsl::schema::Number::UInt(position as u64)));
        assert_eq!(read(semio_framework::kernel::IMPORT_ARGUMENT_TOTAL), DslValue::Number(dsl::os_dsl::schema::Number::UInt(2)));
    }
}

/// 🕳️ A cancelled picker dispatches nothing — never an import of zero files reported as a document.
#[test]
fn a_cancelled_pick_dispatches_nothing() {
    assert!(file_open_import_actions("procedural", "importDocument", Vec::new(), false).is_empty());
}

/// 📤️ The host-effect funnel names the import effect, and the picker it reaches is a real one.
#[test]
fn the_host_effect_funnel_owns_request_file_open() {
    assert!(WGPU_SHELL_SOURCE.contains("Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {"), "queue_host_effects must own RequestFileOpen");
    assert!(WGPU_SHELL_SOURCE.contains(r#""op": "request-file-open""#), "the browser half must ask the page for a real picker");
    assert!(!WGPU_SHELL_SOURCE.contains("fn request_file_open(_accept: &str"), "a stubbed picker means Import Document… opens nothing at all");
}

/// 🕸️ NEITHER browser half may reach for `window`/`document` itself. This shell runs inside the frame
/// Worker that owns the `OffscreenCanvas`, where both are absent — so a `web_sys::window()?` prologue is
/// a SILENT no-op, which is exactly how an export handed its bytes to nobody and an import opened
/// nothing. Both go through the one page-owned door instead.
#[test]
fn the_browser_file_door_never_assumes_a_document_in_the_worker() {
    for needle in [r#""op": "download-media-export""#, r#""op": "request-file-open""#, r#""semioWgpuHostIo""#] {
        assert!(WGPU_SHELL_SOURCE.contains(needle), "the browser file door must route through the page: missing {needle}");
    }
    let door = WGPU_SHELL_SOURCE.split("//#region 📤️FileOpenImport").nth(1).expect("the file-door region");
    let door = door.split("//#endregion 📤️FileOpenImport").next().expect("the file-door region end");
    // 📝️ CODE lines only: this region's own docstring names the defect it closes, and a law that could
    // not tell a comment from a call would forbid saying so.
    let code = door.lines().filter(|line| !line.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n");
    assert!(!code.contains("web_sys::window()"), "the file door must not ask a Worker isolate for a window");
    assert!(!WGPU_SHELL_SOURCE.contains("HtmlAnchorElement"), "the `<a download>` belongs to the page half (`🚪️host-io/🟦️.ts`)");
    assert!(!WGPU_SHELL_SOURCE.contains("create_object_url_with_blob"), "the download Blob belongs to the page half");
}

/// 🗑️ The CRUD-era MUTATION readers are gone. They were this shell's only import/export route and no
/// producer in the repo ever emitted them, so both journeys looked wired and reached nobody.
#[test]
fn no_mutation_payload_reader_shadows_the_effect_funnel() {
    assert!(!WGPU_SHELL_SOURCE.contains(r#"Some("requestFileOpen")"#), "a mutation-payload import reader has no producer");
    assert!(!WGPU_SHELL_SOURCE.contains(r#"Some("downloadMediaExport")"#), "a mutation-payload export reader has no producer");
}

/// 🔗️ Backbone sync attach: file mode must reach a file picker, folder mode a folder picker.
#[test]
fn sync_backbone_selection_opens_matching_path_pickers() {
    assert!(WGPU_SHELL_SOURCE.contains("schedule_sync_path_pick(pick_file_path())"), "file sync must open a file picker on native");
    assert!(WGPU_SHELL_SOURCE.contains("schedule_sync_path_pick(pick_folder())"), "folder sync must open a folder picker on native");
    assert!(WGPU_SHELL_SOURCE.contains(r#""op": "request-native-file-path""#), "wasm file sync must ask the page door for a file path");
    assert!(WGPU_SHELL_SOURCE.contains(r#""op": "request-native-folder-path""#), "wasm folder sync must ask the page door for a folder path");
    assert!(WGPU_SHELL_SOURCE.contains(r#""setSyncDraft""#), "picker answers must land in the sync draft field");
}

/// 🧾️ ONE host-effect funnel for both dispatch paths. `dispatch_action`'s own partial fold ending in
/// `_ => {}` is the defect that silently swallowed every `DownloadMediaExport` an ACTION produced.
#[test]
fn both_dispatch_paths_answer_through_the_one_host_effect_funnel() {
    assert_eq!(WGPU_SHELL_SOURCE.matches("for effect in core::mem::take(&mut result.requested_effects) {").count(), 2, "the action and the command path must share one fold shape");
    assert!(!WGPU_SHELL_SOURCE.contains("for effect in &result.requested_effects {\n            match effect {"), "a second, partial effect fold drops every effect it does not name");
    assert!(WGPU_SHELL_SOURCE.contains("self.queue_host_effects(&action.controller_id.clone(), queued);"), "the action path must feed the funnel");
    assert!(WGPU_SHELL_SOURCE.contains("self.queue_host_effects(&session.app.controller_id.clone(), queued);"), "the command path must feed the funnel");
}
