/// 🥦️ Fill and distribution laws: the fill tool run, its count measure and the kind-weight, vortex and joint distribution.


//#region 🔖️Fill
/// 🛠️ Wave B14: Fill is a mode-level tool. A leftover/window `select` utility must not take the world lane —
/// the guest world lane publishes `fill` while `active_tool_id` is fill.
#[test]
fn fill_tool_wins_the_world_lane_over_a_select_window_utility() {
    let mut runtime = Puzzle3dRuntime::default();
    runtime.active_tool_id = Some(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into());
    let mut view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    view.active_utility_id = Some("select".into());
    view.active_utility_by_window_id.insert("pane".into(), "select".into());
    assert!(puzzle3d_fill_tool_active(&runtime), "the Fill tab arms the mode tool, not a window utility");
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID, "guest world lane must stay fill while the tool is armed");
    runtime.active_tool_id = None;
    assert!(!puzzle3d_fill_tool_active(&runtime));
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), "select");
}

/// ⏯️ Fill is a framework tool run declared through the manifest (`📋️tool-run-contract.md` §2.4): a mutating
/// `instance3d` run with revalidate rebase and resume reconfigure, run and revalidate jobs, and one reason per
/// schema reason spelling that reason's own code, id and verdict. The framework injects the seven run actions;
/// no plugin tick or cancel verb survives.
#[semio_framework_async_macros::async_test]
async fn the_fill_tool_declares_its_tool_run_through_the_manifest() {
    use semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::FillRunReason;
    use semio_framework_tool_run::ToolRunRebasePolicy;
    use semio_framework_tool_run::ToolRunReconfigurePolicy;
    use semio_framework_tool_run::ToolRunTraceKind;
    use semio_framework_tool_run::TOOL_RUN_ACTION_IDS;
    let definition = create_puzzle3d_app();
    let fill = definition.tools.iter().find(|tool| tool.id == semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool");
    let run = fill.run.as_ref().expect("the fill tool declares its ToolRunDefinition");
    run.validate().expect("the fill run definition validates");
    assert!(run.mutating, "fill writes the document, so its run is transactional");
    assert_eq!((run.rebase, run.reconfigure, run.trace), (ToolRunRebasePolicy::Revalidate, ToolRunReconfigurePolicy::Resume, ToolRunTraceKind::Instance3d));
    assert_eq!((run.run_job.as_str(), run.revalidate_job.as_ref().map(|job| job.as_str())), (fill_tool::RUN_JOB_KIND, Some(fill_tool::REVALIDATE_JOB_KIND)));
    assert_eq!(run.reasons.len(), FillRunReason::ALL.len());
    for (reason, declared) in FillRunReason::ALL.iter().zip(&run.reasons) {
        assert_eq!((declared.code, declared.id.as_str(), declared.verdict), (reason.code(), reason.id(), reason.verdict()), "{} spells the schema's own code, id and verdict", reason.id());
    }
    let actions: Vec<&str> = dispatchable_actions(&definition).into_iter().map(|action| action.id.as_str()).collect();
    for retired in ["fillBuildTick", "cancelFillBuild"] {
        assert!(!actions.contains(&retired), "{retired} is a per-plugin run verb the framework tool run replaces");
    }
    for id in TOOL_RUN_ACTION_IDS {
        assert!(actions.contains(&id), "declaring a run injects {id}");
    }
}

/// ⏯️ Red→green successor of the lock-is-commit tick law (`📋️tool-run-contract.md` §5 W1-F): a fill run holds
/// every placement provisionally in the instance-owned tool run ledger — the committed document never moves
/// while it runs and the viewport renders committed ⊕ provisional — and finalize publishes ONE edit carrying
/// every provisional placement, which one undo removes and one redo restores. No process-global run state
/// exists, so the law holds at any test thread count.
#[semio_framework_async_macros::async_test]
async fn fill_run_finalize_publishes_one_edit_with_every_provisional_placement() {
    use semio_framework_tool_run::TOOL_RUN_FINALIZE_ACTION_ID;
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("setFillCount");
    let committed = committed_document(&app);
    let objects = object_count(&app);
    assert_eq!(start_fill_run(&mut app).await["toolRun"], json!("spawnJob"));
    pump_fill_run(&mut app, "the run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let placed = fill_run_presence(&app).completed as usize;
    assert_eq!(placed, 3, "the run places exactly what was requested");
    assert_eq!(committed_document(&app), committed, "a complete run has committed nothing yet");
    assert_eq!(rendered_object_count(&mut app).await, objects + placed, "the viewport renders committed ⊕ provisional");
    let provisional_instances = |node: &Value| instances_of(node).iter().filter(|instance| instance["provisional"] == json!(true)).count();
    assert_eq!(provisional_instances(&render_composite(&mut app).await), placed, "exactly the provisional placements carry the provisional instance style");
    let identity = fill_run_identity(&mut app).await.expect("a complete run is live until it is finalized");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_FINALIZE_ACTION_ID, identity).await["toolRun"], json!("beginFinalize"));
    pump_fill_run(&mut app, "finalize publishes", |presence| presence.state == protocol::PresenceToolRunState::Finalized).await;
    assert_eq!(object_count(&app), objects + placed, "finalize publishes every provisional placement");
    assert_eq!(rendered_object_count(&mut app).await, objects + placed, "the overlay is released onto the committed document");
    assert_eq!(provisional_instances(&render_composite(&mut app).await), 0, "finalized placements render as committed objects");
    assert!(fill_run_identity(&mut app).await.is_none(), "a finalized run is terminal, so Escape no longer aborts it");
    let finalized = committed_document(&app);
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "one undo removes every placement: start → complete → finalize is ONE undo entry");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(committed_document(&app), finalized, "one redo restores all of them");
}

