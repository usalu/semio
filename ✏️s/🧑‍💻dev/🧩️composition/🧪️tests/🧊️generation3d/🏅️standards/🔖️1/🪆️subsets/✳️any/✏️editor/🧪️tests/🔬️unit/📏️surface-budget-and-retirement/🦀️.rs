/// 📏️ Surface budget and retirement: example select options, the resident surface bound and flow host retirement.

//#region 🔖️ExamplesTests
/// 📚️ Ticket 26/09/03/PROCEDURAL-3D-END-TO-END — `examples()`, returned by
/// `Generation3dPlayApp::examples` and stamped by `.editor`, must
/// carry the same ids, in the same order, as the `setActiveExample` select options this app
/// declares — otherwise the navbar dropdown and the action's own arg picker disagree.
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
    let bound = semio_framework_ui_contract::UI_RESIDENT_SURFACE_BYTES;
    let config = Generation3dConfig::default();
    let view_state = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    for example_id in examples().into_iter().map(|source| source.id().to_string()) {
        let snapshot = semio_s_artifact_procedural_3d_model::example_snapshot(&example_id).unwrap_or_else(|| panic!("{example_id}: missing projection"));
        for body_key in GENERATION3D_BODY_KEYS {
            let session = FlowEvalSession::new();
            let tree = generation3d_render_body(body_key, &snapshot, &config, None, &view_state, &PreviewInteractionMarks::default(), &session, None).expect("render");
            context::retire_flow_eval_session(session);
            let rendered = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render json");
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
    use semio_framework_plugin::{ArtifactView, ConfigView};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;

    let snapshot = semio_s_artifact_procedural_3d_model::default_snapshot();
    let config = Generation3dConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let (_, preview_view) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    let _ = (&doc, &cfg);
    // 🔒️ Through the app instance, because the poll now reads the RETAINED evaluation session's
    // per-window arming latch and only the instance owns the handle to it.
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
    let mut closed = false;
    for _ in 0..1_000_000 {
        if matches!(semio_framework_plugin::ArtifactInstanceOperationOwner::close_step(&mut owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            closed = true;
            break;
        }
    }
    assert!(closed, "the retained evaluation-session owner never reached its terminal close");
    assert!(semio_framework_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner));
}
//#endregion 🧹️RetirementTests
