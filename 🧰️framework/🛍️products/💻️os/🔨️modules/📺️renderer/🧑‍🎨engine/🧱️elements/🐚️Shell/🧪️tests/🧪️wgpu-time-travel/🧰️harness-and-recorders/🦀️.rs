/// 🧰️ Harness and recorders: the corpus and session builders, verb and refusal names, chords, the session shell, the band painter, the action recorder and the retained body painter.

//#region 🧰️Harness
fn corpus() -> Value {
    serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️time-travel-band/🔣️.json")).expect("the shared time-travel band corpus parses")
}

/// 🧾️ The kernel session of the corpus case whose name starts with `name`.
fn session(name: &str) -> HistoryTimeTravel {
    let corpus = corpus();
    let case = corpus["cases"].as_array().expect("cases").iter().find(|case| case["name"].as_str().is_some_and(|case| case.starts_with(name))).unwrap_or_else(|| panic!("a corpus case named {name}…")).clone();
    serde_json::from_value(case["session"].clone()).unwrap_or_else(|error| panic!("{name}: a kernel HistoryTimeTravel: {error}"))
}

fn verb_name(verb: TimeTravelVerb) -> &'static str {
    match verb {
        TimeTravelVerb::NextProblem => "nextProblem",
        TimeTravelVerb::Accept => "accept",
        TimeTravelVerb::Discard => "discard",
        TimeTravelVerb::CancelReplay => "cancelReplay",
        TimeTravelVerb::Rerun => "rerun",
        TimeTravelVerb::Finalize => "finalize",
        TimeTravelVerb::Back => "back",
        TimeTravelVerb::Exit => "exit",
    }
}

/// 🔑️ React's label key of the refusal that disables a control.
fn refusal_key(label: TimeTravelLabel) -> String {
    format!("ui.timeTravel.refusal.{}", label.key().trim_start_matches("refusal").to_lowercase())
}

const ALL_VERBS: [TimeTravelVerb; 8] = [TimeTravelVerb::NextProblem, TimeTravelVerb::Accept, TimeTravelVerb::Discard, TimeTravelVerb::CancelReplay, TimeTravelVerb::Rerun, TimeTravelVerb::Finalize, TimeTravelVerb::Back, TimeTravelVerb::Exit];

fn chord(chord: &str) -> (semio_framework_ui_wgpu_input::KeyAction, PointerModifiers) {
    let mut modifiers = PointerModifiers::default();
    let mut key = semio_framework_ui_wgpu_input::KeyAction::Escape;
    for token in chord.split('+') {
        match token {
            "alt" => modifiers.alt = true,
            "shift" => modifiers.shift = true,
            "ctrl" => modifiers.ctrl = true,
            "meta" | "mod" => modifiers.meta = true,
            "enter" => key = semio_framework_ui_wgpu_input::KeyAction::Enter,
            "backspace" => key = semio_framework_ui_wgpu_input::KeyAction::Backspace,
            "escape" => key = semio_framework_ui_wgpu_input::KeyAction::Escape,
            other => key = semio_framework_ui_wgpu_input::KeyAction::Char(other.to_string()),
        }
    }
    (key, modifiers)
}

/// 🧪️ A shell on the command-registry test app, with the framework History tab the plugin runtime injects.
fn session_shell() -> ShellState {
    let mut app = test_app(Vec::new(), Vec::new());
    app.panel_tabs.push(PanelTabDefinition {
        kind: semio_framework::PanelTabKind::App(FRAMEWORK_PANEL_TAB_HISTORY_ID.into()),
        label: LocalizedLabel::native("History", "Verlauf"),
        group: PanelGroup::Settings,
        body_key: Some("framework.body.history".into()),
        children: vec![],
    });
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.dock.dock.root = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("main"), DockStackTab::new("side")], active: "main".into() };
    shell.dock.dock.active_window_id = Some("main".into());
    shell.windows.active_window_id = Some("main".into());
    shell.docs.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
    shell.chrome.screen_w = 1280.0;
    shell.chrome.screen_h = 720.0;
    shell
}

