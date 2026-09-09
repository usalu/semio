
use super::*;
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, PluginCloseStep, VcsArtifactApp, ViewModel, testkit};

pub type Puzzle3dRawApp = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;

/// 🧪️ The one puzzle3d fixture app. Wraps the raw wrapper so every fixture drains its stores on the
/// way out: a registry-backed `VcsArtifactApp` installs framework-owned `ArtifactStoreCursorDisposer`
/// members whose own `Drop` asserts terminal-empty ownership (`🏪️store/🦀️.rs`), so a bare drop panics
/// inside a destructor. `Drop` here therefore NEVER panics and never asserts: a second panic while a
/// failing assertion is already unwinding is a non-unwinding abort that kills the whole test binary
/// and hides the assertion that actually failed. A drain that cannot reach the witness leaks the raw
/// app instead ([`std::mem::forget`]) — leaking a fixture inside a test process costs nothing, and the
/// close contract itself is stated exactly once, explicitly, by [`close_witness`].
pub struct Puzzle3dApp(Option<Puzzle3dRawApp>);

impl std::ops::Deref for Puzzle3dApp {
    type Target = Puzzle3dRawApp;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("fixture app was already consumed by close_witness")
    }
}

impl std::ops::DerefMut for Puzzle3dApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect("fixture app was already consumed by close_witness")
    }
}

/// 🧹️ Drives `app` through the real `PluginApp` close state machine, one item and one envelope page
/// per turn exactly as a host actor tick does, and returns the terminal-empty witness or the fault
/// that stopped it. Bounded only as a runaway guard: this app carries five stores plus its retained
/// tool operations, which outgrows `testkit::close_registered_fixture_app`'s own 64-turn cap.
fn drain_close(app: &mut Puzzle3dRawApp) -> Result<bool, Fault> {
    for _ in 0..1_048_576 {
        if app.close_terminal_is_empty() {
            return Ok(true);
        }
        if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)? == PluginCloseStep::Complete {
            break;
        }
    }
    Ok(app.close_terminal_is_empty())
}

/// 🧹️ Consumes one fixture app through its real close state machine and reports the outcome, so the
/// close contract is asserted by a test rather than by a destructor. `Err` carries the framework's own
/// fault; `Ok(false)` means the machine reported `Complete` without reaching terminal-empty ownership.
pub fn close_witness(mut app: Puzzle3dApp) -> Result<bool, Fault> {
    let mut raw = app.0.take().expect("fixture app was already consumed by close_witness");
    match drain_close(&mut raw) {
        Ok(true) => Ok(true),
        other => {
            std::mem::forget(raw);
            other
        }
    }
}

impl Drop for Puzzle3dApp {
    fn drop(&mut self) {
        let Some(mut raw) = self.0.take() else {
            return;
        };
        if !matches!(drain_close(&mut raw), Ok(true)) {
            std::mem::forget(raw);
        }
    }
}

pub fn meta(actor: &str) -> ActionMeta {
    testkit::meta(actor)
}

/// 🌉️ `new_app_with_registry`'s `manifest: fn() -> App` shape predates the `AppDefinition`-returning
/// `create_puzzle3d_app()` convention (contract §2.4 / SDK gap 3) — this tiny local wrapper bridges
/// the two, mirroring `📓️w2-cad-report.md`'s recipe step 7.
pub fn puzzle3d_manifest_for_testkit() -> App {
    App { definition: create_puzzle3d_app(), examples: Vec::new() }
}

/// 🧰️ The registry-backed, instance-bound fixture app — the ONLY constructor this plugin can use.
/// puzzle3d declares `bounded_first_step_tool_proofs!`, so `VcsArtifactApp::with_registry_on_bus`'s
/// `registry.tool_job_registration::<A>(…)` join needs the manifest's migrated generated
/// declarations; the registry-less `testkit::new_app` carries an `AppActionRegistry::default()` and
/// fails closed with `interactive-job.catalog-authority` before any dispatch is attempted. The bound
/// instance id is what `advance_typed_operation_publication`/`maintenance_step` and the local
/// interaction query authority key their live-runtime bookkeeping on, exactly as the host binds it.
pub async fn app() -> Puzzle3dApp {
    let mut app = testkit::new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_testkit).await;
    app.bind_instance_id(1).await;
    Puzzle3dApp(Some(app))
}