/// 🛑 Red→green (`📋️tool-run-contract.md` §5 W1-F): the run's abort is offered from `starting` onwards with the
/// run's current identity, so it exists whenever provisional pieces are on screen. Aborting leaves zero
/// provisional placements and a committed document byte-identical to the one at start — no edit, no undo entry.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run() {
    use semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID;
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 40 })), None).await.expect("setFillCount");
    let committed = committed_document(&app);
    let objects = object_count(&app);
    start_fill_run(&mut app).await;
    assert!(fill_run_identity(&mut app).await.is_some(), "abort is enabled while the run is still starting");
    pump_fill_run(&mut app, "a provisional placement", |presence| presence.completed > 0).await;
    assert!(rendered_object_count(&mut app).await > objects, "provisional pieces are on screen before the abort");
    let identity = fill_run_identity(&mut app).await.expect("abort carries the run's current identity in every non-terminal state");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_ABORT_ACTION_ID, identity).await["toolRun"], json!("closeJob"));
    pump_fill_run(&mut app, "the abort settles", |presence| presence.state == protocol::PresenceToolRunState::Aborted).await;
    assert_eq!(committed_document(&app), committed, "an aborted run leaves the committed document byte-identical");
    assert_eq!(rendered_object_count(&mut app).await, objects, "an aborted run holds zero provisional placements");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "and left no undo entry behind");
}

/// 🛑 Red→green (`📋️tool-run-contract.md` §5 W1-F): Escape (`engagementAbort`) maps to the framework-reserved
/// `toolRunAbort` of the run this window shows while it is non-terminal, and disarms the tool. The abort only
/// BEGINS inside that call — the run is `aborting` — and the teardown is spent in the driver's bounded close
/// turns, never inside one turn. Nothing was ever written, so no artifact lane is involved.
#[semio_framework_async_macros::async_test]
async fn engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one() {
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 40 })), None).await.expect("setFillCount");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("arm fill");
    let committed = committed_document(&app);
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "a provisional placement", |presence| presence.completed > 0).await;
    let identity = fill_run_identity(&mut app).await.expect("a live fill run");
    let run = identity[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID].as_str().and_then(|run| run.parse().ok()).expect("run id");
    let generation = identity[semio_framework_tool_run::TOOL_RUN_ARG_GENERATION].as_u64().and_then(|generation| u32::try_from(generation).ok()).expect("generation");
    app.echo_tool_run_trace_cursor(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, semio_framework_tool_run::ToolRunTraceCursor { run, generation, page: 1 });
    let escape = dispatch_unsettled(&mut app, "engagementAbort", None, None).await.expect("Escape");
    let settled = settle(&mut app).await;
    let effects: Vec<&Effect> = escape.requested_effects.iter().chain(&settled.effects).collect();
    let abort = effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID => args.clone(),
            _ => None,
        })
        .expect("Escape dispatches toolRunAbort while a fill run is non-terminal");
    assert!(effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())), "Escape also disarms the Fill tool");
    let abort = semio_framework_pack_json::from_dsl_value(&abort);
    assert_eq!(abort[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID], identity[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID], "the abort names the run this window shows");
    tool_run_action(&mut app, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, abort).await;
    assert_eq!(fill_run_presence(&app).state, protocol::PresenceToolRunState::Aborting, "the abort only begins inside the call");
    let turns = pump_fill_run(&mut app, "the teardown", |presence| presence.state == protocol::PresenceToolRunState::Aborted).await;
    assert!(turns >= 1, "the run tears down in the driver's close turns, never inside the abort's own turn");
    assert_eq!(committed_document(&app), committed, "nothing was ever written, so nothing is rolled back");
}