/// 🖌️ Walks the band's retained step to completion and answers the hits it staged.
fn paint_band(shell: &mut ShellState) -> Vec<HitTarget<ActionDescriptor>> {
    let mut cursor = ShellChromeChildCursor::default();
    let (mut overlay, mut atlas, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), InputState::<ActionDescriptor>::default(), Theme::light());
    for _ in 0..100_000 {
        if OverlaysViewCtx::of(&mut shell).render_time_travel_band_step(&mut cursor, &mut overlay, &mut atlas, &mut input, &theme) {
            return input.staged_hits().to_vec();
        }
    }
    panic!("the time-travel band step never completed");
}

fn hit_events(hits: &[HitTarget<ActionDescriptor>]) -> Vec<(String, Option<String>)> {
    hits.iter().filter_map(|hit| Some((hit.control_id.clone()?, hit.event.as_ref().map(|event| event.action.clone())))).collect()
}

thread_local! {
    static DISPATCHED: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 🎬️ A guest that records every dispatched action and answers each with the corpus's replaying session.
fn record_action(_instance_id: u32, action_json: &str, _view_state: &ViewModel) -> Result<InvocationResult, String> {
    DISPATCHED.with(|log| log.borrow_mut().push(action_json.to_string()));
    Ok(InvocationResult {
        output: DslValue::Null,
        mutations: Vec::new(),
        inverse_group: semio_framework::kernel::UndoGroup { invocation_id: semio_framework::kernel::InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
        diagnostics: Vec::new(),
        requested_effects: Vec::new(),
        events: Vec::new(),
        ui_scope: UiDirtyScope::default(),
        history_patch: Some(HistoryPatch { cursor: 9, time_travel: Some(session("a running replay")), ..Default::default() }),
    })
}

fn dispatched() -> Vec<String> {
    DISPATCHED.with(|log| log.borrow().clone())
}

/// 🎨️ Paints `document` in `surface` to completion with the stepped paint every retained body uses, then registers and
/// publishes its hits, so the surface's accessibility projection is the one the mirror reads; answers the staged input.
fn paint_retained_body(shell: &mut ShellState, surface: &str, document: &UiDocumentLease, body: Rect) -> InputState<ActionDescriptor> {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let (mut draw, mut atlas, icons, theme) = (DrawList::default(), FontAtlas::builtin(), IconAtlas::default(), Theme::default());
    let mut input = InputState::<ActionDescriptor>::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = std::mem::take(&mut shell.surfaces.world3d_states);
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    let mut cursor = UiDocumentFrameCursor::default();
    let painted = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
        let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
        caller.turn(|cx|render_ui_document_step(&mut cursor, document, body, &mut ctx, surface, "test", semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Handle, &mut hosts, cx))
    });
    assert!(painted, "{surface} painted within its opportunity ceiling");
    if surface == FRAMEWORK_PANEL_TAB_HISTORY_ID {
        shell.input.register_retained_panel_scroll_region(surface, body, &mut input);
    }
    shell.input.register_retained_body_hits(surface, body, &mut input);
    shell.surfaces.world3d_states = world3d_states;
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    caller.retire_world(&mut world_resources);
    caller.finish();
    input
}

/// 🧹️ Closes a law's retained surface and retires every document it published there.
fn close_retained_body(surface: &str, documents: Vec<UiDocumentLease>) {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    if semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(surface) {
        while semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface) {
            assert!(caller.turn(|cx|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one(cx)).expect("original fixture caller turn"), "each admitted close opportunity advances");
        }
    }
    for mut document in documents {
        while !document.close_read_step_with_grant(1, 4096).expect("the law releases its captured document reader").complete {}
    }
    caller.finish();
}
//#endregion 🧰️Harness
