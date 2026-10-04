//! 🧪️ Laws of the wgpu shell's remembered folder bindings (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, W2-C
//! follow-up 5): the shared `🧫️local-folder-bindings` corpus (event log, reconnect offer, folder name, copy), the native
//! lifecycle (attach → edit → restart → reattach restores rows and head; detach → restart → nothing), and the accessible
//! "Reconnect folder" band a failed reattach leaves — React's browser band, which a browser always shows — and the
//! browser build's folder door over the dev host's backbone route (shared corpus `🧫️wgpu-backbone-folder-door`).

use super::*;
use semio_framework::kernel::{HistoryEntry, HistoryPatch};

//#region 🧰️Harness
fn corpus() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/📎️local-folder-bindings/🔣️.json")).expect("the shared local-folder corpus parses")
}

fn binding_of(value: &Value) -> LocalFolderBinding {
    serde_json::from_value(value.clone()).expect("a corpus binding")
}

fn bindings_of(value: &Value) -> Vec<LocalFolderBinding> {
    value.as_array().expect("bindings").iter().map(binding_of).collect()
}

fn mutation_of(value: &Value) -> LocalFoldersConfigMutation {
    local_folders_mutation_of_value(value).unwrap_or_else(|| panic!("a corpus mutation: {value}"))
}

/// 🧪️ A fresh shell on the host fixture app, as a process boot would make it, sharing the law's preference slot.
fn booted_shell(document_id: Option<&str>) -> ShellState {
    let mut shell = panel_anchor_model_tests::host_test_shell();
    if let Some(document_id) = document_id {
        let session = shell.session.clone().expect("the host fixture holds a session");
        let key = (session.plugin_id.clone(), session.instance_id);
        let request = shell.app_document_identities.begin(key.clone()).expect("an identity request");
        assert!(shell.app_document_identities.accept(&key, request, session.instance_id, Some(document_id.to_string())));
    }
    shell.ensure_local_folder_bindings();
    shell
}

fn session_binding(shell: &ShellState, document_id: &str, path: &str) -> LocalFolderBinding {
    let session = shell.session.as_ref().expect("session");
    LocalFolderBinding { document_id: document_id.into(), plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), folder: LocalFolderRef::Path { path: path.into() } }
}

