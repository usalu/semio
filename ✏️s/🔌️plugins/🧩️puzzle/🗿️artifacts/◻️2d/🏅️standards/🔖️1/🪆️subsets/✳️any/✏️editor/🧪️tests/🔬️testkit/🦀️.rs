use super::*;
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Puzzle2dApp = VcsArtifactApp<EditorApp<Puzzle2dPlayApp>>;

pub fn meta(actor: &str) -> ActionMeta {
    semio_framework_plugin::testkit::meta(actor)
}

pub fn app() -> Puzzle2dApp {
    std::sync::LazyLock::force(&crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE);
    std::sync::LazyLock::force(&crate::examples::puzzle2d::concrete_forest::SOURCE);
    semio_framework::io::resolve_ready(semio_framework_plugin::testkit::new_app::<EditorApp<Puzzle2dPlayApp>>())
}

/// 🧾️ `assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take a `fn() ->
/// App` manifest (framework testkit gap, not this packet's to fix — see the sibling `w2-cad-report`
/// "SDK gaps" §3); `create_puzzle2d_app` now returns `AppDefinition`, so this wraps it.
fn puzzle2d_manifest_for_testkit() -> App {
    App { definition: create_puzzle2d_app(), examples: Vec::new() }
}

/// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
/// utility contract are enforced exactly as in production.
pub fn app_with_registry() -> Puzzle2dApp {
    semio_framework::io::resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest_for_testkit))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle2dCommand` from the same
/// `(action, args, window_id)` triple every pre-B1 test already passed.
pub fn dispatch(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
    // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
    // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
    // setInteractionGranularity to this reserved set.
    if matches!(
        action,
        "undo"
            | "redo"
            | "commitCheckpoint"
            | "createAlternative"
            | "switchAlternative"
            | "checkoutCheckpoint"
            | "copy"
            | "cut"
            | "paste"
            | "revertToCommand"
            | "historyFilter"
            | "noteShellCommand"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
    ) {
        let dsl_args = args.map(dsl::DslValue::from);
        return semio_framework::io::resolve_ready(app.handle_action(action, dsl_args.as_ref(), &meta("local")));
    }
    semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle2dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &meta("local")))
}

/// 🧵️ Drives the same host-owned `DispatchAction` continuation used in production until the example is complete.
/// 🛍️ One dispatch is the whole load: `setActiveExample` drives `Puzzle2dActiveExampleWork` to its
/// terminal emit (retained job and batch path alike), so no `DispatchAction` continuation ladder
/// remains to pump. Returns the mutation count the load committed.
pub fn load_example(app: &mut Puzzle2dApp, example_id: &str) -> usize {
    let result = dispatch(app, "setActiveExample", Some(&json!({ "exampleId": example_id })), None).expect("load example");
    assert!(result.requested_effects.is_empty(), "the example load must not request a continuation effect");
    result.mutations.len()
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `setSelection` action.
pub fn select_id(app: &mut Puzzle2dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
}

pub fn concrete_forest_app() -> Puzzle2dApp {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    app
}

/// 🖼️ The rendered body, serialized — every panel/window assertion greps this string.
pub fn render_body(app: &mut Puzzle2dApp, body_key: &str) -> String {
    let tree = semio_framework::io::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render");
    let mut stack = vec![&tree.root];
    while let Some(node) = stack.pop() {
        if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
            if surface.doc_schema.as_str() == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                let scene: semio_framework_ui_scene::Board2dScene = semio_framework_ui_scene::decode(surface).expect("decode board scene");
                return serde_json::to_string(&json!({ "schema": surface.doc_schema, "board2d": scene })).expect("serialize board scene");
            }
        }
        stack.extend(node.children.iter());
    }
    serde_json::to_string(&tree.root).expect("serialize rendered node")
}

/// 🧾️ A standalone `Puzzle2dScene` for the measure/engagement builders that take one directly.
pub fn scene(fixture: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str) -> Puzzle2dScene {
    Puzzle2dScene { fixture, runtime, active_utility: active_utility.into() }
}

pub fn fixture_of(app: &Puzzle2dApp) -> Value {
    app.snapshot().expect("projection").0
}

pub fn first_node_id(app: &Puzzle2dApp) -> String {
    fixture_nodes(&fixture_of(app))[0].get("id").and_then(|value| value.as_str()).expect("node id").to_string()
}
