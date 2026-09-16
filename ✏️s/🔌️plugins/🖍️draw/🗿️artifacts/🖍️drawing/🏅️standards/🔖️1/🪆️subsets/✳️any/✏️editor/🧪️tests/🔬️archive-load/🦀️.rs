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
        if line != last || round % 2_000 == 0 {
            eprintln!("[DEBUG] archive poll round={round} {line}");
            last = line;
        }
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let step = PluginApp::maintenance_step(&mut *app, 1, 4_096).expect("archive maintenance step");
        if round % 2_000 == 0 {
            eprintln!("[DEBUG] archive maintenance step round={round} {step:?}");
        }
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
    eprintln!("[DEBUG] published pack={} spr={} spr_text={:?}", pack.len(), spr.len(), String::from_utf8_lossy(&spr));
    {
        let limits = protocol::RetainedSprLimits { file_bytes: protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u64, frame_body_bytes: store::OWNED_SCHEMA_DECODE_PAGE_BYTES as u64 * 256, records: protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 * 8 };
        let mut decoder = protocol::RetainedHistoryDecode::new_persisted_document(spr.len(), limits).expect("decoder");
        for round in 0..64 {
            let step = decoder.step(&spr, 4_096, 1);
            eprintln!("[DEBUG] spr decode round={round} {step:?}");
            if !matches!(step, Ok(protocol::RetainedHistoryDecodeStep::Pending { .. })) { break; }
        }
        let history = decoder.take_ready().expect("ready history");
        eprintln!("[DEBUG] history doc_id={} schema={} composition={:?}", history.doc_id, history.schema, history.composition);
        let owners = <semio_framework_plugin::EditorApp<DrawingPlayApp> as semio_framework_plugin::ArtifactApp>::build_document_store_owners().expect("owners");
        let expected = store::os_io::ArtifactRef { artifact_id: history.doc_id.clone(), dialect: crate::DRAWING_DIALECT.into() };
        let mut hydration = store::RetainedPersistedDocumentHydration::<DrawingSnapshot, DrawingMutation>::from_pack(pack.clone(), history, expected, None, DRAWING_DOCUMENT_SCHEMA.to_string(), owners, semio_framework_job::OperationId(5), semio_framework_job::Generation(1), u64::MAX, store::PersistedDocumentHydrationTarget::Envelope);
        let mut sequence = 0;
        let mut last = String::new();
        for round in 0..400 {
            let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(5), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(4_096, u64::MAX), semio_framework_job::CancelToken::root_now(), semio_framework_job::default_now_us, &mut sequence);
            let step = hydration.step(&mut cx);
            let line = match &step { store::PersistedDocumentHydrationStep::Pending(progress) => format!("pending {progress:?} fuel_left={}", cx.fuel_remaining()), store::PersistedDocumentHydrationStep::Ready(_) => "ready".to_string(), store::PersistedDocumentHydrationStep::Rejected(diagnostic) => format!("rejected {diagnostic:?}") };
            if line != last || round % 20_000 == 0 { eprintln!("[DEBUG] hydration round={round} {line}"); last = line; }
            if !matches!(step, store::PersistedDocumentHydrationStep::Pending(_)) { break; }
        }
        std::mem::forget(hydration);
    }
    let status = settle_archive(&mut app, 91, pack, spr).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    let snapshot = app.snapshot().expect("loaded snapshot");
    assert_eq!(snapshot.id, "semio", "the demo document replaced the boot document");
    assert!(!snapshot.layers.is_empty(), "the demo example carries layers");
}