/// 🪪️ The instance identity every fixture binds, and therefore the receiver `take_typed_operation_result_page`
/// answers for.
pub const FIXTURE_INSTANCE_ID: u32 = 1;

/// 🔁️ Runs the host's continuation loop to quiescence and hands back the effects/events/UI scope the
/// host would have forwarded to the shell. A registry-backed `VcsArtifactApp` does NOT apply a retained
/// tool job inline — `dispatch_typed` mints a `ToolOperationSpec` whose worker, store publication,
/// result page and outboxes are advanced only by later continuation turns, which is why the
/// registry-less fixture used to see mutations land synchronously and this one does not. One turn is
/// exactly the host's own: `maintenance_step` (worker + retirement stages), one
/// `advance_typed_operation_publication` unit, the result page for the bound receiver plus its
/// mandatory ACK, then one effect, one event and one UI scope. The ACK is what actually retires the
/// operation — without it the app stays pending forever. Bounded only as a runaway guard.
pub async fn settle(app: &mut Puzzle3dApp) -> (Vec<Effect>, Vec<semio_framework_plugin::AppEvent>, Option<UiDirtyScope>) {
    let (mut effects, mut events, mut scope) = (Vec::new(), Vec::new(), None);
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            return (effects, events, scope);
        }
        app.maintenance_step(1, SETTLE_TURN_BYTES).expect("maintenance step drives the mounted operation's worker and retirement stages");
        app.advance_typed_operation_publication().await.expect("advance one typed operation publication unit");
        if let Some(page) = app.take_typed_operation_result_page(FIXTURE_INSTANCE_ID) {
            assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "retained operation faulted: {}", String::from_utf8_lossy(page.bytes()));
            assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge one presented result page"), "the app's own presented result page must accept its exact token");
        }
        effects.extend(app.take_typed_operation_effect());
        events.extend(app.take_typed_operation_event());
        scope = app.take_typed_operation_ui_scope().or(scope);
        acknowledge_local_interaction_pages(app);
    }
    panic!("puzzle3d app never quiesced: pending typed operations outlived the settle budget");
}

/// 📏️ One continuation turn's byte grant. Deliberately one fixed envelope page, not a huge number: the
/// point of the loop is that every unit is bounded exactly as a host actor tick bounds it.
const SETTLE_TURN_BYTES: usize = 16_384;

/// 🕹️ Plays the CLIENT half of the local interaction query protocol: drains the replies the app
/// published this turn and acknowledges every page, which is what returns the document snapshot-read
/// leases the query captured — the same Started → Page → acknowledge → Closed round trip
/// `local_interaction_query_return_does_not_fault_the_next_maintenance_step` drives by hand.
fn acknowledge_local_interaction_pages(app: &mut Puzzle3dApp) {
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "the app's own local-interaction page must accept its exact token");
        }
    }
}