/// 🎚️ Red→green (`📋️tool-run-contract.md` §5 W1-F): a fresh document asks for the default of 100, and
/// `setFillCount` is configuration only — it never emits an artifact mutation. During a run it reconfigures
/// that run (`reconfigure: resume`): the same run id continues under a newer generation.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count() {
    async fn fill_count_entry(app: &mut Puzzle3dApp) -> Option<f64> {
        let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
        let measures = app.tool_measures(&view).await;
        find_measure_number(measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count")
    }
    let mut app = app().await;
    assert_eq!(fill_count_entry(&mut app).await, Some(100.0), "a fresh document asks for the default of 100");
    let committed = committed_document(&app);
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("setFillCount through the migrated tool-job path");
    assert_eq!(fill_count_entry(&mut app).await, Some(3.0), "setFillCount updates the requested count");
    assert_eq!(committed_document(&app), committed, "the count is configuration: no artifact mutation");
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "the first run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let before = fill_run_identity(&mut app).await.expect("a live run");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 5 })), None).await.expect("setFillCount during a run");
    pump_fill_run(&mut app, "the reconfigured run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 5).await;
    let after = fill_run_identity(&mut app).await.expect("the same run is still live");
    assert_eq!(after["runId"], before["runId"], "a count change resumes the SAME run");
    assert!(after["generation"].as_u64() > before["generation"].as_u64(), "under a newer generation");
    assert_eq!(committed_document(&app), committed, "a reconfigured run still committed nothing");
}

/// 🔁️ A count raised during a run continues the same deterministic sequence, a lowered one retracts its tail:
/// the provisional placements at 2 ⊂ at 4 ⊂ at 6, all under one run id, and finalize commits exactly the
/// lowered count.
#[semio_framework_async_macros::async_test]
async fn raising_and_lowering_the_fill_count_during_a_run_continues_and_retracts_the_same_sequence() {
    use semio_framework_tool_run::TOOL_RUN_FINALIZE_ACTION_ID;
    async fn provisional_ids(app: &mut Puzzle3dApp, committed: &std::collections::BTreeSet<String>) -> std::collections::BTreeSet<String> {
        instances_of(&render_composite(app).await).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str)).filter(|id| !committed.contains(*id)).map(str::to_string).collect()
    }
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 4 })), None).await.expect("setFillCount 4");
    let committed: std::collections::BTreeSet<String> = instances_of(&render_composite(&mut app).await).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    let objects = object_count(&app);
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "four placements", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let run = fill_run_identity(&mut app).await.expect("live run")["runId"].clone();
    let four = provisional_ids(&mut app, &committed).await;
    assert_eq!(four.len(), 4);
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 6 })), None).await.expect("raise to 6");
    pump_fill_run(&mut app, "six placements", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 6).await;
    let six = provisional_ids(&mut app, &committed).await;
    assert!(four.is_subset(&six) && six.len() == 6, "a raise continues the sequence: {four:?} ⊂ {six:?}");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 2 })), None).await.expect("lower to 2");
    pump_fill_run(&mut app, "two placements", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 2).await;
    let two = provisional_ids(&mut app, &committed).await;
    assert!(two.is_subset(&four) && two.len() == 2, "a lower retracts the tail: {two:?} ⊂ {four:?}");
    let identity = fill_run_identity(&mut app).await.expect("live run");
    assert_eq!(identity["runId"], run, "raise and lower both resume the same run");
    tool_run_action(&mut app, TOOL_RUN_FINALIZE_ACTION_ID, identity).await;
    pump_fill_run(&mut app, "finalize", |presence| presence.state == protocol::PresenceToolRunState::Finalized).await;
    assert_eq!(object_count(&app), objects + 2, "finalize commits exactly the lowered count");
}

