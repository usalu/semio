/// 📏️ Measure and tool run probes: the window measure finders, the fill run and brush suggestion pumps, the context menu drivers, the heap witness and the editor source reader.

//#region 🔖️MeasureProbes
/// 🔍️ Depth-first search for a `WindowMeasure::Slider`'s value by id, descending into groups (the
/// fill-count slider nests inside the fill tool's measure group rather than sitting on the engagement).
pub fn find_measure_slider(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Slider { id, value, .. } if id == slider_id => Some(*value),
        WindowMeasure::Group { children, .. } => find_measure_slider(children, slider_id),
        _ => None,
    })
}

pub fn find_measure_number(measures: &[WindowMeasure], number_id: &str) -> Option<f64> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Number { id, value, .. } if id == number_id => Some(*value),
        WindowMeasure::Group { children, .. } => find_measure_number(children, number_id),
        _ => None,
    })
}

pub fn find_measure_number_max(measures: &[WindowMeasure], number_id: &str) -> Option<Option<f64>> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Number { id, max, .. } if id == number_id => Some(*max),
        WindowMeasure::Group { children, .. } => find_measure_number_max(children, number_id),
        _ => None,
    })
}

pub fn find_measure_select(measures: &[WindowMeasure], select_id: &str) -> Option<String> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Select { id, value, .. } if id == select_id => Some(value.clone()),
        WindowMeasure::Group { children, .. } => find_measure_select(children, select_id),
        _ => None,
    })
}

pub fn find_measure_toggle(measures: &[WindowMeasure], toggle_id: &str) -> Option<bool> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Toggle { id, pressed, .. } if id == toggle_id => Some(*pressed),
        WindowMeasure::Group { children, .. } => find_measure_toggle(children, toggle_id),
        _ => None,
    })
}

/// 🎯️ Top-level utility tag of a `WindowMeasure::Group` by id, or `None` when the group is absent.
pub fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
        _ => None,
    })
}

//#endregion 🔖️MeasureProbes

//#region ⏯️ToolRunProbes
/// ⏱️ Host turns one fill run law may spend before it calls the run stuck.
pub const FILL_RUN_TURNS: usize = 262_144;

/// ⏯️ Dispatches one framework tool run action exactly as the host routes it: straight to
/// `handle_action`, applied to the ledger inside the same call and never queued as a typed operation.
/// Answers the action's output (`{"toolRun": effect}` or `{"rejected": code}`).
pub async fn tool_run_action(app: &mut Puzzle3dApp, action: &str, args: Value) -> Value {
    app.ensure_window(main::WINDOW_KIND_ID);
    let action_meta = ActionMeta { view_state: Some(app.window_view(main::WINDOW_KIND_ID)), ..app.meta("local") };
    let (raw, cx) = app.parts();
    let result = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(raw, action, Some(semio_framework_pack_json::to_dsl_value(&args)), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("{action} faulted: {fault:?}"));
    semio_framework_pack_json::from_dsl_value(&result.output)
}

/// 🎬️ Registers the lifecycle scene_snapshot's cube meshes, then starts a fill run through the panel action.
pub async fn start_fill_run(app: &mut Puzzle3dApp) -> Value {
    let scene_snapshot = puzzle3d_scene_snapshot_from_document(app.snapshot().expect("live fill scene_snapshot").typed());
    let (positions, indices) = puzzle3d_fallback_mesh_buffers();
    for url in collect_mesh_urls(&scene_snapshot) {
        assert!(semio_s_artifact_puzzle_3d_precompute::derive_brush_mesh(&url, &positions, &indices).is_some(), "scene_snapshot mesh {url}");
    }
    tool_run_action(app, semio_framework_tool_run::TOOL_RUN_START_ACTION_ID, object([(semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID.to_string(), Value::from(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID))])).await
}

/// 🧭️ The live fill run's `{runId, generation}` exactly as the main window's Escape binding carries it
/// (`on_abort` of its engagement input), or `None` while no fill run is live.
pub async fn fill_run_identity(app: &mut Puzzle3dApp) -> Option<Value> {
    let view = app.window_view(main::WINDOW_KIND_ID);
    let engagements = app.window_engagements(&view).await;
    let abort = engagements.get(main::WINDOW_KIND_ID)?.input.as_ref()?.on_abort.clone()?;
    (abort.action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID).then(|| semio_framework_pack_json::from_dsl_value(abort.args.as_ref().expect("a tool run abort carries its identity")))
}

