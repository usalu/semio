//! 🚪️ The browser host answers `setActiveExample`'s `Effect::LoadDocument` by handing the pack/spr
//! pair back through the document archive door with an empty member roster (`ShellHost`
//! `loadDocumentPair`); the load must settle `Ready` and publish the example's layers rather than
//! stall or trap the instance (ticket 26/09/05/DRAW-PLUGIN-END-TO-END, after the sourcing recipe).

use super::unit_tests::context::drawing_app;
use semio_s_artifact_draw_drawing_editor::DrawingCommand;
use semio_s_artifact_draw_drawing_editor_commands::set_active_example;
use semio_framework_plugin::{artifact_app_laws as artifact_laws, PluginApp};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

/// 🔁️ Drives one dispatched command to its published `LoadDocument` effect the way the host does.
async fn published_load(app: &mut super::unit_tests::context::DrawingAppFixture, command: DrawingCommand, instance: u32) -> (Vec<u8>, Vec<u8>) {
    let meta = semio_framework_plugin::ActionMeta { instance_id: instance, ..artifact_laws::meta("fixture") };
    app.0.dispatch_typed(command, &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("example dispatch");
    let mut loaded = None;
    for _ in 0..100_000 {
        app.0.maintenance_step(app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).unwrap();
        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
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

async fn settle_archive(app: &mut super::unit_tests::context::DrawingAppFixture, operation: u64, parent_pack: Vec<u8>, parent_spr: Vec<u8>) -> protocol::DocumentArchiveLoadStatus {
    PluginApp::begin_document_archive_load(&mut app.0, operation, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
    let mut status = None;
    let mut last = String::new();
    for round in 0..20_000 {
        let polled = PluginApp::poll_document_archive_load(&mut app.0, operation, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("archive status");
        let line = format!("{:?} {}/{}", polled.state, polled.completed, polled.total);
        if round % 1 == 0 {
            let pack = app.document_pack().await.map(|files| files.pack.len()).unwrap_or(0);
            let current = app.snapshot().map(|snapshot| snapshot.encode_pack().len()).unwrap_or(0);
            eprintln!("archive poll round={round} {line} initial_pack={pack} current_pack={current}");
        }
        if line != last {
            last = line;
        }
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let _ = PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("archive maintenance step");
        std::thread::yield_now();
    }
    let status = status.expect("archive load reaches a terminal state");
    PluginApp::acknowledge_document_archive_load(&mut app.0, operation).expect("archive acknowledgement");
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
    let loaded = app.document_pack().await.expect("live document pack");
    let initial = <DrawingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&loaded.pack).expect("initial pack decodes");
    assert_eq!(initial, snapshot, "an edit-free load reads back its exact initial snapshot");
    assert_eq!(initial.assets.values().map(|asset| asset.samples.len()).sum::<usize>(), 262_144, "the demo's emblem asset survives the paged initial clone");
    for _ in 0..100_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.0.close_step(app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).unwrap();
        std::thread::yield_now();
    }
    assert!(app.close_terminal_is_empty());
}
