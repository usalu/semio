
use super::*;
use semio_framework_plugin::{ActionMeta, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, testkit};

/// ✏️ `Puzzle5dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<Puzzle5dPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<Puzzle5dPlayApp>` builds it.
pub type Puzzle5dApp = VcsArtifactApp<EditorApp<Puzzle5dPlayApp>>;

pub fn meta(actor: &str) -> ActionMeta {
    testkit::meta(actor)
}

pub fn app() -> Puzzle5dApp {
    let mut app = semio_framework::io::resolve_ready(testkit::new_app::<EditorApp<Puzzle5dPlayApp>>());
    semio_framework::io::resolve_ready(app.bind_instance_id(1));
    app
}

/// ✏️ Adapts `create_puzzle5d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::new_app_with_registry` still expects — framework testkit gap, not
/// modifiable here (`🧰️framework/**` is outside this packet's lease).
pub fn puzzle5d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_puzzle5d_app(), examples: Vec::new() }
}

/// 🧰️ A registry-backed app so kind discipline (View actions must emit no operations) and the
/// utility contract are enforced exactly as in production.
pub fn app_with_registry() -> Puzzle5dApp {
    let mut app = semio_framework::io::resolve_ready(testkit::new_app_with_registry::<EditorApp<Puzzle5dPlayApp>>(puzzle5d_app_manifest_for_testkit));
    semio_framework::io::resolve_ready(app.bind_instance_id(1));
    app
}

fn action_window_kind(action: &str) -> &'static str {
    if matches!(action, "setCamera2d" | "setLodMode" | "setGridSnapEnabled" | "setGridFactor" | "setSuggestionOffset" | "setFillCount" | "canvasPointerDown") {
        board2d::WINDOW_KIND_ID
    } else {
        world3d::WINDOW_KIND_ID
    }
}

pub fn window_view(kind: &str, id: &str) -> ViewModel {
    let mut window_instances = vec![
        ViewWindowInstance { id: board2d::WINDOW_KIND_ID.into(), window_kind_id: board2d::WINDOW_KIND_ID.into() },
        ViewWindowInstance { id: world3d::WINDOW_KIND_ID.into(), window_kind_id: world3d::WINDOW_KIND_ID.into() },
    ];
    if !window_instances.iter().any(|window| window.id == id) {
        window_instances.push(ViewWindowInstance { id: id.into(), window_kind_id: kind.into() });
    }
    ViewModel { window_instances, ..Default::default() }.for_window_instance(id).expect("puzzle5d test window roster")
}

fn action_meta(action: &str, args: Option<&Value>, window_id: Option<&str>) -> ActionMeta {
    let requested_window = args.and_then(|value| value.get("windowId").or_else(|| value.get("window"))).and_then(Value::as_str);
    let kind = requested_window.filter(|window| PUZZLE5D_PLAY_WINDOWS.contains(window)).unwrap_or_else(|| action_window_kind(action));
    let id = window_id.or(requested_window).unwrap_or(kind);
    ActionMeta { view_state: Some(window_view(kind, id)), ..meta("local") }
}

fn settle(app: &mut Puzzle5dApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
    let mut result = result?;
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            return Ok(result);
        }
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
        semio_framework::io::resolve_ready(app.advance_typed_operation_publication())?;
        if let Some(page) = app.take_typed_operation_result_page(1) {
            if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                return Err(Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
            }
            app.acknowledge_typed_operation_result(page.token)?;
        }
        result.requested_effects.extend(app.take_typed_operation_effect());
        result.events.extend(app.take_typed_operation_event());
        if let Some(scope) = app.take_typed_operation_ui_scope() {
            result.ui_scope = scope;
        }
    }
    Err(Fault::from("puzzle5d test operation did not settle"))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle5dCommand` from the same
/// `(action, args, window_id)` triple every pre-migration test already passed.
pub fn dispatch(app: &mut Puzzle5dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let action_meta = action_meta(action, args, window_id);
    // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
    // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
    // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
    // setInteractionGranularity to this reserved set.
    if matches!(
        action,
        "undo"
            | "redo"
            | "checkpoint"
            | "alternative"
            | "revertToCommand"
            | "historyFilter"
            | "noteShellCommand"
            | "copy"
            | "cut"
            | "paste"
            | "setActiveUtility"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
    ) {
        let dsl_args = args.map(dsl::os_pack::json::to_dsl_value);
        let result = semio_framework::io::resolve_ready(app.handle_action(action, dsl_args.as_ref(), &action_meta));
        return settle(app, result);
    }
    let result = semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta));
    settle(app, result)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `setSelection` action.
pub fn select_id(app: &mut Puzzle5dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    dispatch(app, "interactionSelect", Some(&dsl::json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
}

/// 🖼️ The rendered body, as a JSON string — every panel/window assertion greps this value.
pub fn render_body(app: &mut Puzzle5dApp, body_key: &str) -> String {
    let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
    let mut scene_json = None;
    let mut stack = vec![&tree.root];
    while let Some(node) = stack.pop() {
        if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
            let scene = match surface.doc_schema.as_str() {
                schema if schema == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                    serde_json::to_value(semio_framework_ui_scene::decode::<semio_framework_ui_scene::Board2dScene>(surface).expect("decode board scene"))
                }
                schema if schema == <semio_framework_ui_scene::World3dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA => {
                    serde_json::to_value(semio_framework_plugin::testkit::built_surface_scene::<semio_framework_ui_scene::World3dScene>(node).expect("assemble world scene"))
                }
                _ => continue,
            }
            .expect("serialize scene");
            scene_json = Some(serde_json::json!({ "schema": surface.doc_schema, "scene": scene }).to_string());
            break;
        }
        stack.extend(node.children.iter());
    }
    let projected = testkit::project_and_retire_fixture_tree(tree).expect("retire rendered node");
    scene_json.unwrap_or(projected)
}

pub fn render_window(app: &mut Puzzle5dApp, body_key: &str, window_id: &str) -> String {
    render_body(app, &format!("{body_key}:{window_id}"))
}

pub fn close_app(app: &mut Puzzle5dApp) {
    for _ in 0..1_048_576 {
        if app.close_terminal_is_empty() {
            return;
        }
        if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Puzzle 5D registered app close") == semio_framework_plugin::PluginCloseStep::Complete {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "Puzzle 5D registered app close did not reach terminal-empty ownership");
}

pub fn projection_of(app: &Puzzle5dApp) -> Value {
    parse(&app.snapshot().expect("projection").0.to_string()).expect("snapshot JSON")
}

pub fn part_count(app: &Puzzle5dApp) -> usize {
    projection_of(app).get("parts").and_then(|value| value.as_array()).map_or(0, Vec::len)
}

pub fn first_part_id(app: &Puzzle5dApp) -> String {
    projection_of(app).get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("first part id").to_string()
}

/// 🎯️ Top-level utility tag of a `WindowMeasure::Group` by id, or `None` when the group is absent.
pub fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
    measures.iter().find_map(|measure| match measure {
        WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
        _ => None,
    })
}

/// 🔍️ Depth-first search for a `WindowMeasure::Slider`'s presence by id, descending into groups.
pub fn has_measure_slider(measures: &[WindowMeasure], slider_id: &str) -> bool {
    measures.iter().any(|measure| match measure {
        WindowMeasure::Slider { id, .. } => id == slider_id,
        WindowMeasure::Group { children, .. } => has_measure_slider(children, slider_id),
        _ => false,
    })
}