/// ⏯️ The run's ephemeral-shared presence summary: state, stage, placements and requested count.
pub fn fill_run_presence(app: &Puzzle3dApp) -> protocol::PresenceToolRun {
    app.tool_run_presence().expect("a fill run slot exists")
}

/// 🔁️ Host turns until `until` holds for the run's presence summary; answers the turns it took.
pub async fn pump_fill_run(app: &mut Puzzle3dApp, what: &str, until: impl Fn(&protocol::PresenceToolRun) -> bool) -> usize {
    let mut settled = Puzzle3dSettled::default();
    for turn in 0..FILL_RUN_TURNS {
        if app.tool_run_presence().is_some_and(|presence| until(&presence)) {
            return turn;
        }
        host_turn(app, 1, &mut settled).await;
    }
    panic!("{what} never happened within {FILL_RUN_TURNS} host turns; presence {:?}", app.tool_run_presence());
}

/// 🚦️ Plays the refresh half of the host for the brush suggestions run: dispatches what `pending_effects`
/// asks for exactly as the host routes a tool run action, then one host turn, until `until` holds for the
/// brush run's presence summary; answers the turns it took.
pub async fn pump_brush_suggestions(app: &mut Puzzle3dApp, what: &str, until: impl Fn(Option<&protocol::PresenceToolRun>) -> bool) -> usize {
    let mut settled = Puzzle3dSettled::default();
    for turn in 0..FILL_RUN_TURNS {
        let view = app.window_view(main::WINDOW_KIND_ID);
        for effect in app.pending_effects(Some(&view)).await {
            if let Effect::DispatchAction { action, args, .. } = effect {
                tool_run_action(app, &action, args.as_ref().map(semio_framework_pack_json::from_dsl_value).unwrap_or(Value::Null)).await;
            }
        }
        if until(app.tool_run_presence().as_ref()) {
            return turn;
        }
        host_turn(app, 1, &mut settled).await;
    }
    panic!("{what} never happened within {FILL_RUN_TURNS} host turns; presence {:?}", app.tool_run_presence());
}

/// 🔎️ The brush suggestions search settled: a live run has decided every candidate of its target.
pub fn brush_suggestions_settled(presence: Option<&protocol::PresenceToolRun>) -> bool {
    presence.is_some_and(|presence| presence.state == protocol::PresenceToolRunState::Running && presence.total.is_some_and(|total| total > 0 && presence.completed == total))
}

/// 🎣️ Points the armed brush of the main window at `vortex` and pumps until its search settles.
pub async fn target_brush_suggestions(app: &mut Puzzle3dApp, vortex: &str) {
    dispatch(app, "targetBrushSuggestions", Some(&json!({ "fullId": vortex })), Some(main::WINDOW_KIND_ID)).await.expect("targetBrushSuggestions");
    pump_brush_suggestions(app, "the brush suggestions search settles", brush_suggestions_settled).await;
}

/// 🪪️ The run id the live trace belongs to.
pub fn tool_run_trace_run(app: &Puzzle3dApp) -> Option<u64> {
    let encoded = app.tool_run_trace_delta(None)?;
    Some(semio_framework_tool_run::ToolRunTraceDelta::decode(&semio_framework_io_base64::base64_url_decode(encoded).expect("base64url")).expect("trace delta").identity.id.run)
}

/// 🪞️ Every trace record the ledger holds for the live run, decoded from its base64url delta.
pub fn tool_run_trace_records(app: &Puzzle3dApp) -> Vec<(u64, semio_framework_tool_run::ToolRunVerdict)> {
    let Some(encoded) = app.tool_run_trace_delta(None) else { return Vec::new() };
    let delta = semio_framework_tool_run::ToolRunTraceDelta::decode(&semio_framework_io_base64::base64_url_decode(encoded).expect("base64url")).expect("trace delta");
    let mut store = semio_framework_tool_run::ToolRunTraceStore::new(delta.identity);
    for page in &delta.pages {
        store.apply_page(page).expect("trace page");
    }
    let mut records: Vec<(u64, semio_framework_tool_run::ToolRunVerdict)> = store.records().map(|(key, record)| (key, record.verdict)).collect();
    records.sort_unstable_by_key(|(key, _)| *key);
    records
}

