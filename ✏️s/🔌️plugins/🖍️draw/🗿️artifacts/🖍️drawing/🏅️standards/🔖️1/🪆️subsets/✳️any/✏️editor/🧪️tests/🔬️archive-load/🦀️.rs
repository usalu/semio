//! 🚪️ The browser host answers `setActiveExample`'s `Effect::LoadDocument` by handing the pack/spr
//! pair back through the document archive door with an empty member roster (`ShellHost`
//! `loadDocumentPair`); the load must settle `Ready` and publish the example's layers rather than
//! stall or trap the instance (ticket 26/09/05/DRAW-PLUGIN-END-TO-END, after the sourcing recipe).

use super::unit_tests::context::drawing_app;
use super::*;
use semio_framework_plugin::{artifact_app_laws as artifact_laws, PluginApp};

/// 🔁️ Drives one dispatched command to its published `LoadDocument` effect the way the host does.
async fn published_load(app: &mut super::unit_tests::context::DrawingApp, command: DrawingCommand, instance: u32) -> (Vec<u8>, Vec<u8>) {
    let meta = semio_framework_plugin::ActionMeta { instance_id: instance, ..artifact_laws::meta("fixture") };
    app.dispatch_typed(command, &meta).await.expect("example dispatch");
    let mut loaded = None;
    for _ in 0..100_000 {
        app.maintenance_step(1, 4_096).unwrap();
        app.advance_typed_operation_publication().await.unwrap();
        if let Some(page) = app.take_typed_operation_result_page(instance) {
            assert!(app.acknowledge_typed_operation_result(page.token).unwrap());
        }
        if let Some(semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
            loaded = Some((pack, spr));
        }
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
        if !app.has_pending_typed_operations() {
            break;
        }
        std::thread::yield_now();
    }
    loaded.expect("the example publishes a document load")
}

async fn settle_archive(app: &mut super::unit_tests::context::DrawingApp, operation: u64, parent_pack: Vec<u8>, parent_spr: Vec<u8>) -> protocol::DocumentArchiveLoadStatus {
    PluginApp::begin_document_archive_load(&mut *app, operation, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
    let mut status = None;
    let mut last = String::new();
    for round in 0..20_000 {
        let polled = PluginApp::poll_document_archive_load(&mut *app, operation).await.expect("archive status");
        let line = format!("{:?} {}/{}", polled.state, polled.completed, polled.total);
        if round % 1 == 0 {
            let pack = app.document_pack().await.map(|files| files.pack.len()).unwrap_or(0);
            let current = app.snapshot().map(|snapshot| snapshot.encode_pack().len()).unwrap_or(0);
            eprintln!("[DEBUG] archive poll round={round} {line} initial_pack={pack} current_pack={current}");
        }
        if line != last {
            eprintln!("[DEBUG] archive poll round={round} {line}");
            last = line;
        }
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let _ = PluginApp::maintenance_step(&mut *app, 1, 4_096).expect("archive maintenance step");
        std::thread::yield_now();
    }
    let status = status.expect("archive load reaches a terminal state");
    PluginApp::acknowledge_document_archive_load(&mut *app, operation).expect("archive acknowledgement");
    status
}

#[semio_framework_async_macros::async_test]
async fn demo_example_load_settles_through_the_host_document_archive_door() {
    let mut app = drawing_app().await;
    app.bind_instance_id(7).await;
    let (pack, spr) = published_load(&mut app, DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }), 7).await;
    let status = settle_archive(&mut app, 91, pack, spr).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    let snapshot = app.snapshot().expect("loaded snapshot");
    assert_eq!(snapshot.id, "semio", "the demo document replaced the boot document");
    assert!(!snapshot.layers.is_empty(), "the demo example carries layers");
    // 📦️ The store's envelope keeps the loaded initial snapshot: the host reads the document back
    // through `print_document_pack` (initial + `.spr`), so an edit-free load must read back byte-equal
    // to its live fold — draw's initializer used to MOVE the initial out and leave an empty one behind.
    let loaded = app.document_pack().await.expect("live document pack");
    let initial = <DrawingSnapshot as store::ArtifactPack>::decode_pack(&loaded.pack).expect("initial pack decodes");
    assert_eq!(initial, snapshot, "an edit-free load reads back its exact initial snapshot");
    assert_eq!(initial.assets.values().map(|asset| asset.data.len()).sum::<usize>(), 29_104, "the demo's emblem asset survives the paged initial clone");
    // 🧹️ Every store closes before drop (the store's shallow-shell Drop witness is fail-closed).
    for _ in 0..100_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, 4_096).unwrap();
        std::thread::yield_now();
    }
    assert!(app.close_terminal_is_empty());
}
