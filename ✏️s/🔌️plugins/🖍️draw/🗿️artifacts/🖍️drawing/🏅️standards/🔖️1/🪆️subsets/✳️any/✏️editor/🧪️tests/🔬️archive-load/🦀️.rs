//! 🚪️ The host answers `setActiveExample`'s `Effect::LoadDocument` through the archive door with an empty roster; the load must settle `Ready` within bounded rounds.

use super::unit_tests::context::drawing_app;
use semio_s_artifact_draw_drawing_editor::DrawingCommand;
use semio_s_artifact_draw_drawing_editor_commands::{add_layer, set_active_example};
use semio_plugin::{artifact_app_laws as artifact_laws, MountedOwnerPolicyV1, PluginApp};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

/// 🔁️ Drives one command to its published `LoadDocument` effect.
async fn published_load(app: &mut super::unit_tests::context::DrawingAppFixture, command: DrawingCommand, instance: u32) -> (Vec<u8>, Vec<u8>) {
    let meta = semio_plugin::ActionMeta { instance_id: instance, ..artifact_laws::meta("fixture") };
    let policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let mut identity = artifact_laws::fixture_identity();
    let result = artifact_laws::dispatch_original_fixture_command(&mut app.0, &command, &meta, policy, &mut identity, app.1.cx()).await.expect("example dispatch");
    let settled = artifact_laws::settle_registered_typed_operation(&mut app.0, instance, policy, &mut identity, app.1.cx()).await.expect("example settlement");
    let mut loads = result.requested_effects.into_iter().chain(settled.effects).filter_map(|effect| match effect {
        semio_plugin::kernel::Effect::LoadDocument { pack, spr } => Some((pack, spr)),
        _ => None,
    });
    let loaded = loads.next().expect("load");
    assert!(loads.next().is_none(), "one load");
    loaded
}

async fn settle_archive(app: &mut super::unit_tests::context::DrawingAppFixture, operation: u64, parent_pack: Vec<u8>, parent_spr: Vec<u8>) -> (protocol::DocumentArchiveLoadStatus, usize) {
    artifact_laws::admit_document_archive_load(&mut app.0, operation, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }, app.1.cx()).expect("archive admission");
    let mut status = None;
    let grant = app.1.cx().retained_grant();
    let mut identity = artifact_laws::fixture_identity();
    let mut rounds = 0;
    let mut last = u64::MAX;
    for _ in 0..20_000 {
        rounds += 1;
        artifact_laws::fixture_renew_turn(app.1.cx(), artifact_laws::OriginalFixtureCaller::turn_fuel(), grant).expect("archive fixture turn");
        let polled = PluginApp::poll_document_archive_load(&mut app.0, operation, &mut identity, app.1.cx()).await.expect("archive status");
        if polled.completed != last { eprintln!("[DEBUG] {rounds} {}", polled.completed); last = polled.completed; }
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let _ = PluginApp::maintenance_step(&mut app.0, app.1.cx(), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("archive maintenance step");
        std::thread::yield_now();
    }
    let status = status.expect("archive load reaches a terminal state");
    PluginApp::acknowledge_document_archive_load(&mut app.0, operation).expect("archive acknowledgement");
    eprintln!("[DEBUG] rounds {rounds}");
    (status, rounds)
}

#[semio_framework_async_macros::async_test]
async fn demo_example_load_settles_through_the_host_document_archive_door() {
    let mut app = drawing_app().await;
    app.bind_instance_id(7).await;
    let (pack, spr) = published_load(&mut app, DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }), 7).await;
    let (status, _) = settle_archive(&mut app, 91, pack, spr).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    let snapshot = app.snapshot().expect("loaded snapshot");
    assert_eq!(snapshot.id, "semio", "demo replaced boot");
    assert!(!snapshot.layers.is_empty(), "layers");
    let loaded = app.document_pack().await.expect("live document pack");
    let initial = <DrawingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&loaded.pack).expect("initial pack decodes");
    assert_eq!(initial, snapshot, "initial snapshot");
    let authored = semio_s_artifact_draw_drawing_model::io::text::snapshot::parse_dsl(semio_s_artifact_draw_drawing_examples::demo::PRIMARY_TEXT).expect("parse authored Demo");
    assert_eq!(initial, authored, "authored emblem");
    artifact_laws::close_registered_fixture_app(&mut app.0, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("archive close");
    assert!(app.close_terminal_is_empty());
}

/// 🧾️ Reopens the genesis Pack with a nonempty SPR.
#[semio_framework_async_macros::async_test]
async fn document_archive_replays_the_edited_drawing_on_reopen() {
    let mut app = drawing_app().await;
    let policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let meta = artifact_laws::meta("local");
    let mut identity = artifact_laws::fixture_identity();
    let initial = app.document_pack().await.expect("initial document archive");
    let initial_layers = app.snapshot_ref().layers.len();
    let command = DrawingCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() });
    artifact_laws::dispatch_original_fixture_command(&mut app.0, &command, &meta, policy, &mut identity, app.1.cx()).await.expect("rectangle dispatch");
    artifact_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, policy, &mut identity, app.1.cx()).await.expect("rectangle settlement");
    let edited = app.snapshot().expect("edited Drawing snapshot");
    assert_eq!(edited.layers.len(), initial_layers + 1);
    let persisted = app.document_pack().await.expect("edited document archive");
    assert_eq!(persisted.pack, initial.pack);
    assert_ne!(persisted.spr, initial.spr);
    artifact_laws::close_registered_fixture_app(&mut app.0, policy, app.1.cx()).expect("close edited Drawing");
    drop(app);

    let mut reopened = drawing_app().await;
    let (status, _) = settle_archive(&mut reopened, 92, persisted.pack, persisted.spr).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    assert_eq!(reopened.snapshot_ref(), &edited);
    artifact_laws::close_registered_fixture_app(&mut reopened.0, policy, reopened.1.cx()).expect("close reopened Drawing");
    assert!(reopened.close_terminal_is_empty());
}

/// 🔂️ The browser's boot-then-pick sequence: Demo replaces the boot store, then Blank replaces Demo while Demo's displaced store may still retire.
#[semio_framework_async_macros::async_test]
async fn demo_then_blank_example_load_settles_two_consecutive_replacements() {
    let mut app = drawing_app().await;
    app.bind_instance_id(7).await;
    let (pack, spr) = published_load(&mut app, DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }), 7).await;
    let (status, _) = settle_archive(&mut app, 91, pack, spr).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    assert_eq!(app.snapshot().expect("demo snapshot").id, "semio");
    let (pack, spr) = published_load(&mut app, DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "blank".into() }), 7).await;
    let (status, _) = settle_archive(&mut app, 93, pack, spr).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    let snapshot = app.snapshot().expect("blank snapshot");
    assert_eq!(snapshot.id, "blank", "blank replaced demo");
    assert!(snapshot.layers.is_empty(), "blank has no layers");
    artifact_laws::close_registered_fixture_app(&mut app.0, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("archive close");
    assert!(app.close_terminal_is_empty());
}
