/// 📏️ Surface budget and retirement: example select options, the resident surface bound and flow host retirement.

//#region 🔖️ExamplesTests
/// 📚️ Ticket 26/09/03/PROCEDURAL-3D-END-TO-END — `examples()`, returned by
/// `Generation3dPlayApp::examples` and stamped by `.editor`, must
/// carry the same ids, in the same order, as the `setActiveExample` select options this app
/// declares — otherwise the navbar dropdown and the action's own arg picker disagree.
#[cfg(test)] use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
#[test]
fn examples_match_set_active_example_select_options() {
    let definition = create_generation3d_app();
    let select_ids: Vec<String> = declared_actions(&definition)
        .find(|action| action.id == "setActiveExample")
        .and_then(|action| action.args.first())
        .and_then(|arg| match arg.control() {
            semio_framework::ActionArgControl::Select { options } => Some(options.into_iter().map(|option| option.value).collect::<Vec<_>>()),
            _ => None,
        })
        .expect("setActiveExample must declare a Select arg");
    let example_ids: Vec<String> = examples().into_iter().map(|source| source.id().to_string()).collect();
    assert_eq!(example_ids.len(), 9);
    assert_eq!(example_ids, select_ids);
}
//#endregion 🔖️ExamplesTests

//#region 📏️SurfaceBudgetTests
/// 🪟️ Every authored body key of this app: five window bodies then the three panel bodies, in the
/// order `generation3d_render_body` matches them.
const GENERATION3D_BODY_KEYS: [&str; 8] = [
    flow_window::GENERATION_3D_PLAY_BODY_MAIN,
    edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW,
    generations::GENERATION_3D_PLAY_BODY_GENERATIONS,
    form::GENERATION_3D_PLAY_BODY_GENERATE_FORM,
    generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW,
    artifact_panel::GENERATION_3D_PLAY_BODY_ARTIFACT,
    catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE,
    inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION,
];

/// 📏️ Every window and panel surface of every bundled example must fit the framework's ONE resident
/// surface capacity (`ui_contract::UI_RESIDENT_SURFACE_BYTES`, which `ui_runtime`'s
/// `SURFACE_RECONCILE_SURFACE_BYTES` is defined as). Renders each example's own projection through
/// the SAME `generation3d_render_body` every live window goes through, so the measurement is per
/// example rather than per app state. Prints the table the boot report reads: no surface may be
/// silently oversized behind the mount-time admission fault
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, defect 2).
#[test]
fn every_window_and_panel_surface_fits_the_resident_surface_bound() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let bound = semio_ui_contract::UI_RESIDENT_SURFACE_BYTES;
    let config = Generation3dConfig::default();
    let view_state = semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    for example_id in examples().into_iter().map(|source| source.id().to_string()) {
        let snapshot = semio_s_artifact_procedural_3d_io_text::snapshot::example_snapshot(&example_id).unwrap_or_else(|| panic!("{example_id}: missing projection"));
        for body_key in GENERATION3D_BODY_KEYS {
            let session = FlowEvalSession::new();
            let tree = generation3d_render_body(body_key, &snapshot, &config, None, &view_state, &PreviewInteractionMarks::default(), &session, None).expect("render");
            context::retire_flow_eval_session(session);
            let rendered = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render json");
            println!("[STATS] surface example={example_id} body={body_key} bytes={} bound={bound}", rendered.len());
            assert!(!rendered.is_empty(), "{example_id}/{body_key} rendered empty");
            assert!(rendered.len() <= bound, "{example_id}/{body_key} is {} B, over the {bound} B resident surface bound", rendered.len());
        }
        snapshot.retire_cold();
    }
}
//#endregion 📏️SurfaceBudgetTests