/// 📄️ The committed document, as the exact text the store holds.
pub fn committed_document(app: &Puzzle3dApp) -> String {
    semio_framework_pack_json::to_json_string(app.snapshot().expect("committed snapshot").typed())
}

/// 🪞️ How many objects the main window renders — committed ⊕ provisional while a run holds placements.
pub async fn rendered_object_count(app: &mut Puzzle3dApp) -> usize {
    instance_count(&render_composite(app).await)
}
//#endregion ⏯️ToolRunProbes

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `context_menu` reads the
/// CLIENT-supplied `request.surface.selection` now (selection is framework-owned, no live config
/// field to derive it from) — the test-side replacement for the deleted `contextMenuAt` command's
/// "select then open the menu" round trip.
pub async fn context_menu_for_selection(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: "world3d".into(), kind: "world3d".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: granularity.into(), ids: vec![id.to_string()] }], text: None }),
        window_instance_id: None,
        point: None,
    };
    let view = app.window_view(main::WINDOW_KIND_ID);
    app.context_menu(&request, &view).await
}

/// 🎯️ The menu a right-click on an UNSELECTED entity opens: the surface carries a `hits` entry and an
/// empty `selection`, exactly what `World3dHost` sends when `resolveWorldContextMenuTarget` resolves a
/// pointer target the document has not selected.
pub async fn context_menu_for_hit(app: &mut Puzzle3dApp, domain: &str, id: &str) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{ContextMenuHit, ContextMenuRequest, ContextMenuSurfaceTarget, UiMenuRef};
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: "world3d".into(),
            kind: "world3d".into(),
            hits: vec![ContextMenuHit { domain: domain.into(), id: id.to_string(), label: None }],
            selection: Vec::new(),
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let view = app.window_view(main::WINDOW_KIND_ID);
    app.context_menu(&request, &view).await
}

//#region 🧮️HeapWitness
/// 🧮️ The test binary's own allocator, counting retained bytes across the WHOLE PROCESS. The guest
/// heap this artifact ships into is a fixed `GUEST_LINEAR_MEMORY_MAXIMUM_BYTES` wasm linear memory
/// with ONE allocator and no threads at all, so an owner the tick loop never frees is a hard trap in
/// production and nothing at all in a native suite — the only way a law can state the growth bound
/// is to weigh the heap itself.
///
/// 🧵️ Process-wide, deliberately NOT per-thread: natively a mounted worker session allocates its
/// owners on a pool thread and the tick loop frees them on the caller's, so a per-thread counter
/// reads a growing leak as a NEGATIVE number on the law's own thread and reports the exact opposite
/// of the truth. A growth law therefore runs its ticks with the fill registry guard held
/// ([`crate::editor::puzzle3d::precompute::fill_envelope_test_guard`]) and differences this counter.
///
/// 🧬️ The instrument itself is `semio_framework_trace::HeapWitness`, shared with the procedural
/// artifact's guest-memory laws instead of written twice
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[global_allocator]
static PUZZLE3D_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

//#endregion 🧮️HeapWitness

pub fn edit_rows(settled: &Puzzle3dSettled) -> Vec<semio_framework::kernel::HistoryEntry> {
    settled.completions.iter().filter_map(|completion| completion.history_patch.as_ref()).flat_map(|patch| patch.upserts.iter()).filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()
}

pub fn object_origin(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

pub fn object_origin_x(app: &Puzzle3dApp, object_id: &str) -> f64 {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).and_then(|origin| origin.first()).and_then(Value::as_f64))
        .expect("origin.x")
}

pub fn dispatchable_actions(definition: &semio_framework_plugin::AppDefinition) -> Vec<&ActionDefinition> {
    let mut seen = std::collections::BTreeSet::new();
    definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(definition, window))
        .filter(|action| seen.insert(action.id.clone()))
        .collect()
}

/// 📚️ The editor lib root and every module mounted below it, read as the one text the unsplit root held: the children first, the root last.
pub fn editor_source() -> String {
    let editor = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    ["🧰️retained-commands", "📬️store-preparation", "✂️clipboard-job", "🧩️play-app", "📋️manifest"]
        .iter()
        .map(|child| format!("{editor}/{child}/🦀️.rs"))
        .chain([format!("{editor}/🦀️.rs")])
        .map(|path| std::fs::read_to_string(path).expect("committed test data reads"))
        .collect::<Vec<_>>()
        .join("\n")
}