fn scratch_folder(name: &str) -> std::path::PathBuf {
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    let dir = root.join(format!("wgpu-local-folders-{name}-{}-{:?}", std::process::id(), std::thread::current().id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch folder");
    dir
}

thread_local! {
    static LOADED: std::cell::RefCell<Vec<protocol::DocumentArchivePack>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn history_row(seq: u64) -> HistoryEntry {
    HistoryEntry { seq, edit_id: Some(format!("e-{seq}")), action_id: "apply".into(), label: LocalizedLabel::native("Apply", "Anwenden"), kind: "mutation".into(), applied: true, ..Default::default() }
}

fn program_archive() -> protocol::DocumentArchivePack {
    protocol::DocumentArchivePack { parent_pack: vec![7, 7, 7], parent_spr: vec![8], members: Vec::new() }
}

/// 🎬️ A program whose document doors answer from the archive the law restored: it records each loaded archive, hands
/// out its own archive `[7, 7, 7] / [8]`, and answers the history the archive holds — two rows, head `cp-2`.
fn restored_program() -> crate::program_bridge::ProgramFixtureDocument {
    crate::program_bridge::ProgramFixtureDocument {
        load: |_, archive| {
            LOADED.with(|loaded| loaded.borrow_mut().push(archive.clone()));
            Ok(())
        },
        archive: |_| Ok(program_archive()),
        history: |_| Ok(HistoryPatch { cursor: 2, upserts: vec![history_row(2), history_row(1)], current_checkpoint_id: Some("cp-2".into()), ..Default::default() }),
    }
}
//#endregion 🧰️Harness

//#region 📚️Corpus
/// ⚖️ LAW (shared with React's `📎️local-folders`): the corpus's commits record exactly the changing mutations and fold to
/// its bindings, ending in its stored log; every stored log replays to its bindings (a log that does not read whole
/// reattaches nothing); every offer, folder name and line of copy is the corpus's in both locales.
#[test]
fn the_shared_local_folder_corpus_holds_on_wgpu() {
    let corpus = corpus();
    assert_eq!(corpus["schema"].as_str(), Some(LOCAL_FOLDERS_CONFIG_SCHEMA));
    let mut raw: Option<String> = None;
    for step in corpus["commits"]["steps"].as_array().expect("steps") {
        let name = step["name"].as_str().expect("name");
        let next = local_folders_log_after(raw.as_deref(), &mutation_of(&step["mutation"]));
        assert_eq!(next.is_some(), step["recorded"].as_bool().expect("recorded"), "{name}");
        if next.is_some() {
            raw = next;
        }
        assert_eq!(replay_local_folder_events(&local_folder_events_of_log(raw.as_deref())).bindings, bindings_of(&step["bindings"]), "{name}");
    }
    assert_eq!(serde_json::from_str::<Value>(raw.as_deref().expect("a stored log")).expect("JSON"), corpus["commits"]["log"], "the stored log is React's `{{version, events}}` value");
    for log in corpus["logs"].as_array().expect("logs") {
        assert_eq!(replay_local_folder_events(&local_folder_events_of_log(log["raw"].as_str())).bindings, bindings_of(&log["bindings"]), "{}", log["name"]);
    }
    for offer in corpus["offers"].as_array().expect("offers") {
        let bindings = LocalFolderBindings { bindings: bindings_of(&offer["bindings"]) };
        let identity = offer["identity"].as_object().map(|identity| LocalFolderIdentity {
            document_id: identity["documentId"].as_str().expect("documentId").into(),
            plugin_id: identity["pluginId"].as_str().expect("pluginId").into(),
            app_id: identity["appId"].as_str().expect("appId").into(),
        });
        let expected = (!offer["offer"].is_null()).then(|| binding_of(&offer["offer"]));
        assert_eq!(local_folder_reconnect_offer(&bindings, identity.as_ref(), offer["attached"].as_str()), expected, "{}", offer["name"]);
    }
    for pair in corpus["names"].as_array().expect("names") {
        assert_eq!(local_folder_name(pair[0].as_str().expect("path")), pair[1].as_str().expect("name"));
    }
    let folder = corpus["texts"]["folder"].as_str().expect("folder");
    for (locale, tongue) in [(Locale::En, "en"), (Locale::De, "de")] {
        for (text, key) in [(LocalFolderText::Label, "label"), (LocalFolderText::Message, "message"), (LocalFolderText::Attach, "attach"), (LocalFolderText::Forget, "forget")] {
            assert_eq!(local_folder_text(text, folder, locale), corpus["texts"][tongue][key].as_str().expect("text"), "{tongue}.{key}");
        }
    }
}

/// ⚖️ LAW: the payload fixtures W2-B committed for `📎️attach-local-folder` and `✂️detach-local-folder` read as exactly the
/// mutation the config crate decodes them to.
#[test]
fn the_config_payload_fixtures_read_as_their_mutations() {
    for fixture in [
        include_str!("../../../../../../../🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧫️fixtures/📎️remembers-the-folder-beside-another-document/🦠️mutation/🔣️.json"),
        include_str!("../../../../../../../🎚️config/🧬️schema/🧬️mutations/✂️detach-local-folder/🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/🦠️mutation/🔣️.json"),
    ] {
        let value: Value = serde_json::from_str(fixture).expect("JSON");
        let decoded = semio_framework_os_config::opening_config::mutations::decode_local_folders_config_mutation_json(fixture).expect("the config crate decodes it");
        assert_eq!(local_folders_mutation_of_value(&value), Some(decoded));
    }
}
//#endregion 📚️Corpus

//#region 🖥️NativeLifecycle
/// ⚖️ LAW (native): a folder attach is remembered for the document's own identity; after a restart the native shell
/// takes that folder up once, without a gesture, and restoring the folder's archive brings the program's history rows
/// and head back; a failed reattach is offered on the band instead of retried. A detach forgets it, so the next restart
/// has no binding and offers nothing. A program holding no document is told so and nothing is remembered.
#[test]
fn a_remembered_folder_comes_back_after_a_restart_and_a_detach_forgets_it() {
    TEST_LOCAL_FOLDERS_LOG.with(|slot| *slot.borrow_mut() = None);
    LOADED.with(|loaded| loaded.borrow_mut().clear());
    let folder = scratch_folder("lifecycle");
    let path = folder.display().to_string();

    let mut unidentified = booted_shell(None);
    unidentified.locale_id = "de".into();
    semio_framework_async::block_on(unidentified.attach_sync_backbone(format!("folder://{path}"))).expect("an unidentified attach is told, not faulted");
    let notice = unidentified.transient_notice().expect("a notice");
    assert_eq!((notice.code.as_deref(), notice.message.as_str()), (Some(SYNC_DOCUMENT_UNIDENTIFIED_CODE), "Dieses Programm hat kein Dokument zum Verbinden"));
    assert!(unidentified.local_folder_bindings.clone().unwrap_or_default().bindings.is_empty(), "nothing is remembered for no document");

    let mut first = booted_shell(Some("doc-7"));
    let binding = session_binding(&first, "doc-7", &path);
    first.remember_local_folder(binding.clone());
    let archive = protocol::DocumentArchivePack { parent_pack: vec![1, 2, 3], parent_spr: vec![4, 5], members: Vec::new() };
    let bytes = protocol::encode_document_archive_bytes(&archive).expect("an archive");
    semio_framework_async::block_on(store_sync::sync::FolderEventLogStorage::new(folder.clone()).write_archive("doc-7", "s.test", &bytes)).expect("the edit persists in the folder");
    drop(first);

    let mut restarted = booted_shell(Some("doc-7"));
    assert!(SHELL_DOCUMENT_TRANSPORTS.folder, "the native shell serves folders");
    assert_eq!(restarted.folder_reconnect_offer(), Some(binding.clone()), "the restart remembers the folder for the same document");
    assert_eq!(restarted.direct_reattach_candidate(), Some(binding.clone()), "the native shell takes it up itself");
    assert_eq!(restarted.folder_reconnect_band_offer(), None, "no gesture is asked for");
    restarted.local_folder_reattach_tried = Some("doc-7".into());
    assert_eq!(restarted.direct_reattach_candidate(), None, "a reattach that ran is not retried");
    assert_eq!(restarted.folder_reconnect_band_offer(), Some(binding.clone()), "a failed reattach is offered on the band");
    restarted.plugins.iter_mut().find(|program| program.plugin_id == binding.plugin_id).expect("the session's program").install_fixture_document(restored_program());
    assert_eq!(semio_framework_async::block_on(restarted.restore_folder_archive(&path, "doc-7")), Ok(true));
    assert_eq!(LOADED.with(|loaded| loaded.borrow().clone()), vec![archive], "the folder's own archive reached the program");
    assert_eq!(history_rows_oldest_first(&restarted.history_entries).into_iter().map(|entry| entry.key()).collect::<Vec<_>>(), ["edit:e-1", "edit:e-2"], "the rows are the archive's");
    assert_eq!(restarted.history_current_checkpoint_id.as_deref(), Some("cp-2"), "the head is the archive's");
    assert_eq!(semio_framework_async::block_on(restarted.restore_folder_archive(&path, "doc-other")), Ok(false), "a folder holding no archive for the document restores nothing");

    restarted.forget_local_folder("doc-7");
    drop(restarted);
    let after_detach = booted_shell(Some("doc-7"));
    assert!(after_detach.local_folder_bindings.clone().unwrap_or_default().bindings.is_empty(), "a detach leaves no binding");
    assert_eq!((after_detach.folder_reconnect_offer(), after_detach.direct_reattach_candidate()), (None, None));
    let _ = std::fs::remove_dir_all(folder);
}
//#endregion 🖥️NativeLifecycle

//#region 🌐️ReconnectBand
fn paint_folder_band(shell: &mut ShellState) -> Vec<HitTarget<ActionDescriptor>> {
    let mut cursor = ShellChromeChildCursor::default();
    let (mut overlay, mut atlas, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), InputState::<ActionDescriptor>::default(), Theme::light());
    for _ in 0..100_000 {
        if shell.render_folder_reconnect_band_step(&mut cursor, &mut overlay, &mut atlas, &mut input, &theme) {
            return input.staged_hits().to_vec();
        }
    }
    panic!("the folder reconnect band step never completed");
}

/// ⚖️ LAW (React's `LocalFolderReconnectBand`): once its direct reattach ran and left the document unattached, the shell
/// offers the remembered folder as a polite status named by its message and described by what it is about, with
/// "Reconnect folder" and "Forget folder" buttons dispatching the shell's sync verbs for the document — in both locales;
/// before that reattach ran nothing is offered, and "Forget folder" forgets it and the band goes.
#[test]
fn a_failed_reattach_offers_an_accessible_reconnect_band_and_forget_retires_it() {
    TEST_LOCAL_FOLDERS_LOG.with(|slot| *slot.borrow_mut() = None);
    let corpus = corpus();
    let mut shell = booted_shell(Some("doc-7"));
    let binding = session_binding(&shell, "doc-7", "/Users/ada/Documents/drawings");
    shell.remember_local_folder(binding.clone());
    assert!(paint_folder_band(&mut shell).is_empty() && shell.folder_reconnect_band_offer().is_none(), "no band before the shell tried the folder itself");
    shell.local_folder_reattach_tried = Some("doc-7".into());
    assert_eq!(shell.direct_reattach_candidate(), None, "a reattach that ran is not retried");
    for (locale, tongue) in [("en", "en"), ("de", "de")] {
        shell.locale_id = locale.into();
        let hits = paint_folder_band(&mut shell);
        let events: Vec<(String, String, String)> = hits.iter().filter_map(|hit| Some((hit.control_id.clone()?, hit.event.as_ref()?.action.clone(), hit.event.as_ref()?.args.as_ref()?.get("documentId")?.as_str()?.to_string()))).collect();
        assert_eq!(events, [(FOLDER_RECONNECT_CONTROL_ID.to_string(), FOLDER_RECONNECT_ACTION.to_string(), "doc-7".to_string()), (FOLDER_FORGET_CONTROL_ID.to_string(), FOLDER_FORGET_ACTION.to_string(), "doc-7".to_string())]);
        assert!(hits.iter().all(|hit| hit.event.as_ref().is_some_and(|event| event.controller_id == "framework.sync")));
        let nodes = shell.chrome_accessibility_nodes(&hits);
        let status = nodes.iter().find(|node| node.key == FOLDER_RECONNECT_STATUS_ID).expect("the band's status node");
        assert_eq!((status.role.as_str(), status.live.as_str(), status.label.as_deref(), status.description.as_deref()), ("status", "polite", corpus["texts"][tongue]["message"].as_str(), corpus["texts"][tongue]["label"].as_str()), "{tongue}");
        for (control_id, key) in [(FOLDER_RECONNECT_CONTROL_ID, "attach"), (FOLDER_FORGET_CONTROL_ID, "forget")] {
            let button = nodes.iter().find(|node| node.key == control_id).unwrap_or_else(|| panic!("{control_id} is projected"));
            assert_eq!((button.role.as_str(), button.label.as_deref()), ("button", corpus["texts"][tongue][key].as_str()), "{tongue}: {control_id}");
        }
    }
    let forget = ActionDescriptor { controller_id: "framework.sync".into(), action: FOLDER_FORGET_ACTION.into(), args: crate::action_args_json!({ "documentId": "doc-7" }) };
    semio_framework_async::block_on(shell.handle_sync_action(forget)).expect("forget routes");
    assert!(shell.local_folder_bindings.clone().unwrap_or_default().bindings.is_empty());
    assert!(paint_folder_band(&mut shell).is_empty(), "the band is gone");
    assert!(shell.chrome_accessibility_nodes(&[]).iter().all(|node| node.key != FOLDER_RECONNECT_STATUS_ID));
}

/// ⚖️ LAW (React's `max-w-[90vw] flex-wrap` band): on a desktop the reconnect band is one row with its two buttons
/// right-aligned after the message; on a phone-width viewport it stays within 90 % of the viewport, wraps the message at
/// its words without losing one, and flows both buttons below it inside the band; above a time-travel band it stacks
/// just over it.
#[test]
fn the_reconnect_band_is_one_row_on_a_desktop_and_wraps_compact_on_a_phone() {
    let theme = Theme::light();
    let corpus = corpus();
    let texts = &corpus["texts"]["de"];
    let (message, reconnect, forget) = (texts["message"].as_str().expect("message").to_string(), texts["attach"].as_str().expect("attach").to_string(), texts["forget"].as_str().expect("forget").to_string());
    let wide = folder_reconnect_band_plan(message.clone(), reconnect.clone(), forget.clone(), None, 1280.0, 720.0, &theme);
    assert_eq!(wide.lines.len(), 1, "one row on a desktop");
    assert!(wide.lines[0].rect.x + wide.lines[0].rect.w <= wide.buttons[0].3.x && wide.buttons[0].3.x + wide.buttons[0].3.w <= wide.buttons[1].3.x, "message, Reconnect, Forget in reading order");
    for width in [375.0_f32, 320.0] {
        let phone = folder_reconnect_band_plan(message.clone(), reconnect.clone(), forget.clone(), None, width, 812.0, &theme);
        let band = phone.band;
        assert!(band.x >= 0.0 && band.w <= width * 0.9 + 0.001, "{width}: within 90 % of the viewport");
        assert!(phone.lines.len() > 1, "{width}: the message wraps: {:?}", phone.lines);
        assert_eq!(phone.lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join(" "), message, "{width}: every word, in order");
        let last = phone.lines.last().expect("a line").rect;
        for (_, _, _, rect) in &phone.buttons {
            assert!(rect.x >= band.x && rect.x + rect.w <= band.x + band.w + 0.001 && rect.y >= last.y + last.h - 0.001 && rect.y + rect.h <= band.y + band.h + 0.001, "{width}: a button below the message, inside the band");
        }
        let below = Rect::new(0.0, 700.0, width, 60.0);
        let stacked = folder_reconnect_band_plan(message.clone(), reconnect.clone(), forget.clone(), Some(below), width, 812.0, &theme);
        assert!(stacked.band.y + stacked.band.h <= below.y, "{width}: stacked over the time-travel band");
    }
}
//#endregion 🌐️ReconnectBand

//#region 🗃️HostRouteFolder
fn door_corpus() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🧫️wgpu-backbone-folder-door/🔣️.json")).expect("the shared folder door corpus parses")
}

/// ⚖️ LAW (shared with the page's `backboneFolderHop`): every request of the corpus is exactly the JSON the shell sends,
/// and every answer the page gives reads as the corpus says — the stored archive's bytes, nothing written yet, a write
/// acknowledged, or the refusal by status or transport error.
#[test]
fn the_shared_backbone_folder_door_corpus_holds_on_wgpu() {
    let corpus = door_corpus();
    for request in corpus["requests"].as_array().expect("requests") {
        let verb = if request["verb"] == "write" { BackboneFolderVerb::Write } else { BackboneFolderVerb::Read };
        let json: Value = serde_json::from_str(&backbone_folder_request(verb, request["uri"].as_str().expect("uri"), request["documentId"].as_str().expect("documentId"), request["schema"].as_str())).expect("JSON");
        assert_eq!(json, request["json"], "{}", request["name"]);
    }
    for answer in corpus["answers"].as_array().expect("answers") {
        let page = answer["page"].to_string();
        let shell = if answer["verb"] == "write" {
            backbone_folder_write_answer(&page).map(|()| serde_json::json!({ "written": true }))
        } else {
            backbone_folder_read_answer(&page).map(|bytes| serde_json::json!({ "bytes": bytes.map(semio_framework_io_base64::base64_standard_encode) }))
        };
        assert_eq!(shell.unwrap_or_else(|error| serde_json::json!({ "error": error })), answer["shell"], "{}", answer["name"]);
    }
}

fn door_requests() -> Vec<(String, Option<String>)> {
    TEST_BACKBONE_FOLDER_HOST.with(|host| host.borrow().1.iter().map(|request| (request["verb"].as_str().unwrap_or_default().to_string(), request["schema"].as_str().map(str::to_string))).collect())
}

/// ⚖️ LAW (browser build, React's worker folder transport): a folder attach through the host route with nothing stored
/// writes the document's archive there under its schema and remembers the folder; while bound the document is not
/// offered again, every history change writes the archive once — never while a history edit is open — and a detach
/// writes the last change and forgets the folder. After a restart the browser offers the band (it waits for the
/// person's gesture), and reconnecting restores the folder's archive with its rows and head.
#[test]
fn a_browser_folder_keeps_its_archive_through_the_host_route_and_comes_back_after_a_restart() {
    TEST_LOCAL_FOLDERS_LOG.with(|slot| *slot.borrow_mut() = None);
    TEST_BACKBONE_FOLDER_HOST.with(|host| *host.borrow_mut() = Default::default());
    LOADED.with(|loaded| loaded.borrow_mut().clear());
    let uri = "folder:///Users/ada/drawings".to_string();
    let mut shell = booted_shell(Some("doc-7"));
    shell.local_folder_direct_reattach = false;
    let binding = session_binding(&shell, "doc-7", "/Users/ada/drawings");
    let program = binding.plugin_id.clone();
    shell.plugins.iter_mut().find(|entry| entry.plugin_id == program).expect("the session's program").install_fixture_document(restored_program());
    let identity = shell.sync_program_identity().expect("the session's document identity");
    semio_framework_async::block_on(shell.attach_host_route_backbone(uri.clone(), identity.clone())).expect("the host route attach");
    let schema = shell.session.as_ref().expect("session").app.io.artifact_schema.clone();
    assert_eq!(door_requests(), [("read".to_string(), None), ("write".to_string(), Some(schema.clone()))], "nothing stored: read, then the document's own archive written under its schema");
    let stored = TEST_BACKBONE_FOLDER_HOST.with(|host| host.borrow().0.get(&(uri.clone(), "doc-7".to_string())).cloned()).expect("the host holds the archive");
    assert_eq!(semio_framework_async::block_on(protocol::decode_document_archive_bytes(&stored)).expect("an archive"), program_archive());
    assert_eq!((shell.sync_backbone_uri.as_deref(), shell.local_folder_bindings.clone().unwrap_or_default().bindings), (Some(uri.as_str()), vec![binding.clone()]), "bound, shown on the sync card and remembered");
    assert_eq!((shell.folder_reconnect_offer(), shell.folder_reconnect_band_offer()), (None, None), "a bound document is not offered");

    assert!(!semio_framework_async::block_on(shell.flush_host_route_folder()), "an unchanged history writes nothing");
    shell.history_cursor += 1;
    assert!(semio_framework_async::block_on(shell.flush_host_route_folder()), "a history change writes the archive");
    assert!(!semio_framework_async::block_on(shell.flush_host_route_folder()), "once");
    shell.history_cursor += 1;
    shell.observe_history_time_travel(Some(&semio_framework::kernel::HistoryTimeTravel { session_id: "1".into(), generation: 1, stage: semio_framework::kernel::HistoryTimeTravelStage::Editing, ..Default::default() }));
    assert!(!semio_framework_async::block_on(shell.flush_host_route_folder()), "never while a history edit is open");
    shell.observe_history_time_travel(None);
    assert!(semio_framework_async::block_on(shell.detach_host_route_folder()), "the detach writes the last change");
    assert_eq!(door_requests().iter().filter(|(verb, _)| verb == "write").count(), 3);
    assert!(shell.sync_backbone_uri.is_none() && shell.local_folder_bindings.clone().unwrap_or_default().bindings.is_empty(), "unbound and forgotten");

    shell.remember_local_folder(binding.clone());
    drop(shell);
    let mut restarted = booted_shell(Some("doc-7"));
    restarted.local_folder_direct_reattach = false;
    restarted.plugins.iter_mut().find(|entry| entry.plugin_id == program).expect("the session's program").install_fixture_document(restored_program());
    assert_eq!((restarted.direct_reattach_candidate(), restarted.folder_reconnect_band_offer()), (None, Some(binding.clone())), "the browser never reopens a folder by itself; it offers the band");
    semio_framework_async::block_on(restarted.attach_host_route_backbone(uri.clone(), identity)).expect("the reconnect");
    assert_eq!(LOADED.with(|loaded| loaded.borrow().clone()), vec![program_archive()], "the folder's archive reached the program");
    assert_eq!(history_rows_oldest_first(&restarted.history_entries).into_iter().map(|entry| entry.key()).collect::<Vec<_>>(), ["edit:e-1", "edit:e-2"], "the rows are the archive's");
    assert_eq!(restarted.history_current_checkpoint_id.as_deref(), Some("cp-2"), "the head is the archive's");
    assert_eq!(restarted.folder_reconnect_band_offer(), None, "reconnected, the band is gone");
}

/// ⚖️ LAW (`📓️api-stepped-document-load.md` §4): a whole-document load of a `(pack, spr)` pair — a hub checkpoint seed, a
/// `LoadDocument` effect, a rebootstrap reseed — reaches the program as the stepped archive load of an archive without
/// members, the one load path; nothing loads a pack in one shot.
#[test]
fn a_plain_document_load_is_the_stepped_archive_load_without_members() {
    LOADED.with(|loaded| loaded.borrow_mut().clear());
    let mut shell = booted_shell(Some("doc-3"));
    let program = shell.session.as_ref().expect("session").plugin_id.clone();
    let entry = shell.plugins.iter_mut().find(|entry| entry.plugin_id == program).expect("the session's program");
    entry.install_fixture_document(restored_program());
    semio_framework_async::block_on(entry.load_app_document_pack(1, &[1, 2], &[3])).expect("the load");
    assert_eq!(LOADED.with(|loaded| loaded.borrow().clone()), vec![protocol::DocumentArchivePack { parent_pack: vec![1, 2], parent_spr: vec![3], members: Vec::new() }]);
}
//#endregion 🗃️HostRouteFolder
