
use super::*;
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, testkit};

pub type Puzzle3dApp = VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>;

pub fn meta(actor: &str) -> ActionMeta {
    testkit::meta(actor)
}

pub fn app() -> Puzzle3dApp {
    semio_framework::io::resolve_ready(testkit::new_app::<EditorApp<Puzzle3dPlayApp>>())
}

/// 🌉️ `new_app_with_registry`'s `manifest: fn() -> App` shape predates the `AppDefinition`-returning
/// `create_puzzle3d_app()` convention (contract §2.4 / SDK gap 3) — this tiny local wrapper bridges
/// the two, mirroring `📓️w2-cad-report.md`'s recipe step 7.
pub fn puzzle3d_manifest_for_testkit() -> App {
    App { definition: create_puzzle3d_app(), examples: Vec::new() }
}

/// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
/// utility contract are enforced exactly as in production.
pub fn app_with_registry() -> Puzzle3dApp {
    semio_framework::io::resolve_ready(testkit::new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_testkit))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle3dCommand` from the same
/// `(action, args, window_id)` triple every pre-B1 test already passed.
pub fn dispatch(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
    // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
    // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
    // setInteractionGranularity to this reserved set.
    if matches!(
        action,
        "undo"
            | "redo"
            | "checkpoint"
            | "commitCheckpoint"
            | "createAlternative"
            | "switchAlternative"
            | "checkoutCheckpoint"
            | "alternative"
            | "revertToCommand"
            | "historyFilter"
            | "noteShellCommand"
            | "copy"
            | "cut"
            | "paste"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
    ) {
        let dsl_args = args.map(json::to_dsl_value);
        return semio_framework::io::resolve_ready(app.handle_action(action, dsl_args.as_ref(), &meta("local")));
    }
    semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle3dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)).unwrap_or_else(|| panic!("unknown puzzle3d action id in test: {action}")), &meta("local")))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `worldPick`/`worldSelect`/`worldVortexSelect`/`setSelection` actions.
pub fn select_id(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionHover`
/// for one `(granularity, id)` pair on the `pointer` channel in the `vortex` domain — the
/// test-side replacement for the deleted `worldHover`/`setHover`/`worldVortexHover`/`setKindHover`
/// actions. `id: None` clears the hover (mirrors the old "hover nothing" call shape).
pub fn hover_id(app: &mut Puzzle3dApp, granularity: &str, id: Option<&str>) -> Result<InvocationResult, Fault> {
    let targets: Vec<InteractionTarget> = id.map(|id| InteractionTarget { granularity: granularity.into(), id: id.into() }).into_iter().collect();
    let targets_json = to_json_string(&targets);
    dispatch(app, "interactionHover", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets_json })), None)
}

/// 🖼️ The rendered body, as JSON — every panel/window assertion navigates this value.
pub fn render_body(app: &mut Puzzle3dApp, body_key: &str) -> Value {
    let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
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
pub fn render_window(app: &mut Puzzle3dApp, window_id: &str) -> Value {
    render_body(app, &format!("{}:{window_id}", main::BODY_KEY))
}

pub fn render_composite(app: &mut Puzzle3dApp) -> Value {
    render_body(app, main::BODY_KEY)
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
pub fn fill_ready(app: &mut Puzzle3dApp) -> f64 {
    semio_framework::io::resolve_ready(app.tool_measures()).get(fill_tool::TOOL_ID).and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count")).unwrap_or(0.0)
}

/// 🪣️ Drives `fillBuildTick` until planning has reached `target` placements (or the budget runs out).
pub fn drive_fill_until_ready(app: &mut Puzzle3dApp, target: f64) -> f64 {
    for _ in 0..256 {
        dispatch(app, "fillBuildTick", None, None).expect("fillBuildTick");
        if fill_ready(app) >= target {
            break;
        }
        with_puzzle3d_app_mut(|inner| inner.precompute.borrow_mut().drive_enqueued_fill_job_for_test(64));
    }
    fill_ready(app)
}

/// 🧵️ Drives the same generation-tagged fill continuation the host executes in production.
pub fn finish_fill_count(app: &mut Puzzle3dApp, mut result: InvocationResult) -> (usize, std::time::Duration) {
    let mut max_step = std::time::Duration::ZERO;
    for step in 0..1_024 {
        let next = result.requested_effects.into_iter().find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == set_fill_count::STEP_ACTION_ID => Some(args.map(|value| semio_framework::from_dsl_value::<Value>(value).expect("fill-count step args decode"))),
            _ => None,
        });
        let Some(args) = next else {
            return (step, max_step);
        };
        let started = std::time::Instant::now();
        result = dispatch(app, set_fill_count::STEP_ACTION_ID, args.as_ref(), None).expect("advance fill-count materialization");
        let elapsed = started.elapsed();
        max_step = max_step.max(elapsed);
        assert!(result.mutations.len() <= set_fill_count::MAX_PLACEMENTS_PER_STEP * 2, "one fill-count continuation exceeded its fixed semantic mutation bound");
    }
    panic!("fill-count materialization did not finish within its deterministic step bound");
}

pub fn set_fill_count_and_finish(app: &mut Puzzle3dApp, value: u32, window_id: Option<&str>) -> (usize, std::time::Duration) {
    let result = dispatch(app, "setFillCount", Some(&json!({ "value": value })), window_id).expect("begin setFillCount");
    finish_fill_count(app, result)
}
//#endregion 🔖️MeasureProbes

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `context_menu` reads the
/// CLIENT-supplied `request.surface.selection` now (selection is framework-owned, no live config
/// field to derive it from) — the test-side replacement for the deleted `contextMenuAt` command's
/// "select then open the menu" round trip.
pub fn context_menu_for_selection(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: "world3d".into(), kind: "world3d".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: granularity.into(), ids: vec![id.to_string()] }], text: None }),
        window_instance_id: None,
        point: None,
    };
    semio_framework::io::resolve_ready(app.context_menu(&request))
}
