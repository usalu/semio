//! 🧩️ Drawing editor unit harness: the fixtures, settle drivers and canvas projections every unit law crate shares.

#![allow(unused_imports)]

use super::*;

pub(super) fn admit_drawing_envelope(app: &mut DrawingAppFixture, wire: &[u8]) -> semio_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Drawing live envelope ingress credits");
    for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Drawing envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Drawing envelope page admission failed: {fault:?}"));
    }
    assert!(app.0.seal_artifact_envelope_ingress(handle, app.1.cx()).expect("Drawing envelope seal"));
    handle
}

/// ⏱️ Turn budget for one live Drawing envelope load. An accepted load ends by retiring the DISPLACED boot store one item per maintenance turn (the GIS twin measured 121 160 turns to `Ready`), so 100 000 could cut a correct load off mid-retirement. The bound asserts termination, not speed.
const DRAWING_LIVE_LOAD_TURN_BUDGET: usize = 1_000_000;

pub(super) async fn drive_drawing_load(app: &mut DrawingAppFixture, handle: semio_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_plugin::ArtifactEnvelopeDecodeOperationPoll {
    let mut identity = artifact_laws::fixture_identity();
    let grant = app.1.cx().retained_grant();
    for _ in 0..DRAWING_LIVE_LOAD_TURN_BUDGET {
        semio_plugin::artifact_app_laws::fixture_renew_turn(app.1.cx(), semio_plugin::artifact_app_laws::OriginalFixtureCaller::turn_fuel(), grant).expect("one Drawing load turn");
        app.0.advance_typed_operation_publication(&mut identity, app.1.cx()).await.expect("one Drawing original decode publication turn");
        app.0.maintenance_step(app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("one Drawing maintenance turn");
        let poll = app.0.advance_artifact_envelope_load(handle, app.1.cx()).expect("Drawing load advancement");
        if matches!(poll, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Drawing retained envelope load did not reach terminal")
}

/// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C: the `interactionSelect` a gesture emits as
/// `Effect::ReplayShellCommand` is folded in-reactor by the typed-operation ladder, so these two
/// selection tests witness the selection snapshot itself instead of the effect. They need what the
/// plugin host supplies: the self-closing `drawing_app()` binds the live instance, while this helper
/// adds a `ViewModel` naming the canvas window (`setCamera` addresses it) with the active utility and the host's
/// settle protocol after every dispatch (`settle_registered_typed_operation`) — exactly
/// `🎚️config/🧪️tests/🔬️window`'s recipe.
pub(super) async fn inline_selection_app() -> (DrawingAppFixture, semio_plugin::ActionMeta) {
    use semio_plugin::{ViewWindowInstance, WindowConfigOwner};
    let app = drawing_app().await;
    let view = ViewModel {
        window_instances: vec![ViewWindowInstance { id: "drawing-canvas".into(), window_kind_id: semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::config::DrawingCanvasWindowConfigOwner::WINDOW_KIND_ID.into() }],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    let meta = semio_plugin::ActionMeta { view_state: Some(view.for_window_instance("drawing-canvas").expect("canvas window instance").into()), ..artifact_laws::meta("local") };
    (app, meta)
}

/// 🔁️ One dispatch settled the way the plugin host settles it; the receipt's `effects` are exactly
/// what the host would have been handed.
pub(super) async fn settled(app: &mut DrawingAppFixture, command: DrawingCommand, meta: &semio_plugin::ActionMeta) -> (semio_plugin::InvocationResult, artifact_laws::TypedOperationFixtureReceipt) {
    settle_drawing_geometry(app,meta.view_state.as_deref()).await;
    let command_id = command.command_id();
    let result = semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("dispatch {command_id}: {fault:?}"));
    let receipt = artifact_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("retained publication of {command_id} settles: {fault:?}"));
    (result, receipt)
}

/// 📸️ Loads test geometry through the same retained envelope owner as the host.
pub(super) async fn load_drawing_fixture(app: &mut DrawingAppFixture, snapshot: &DrawingSnapshot) {
    use semio_os_kernel::os_pack::snapshot::ArtifactPack;
    let pack = snapshot.encode_pack().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": DRAWING_DOCUMENT_SCHEMA, "id": snapshot.id,
        "vcs": { "initialPack": pack, "edits": [], "changes": [], "checkpoints": [], "alternatives": [] },
        "editMessages": [], "conflicts": []
    })).unwrap();
    let handle = admit_drawing_envelope(app, &wire);
    assert_eq!(drive_drawing_load(app, handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "fixture admission: {:?}", app.artifact_store_replacement_refusal(handle));
    assert!(app.acknowledge_artifact_store_replacement(handle).unwrap());
    assert_eq!(app.snapshot().unwrap(), *snapshot);
}

pub(super) fn assert_one_artifact_publication(receipt: &artifact_laws::TypedOperationFixtureReceipt) {
    assert_artifact_publication_units(receipt, 1);
}

pub(super) fn assert_artifact_publication_units(receipt: &artifact_laws::TypedOperationFixtureReceipt, expected_completions: usize) {
    assert_eq!(receipt.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::Artifact).count(), 1, "one retained artifact result page: {:?}", receipt.lanes);
    assert_eq!(receipt.completions, expected_completions, "every retained component operation reaches one exact terminal witness");
    assert_eq!(receipt.completion_operations.len(), expected_completions, "every terminal witness names its component operation");
    assert_eq!(receipt.revisions.len(), expected_completions, "every terminal witness carries its committed revision");
    let operations = receipt.completion_operations.iter().copied().collect::<std::collections::HashSet<_>>();
    assert_eq!(operations.len(), expected_completions, "component operation terminal witnesses are distinct: {:?}", receipt.completion_operations);
}