/// ♾️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §1 decisions 1 and 4 — the count is any `u32`.
/// Nothing clamps a request against a planner ceiling; a document that cannot hold the ask reports a
/// stall step instead.
#[semio_framework_async_macros::async_test]
async fn the_fill_count_has_no_ceiling_and_the_entry_declares_none() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 5_000 })), None).await.expect("setFillCount");
    let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(5_000.0), "a request far above anything planned is kept verbatim — no clamp");
    assert_eq!(find_measure_number_max(tool_measures, "puzzle3d-fill-count"), Some(None), "the count entry declares no maximum");
    assert_eq!(crate::editor::puzzle3d::commands::set_fill_count::parse_count(Some(&json!({ "value": 4_000_000_000u32 }))), 4_000_000_000, "parse_count carries the whole u32 range");
}

/// 🪣️ The count is shared configuration: a count set in one split pane is the count every pane shows.
#[semio_framework_async_macros::async_test]
async fn fill_count_is_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 6 })), Some(top)).await.expect("setFillCount in the top pane");
    for window in [top, perspective] {
        let view = app.window_view(window);
        let measures = app.tool_measures(&view).await;
        assert_eq!(find_measure_number(measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count"), Some(6.0), "{window} shows the shared count");
    }
}

/// 🔢️ The count control is an unbounded `Number` entry showing the REQUESTED count; progress and placements
/// are the framework ToolRun panel's, so the entry carries no `ready` extent and loads only while a fill run
/// is live.
#[test]
fn fill_count_measure_is_an_unbounded_number_entry_reporting_the_requested_count() {
    let scene = Puzzle3dScene { scene_snapshot: nakagin_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    match fill_tool::count_measure(&scene, &Puzzle3dLabels::NATIVE_EN, None) {
        WindowMeasure::Number { label: Some(label), value, min, max, ready, loading, .. } => {
            assert_eq!(label, Puzzle3dLabels::NATIVE_EN.count.as_str());
            assert_eq!(value, scene.runtime.fill_count as f64, "the entry shows the REQUESTED count, verbatim");
            assert_eq!((min, max, ready, loading), (Some(0.0), None, None, None), "no ceiling, no locked extent, and no loading ring without a run");
        }
        other => panic!("expected a Number measure, got {other:?}"),
    }
}

/// 🔁️ A committed `setFillCount` survives the renders that follow it: the config mutation IS the count.
#[semio_framework_async_macros::async_test]
async fn a_committed_fill_count_survives_the_renders_that_follow_it() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 250 })), None).await.expect("setFillCount");
    for _ in 0..3 {
        render_composite(&mut app).await;
    }
    let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(250.0), "three renders later the requested count is still what the user asked for");
}
//#endregion 🔖️Fill