/// 🔁️ Folds one settled turn's forwarded output into the invocation the shell observes, so a test reads
/// the same `(requested_effects, events, ui_scope)` triple a client would after the continuation turns.
async fn settle_into(app: &mut Puzzle3dApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
    let (effects, events, scope) = settle(app).await;
    result.map(|mut result| {
        result.requested_effects.extend(effects);
        result.events.extend(events);
        if let Some(scope) = scope {
            result.ui_scope = scope;
        }
        result
    })
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle3dCommand` from the same
/// `(action, args, window_id)` triple every pre-B1 test already passed.
pub async fn dispatch(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    // 🕰️ Framework-reserved verbs stay on `handle_action`: this is exactly the `skip` set
    // `PluginBuilder`'s own declared-action bridge check uses (`🧰️framework/…/🔌️plugin/🦀️.rs`), i.e. every
    // verb the framework injects and handles itself. A reserved verb sent down the typed channel faults
    // `interactive-job.missing-factory` — the app owns no tool proof for a verb it never declared.
    if matches!(
        action,
        "undo"
            | "redo"
            | "commitCheckpoint"
            | "createAlternative"
            | "switchAlternative"
            | "checkoutCheckpoint"
            | "revertToCommand"
            | "setHistoryCommandFilter"
            | "noteShellCommand"
            | "recordTutorial"
            | "startIntroduction"
            | "startTutorial"
            | "copy"
            | "cut"
            | "paste"
            | "setActiveUtility"
            | "setActiveTool"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
    ) {
        let dsl_args = args.map(json::to_dsl_value);
        let reserved = app.handle_action(action, dsl_args.as_ref(), &meta("local")).await;
        return settle_into(app, reserved).await;
    }
    let typed = app.dispatch_typed(Puzzle3dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)).unwrap_or_else(|| panic!("unknown puzzle3d action id in test: {action}")), &meta("local")).await;
    settle_into(app, typed).await
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `worldPick`/`worldSelect`/`worldVortexSelect`/`setSelection` actions.
pub async fn select_id(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None).await
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionHover`
/// for one `(granularity, id)` pair on the `pointer` channel in the `vortex` domain — the
/// test-side replacement for the deleted `worldHover`/`setHover`/`worldVortexHover`/`setKindHover`
/// actions. `id: None` clears the hover (mirrors the old "hover nothing" call shape).
pub async fn hover_id(app: &mut Puzzle3dApp, granularity: &str, id: Option<&str>) -> Result<InvocationResult, Fault> {
    let targets: Vec<InteractionTarget> = id.map(|id| InteractionTarget { granularity: granularity.into(), id: id.into() }).into_iter().collect();
    let targets_json = to_json_string(&targets);
    dispatch(app, "interactionHover", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets_json })), None).await
}

/// 🖼️ The rendered body, as JSON — every panel/window assertion navigates this value.
pub async fn render_body(app: &mut Puzzle3dApp, body_key: &str) -> Value {
    let tree = app.render(body_key, None, &ViewModel::default()).await.expect("render");
    let mut stack = vec![&tree.root];
    let mut rendered_scene = None;
    while let Some(node) = stack.pop() {
        if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
            if surface.doc_schema.as_str() == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(surface).expect("decode world scene");
                let world3d = json::from_dsl_value(&dsl::ToValue::to_value(&scene));
                rendered_scene = Some(object([("schema".to_string(), Value::from(surface.doc_schema.as_str())), ("world3d".to_string(), world3d)]));
                if scene.interaction_json.is_some() {
                    break;
                }
            }
        }
        stack.extend(node.children.iter());
    }
    let projected = testkit::project_and_retire_fixture_tree(tree).expect("render projection");
    rendered_scene.unwrap_or_else(|| parse(&projected.to_string()).expect("rendered node JSON"))
}

/// 🪟️ The world composite body for one window INSTANCE — the `<body>:<windowInstanceId>` form is
/// how a split pane asks for its own materialized options (see `ArtifactApp::render`).
pub async fn render_window(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    render_body(app, &format!("{}:{window_id}", main::BODY_KEY)).await
}

pub async fn render_composite(app: &mut Puzzle3dApp) -> Value {
    render_body(app, main::BODY_KEY).await
}

pub fn projection_of(app: &Puzzle3dApp) -> Value {
    parse(&app.snapshot().expect("projection").value().to_string()).expect("snapshot JSON")
}

pub fn object_count(app: &Puzzle3dApp) -> usize {
    projection_of(app).get("objects").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
}

pub fn first_object_id(app: &Puzzle3dApp) -> String {
    projection_of(app).get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("first object id").to_string()
}