pub(super) fn canvas_scene_of_tree(tree: semio_plugin::ComponentTree) -> semio_plugin::Canvas2dScene {
    canvas_scene(artifact_laws::project_and_retire_fixture_tree(tree).expect("project and retire canvas tree"))
}

pub(super) fn canvas_scene(projection: String) -> semio_plugin::Canvas2dScene {
    artifact_laws::decode_fixture_scene_with_lanes::<semio_plugin::Canvas2dScene>(&projection).expect("typed canvas")
}

pub(super) async fn settle_drawing_geometry(app:&mut DrawingAppFixture,view:Option<&ViewModel>){
    use semio_plugin::reactor::jobs::{registry_driver::RegistryJobDriver,JobStep};
    for effect in app.prepared_effects(view).await{match effect{
        Effect::CancelJob{job}=>{RegistryJobDriver::generous(job).cancel().expect("registered Drawing geometry cancels");},
        Effect::SpawnJob{job,kind,input,..}if kind=="semio.draw.mounted-vector"=>{let mut driver=RegistryJobDriver::generous(job);driver.start(&kind,input,None).expect("registered Drawing geometry admits");let mut complete=false;for _ in 0..100000{match driver.step().expect("registered Drawing geometry steps"){JobStep::Running(_)=>{},JobStep::Done(_)=>{complete=true;break;},JobStep::Failed(error)=>panic!("registered Drawing geometry failed: {}",String::from_utf8_lossy(&error))}}assert!(complete,"registered Drawing geometry must settle before its canvas witness");},
        _=>{},
    }}
}

pub(super) async fn rendered_drawing_canvas(app:&mut DrawingAppFixture,fixture:Option<&str>,view:&ViewModel)->Result<String,String>{
    settle_drawing_geometry(app,Some(view)).await;
    artifact_laws::render_registered_fixture_app_over(&mut app.0,DRAWING_PLAY_BODY_COMPOSITE,fixture,view,app.1.cx()).await.map_err(|error|format!("{error:?}"))
}

pub(super) fn first_layer_id(app: &DrawingAppFixture) -> String {
    layer_id(&app.snapshot().expect("materialize projection").layers[0]).to_string()
}

pub(super) fn last_layer_id(app: &DrawingAppFixture) -> String {
    let projection = app.snapshot().expect("materialize projection");
    layer_id(projection.layers.last().expect("layer")).to_string()
}

pub(super) async fn selected_strokes(app: &DrawingAppFixture) -> Vec<String> {
    app.interaction_state().await.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}