//#region 🔖️Distribution
#[semio_framework_async_macros::async_test]
async fn puzzle3d_normalize_kind_weight_group_redistributes_siblings_proportionally() {
    let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut weights = HashMap::from([("a".to_string(), 0.2), ("b".to_string(), 0.3), ("c".to_string(), 0.5)]);
    weights = puzzle3d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
    let sum: f64 = ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    assert!((sum - 1.0).abs() < 1e-9, "simplex must stay at 1, got {sum}");
    assert!((weights.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 1e-9);
    assert!((weights.get("b").copied().unwrap_or(0.0) - 0.5 * 0.3 / 0.8).abs() < 1e-9);
    assert!((weights.get("c").copied().unwrap_or(0.0) - 0.5 * 0.5 / 0.8).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_measure_exposes_joint_weight_scaled_by_object() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let object_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let measures = puzzle3d_joint_vortex_measures("Object", object_weight, &vortex_ids, &object_weights, &joint_weights);
    let expected_joint = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    match &measures[0] {
        WindowMeasure::Slider { value, max, step, disabled, .. } => {
            assert!((*value - expected_joint).abs() < 1e-9, "slider must show the joint probability, got {value}");
            assert!((*max - 1.0).abs() < 1e-9, "joint range spans the global simplex");
            assert_eq!(*step, Some(0.01));
            assert_eq!(*disabled, None);
        }
        other => panic!("expected vortex slider, got {other:?}"),
    }
    puzzle3d_set_object_kind_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", 0.8);
    let raised_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let raised_measures = puzzle3d_joint_vortex_measures("Object", raised_weight, &vortex_ids, &object_weights, &joint_weights);
    match (&measures[0], &raised_measures[0]) {
        (WindowMeasure::Slider { value: before, .. }, WindowMeasure::Slider { value: after, .. }) => {
            assert!(*after > *before, "raising P(object) must raise joint vortex percentages");
            let expected_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
            assert!((*after - expected_after).abs() < 1e-9);
        }
        _ => panic!("expected vortex sliders"),
    }
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_distribution_lists_global_vortices_and_joints_sum_to_one() {
    let scene_snapshot = nakagin_scene_snapshot();
    let object_ids = puzzle3d_kind_ids(&scene_snapshot, "objects");
    let vortex_ids = puzzle3d_kind_ids(&scene_snapshot, "vortices");
    assert!(object_ids.len() >= 2, "default scene_snapshot needs multiple object kinds");
    assert!(vortex_ids.len() >= 2, "default scene_snapshot needs multiple vortex kinds");
    let object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_kind_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
    let scene = Puzzle3dScene { scene_snapshot, runtime: Puzzle3dRuntime { object_kind_weights, vortex_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    let distribution_children = puzzle3d_distribution_children(&scene, Some(true));
    assert_eq!(distribution_children.len(), object_ids.len());
    let mut joint_sum = 0.0;
    for measure in &distribution_children {
        let WindowMeasure::Group { children, value: Some(object_weight), .. } = measure else {
            panic!("expected object-kind group");
        };
        assert_eq!(children.len(), vortex_ids.len(), "each object must list the full global vortex catalog");
        let local_sum: f64 = children
            .iter()
            .map(|child| match child {
                WindowMeasure::Slider { value, .. } => *value,
                _ => panic!("expected vortex slider"),
            })
            .sum();
        assert!((local_sum - object_weight).abs() < 1e-6, "under one object joints sum to P(object), not 1");
        joint_sum += local_sum;
    }
    assert!((joint_sum - 1.0).abs() < 1e-6, "all nested joint percentages across objects must sum to 1, got {joint_sum}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_object_weight_change_scales_joint_sampling_product() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let joint_before = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    puzzle3d_set_object_kind_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", 0.6);
    let joint_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    assert!(joint_after > joint_before);
    let global = puzzle3d_global_joint_total(&joint_weights, &object_weights, &object_ids, &vortex_ids);
    assert!((global - 1.0).abs() < 1e-6, "global joints must stay normalized, got {global}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_joint_edit_redistributes_siblings_and_preserves_object_total() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let object_total_before = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let sibling_before = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "b-s");
    let new_joint = object_total_before * 0.7;
    puzzle3d_set_vortex_joint_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", "c-b", new_joint);
    let object_total_after = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    assert!((object_total_after - object_total_before).abs() < 1e-6);
    let sibling_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "b-s");
    assert!(sibling_after < sibling_before, "raising one vortex must lower its siblings");
    let global = puzzle3d_global_joint_total(&joint_weights, &object_weights, &object_ids, &vortex_ids);
    assert!((global - 1.0).abs() < 1e-6);
}

/// 🚫️ Zero object-kind weight disables every vortex slider under that kind — anything × 0 is 0.
#[semio_framework_async_macros::async_test]
async fn zero_object_kind_weight_disables_joint_vortex_sliders() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene_snapshot = nakagin_scene_snapshot();
    let object_ids = puzzle3d_kind_ids(&scene_snapshot, "objects");
    assert!(!object_ids.is_empty(), "default scene_snapshot must expose object kinds");
    let zeroed_id = object_ids[0].clone();
    let mut object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    object_kind_weights = puzzle3d_normalize_kind_weight_group(&object_kind_weights, &object_ids, &zeroed_id, 0.0);
    assert!(object_kind_weights.get(&zeroed_id).copied().unwrap_or(1.0) <= f64::EPSILON);
    let scene = Puzzle3dScene { scene_snapshot, runtime: Puzzle3dRuntime { object_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&scene, labels, None);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    let zeroed_group_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{zeroed_id}");
    let zeroed_group = distribution_children.iter().find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == &zeroed_group_id)).expect("zeroed object kind must appear in distribution");
    match zeroed_group {
        WindowMeasure::Group { value: Some(value), children, .. } => {
            assert!(*value <= f64::EPSILON, "object-kind header must read 0%");
            assert!(!children.is_empty(), "object kind must still list vortex sliders");
            assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: Some(true), value, .. } if *value <= f64::EPSILON)), "every joint vortex slider under a 0% object kind must be disabled at 0%");
        }
        other => panic!("expected object-kind group, got {other:?}"),
    }
    let live_group = distribution_children.iter().find(|measure| match measure {
        WindowMeasure::Group { id, value: Some(value), .. } if id != &zeroed_group_id => *value > f64::EPSILON,
        _ => false,
    });
    if let Some(WindowMeasure::Group { children, .. }) = live_group {
        assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: None | Some(false), .. })), "joint vortex sliders under a non-zero object kind must stay enabled");
    }
}