//#region 🧹️RetirementTests
/// 🧹️ The live first-turn sequence in ONE process: `pending_effects` (which built an unretired
/// scratch `FlowHost` and trapped the plugin actor with `ordered-map root must be explicitly retired
/// before drop`), a render of every body, an example switch that replaces the whole host_snapshot, and the
/// retained `FlowEvalSession` owner's own close. Nothing here may abort
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, defect 1).
#[semio_framework_async_macros::async_test]
async fn the_first_turn_sequence_retires_every_flow_host_it_builds() {
    use semio_plugin::{ArtifactView, ConfigView};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;

    let snapshot = semio_s_artifact_procedural_3d_io_text::snapshot::default_snapshot();
    let config = Generation3dConfig::default();
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let (_, preview_view) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    let _ = (&doc, &cfg);
    let effects = app.pending_effects(Some(&preview_view)).await;
    println!("[STATS] pending_effects effects={}", effects.len());
    snapshot.retire_cold();

    for body_key in GENERATION3D_BODY_KEYS {
        let rendered = context::render(&mut app, body_key).await;
        assert!(!rendered.is_empty(), "{body_key} rendered empty");
    }

    for example_id in [semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_BOX_FILLET, semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_HEX_COLUMN] {
        context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() })).await;
        let rendered = context::render(&mut app, GENERATION3D_BODY_KEYS[0]).await;
        assert!(!rendered.is_empty(), "{example_id} main body rendered empty after the switch");
    }

    let mut owner = Generation3dInstanceOperationOwner::new();
    let closed = context::close_instance_operation_owner(&mut owner);
    assert!(closed, "the retained evaluation-session owner never reached its terminal close");
    assert!(semio_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner));
}
/// 🛑️ The abort affordance through the PRODUCTION binding (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-11-25).
///
/// The host binds a window's run with `ArtifactView::with_tool_run_poll`; the owned `tool_run` field only
/// tests ever set, so rendering from `doc.tool_run()` published `cancellable: false` for every live run
/// (190 s of `computing 4/7` with no stop). This law binds the run the way the host does, renders every
/// body from `doc.tool_run_poll()` over a working chain and requires the preview body to offer the abort
/// with the run's own identity, so the dispatched `toolRunAbort` addresses the run the ledger holds.
#[test]
fn a_running_preview_run_bound_by_the_host_is_cancellable_and_addresses_its_run() {
    use semio_plugin::{ArtifactView, ToolRunPollView};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let identity = semio_framework_tool_run::ToolRunIdentity { id: semio_framework_tool_run::ToolRunId { app_instance_id: 1, run: 7 }, generation: 3, base_revision: [0; 32] };
    let poll = ToolRunPollView { tool_id: semio_s_artifact_procedural_3d_preview_eval::PREVIEW_EVAL_TOOL_ID, identity, state: semio_framework_tool_run::ToolRunState::Running };
    let snapshot = semio_s_artifact_procedural_3d_io_text::snapshot::default_snapshot();
    let history = semio_plugin::HistoryView::empty();
    let bound = ArtifactView::new(&snapshot, &history).with_tool_run_poll(Some(poll));
    assert!(bound.tool_run().is_none() && bound.tool_run_poll().is_some(), "the host binds the poll view only; the owned run field is a test setter");
    let config = Generation3dConfig::default();
    let view_state = semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let mut host = semio_os_flow::FlowHost::default();
    let mut session = FlowEvalSession::new();
    assert!(session.sync(&host), "the default demo graph has pending nodes, so the chain is unsettled");
    session.arm_window_tick("preview-1", crate::preview_reply::TURN).expect("window tick admission");
    let mut offered = 0;
    for body_key in GENERATION3D_BODY_KEYS {
        let tree = generation3d_render_body(body_key, &snapshot, &config, None, &view_state, &PreviewInteractionMarks::default(), &session, bound.tool_run_poll(), &mut None).expect("render");
        let projection = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render json").replace('\\', "");
        if projection.contains("\"cancellable\":true") {
            offered += 1;
            assert!(projection.contains("\"cancelAction\":\"toolRunAbort\""), "{body_key}: the abort is the framework's toolRunAbort");
            assert!(projection.contains("\"runId\":\"") && projection.contains(&format!("\"generation\":{}", identity.generation)), "{body_key}: the abort carries the run's own id and generation");
        }
    }
    assert!(offered >= 1, "no preview body offered the abort for a running preview run bound the way the host binds it");
    let unbound = ArtifactView::new(&snapshot, &history);
    for body_key in GENERATION3D_BODY_KEYS {
        let tree = generation3d_render_body(body_key, &snapshot, &config, None, &view_state, &PreviewInteractionMarks::default(), &session, unbound.tool_run_poll(), &mut None).expect("render");
        let projection = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render json").replace('\\', "");
        assert!(!projection.contains("\"cancellable\":true"), "{body_key}: with no run bound there is nothing to abort");
    }
    context::retire_flow_eval_session(session);
    host.retire_cold();
    snapshot.retire_cold();
}
//#endregion 🧹️RetirementTests