pub fn vortex_full_ids(app: &Puzzle3dApp) -> Vec<String> {
    let projection = projection_of(app);
    let mut ids = Vec::new();
    for object in projection.get("objects").and_then(Value::as_array).into_iter().flatten() {
        let object_id = object.get("id").and_then(Value::as_str).unwrap_or_default();
        for vortex in object.get("vortices").and_then(Value::as_array).into_iter().flatten() {
            if let Some(vortex_id) = vortex.get("id").and_then(Value::as_str) {
                ids.push(puzzle3d_vortex_full_id(object_id, vortex_id));
            }
        }
    }
    ids
}

pub fn first_vortex_full_id(app: &Puzzle3dApp) -> String {
    vortex_full_ids(app).into_iter().next().expect("seed vortex")
}

//#region 🔖️SceneProbes
fn scene_field(node: &Value, field: &str) -> Value {
    node.pointer(&format!("/world3d/{field}")).and_then(Value::as_str).and_then(|raw| parse(raw).ok()).unwrap_or(Value::Null)
}

pub fn instances_of(node: &Value) -> Vec<Value> {
    scene_field(node, "instancesJson").as_array().cloned().unwrap_or_default()
}

pub fn instance_count(node: &Value) -> usize {
    instances_of(node).len()
}

pub fn vortices_of(node: &Value) -> Vec<Value> {
    scene_field(node, "vorticesJson").as_array().cloned().unwrap_or_default()
}

pub fn interaction_of(node: &Value) -> Value {
    scene_field(node, "interactionJson")
}

pub fn selection_of(node: &Value) -> Value {
    scene_field(node, "selectionJson")
}

pub fn lod_of(node: &Value) -> Value {
    scene_field(node, "lodJson")
}

pub fn camera_of(node: &Value) -> Value {
    scene_field(node, "cameraJson")
}

pub fn brush_preview_of(node: &Value) -> Value {
    scene_field(node, "brushPreviewJson")
}
//#endregion 🔖️SceneProbes

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

pub fn find_measure_slider_max(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Slider { id, max, .. } if id == slider_id => Some(*max),
        WindowMeasure::Group { children, .. } => find_measure_slider_max(children, slider_id),
        _ => None,
    })
}

pub fn find_measure_slider_ready(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Slider { id, ready, .. } if id == slider_id => *ready,
        WindowMeasure::Group { children, .. } => find_measure_slider_ready(children, slider_id),
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

/// 🪣️ How far background fill planning has preloaded, read off the fill tool's own count slider.
pub async fn fill_ready(app: &mut Puzzle3dApp) -> f64 {
    app.tool_measures(&ViewModel::default()).await.get(fill_tool::TOOL_ID).and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count")).unwrap_or(0.0)
}

/// 🪣️ Drives `fillBuildTick` until planning has reached `target` placements (or the budget runs out).
pub async fn drive_fill_until_ready(app: &mut Puzzle3dApp, target: f64) -> f64 {
    for _ in 0..256 {
        dispatch(app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        if fill_ready(app).await >= target {
            break;
        }
        with_puzzle3d_app_mut(|inner| inner.precompute.borrow_mut().drive_enqueued_fill_job_for_test(64));
    }
    fill_ready(app).await
}

/// 🧵️ The retained command completes the bounded fill delta before publishing one atomic result.
pub async fn finish_fill_count(_app: &mut Puzzle3dApp, _result: InvocationResult) -> (usize, std::time::Duration) {
    (0, std::time::Duration::ZERO)
}

pub async fn set_fill_count_and_finish(app: &mut Puzzle3dApp, value: u32, window_id: Option<&str>) -> (usize, std::time::Duration) {
    let result = dispatch(app, "setFillCount", Some(&json!({ "value": value })), window_id).await.expect("begin setFillCount");
    finish_fill_count(app, result).await
}
//#endregion 🔖️MeasureProbes

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
    app.context_menu(&request, &ViewModel::default()).await
}