/// 🎯️ Fill tool measures expose count + nested distribution tree under the Fill toggle; the Volume
/// Brush voxel dims live in a utility-options group in the window's own measures.
#[semio_framework_async_macros::async_test]
async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
    {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let fill_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&fill_scene, labels, None);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    assert!(!fill_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle3d-play-tool-options-fill")), "fill must not wrap its options in a nested Fill group — the tool toggle already owns that row");
    assert_eq!(measure_group_tag(&fill_measures, &distribution_id), Some(None));
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    assert!(!distribution_children.is_empty(), "distribution must list object-kind groups");
    assert!(distribution_children.iter().all(|measure| matches!(measure, WindowMeasure::Group { value: Some(_), on_change: Some(_), .. })), "each object-kind group must carry a header weight slider");
    assert!(
        distribution_children.iter().all(|measure| match measure {
            WindowMeasure::Group { label, value: Some(_), on_change: Some(_), .. } => !label.contains('%'),
            _ => false,
        }),
        "object-kind group labels must not embed percentages — the header slider owns the value readout"
    );
    assert!(
        distribution_children.iter().any(|measure| match measure {
            WindowMeasure::Group { children, .. } => children.iter().any(|child| matches!(child, WindowMeasure::Slider { label: Some(label), .. } if !label.contains('%'))),
            _ => false,
        }),
        "vortex joint sliders must label kinds without embedding percentages"
    );
    assert!(find_measure_toggle(&fill_measures, "puzzle3d-edit-volumes").is_none(), "fill must not carry edit-volumes toggle");
    assert!(find_measure_slider(&fill_measures, "puzzle3d-voxel-w").is_none(), "fill must not carry voxel-dimension sliders");
    assert!(find_measure_number(&fill_measures, "puzzle3d-fill-count").is_some(), "the fill-count entry always lives in the fill tool measures");
    assert!(
        !main::window_measures(&fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
        "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
    );
    let volume_brush_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::volume_brush::UTILITY_ID.into() };
    let volume_brush_measures = main::window_measures(&volume_brush_scene, labels);
    assert_eq!(measure_group_tag(&volume_brush_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush")), Some(Some(utilities::volume_brush::UTILITY_ID.into())));
    assert!(find_measure_slider(&volume_brush_measures, "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
    let fill_engagement = main::engagement(&fill_scene, &Puzzle3dLabels::NATIVE_EN, None);
    assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
    let brush_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(measure_group_tag(&main::window_measures(&brush_scene, labels), &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())));
    let brush_engagement = main::engagement(&brush_scene, &Puzzle3dLabels::NATIVE_EN, None);
    assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
    }
    assert_brush_options_after_open_vortex_suggestions().await;
}

#[inline(never)]
async fn assert_brush_options_after_open_vortex_suggestions() {
    let mut app = app().await;
    activate_window_utility(&mut app, utilities::brush::UTILITY_ID);
    let view = app.window_view(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    let brush_app_measures = app.window_measures(&view).await;
    let window_measures = brush_app_measures.get(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())), "the brush Utility Options group surfaces on the live window-measures path when brush is the active utility");
}
//#endregion 🔖️Distribution
