use super::*;
use crate::vcs::forms_bridge;
use canvas::camera::{world_to_screen, Camera, Viewport};
use canvas::Point;
use dag::{computation_node_width, slider_widget_height, HandleRole};
use graph::dsl::{WireEdge, WireNode};
use graph::manifest::PropertyBag;
use neural::{ChannelSpec as InputSpec, OperatorInfo as NeuronKindInfo};
use semio_framework_artifact_infinite_dag::DagPreviewContent;
// 🌿️ The flow ARTIFACT crate's own vcs surface — `crate::vcs` glob-imports it privately, so the
// undo/redo law names it at its source.
use semio_framework_artifact_flow_flow::{flow_fixture_operations, FlowEnvelope};
use std::sync::{Mutex, OnceLock};

const NUMBER_OPS: &[&str] = &["core.number"];
static RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn tree_from_dag_builds_neurons_and_synapses() {
    let nodes = vec![WireNode { id: "a".into(), kind: "core.number".into(), port: None, properties: PropertyBag::new() }, WireNode { id: "b".into(), kind: "math.add".into(), port: None, properties: PropertyBag::new() }];
    let edges = vec![WireEdge { from: "a".into(), from_port: "number".into(), to: "b".into(), to_port: "a".into(), directed: true, properties: PropertyBag::new() }];
    let tree = FlowHost::tree_from_dag(&nodes, &edges);
    assert_eq!(tree.neurons.len(), 2);
    assert_eq!(tree.synapses.len(), 1);
}

fn test_math_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
    if kind == "core.number" {
        let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
        return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value)))));
    }
    if kind == "core.text" {
        let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
        return Ok(channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(value.into())))));
    }
    if kind == "core.image" {
        let value = input.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap_or_default();
        return Ok(channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(value.into())))));
    }
    if kind == "math.add" {
        let a = input.get("a").or_else(|| input.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("a".into()))?;
        let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
        return Ok(channel_output("sum", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(a + b)))));
    }
    if kind == "math.passThrough" {
        let n = input.get("number").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number".into()))?;
        return Ok(channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(n)))));
    }
    if kind == "core.variable" {
        let name = input.get("name").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).ok_or_else(|| EvalError::MissingInput("name".into()))?;
        let payload = input.get(name).and_then(|v| v.as_dictionary()).cloned().ok_or_else(|| EvalError::MissingInput(name.into()))?;
        return Ok(channel_output(name, payload));
    }
    Err(EvalError::UnknownKind(kind.into()))
}

fn complete_fixture_registration<T>(future: impl std::future::Future<Output = T>) -> T {
    match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("fixture registration must not depend on external work"),
    }
}

/// 🧪️ Installs first-party light (+brep) flow extension manifests and real in-process ops for fixture tests.
fn install_first_party_light_flow_extensions_for_tests() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        for (plugin_id, manifest) in [
            ("flow-extension-primitive", semio_s_plugin_flow_extension_primitive::extension_manifest_json()),
            ("flow-extension-math", semio_s_plugin_flow_extension_math::extension_manifest_json()),
            ("flow-extension-text", semio_s_plugin_flow_extension_text::extension_manifest_json()),
            ("flow-extension-logic", semio_s_plugin_flow_extension_logic::extension_manifest_json()),
            ("flow-extension-dictionary", semio_s_plugin_flow_extension_dictionary::extension_manifest_json()),
            ("flow-extension-list", semio_s_plugin_flow_extension_list::extension_manifest_json()),
            ("flow-extension-draw", semio_s_plugin_flow_extension_draw::extension_manifest_json()),
            ("flow-extension-bim", semio_s_plugin_flow_extension_bim::extension_manifest_json()),
            ("flow-extension-brep", complete_fixture_registration(semio_s_plugin_flow_extension_brep::extension_manifest_json())),
        ] {
            install_flow_extension_manifest(plugin_id, &manifest).expect("fixture extension admission");
        }
        let mut state = flow_extension_state().lock().expect("flow extension registry");
        let admission = begin_flow_registry_replacement(&mut state).expect("fixture registry admission");
        let mut registry = neural::ColdOwner::new(neural::Registry::new());
        semio_s_plugin_flow_extension_primitive::register(&mut registry);
        semio_s_plugin_flow_extension_math::register(&mut registry);
        semio_s_plugin_flow_extension_text::register(&mut registry);
        semio_s_plugin_flow_extension_logic::register(&mut registry);
        semio_s_plugin_flow_extension_dictionary::register(&mut registry);
        semio_s_plugin_flow_extension_list::register(&mut registry);
        semio_s_plugin_flow_extension_draw::register(&mut registry);
        semio_s_plugin_flow_extension_bim::register(&mut registry);
        complete_fixture_registration(semio_s_plugin_flow_extension_brep::register(&mut registry));
        registry.finalize();
        admission.publish(registry.into_inner());
        drop(state);
        while retire_flow_extension_registries_step(1, 4096).expect("fixture registry retirement") != neural::ValueRetirementStep::Complete {}
    });
}

fn fixture_kind_infos_json() -> String {
    install_first_party_light_flow_extensions_for_tests();
    flow_neuron_kind_infos_json()
}

/// 🌿️ All 9 first-party flow extensions install into the shared registry and each contributes
/// at least one evaluable neuron kind — procedural3d's flow graph can reach every extension.
#[test]
fn fixture_kind_infos_json_covers_every_first_party_extension() {
    // 🧊️ Untyped JSON on purpose: deserializing into `NeuronKindInfo` (= `neural::OperatorInfo`)
    // reconstructs real cold-tracked `Dictionary` values inside `ChannelSpec::default` and panics
    // on drop outside a cold boundary — plain `serde_json::Value` sidesteps that entirely.
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let ids: Vec<&str> = catalogue.as_array().expect("neuron kind infos array").iter().filter_map(|item| item["id"].as_str()).collect();
    for prefix in ["brep.", "bim.", "dictionary.", "draw.", "list.", "logic.", "math.", "core.", "text."] {
        assert!(ids.iter().any(|id| id.starts_with(prefix)), "expected at least one neuron kind id starting with {prefix:?}, got {ids:?}");
    }
}

/// 🧹️ Encodes one law-local operator catalogue and RETIRES it.
///
/// `ChannelSpec::default` carries a `Value`, and `Value`/`Dictionary` fail closed on a bare drop
/// (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop` — "final Dictionary ownership must be explicitly retired
/// or owned by a cold boundary"), so a temporary `Vec<OperatorInfo>` may not simply fall off the
/// end of a law (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn kind_infos_json(kind_infos: Vec<NeuronKindInfo>) -> String {
    let kind_infos = neural::ColdOwner::new(kind_infos);
    crate::os_pack::json::to_json_string(&*kind_infos)
}

/// 🧪️ The two-operator catalogue every host law in this file indexes.
///
/// 🧹️ The catalogue is RETIRED, not dropped: `InputSpec::number_default` puts a `Value` in
/// `ChannelSpec::default`, and `Value`/`Dictionary` fail closed on a bare drop
/// (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop` — "final Dictionary ownership must be explicitly retired
/// or owned by a cold boundary"). Letting the temporary `Vec<OperatorInfo>` fall off the end of
/// this helper aborted every law that builds a host through it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn test_kind_infos_json() -> String {
    kind_infos_json(vec![
        NeuronKindInfo {
            id: "math.add".into(),
            extension: "math".into(),
            name: "Add".into(),
            abbreviation: "Add".into(),
            icon: "emoji:➕️".into(),
            summary: "Sums two numbers".into(),
            inputs: vec![InputSpec::number("a", NUMBER_OPS), InputSpec::number_default("b", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("S", "Sum", "sum", "Sum")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "math.passThrough".into(),
            extension: "math".into(),
            name: "PassThrough".into(),
            abbreviation: "Pass".into(),
            icon: "emoji:➡️".into(),
            summary: "Forwards a number".into(),
            inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("N", "Num", "number", "Number")],
            ..Default::default()
        },
    ])
}

fn host_with_test_bridge() -> FlowHost {
    let mut host = FlowHost::default();
    host.set_eval_bridge_fn(Box::new(test_math_bridge));
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    host.set_host_catalogue_json(
        &serde_json::to_string(&[CatalogueSection {
            id: "math".into(),
            title: "Math".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "neuron".into(), neuron_kind: Some("math.add".into()), action: None, format: None, name: "Add".into(), abbreviation: "Add".into(), icon: "emoji:➕️".into(), summary: "Sums two numbers".into() },
                CatalogueItem {
                    kind: "neuron".into(), neuron_kind: Some("math.passThrough".into()), action: None, format: None, name: "PassThrough".into(), abbreviation: "Pass".into(), icon: "emoji:➡️".into(), summary: "Forwards a number".into()
                },
            ],
        }])
        .unwrap(),
    );
    host.evaluate_internal();
    host
}

fn widget_slider_track_screen_point(host: &FlowHost, widget_id: &str) -> (f64, f64) {
    let node = host.dag.fixture.nodes.iter().find(|n| n.id == widget_id).expect("node");
    let (wx, wy) = dag::slider_track_center(node).expect("slider track");
    let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, Point::new(wx, wy));
    (screen.x, screen.y)
}

#[test]
fn default_fixture_maps_widgets_to_native_dag_kinds() {
    let host = host_with_test_bridge();
    let slider = host.dag.fixture.nodes.iter().find(|n| n.id == "slider").expect("slider");
    assert!(matches!(slider.kind, DagNodeKind::Slider { .. }));
    assert_eq!(slider.height, slider_widget_height());
    let add = host.dag.fixture.nodes.iter().find(|n| n.id == "add").expect("add");
    assert!(matches!(add.kind, DagNodeKind::Computation { .. }));
    assert_eq!(slider.width, add.width, "all components should share one width");
    assert_eq!(slider.width, computation_node_width(&slider.name, &[], &[]));
    let preview = host.dag.fixture.nodes.iter().find(|n| n.id == "preview").expect("preview");
    assert!(matches!(preview.kind, DagNodeKind::Preview { .. }));
    host.retire_cold();
}

#[test]
fn default_fixture_evaluates_add_preview() {
    let host = host_with_test_bridge();
    assert_eq!(host.preview_text(), "3");
    host.retire_cold();
}

#[test]
fn slider_updates_preview() {
    // 🧵️ Mutating a widget never auto-evaluates anymore (see `evaluate_step`'s doc comment) — an
    // off-main-thread ticker outside `flow` is responsible for that; this simulates one tick
    // with a direct `evaluate_internal` call.
    let mut host = host_with_test_bridge();
    host.set_slider_value("slider", 5.0);
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "5");
    host.retire_cold();
}

#[test]
fn evaluate_skips_unchanged_tree_after_move_widget() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_for_bridge = calls.clone();
    let mut host = FlowHost::default();
    host.set_eval_bridge_fn(Box::new(move |kind, input| {
        calls_for_bridge.fetch_add(1, Ordering::Relaxed);
        test_math_bridge(kind, input)
    }));
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    host.evaluate_internal();
    let baseline = calls.load(Ordering::Relaxed);
    host.move_widget("slider", -120.0, 20.0).unwrap();
    host.evaluate_internal();
    assert_eq!(calls.load(Ordering::Relaxed), baseline);
    host.retire_cold();
}

#[test]
fn pending_eval_widget_ids_reports_without_computing() {
    let mut host = host_with_test_bridge();
    let before = host.preview_text();
    host.set_slider_value("slider", 9.0);
    let pending = host.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()), "the widget downstream of the changed slider is pending");
    assert!(!pending.contains(&"slider".to_string()), "the seed slider itself is not a pending neuron");
    assert_eq!(host.preview_text(), before, "a probe must never actually compute anything");
    host.retire_cold();
}

#[test]
fn set_slider_value_marks_downstream_computing_chrome() {
    let mut host = host_with_test_bridge();
    host.set_slider_value("slider", 7.0);
    let pending = host.pending_eval_widget_ids();
    assert!(!pending.is_empty(), "slider change must flag downstream nodes as pending");
    host.refresh_computing_chrome_from_pending();
    let remaining = host.pending_eval_widget_ids();
    assert_eq!(remaining.first().map(String::as_str), pending.first().map(String::as_str));
    host.retire_cold();
}

#[test]
fn apply_eval_outputs_json_establishes_baseline_for_dirty_probe() {
    let host = host_with_test_bridge();
    let eval_json = host.last_eval_json.clone();
    let mut fresh = FlowHost::default();
    fresh.set_eval_bridge_fn(Box::new(test_math_bridge));
    fresh.set_neuron_kind_infos_json(&test_kind_infos_json());
    fresh.apply_eval_outputs_json(&eval_json);
    fresh.set_slider_value("slider", 4.0);
    let pending = fresh.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()));
    assert!(!pending.contains(&"slider".to_string()));
    fresh.retire_cold();
    host.retire_cold();
}

#[test]
fn apply_eval_outputs_json_skips_baseline_when_outputs_stale_for_seeds() {
    let mut host = host_with_test_bridge();
    let stale_eval_json = host.last_eval_json.clone();
    host.set_slider_value("slider", 9.0);
    host.apply_eval_outputs_json(&stale_eval_json);
    let pending = host.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()), "stale channel outputs must not converge the baseline after a seed change");
    assert!(!pending.contains(&"slider".to_string()));
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "9");
    let fresh_eval_json = host.last_eval_json.clone();
    host.apply_eval_outputs_json(&fresh_eval_json);
    assert!(host.pending_eval_widget_ids().is_empty(), "fresh eval for the current seeds must establish a converged baseline");
    host.retire_cold();
}

#[test]
fn flow_eval_session_retains_baseline_across_ephemeral_hosts() {
    let mut session = FlowEvalSession::new();
    let mut host = host_with_test_bridge();
    session.capture_baseline_from(&host);
    host.set_slider_value("slider", 8.0);
    let mut replay = FlowHost::default();
    replay.set_eval_bridge_fn(Box::new(test_math_bridge));
    replay.set_neuron_kind_infos_json(&test_kind_infos_json());
    replay.replace_fixture(host.fixture.clone());
    session.install_baseline_into(&mut replay);
    let pending = replay.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()));
    assert!(!pending.contains(&"slider".to_string()));
    replay.retire_cold();
    host.retire_cold();
}

#[test]
fn flow_eval_session_seeds_its_retained_neural_cache() {
    let session = FlowEvalSession::new();
    let expected = Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(42.0)));
    let output_json = crate::os_pack::json::to_json_string(&expected);
    session.seed_node_cache(17, &output_json).unwrap();
    assert_eq!(session.neural_cache().get(17), Some(expected));
}

/// 🧵️ Builds a two-computable-node chain (`add` -> `pass`, replacing `add`'s direct link to
/// `preview`) on top of the default fixture, for tests that need more than one node to step
/// through with a budgeted `evaluate_step`.
fn host_with_two_node_chain() -> (FlowHost, String) {
    let mut host = host_with_test_bridge();
    let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 240.0, 0.0).unwrap();
    host.connect_ports("add", "sum", &pass_id, "number").unwrap();
    host.connect_ports(&pass_id, "number", "preview", "").unwrap();
    let stale_link = host.fixture.synapses.iter().find(|s| s.from == "add" && s.to == "preview").map(|s| s.id.clone());
    if let Some(id) = stale_link {
        host.disconnect(&id).unwrap();
    }
    host.evaluate_internal();
    (host, pass_id)
}

#[test]
fn evaluate_step_budget_one_converges_over_multiple_calls() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    assert_eq!(host.preview_text(), "3", "chain settles to the same value as the direct add->preview link");
    host.set_slider_value("slider", 6.0);
    // ⏳️ Nothing evaluates until stepped — mirrors a mutation with no tick chain run yet.
    assert_eq!(host.preview_text(), "3");
    // ⏱️ Tick 1: budget for one cache-missed node — computes "add" for free-riding boundary nodes
    // plus that one dispatch, then stops right before the next miss ("pass"). `remaining[0]` is
    // the blocking node; anything after it (here, "preview") is just downstream-and-untouched.
    let remaining_after_tick1 = host.evaluate_step(EvalStepBudget::dispatches(1));
    assert_eq!(remaining_after_tick1.first(), Some(&"pass".to_string()), "pass is the next node blocking completion");
    assert_eq!(host.preview_text(), "3", "the chain hasn't reached \"pass\" (and thus \"preview\") yet");
    // ⏱️ Tick 2: "add" is now cached, so this reaches and computes "pass".
    let remaining_after_tick2 = host.evaluate_step(EvalStepBudget::dispatches(1));
    assert!(remaining_after_tick2.is_empty(), "the walk reached the end of the topo order");
    assert_eq!(host.preview_text(), "6", "converged to the dragged value after both ticks");
    host.retire_cold();
}

#[test]
fn flow_eval_session_sync_and_tick_state_machine() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    let mut session = FlowEvalSession::new();
    session.capture_baseline_from(&host);
    assert!(!session.pending());
    assert!(!session.sync(&host));
    assert!(!session.pending());
    host.set_slider_value("slider", 12.0);
    assert!(session.sync(&host), "a changed slider arms the chain");
    assert!(session.pending());
    assert!(session.status_json().contains("computing"), "the immediate dependent is reported as computing");
    assert!(!session.sync(&host));
    while session.tick(&mut host) {}
    assert!(!session.pending());
    assert_eq!(host.preview_text(), "12");
    host.set_slider_value("slider", 20.0);
    assert!(session.sync(&host));
    assert!(session.pending());
    host.set_slider_value("slider", 30.0);
    assert!(!session.sync(&host), "a chain is already scheduled — sync must not arm a redundant second one");
    assert!(session.pending(), "the in-flight chain is still the one that will pick up 30");
    while session.tick(&mut host) {}
    assert_eq!(host.preview_text(), "30", "converges on the latest value, not the superseded intermediate one");
    host.retire_cold();
}

/// ⚖️ LAW: the flow extension registry GENERATION is a session's invalidation key.
///
/// A node whose operator no plugin has contributed yet does not merely stall — its miss is cached in
/// the session's neural cache, in its incremental baseline and in its published
/// `eval_json`/`status_json`, and the chain that hit it stops. Installing the contribution afterwards
/// changes a process-wide registry and publishes nothing, so a session has to be TOLD its results
/// predate the registry it can now address. Keying that on the generation is what stops a re-push of
/// an unchanged closure from restarting a settled evaluation
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🧪️ Built on a bare `FlowHost::default()` and a seeded cache rather than on
/// `host_with_two_node_chain`, whose `test_kind_infos_json` helper currently trips the neural
/// engine's `final Dictionary ownership` gate on its own (every session law in this file is red for
/// that reason) — the claim here is about the SESSION, and it is stated without inheriting an
/// unrelated red.
#[test]
fn flow_eval_session_invalidates_only_when_the_flow_extension_registry_generation_moves() {
    let host = FlowHost::default();
    let mut session = FlowEvalSession::new();
    let generation = session.flow_extension_generation();
    assert_eq!(generation, crate::flow_extension_registry_generation(), "a fresh session is current with the registry it will evaluate against");
    let cached = Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(42.0)));
    let cached_json = crate::os_pack::json::to_json_string(&cached);
    cached.retire_cold();
    session.seed_node_cache(17, &cached_json).expect("a host-mediated extension answer seeds the retained cache");
    assert!(session.sync(&host), "the default demo graph has pending nodes, so a chain is armed");
    assert!(session.pending());

    assert!(!session.invalidate_for_flow_extension_registry(generation), "an unmoved generation invalidates nothing");
    assert!(session.pending(), "and therefore releases nothing");
    assert!(session.neural_cache().contains(17));

    assert!(session.invalidate_for_flow_extension_registry(generation + 1), "a moved generation invalidates");
    assert_eq!(session.flow_extension_generation(), generation + 1);
    assert_eq!(session.status_json(), "{}", "the per-node status computed against the old registry is released");
    assert!(session.eval_json().is_empty(), "the published evaluation is released");
    assert!(!session.pending(), "a released session admits a new chain rather than waiting on the one that gave up");
    assert!(!session.neural_cache().contains(17), "every node output computed against the old registry is evicted");
    assert!(!session.invalidate_for_flow_extension_registry(generation + 1), "the same generation twice invalidates once");

    session.begin_close();
    for _ in 0..1_000_000 {
        if session.terminal_is_empty() {
            break;
        }
        let _ = session.close_step(usize::MAX, usize::MAX);
    }
    assert!(session.terminal_is_empty(), "an invalidated session still reaches terminal-empty through its own close ladder");
    host.retire_cold();
}

//#region 🔢️RegistryGenerationBaseline
/// 🧵️ Rebuilds the ephemeral host a durable driver would hand a session's baseline to — the same
/// three lines `flow_host_with_session` runs in production, over the test bridge.
fn replay_host_of(host: &FlowHost) -> FlowHost {
    let mut replay = FlowHost::default();
    replay.set_eval_bridge_fn(Box::new(test_math_bridge));
    replay.set_neuron_kind_infos_json(&test_kind_infos_json());
    replay.replace_fixture(host.fixture.clone());
    replay
}

/// ⚖️ LAW: a registry-generation invalidation clears the session's incremental BASELINE, so the
/// next ephemeral host re-dispatches the very same tree instead of skipping it.
///
/// `invalidate_for_flow_extension_registry` releases `eval_json`/`status_json` and sweeps the
/// neural cache — but the incremental baseline is the third place a pre-contribution miss is
/// retained, and it is the one that decides whether `evaluate_step` runs at all. A baseline that
/// outlived the invalidation would let `compute_dirty_set` answer "nothing changed" for a tree
/// that never did change, and the surface would keep its `unknown kind` fault forever with no user
/// action able to clear it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️audit-unknown-kind-2026-09-12.md` §3).
#[test]
fn an_invalidated_session_hands_an_ephemeral_host_a_re_dispatching_baseline() {
    let host = host_with_test_bridge();
    let mut session = FlowEvalSession::new();
    session.capture_baseline_from(&host);
    let mut settled = replay_host_of(&host);
    session.install_baseline_into(&mut settled);
    assert!(settled.pending_eval_widget_ids().is_empty(), "control: an unchanged tree under an unchanged registry owes no dispatch");
    let generation = session.flow_extension_generation();
    assert!(session.invalidate_for_flow_extension_registry(generation + 1), "a moved generation invalidates");
    let mut rearmed = replay_host_of(&host);
    session.install_baseline_into(&mut rearmed);
    assert_eq!(rearmed.eval_baseline_registry_generation(), generation + 1, "the baseline carries the generation it was computed against");
    assert!(!rearmed.pending_eval_widget_ids().is_empty(), "after a registry-generation invalidation the SAME tree must be re-dispatched");
    session.begin_close();
    for _ in 0..1_000_000 {
        if session.terminal_is_empty() { break; }
        let _ = session.close_step(usize::MAX, usize::MAX);
    }
    assert!(session.terminal_is_empty());
    settled.retire_cold();
    rearmed.retire_cold();
    host.retire_cold();
}

/// 🧵️ One ephemeral host of the shared fixture, with its OWN neural cache and a bridge that
/// counts every dispatch — the shape `flow_host_with_session` rebuilds per tick.
fn counting_replay_host_of(host: &FlowHost, dispatches: &std::sync::Arc<std::sync::atomic::AtomicUsize>) -> FlowHost {
    let counter = dispatches.clone();
    let mut replay = FlowHost::default();
    replay.set_eval_bridge_fn(Box::new(move |kind: &str, input: &Dictionary| {
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        test_math_bridge(kind, input)
    }));
    replay.set_neuron_kind_infos_json(&test_kind_infos_json());
    replay.replace_fixture(host.fixture.clone());
    replay
}

/// ⚖️ LAW: a baseline stamped with a SUPERSEDED flow extension registry replacement re-dispatches
/// the whole tree; only a baseline stamped with the installed one may be skipped.
///
/// An unchanged tree is not an unchanged evaluation. `compute_dirty_set` diffs the TREE, and a
/// contributed operator arriving in a registry replacement changes no tree at all — the operator
/// table it is dispatched through is process-wide state swapped out underneath a host that has
/// already settled. Refusing only the skip is not enough either: a refused skip that still diffs
/// against the superseded snapshot computes an EMPTY dirty set and dispatches nothing, which is
/// the same stale answer by a longer road. The baseline must be dropped whole
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🧪️ Both halves run on an ephemeral host with its OWN neural cache, exactly as
/// `flow_host_with_session` rebuilds one per tick: the retained cache is the session's to sweep
/// (`invalidate_for_flow_extension_registry`), and a cache that still answers would mask the very
/// dispatch this law counts.
#[test]
fn a_superseded_registry_generation_re_dispatches_an_unchanged_tree() {
    let _serialized = crate::registry::lock_flow_extension_registry_for_test();
    crate::registry::drain_flow_extension_registry_retirements();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📔️registry/🧫️fixtures/🔣️manifest-admission.json")).unwrap();
    let bump = &fixture["generationBump"];
    let source = host_with_test_bridge();
    let generation = source.eval_baseline_registry_generation();
    assert_eq!(generation, crate::flow_extension_registry_generation(), "a settled host is stamped with the registry it dispatched through");
    let dispatches = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut settled = counting_replay_host_of(&source, &dispatches);
    let (snapshot, channels) = source.eval_baseline();
    settled.install_eval_baseline(snapshot, channels, generation);
    settled.evaluate_internal();
    assert_eq!(dispatches.load(std::sync::atomic::Ordering::Relaxed), 0, "control: a current baseline over an unchanged tree dispatches nothing");
    assert!(settled.pending_eval_widget_ids().is_empty(), "control: and owes no further work");

    crate::install_flow_extension_manifest(bump["pluginId"].as_str().unwrap(), bump["manifestJson"].as_str().unwrap()).expect("the fixture manifest installs");
    assert_ne!(crate::flow_extension_registry_generation(), generation, "installing a manifest replaces the registry");
    let mut rearmed = counting_replay_host_of(&source, &dispatches);
    let (snapshot, channels) = source.eval_baseline();
    rearmed.install_eval_baseline(snapshot, channels, generation);
    assert!(!rearmed.pending_eval_widget_ids().is_empty(), "a superseded baseline owes the whole tree again");
    rearmed.evaluate_internal();
    assert!(dispatches.load(std::sync::atomic::Ordering::Relaxed) > 0, "a superseded registry generation must re-dispatch the unchanged tree");
    assert_eq!(rearmed.preview_text(), source.preview_text(), "re-dispatching an unchanged tree reaches the same answer");

    crate::uninstall_flow_extension(bump["extensionId"].as_str().unwrap()).expect("the law leaves the process-wide registry as it found it");
    for _ in 0..100_000 {
        if crate::retire_flow_extension_registries_step(1, 64) == Ok(neural::ValueRetirementStep::Complete) { break; }
    }
    settled.retire_cold();
    rearmed.retire_cold();
    source.retire_cold();
}
//#endregion 🔢️RegistryGenerationBaseline

#[test]
fn connect_ports_allows_fan_out_from_same_output() {
    let mut host = host_with_test_bridge();
    let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 120.0, 120.0).unwrap();
    host.connect_ports("add", "sum", &pass_id, "number").unwrap();
    let fan_out: Vec<_> = host.fixture.synapses.iter().filter(|s| s.from == "add" && s.from_port == "sum").collect();
    assert_eq!(fan_out.len(), 2);
    assert!(fan_out.iter().any(|s| s.to == "preview"));
    assert!(fan_out.iter().any(|s| s.to == pass_id));
    host.retire_cold();
}

#[test]
fn connect_ports_replaces_existing_incoming_on_same_input() {
    let mut host = host_with_test_bridge();
    assert!(host.fixture.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
    let note_id = host.add_widget(r#"{"kind":"inputNote","id":"note","text":"2"}"#, -120.0, 0.0).unwrap();
    host.connect_ports(&note_id, "text", "add", "a").unwrap();
    let incoming_a: Vec<_> = host.fixture.synapses.iter().filter(|s| s.to == "add" && s.to_port == "a").collect();
    assert_eq!(incoming_a.len(), 1);
    assert_eq!(incoming_a[0].from, note_id);
    assert!(!host.fixture.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
    host.retire_cold();
}

#[test]
fn evaluate_runs_after_tree_change() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_for_bridge = calls.clone();
    let mut host = FlowHost::default();
    host.set_eval_bridge_fn(Box::new(move |kind, input| {
        calls_for_bridge.fetch_add(1, Ordering::Relaxed);
        test_math_bridge(kind, input)
    }));
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    host.evaluate_internal();
    let baseline = calls.load(Ordering::Relaxed);
    host.set_slider_value("slider", 5.0);
    host.evaluate_internal();
    let after_slider = calls.load(Ordering::Relaxed);
    assert!(after_slider > baseline);
    host.disconnect("s1").unwrap();
    host.connect_ports("slider", "number", "add", "b").unwrap();
    host.evaluate_internal();
    assert!(calls.load(Ordering::Relaxed) > after_slider);
    host.retire_cold();
}

#[test]
fn dirty_propagation_only_dispatches_affected_branch() {
    use std::sync::Arc;
    use std::sync::Mutex as StdMutex;
    // Branch A (default fixture): slider -> add -> preview. Branch B (added here): a second,
    // disconnected slider -> passThrough, sharing no synapse with branch A.
    let calls: Arc<StdMutex<Vec<String>>> = Arc::new(StdMutex::new(Vec::new()));
    let calls_for_bridge = calls.clone();
    let mut host = FlowHost::default();
    host.set_eval_bridge_fn(Box::new(move |kind, input| {
        calls_for_bridge.lock().unwrap().push(kind.to_string());
        test_math_bridge(kind, input)
    }));
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    let slider_b_id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.0}"#, 400.0, 0.0).unwrap();
    let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 600.0, 0.0).unwrap();
    host.connect_ports(&slider_b_id, "number", &pass_id, "number").unwrap();
    host.evaluate_internal();
    calls.lock().unwrap().clear();

    host.set_slider_value("slider", 5.0);
    host.evaluate_internal();

    let dispatched = calls.lock().unwrap().clone();
    assert!(dispatched.iter().any(|kind| kind == "math.add"), "branch A (add) should re-dispatch after its slider changed");
    assert!(!dispatched.iter().any(|kind| kind == "math.passThrough"), "branch B (pass) must stay clean when only branch A changed");
    host.retire_cold();
}

#[test]
fn neural_cache_persists_across_evaluations() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_for_bridge = calls.clone();
    let mut host = FlowHost::default();
    host.set_eval_bridge_fn(Box::new(move |kind, input| {
        calls_for_bridge.fetch_add(1, Ordering::Relaxed);
        test_math_bridge(kind, input)
    }));
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    host.evaluate_internal();
    let baseline = calls.load(Ordering::Relaxed);
    assert!(baseline > 0, "first evaluation is a cache miss and must dispatch to the bridge");
    host.evaluate_internal();
    assert_eq!(calls.load(Ordering::Relaxed), baseline, "an unchanged tree must be served entirely from the cache");
    host.set_slider_value("slider", 4.0);
    host.evaluate_internal();
    assert_eq!(calls.load(Ordering::Relaxed), baseline + 1, "only the node downstream of the changed slider should re-dispatch");
    host.retire_cold();
}

#[test]
fn collect_live_geometry_handles_includes_input_channels() {
    let mut outputs = BTreeMap::new();
    outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
    outputs.insert("volume".into(), Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(12.0))));
    let mut inputs = BTreeMap::new();
    inputs.insert("volume".into(), Dictionary::new().insert("geometry", NeuralValue::Dictionary(Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))))));
    let channels = EvalChannels { outputs, inputs };
    let handles = collect_live_geometry_handles_from_channels(&channels);
    assert_eq!(handles, vec![String::from("solid-box")]);
}

#[test]
fn apply_eval_outputs_json_preserves_state_on_global_error() {
    let mut host = host_with_test_bridge();
    let good = host.last_eval_json.clone();
    host.apply_eval_outputs_json(r#"{"error":"missing input: geometry"}"#);
    assert_eq!(host.last_eval_json, good);
    assert!(!host.outputs.is_empty());
    host.retire_cold();
}

fn collect_live_geometry_handles(outputs: &BTreeMap<String, Dictionary>) -> Vec<String> {
    let mut handles = Vec::new();
    for dict in outputs.values() {
        collect_geometry_handles_from_dictionary(dict, &mut handles);
    }
    handles.sort();
    handles.dedup();
    handles
}

#[test]
fn collect_live_geometry_handles_traverses_nested_dictionaries() {
    let mut outputs = BTreeMap::new();
    outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-1".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
    outputs.insert("nested".into(), Dictionary::new().insert("child", NeuralValue::Dictionary(Dictionary::with_schema("face").insert("handle", NeuralValue::Atom(Atom::String("face-2".into()))))));
    let handles = collect_live_geometry_handles(&outputs);
    assert_eq!(handles, vec![String::from("face-2"), String::from("solid-1")]);
}

#[test]
fn collect_live_drawing_handles_traverses_list_values() {
    let mut outputs = BTreeMap::new();
    outputs.insert(
        "get".into(),
        Dictionary::new().insert("value", NeuralValue::Dictionary(Dictionary::with_schema("list").insert("0", NeuralValue::Dictionary(Dictionary::with_schema("draw.drawing").insert("handle", NeuralValue::Atom(Atom::String("drawing-2".into()))))))),
    );
    let channels = EvalChannels { outputs, inputs: BTreeMap::new() };
    assert_eq!(collect_live_drawing_handles_from_channels(&channels), vec![String::from("drawing-2")]);
}

#[test]
fn evaluate_emits_channel_structured_json() {
    let host = host_with_test_bridge();
    let parsed: serde_json::Value = serde_json::from_str(&host.last_eval_json).expect("json");
    let add = parsed.get("add").and_then(|value| value.as_object()).expect("add channels");
    assert!(add.get("in").and_then(|value| value.as_object()).is_some());
    let out = add.get("out").and_then(|value| value.as_object()).expect("add out");
    assert!(out.get("sum").is_some());
    host.retire_cold();
}

#[test]
fn preview_text_formats_geometry_as_tree_summary() {
    let dict = Dictionary::new().insert("geometry", NeuralValue::Atom(Atom::String("solid-3".into())));
    let content = dag_preview_content_from_dict(&dict);
    assert!(matches!(content, DagPreviewContent::Tree { .. }));
    assert_eq!(preview_content_summary(&content), "{1 keys}");
}

#[test]
fn preview_scalar_content_from_number_dict() {
    let dict = Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(3.0)));
    assert!(matches!(
        dag_preview_content_from_dict(&dict),
        DagPreviewContent::Scalar { text } if text == "3"
    ));
}

#[test]
fn image_input_seed_and_preview_content() {
    let mut host = host_with_test_bridge();
    let png = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
    host.fixture.widgets.push(Widget::InputImage { id: "image".into(), src: png.into() });
    host.rebuild_dag();
    let node = host.dag.fixture.nodes.iter().find(|n| n.id == "image").expect("image node");
    assert!(matches!(node.kind, DagNodeKind::Image { .. }));
    let seeds = host.build_seeds();
    assert_eq!(seeds.get("image").and_then(|d| d.get("image")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("dataUrl")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some(png));
    host.retire_cold();
}

#[test]
fn slider_drag_does_not_evaluate_until_explicit_evaluate() {
    // 🧵️ A live drag firing many pointer-move ticks used to re-evaluate the whole graph on every
    // one of them (fine for cheap graphs, a repeated multi-second stall for a heavy one, e.g. a
    // brep boolean). Dragging alone must never evaluate now — the off-main-thread ticker (outside
    // `flow`) picks up the changed slider value at its own pace; an explicit `evaluate`
    // (simulated here) still updates the preview once it runs.
    let mut host = host_with_test_bridge();
    host.set_viewport(800, 600, 1.0);
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    assert_eq!(host.preview_text(), "3");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    assert_eq!(host.preview_text(), "3", "a live drag must not synchronously re-evaluate the graph");
    host.evaluate_internal();
    assert_ne!(host.preview_text(), "3", "an explicit evaluate still picks up the dragged value");
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    host.retire_cold();
}

#[test]
fn dag_slider_drag_syncs_fixture_value() {
    let mut host = host_with_test_bridge();
    host.set_viewport(800, 600, 1.0);
    let slider_node = host.dag.fixture.nodes.iter().find(|n| n.id == "slider").expect("slider").clone();
    let DagNodeKind::Slider { .. } = slider_node.kind else {
        panic!("expected slider kind");
    };
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    let value = host
        .fixture
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(value > 3.0);
    host.retire_cold();
}

#[test]
fn default_fixture_does_not_auto_layout() {
    let host = host_with_test_bridge();
    let slider = host.fixture.layout.get("slider").expect("slider");
    let add = host.fixture.layout.get("add").expect("add");
    let preview = host.fixture.layout.get("preview").expect("preview");
    assert_eq!(slider.x, 0.0);
    assert_eq!(add.x, 200.0);
    assert_eq!(preview.x, 400.0);
    host.retire_cold();
}

#[test]
fn canvas_slider_hit_adjusts_value_playground_viewport() {
    let mut host = host_with_test_bridge();
    host.set_viewport(1259, 706, 1.0);
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 90.0, sy, false, false, false);
    host.pointer_up_screen(sx + 90.0, sy, false, false, false);
    let slider = host
        .fixture
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(slider > 3.0);
    host.retire_cold();
}

#[test]
fn canvas_slider_hit_adjusts_value() {
    let mut host = host_with_test_bridge();
    host.set_viewport(800, 600, 1.0);
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    let slider = host
        .fixture
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(slider > 3.0);
    host.retire_cold();
}

#[test]
fn reorganize_overwrites_saved_layout_left_to_right() {
    let mut host = host_with_test_bridge();
    host.fixture.layout.insert("slider".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.fixture.layout.insert("add".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.fixture.layout.insert("preview".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.rebuild_dag();
    host.reorganize("").unwrap();
    let slider = host.fixture.layout.get("slider").expect("slider layout");
    let add = host.fixture.layout.get("add").expect("add layout");
    let preview = host.fixture.layout.get("preview").expect("preview layout");
    assert!(add.x > slider.x);
    assert!(preview.x > add.x);
    host.retire_cold();
}

#[test]
fn fixture_json_round_trip() {
    let host = FlowHost::default();
    let json = host.fixture_json().unwrap();
    let parsed = FlowHost::parse_fixture_json(&json).unwrap();
    assert_eq!(parsed.schema, "flow.fixture");
    host.retire_cold();
}

#[test]
fn flow_document_tree_is_shakable() {
    let host = host_with_test_bridge();
    let document = host.document();
    assert_eq!(document.schema, "flow.artifact");
    assert!(!document.tree.neurons.is_empty());
    let registry = neural::Registry::new();
    let evaluator = Evaluator::new(&registry);
    let dispatch = |kind: &str, input: &Dictionary| test_math_bridge(kind, input);
    let mut seeds = HashMap::new();
    seeds.insert("slider".into(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(3.0)))));
    let channels = evaluator.evaluate_channels_with(&document.tree, &seeds, &host.kind_infos, &dispatch).unwrap();
    assert_eq!(channels.outputs.get("add").and_then(|d| d.get("sum")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    host.retire_cold();
}

#[test]
fn rebuild_dag_preserves_canvas_theme() {
    use canvas::Color;
    let mut host = FlowHost::default();
    host.dag.canvas_theme.node_fill = Color::from_rgba8(12, 34, 56, 255);
    host.rebuild_dag();
    assert_eq!(host.dag.canvas_theme.node_fill.to_rgba8(), Color::from_rgba8(12, 34, 56, 255).to_rgba8());
    host.retire_cold();
}

#[test]
fn set_canvas_theme_dark_applies_board_dark_strokes() {
    let mut host = FlowHost::default();
    host.set_canvas_theme_dark(true);
    let stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
    assert!(stroke.r > 80 || stroke.g > 80);
    host.set_canvas_theme_dark(false);
    let light_stroke = host.dag.canvas_theme.node_stroke.to_rgba8();
    assert!(light_stroke.r < 80);
    host.retire_cold();
}

#[test]
fn paint_scene_dark_theme_paints_edges_and_nodes() {
    let mut host = host_with_test_bridge();
    host.set_viewport(1280, 800, 1.0);
    host.set_canvas_theme_dark(true);
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    assert!(scene.path_count() > 8, "populated fixture should paint edges, handles, and node bodies under dark board theme");
    host.retire_cold();
}

#[test]
fn flow_host_enables_minimap_widget_on_dag() {
    let mut host = host_with_test_bridge();
    host.set_viewport(1280, 800, 1.0);
    host.dag.set_camera(200.0, 120.0, 0.65);
    let raw: serde_json::Value = serde_json::from_str(&host.dag.label_overlay_paint_state_json().unwrap()).unwrap();
    assert!(raw.get("minimapWidget").is_some());
    host.retire_cold();
}

#[test]
fn replace_fixture_preserves_kind_infos_and_named_input_ports() {
    let mut host = host_with_test_bridge();
    host.replace_fixture(FlowFixture {
        schema: "flow.fixture".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![Widget::Neuron { id: "add".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true }],
        synapses: vec![],
        layout: crate::OrderedMap::new(),
    });
    let node = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add node");
    let input_ids: Vec<&str> = node.inputs().iter().map(|port| port.id.as_str()).collect();
    assert_eq!(input_ids, vec!["a", "b"]);
    host.retire_cold();
}

#[test]
fn catalogue_nested_groups_round_trip() {
    let host_json = serde_json::to_string(&[CatalogueSection {
        id: "brep".into(),
        title: "Brep".into(),
        items: vec![],
        groups: vec![CatalogueGroup {
            id: "brep.primitives-3d".into(),
            title: "Primitives 3D".into(),
            items: vec![CatalogueItem {
                kind: "neuron".into(), neuron_kind: Some("brep.prim3d.box".into()), action: None, format: None, name: "Box".into(), abbreviation: "Box".into(), icon: "emoji:📦️".into(), summary: "Axis-aligned box".into()
            }],
            groups: vec![],
        }],
    }])
    .unwrap();
    let sections = merge_catalogue_sections(&host_json).unwrap();
    let brep = sections.iter().find(|section| section.id == "brep").expect("brep section");
    let prim3d = brep.groups.iter().find(|group| group.title == "Primitives 3D").expect("prim3d group");
    assert_eq!(prim3d.items[0].neuron_kind.as_deref(), Some("brep.prim3d.box"));
}

#[test]
fn catalogue_has_module_sections() {
    let host = host_with_test_bridge();
    let json = host.catalogue_json().unwrap();
    assert!(json.contains("math"));
    assert!(json.contains("math.add"));
    assert!(json.contains("Inputs"));
    assert!(json.contains("Outputs"));
    host.retire_cold();
}

#[test]
fn flow_backed_node_graph_extras_include_fixture_and_flow_engine() {
    install_first_party_light_flow_extensions_for_tests();
    let host = host_with_test_bridge();
    let extras = flow_backed_node_graph_extras(&host.fixture, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
    assert!(extras.fixture_json.as_ref().is_some_and(|json| json.contains("flow.fixture")));
    assert!(extras.capabilities_json.as_ref().is_some_and(|json| json.contains(r#""engine":"flow""#)));
    assert!(extras.lod_json.as_ref().is_some_and(|json| json.contains(r#""automatic":true"#)));
    host.retire_cold();
}

/// 🛍️ The operator catalogue is APP-STATIC — it leaves the per-scene extras entirely and rides
/// `FlowAppCatalogue` on the reserved `framework.section.catalogue` surface instead
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
#[test]
fn app_catalogue_carries_every_operator_and_palette_section() {
    install_first_party_light_flow_extensions_for_tests();
    let catalogue = flow_app_catalogue();
    assert!(catalogue.operators.iter().any(|info| info.id == "math.add"));
    assert!(catalogue.sections.iter().any(|section| section.id == "math"));
    assert!(catalogue.sections.iter().any(|section| section.id == "inputs"), "static widget sections must merge into the app catalogue");
    let json = flow_app_catalogue_json();
    assert!(json.contains("math.add"));
    println!("[STATS] app catalogue operators={} sections={} json_bytes={}", catalogue.operators.len(), catalogue.sections.len(), json.len());
}

/// 🛡️ THE surface bound law. A node-graph surface is admitted against `ui_contract::UI_FIXED_BYTES`
/// (32 KiB, a preallocated per-surface capacity), and before this ticket every flow-backed window
/// embedded the whole registered operator catalogue in its scene: with the real `brep`/`math` sets
/// installed that surface measured 111 031 B and the window could not render at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). The catalogue is app-static, so it moved to the
/// reserved `framework.section.catalogue` retained surface — and the scene now names operators by kind
/// id only. This law installs FIVE HUNDRED operators, an order of magnitude past the real sets, and
/// asserts the surface is unmoved by them.
#[test]
fn a_node_graph_surface_stays_under_the_fixed_admission_with_five_hundred_operators() {
    let _serialized = crate::registry::lock_flow_extension_registry_for_test();
    const OPERATORS: usize = 500;
    let operators = (0..OPERATORS)
        .map(|index| format!(r#"{{"id":"bulk.op{index}","extension":"bulk","name":"Bulk Operator {index}","abbreviation":"B{index}","icon":"box","summary":"Bulk catalogue operator {index} with a deliberately verbose summary line","inputs":[],"outputs":[]}}"#))
        .collect::<Vec<_>>()
        .join(",");
    let manifest = format!(r#"{{"schema":"flow.extension","id":"bulk","name":"Bulk","version":"0.0.1","activationEvents":[],"contributes":{{"schemas":[],"operators":[{operators}],"widgets":[],"commands":[],"settings":[]}}}}"#);
    install_flow_extension_manifest("bulk-plugin", &manifest).expect("bulk extension admission");

    let catalogue_bytes = flow_app_catalogue_json().len();
    let registered = flow_app_catalogue().operators.len();
    assert!(registered >= OPERATORS, "the registry must actually hold the bulk operators, holds {registered}");

    let fixture = FlowFixture::default();
    let extras = flow_backed_node_graph_extras(&fixture, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
    let scene = ui_wgpu::wgpu::NodeGraphScene {
        editable: Some(true),
        capabilities_json: extras.capabilities_json,
        lod_json: extras.lod_json,
        fixture_json: extras.fixture_json,
        eval_json: extras.eval_json,
        status_json: extras.status_json,
        ..ui_wgpu::wgpu::NodeGraphScene::base(Vec::new(), Vec::new(), semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 })
    };
    let props = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::NodeGraph, &scene).expect("a node-graph surface must fit the fixed admission with 500 operators registered");
    let surface_bytes = props.doc.bytes.as_slice().len();
    println!("[STATS] node-graph surface with {registered} registered operators: surface={surface_bytes} B of {} B, app catalogue={catalogue_bytes} B", ui_contract::UI_FIXED_BYTES);
    assert!(surface_bytes < ui_contract::UI_FIXED_BYTES, "surface is {surface_bytes} B, over the {} B fixed admission", ui_contract::UI_FIXED_BYTES);
    assert!(catalogue_bytes > ui_contract::UI_FIXED_BYTES, "the catalogue must be the thing that would not have fit: {catalogue_bytes} B");

    fixture.retire_cold();
    uninstall_flow_extension("bulk").expect("bulk extension uninstall admission");
}

#[test]
fn contributed_extension_manifest_installs_catalogue_operator() {
    let _serialized = crate::registry::lock_flow_extension_registry_for_test();
    let manifest = r#"{"schema":"flow.extension","id":"stubext","name":"Stub","version":"0.0.1","activationEvents":[],"contributes":{"schemas":[],"operators":[{"id":"stubext.echo","extension":"stubext","name":"Echo","abbreviation":"Echo","icon":"emoji:📣️","summary":"Echo","inputs":[],"outputs":[]}],"widgets":[],"commands":[],"settings":[]}}"#;
    install_flow_extension_manifest("stub-plugin", manifest).expect("stub extension admission");
    assert!(flow_extension_registry().operator_info("stubext.echo").is_some());
    let sections = flow_catalogue_sections();
    assert!(sections.iter().any(|section| section.id == "stubext"));
    uninstall_flow_extension("stubext").expect("stub extension uninstall admission");
}

#[test]
fn flow_fixture_with_synapses_builds_dag_edges_and_ports() {
    install_first_party_light_flow_extensions_for_tests();
    let mut host = host_with_test_bridge();
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    host.replace_fixture(<FlowFixture as crate::os_store::ArtifactDsl>::parse_dsl(include_str!("../../../📚️examples/🗣️.dsl.semio")).expect("fixture"));
    assert!(!host.dag.fixture.edges.is_empty(), "synapses should become dag edges");
    let add = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add node");
    assert_eq!(add.inputs().len(), 2);
    assert_eq!(add.outputs().len(), 1);
    let mut scene = canvas::Scene::new();
    host.set_canvas_theme_dark(true);
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    assert!(scene.path_count() > 8, "rich flow graph should paint edges and handles");
    host.retire_cold();
}

#[test]
fn add_widget_and_connect() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.passThrough"}"#, 100.0, 50.0).unwrap();
    host.connect_ports("slider", "number", &id, "number").unwrap();
    host.connect_ports(&id, "number", "preview", "").unwrap();
    host.set_slider_value("slider", 4.0);
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "4");
    host.retire_cold();
}

#[test]
fn output_export_widget_catalogue_descriptor_and_payload() {
    let mut host = host_with_test_bridge();
    let sections = merge_catalogue_sections("").unwrap();
    let exports: Vec<_> = sections.iter().flat_map(|section| section.items.iter()).filter(|item| item.kind == "outputExport").collect();
    assert_eq!(exports.len(), 4);
    assert!(exports.iter().any(|item| item.format.as_deref() == Some("svg")));
    let id = host.add_widget(r#"{"kind":"outputExport","format":"png"}"#, 120.0, 80.0).unwrap();
    host.connect_ports("add", "sum", &id, "").unwrap();
    host.set_slider_value("slider", 4.0);
    host.evaluate_internal();
    let payload_json = host.export_payload_json(&id).expect("export payload");
    assert_ne!(payload_json, "{}");
    assert!(payload_json.contains("4") || payload_json.contains("value") || payload_json.contains("sum"));
    let node = host.dag.fixture.nodes.iter().find(|node| node.id == id).expect("export node");
    assert!(matches!(node.kind, DagNodeKind::Export { .. }));
    host.retire_cold();
}

/// ↩️ Exercises the standard `crate::os_store::ArtifactStore<FlowFixture, FlowMutation>` undo/redo
/// mechanism directly (the same one `FlowHost::undo`/`redo` are built on) — add a widget, undo,
/// confirm it's gone, redo, confirm it's back — in place of the old test's direct assertions on a
/// hand-rolled `Vec<FlowFixture>` snapshot stack.
#[semio_framework_async_macros::async_test]
async fn undo_redo_add_widget() {
    let mut host = host_with_test_bridge();
    let fixture_before = host.fixture.clone();
    let count_before = fixture_before.widgets.len();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"undo me"}"#, 42.0, 42.0).unwrap();
    assert_eq!(host.fixture.widgets.len(), count_before + 1);

    let operations = flow_fixture_operations(&fixture_before, &host.fixture).expect("wire-representable flow fixture");
    assert!(!operations.is_empty(), "add_widget must diff into vcs operations");

    let envelope: FlowEnvelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "test", fixture_before, None);
    let mut store = FlowStore::new(envelope).await.expect("valid flow store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: operations, description: None }).await.expect("apply add-widget operations");
    assert_eq!(store.snapshot().expect("projection").widgets.len(), count_before + 1);

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    let after_undo = store.snapshot().expect("projection");
    assert_eq!(after_undo.widgets.len(), count_before);
    assert!(!after_undo.widgets.iter().any(|w| widget_id_for(w) == id));

    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    let after_redo = store.snapshot().expect("projection");
    assert!(after_redo.widgets.iter().any(|w| widget_id_for(w) == id));
}

#[test]
fn camera_change_does_not_create_undo_step() {
    let mut host = host_with_test_bridge();
    let camera_before = host.fixture.camera.clone();
    host.set_camera(camera_before.x + 50.0, camera_before.y - 30.0, camera_before.zoom * 1.5);
    assert!(!host.can_undo());
    let id = host.add_widget(r#"{"kind":"inputNote","text":"x"}"#, 0.0, 0.0).unwrap();
    assert!(host.can_undo());
    assert!(host.undo());
    assert_eq!(host.fixture.camera.x, camera_before.x + 50.0);
    assert_eq!(host.fixture.camera.y, camera_before.y - 30.0);
    assert!((host.fixture.camera.zoom - camera_before.zoom * 1.5).abs() < 1e-9);
    assert!(!host.fixture.widgets.iter().any(|w| widget_id_for(w) == id));
    host.retire_cold();
}

#[test]
fn replace_fixture_preserves_live_camera() {
    let mut host = host_with_test_bridge();
    host.set_camera(120.0, -45.0, 1.75);
    host.replace_fixture(FlowFixture {
        schema: "flow.fixture".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![Widget::InputNote { id: "note".into(), text: "hello".into() }],
        synapses: vec![],
        layout: crate::OrderedMap::new(),
    });
    assert_eq!(host.fixture.camera.x, 120.0);
    assert_eq!(host.fixture.camera.y, -45.0);
    assert!((host.fixture.camera.zoom - 1.75).abs() < 1e-9);
    assert!(host.fixture.widgets.iter().any(|w| widget_id_for(w) == "note"));
    host.retire_cold();
}

fn test_dictionary_merge_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
    if kind == "core.number" {
        let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
        return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value))));
    }
    if kind != "dictionary.merge" {
        return Err(EvalError::UnknownKind(kind.into()));
    }
    let items = input.get("items").and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput("items".into()))?;
    let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    if indices.len() < 2 {
        return Err(EvalError::MissingInput("items".into()));
    }
    let mut merged = Dictionary::with_schema("dictionary");
    for index in indices {
        let slot = items.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
        merged = merged.merge(slot);
    }
    Ok(channel_output("dictionary", merged))
}

#[test]
fn variadic_merge_evaluates_port_routed_inputs() {
    let mut host = FlowHost::from_fixture(FlowFixture {
        schema: "flow.fixture".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "a".into(), label: "A".into(), value: 1.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
            Widget::InputSlider { id: "b".into(), label: "B".into(), value: 2.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
            Widget::Neuron { id: "merge".into(), neuron_kind: "dictionary.merge".into(), params: Dictionary::new(), input_ports: vec!["0".into(), "1".into()], output_ports: vec![], preview: true },
            Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: crate::OrderedSet::new() },
        ],
        synapses: vec![
            SynapseSpec { id: "s1".into(), from: "a".into(), to: "merge".into(), from_port: "number".into(), to_port: "0".into() },
            SynapseSpec { id: "s2".into(), from: "b".into(), to: "merge".into(), from_port: "number".into(), to_port: "1".into() },
            SynapseSpec { id: "s3".into(), from: "merge".into(), to: "preview".into(), from_port: "dictionary".into(), to_port: String::new() },
        ],
        layout: crate::OrderedMap::new(),
    });
    host.set_eval_bridge_fn(Box::new(test_dictionary_merge_bridge));
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "dictionary.merge".into(),
        extension: "dictionary".into(),
        name: "Merge".into(),
        abbreviation: "Merge".into(),
        icon: "emoji:🔀️".into(),
        summary: "Merge".into(),
        inputs: vec![],
        outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
        variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
        ..Default::default()
    }]));
    host.previous_snapshot = None;
    host.outputs.clear();
    host.evaluate_internal();
    let preview = host
        .fixture
        .widgets
        .iter()
        .find_map(|widget| match widget {
            Widget::OutputPreview { preview, .. } => Some(preview),
            _ => None,
        })
        .expect("preview");
    assert_eq!(preview.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(2.0));
    host.retire_cold();
}

#[test]
fn widget_to_dag_node_carries_display_meta() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 0.0, 0.0).unwrap();
    let node = host.dag.fixture.nodes.iter().find(|node| node.id == id).expect("node");
    assert_eq!(node.name, "Add");
    assert_eq!(node.abbreviation, "Add");
    assert_eq!(node.icon, "emoji:➕️");
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_explicit_range() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":10.2,"min":10.2,"max":15.0,"step":0.1}"#, 0.0, 0.0).unwrap();
    let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 10.2).abs() < 1e-6);
    assert!((min - 10.2).abs() < 1e-6);
    assert!((max - 15.0).abs() < 1e-6);
    assert!((step - 0.1).abs() < 1e-6);
    let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
        panic!("expected slider node");
    };
    assert!((dag_min - 10.2).abs() < 1e-6);
    assert!((dag_max - 15.0).abs() < 1e-6);
    assert!((dag_step - 0.1).abs() < 1e-6);
    assert!((dag_value - 10.2).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn add_note_widget_with_text() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"some text"}"#, 0.0, 0.0).unwrap();
    let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputNote { text, .. } = widget else {
        panic!("expected note widget");
    };
    assert_eq!(text, "some text");
    let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Note { text: dag_text, .. } = &node.kind else {
        panic!("expected note node");
    };
    assert_eq!(dag_text, "some text");
    assert!(node.width >= 40.0);
    assert_eq!(node.height, semio_framework_artifact_infinite_dag::DAG_CHANNEL_ROW_HEIGHT);
    host.retire_cold();
}

#[test]
fn begin_note_edit_groups_undo_into_single_gesture() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
    let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
    let origin_x = node.x - node.width * 0.5 + 4.0;
    host.begin_note_edit(&id, origin_x + 40.0, node.y);
    host.note_insert_text("!");
    host.note_commit_edit();
    let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputNote { text, .. } = widget else {
        panic!("expected note widget");
    };
    assert_eq!(text, "hi!");
    assert!(host.undo());
    let Widget::InputNote { text: restored, .. } = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget") else {
        panic!("expected note widget");
    };
    assert_eq!(restored, "hi");
    host.retire_cold();
}

#[test]
fn wheel_screen_zoom_gesture_changes_zoom() {
    let mut host = host_with_test_bridge();
    let z0 = host.fixture.camera.zoom;
    host.wheel_screen(400.0, 300.0, 0.0, -10.0, true);
    assert_ne!(host.fixture.camera.zoom, z0);
    host.retire_cold();
}

#[test]
fn wheel_plan_matches_direct_and_rejects_stale_revision() {
    let mut direct = host_with_test_bridge();
    let mut planned = host_with_test_bridge();
    direct.set_viewport(800, 600, 1.0);
    planned.set_viewport(800, 600, 1.0);
    direct.wheel_screen(320.0, 240.0, 0.0, -10.0, true);
    let plan = planned.plan_wheel(320.0, 240.0, 0.0, -10.0, true);
    assert!(planned.commit_wheel(plan));
    assert_eq!(direct.fixture.camera, planned.fixture.camera);

    let stale = planned.plan_wheel(320.0, 240.0, 0.0, -10.0, true);
    planned.pointer_down_screen(10.0, 10.0, 0, false, false, false, true);
    let replacement = planned.fixture.camera.clone();
    assert!(!planned.commit_wheel(stale));
    assert_eq!(planned.fixture.camera, replacement);
    planned.retire_cold();
    direct.retire_cold();
}

#[test]
fn set_note_text_keeps_uniform_component_width() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
    let short_w = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node").width;
    host.set_note_text(&id, "a much longer note string");
    let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Note { text, .. } = &node.kind else {
        panic!("expected note node");
    };
    assert_eq!(text, "a much longer note string");
    assert_eq!(node.width, short_w);
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_single_value_uses_sensible_range() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":5.0}"#, 0.0, 0.0).unwrap();
    let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 5.0).abs() < 1e-6);
    assert!((min - 0.0).abs() < 1e-6);
    assert!((max - 10.0).abs() < 1e-6);
    assert!((step - 1.0).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_decimal_value_uses_matching_step() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.3}"#, 0.0, 0.0).unwrap();
    let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 1.3).abs() < 1e-6);
    assert!((min - 0.0).abs() < 1e-6);
    assert!((max - 10.0).abs() < 1e-6);
    assert!((step - 0.1).abs() < 1e-6);
    let node = host.dag.fixture.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
        panic!("expected slider node");
    };
    assert!((dag_min - 0.0).abs() < 1e-6);
    assert!((dag_max - 10.0).abs() < 1e-6);
    assert!((dag_step - 0.1).abs() < 1e-6);
    assert!((dag_value - 1.3).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_two_decimal_places_uses_finer_step() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.25}"#, 0.0, 0.0).unwrap();
    let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((step - 0.01).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn set_slider_value_expands_bounds_when_out_of_range() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":3.0,"min":0.0,"max":10.0,"step":1.0}"#, 0.0, 0.0).unwrap();
    host.set_slider_value(&id, 12.0);
    let widget = host.fixture.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 12.0).abs() < 1e-6);
    assert!((min - 0.0).abs() < 1e-6);
    assert!((max - 20.0).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn ghost_widget_matches_placed_neuron_size() {
    let mut host = host_with_test_bridge();
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "brep.sketch2d.circle".into(),
        extension: "brep".into(),
        name: "Sketch Circle".into(),
        abbreviation: "Circle".into(),
        icon: "emoji:⚪️".into(),
        summary: "Sketched circle profile".into(),
        inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
        outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
        ..Default::default()
    }]));
    let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
    host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
    let ghost_width = host.ghost_node.as_ref().expect("ghost").width;
    let placed_id = host.add_widget(descriptor, 80.0, 80.0).unwrap();
    let placed_width = host.dag.fixture.nodes.iter().find(|node| node.id == placed_id).expect("placed").width;
    assert!((ghost_width - placed_width).abs() < 1e-6, "ghost width {ghost_width} != placed {placed_width}");
    host.retire_cold();
}

#[test]
fn ghost_widget_preview_and_clear() {
    let mut host = host_with_test_bridge();
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 42.0, 24.0).unwrap();
    let ghost = host.ghost_node.as_ref().expect("ghost");
    assert!((ghost.x - 42.0).abs() < 1e-6);
    assert!((ghost.y - 24.0).abs() < 1e-6);
    assert_eq!(ghost.name, "Add");
    assert_eq!(ghost.abbreviation, "Add");
    assert_eq!(ghost.icon, "emoji:➕️");
    host.clear_ghost_widget();
    assert!(host.ghost_node.is_none());
    host.retire_cold();
}

#[test]
fn ghost_widget_label_overlay_matches_placed_at_micro() {
    let mut host = host_with_test_bridge();
    host.set_viewport(1280, 800, 1.0);
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "brep.sketch2d.circle".into(),
        extension: "brep".into(),
        name: "Sketch Circle".into(),
        abbreviation: "Circle".into(),
        icon: "emoji:⚪️".into(),
        summary: "Sketched circle profile".into(),
        inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
        outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
        ..Default::default()
    }]));
    host.dag.set_automatic_lod(false);
    host.dag.set_forced_draw_lod_label("micro");
    let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
    host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
    let ghost = host.dag.ghost_node().expect("ghost");
    assert_eq!(host.draw_lod_label(), "micro");
    assert_eq!(semio_framework_artifact_infinite_dag::DagDrawLod::Micro.node_label(), semio_framework_artifact_infinite_dag::DagNodeLabel::Name);
    assert!(semio_framework_artifact_infinite_dag::DagDrawLod::Micro.shows_port_labels());
    assert!(semio_framework_artifact_infinite_dag::DagDrawLod::Micro.shows_handles());
    let ghost_overlay_rows = host.dag.label_overlay_rows_for_node_spec(ghost, true);
    assert_eq!(ghost_overlay_rows.len(), 3);
    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let overlay_ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
    assert_eq!(overlay_ghost_rows.len(), 3);
    let placed_node = {
        let widget = widget_from_descriptor(&serde_json::from_str::<WidgetDescriptor>(descriptor).unwrap(), "placed".into(), &host.kind_infos);
        let mut layout = crate::OrderedMap::new();
        layout.insert("placed".into(), WidgetLayout { x: 80.0, y: 80.0 });
        let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &host.kind_infos, widget_node_size(&widget, &[], &host.kind_infos));
        widget.retire_cold();
        let mut retirement = crate::retained::FlowRetirement::default();
        retirement.push(crate::retained::FlowOwner::Layouts(layout));
        retirement.retire_cold();
        fit_node_size(&mut node);
        node
    };
    let placed_rows = host.dag.label_overlay_rows_for_node_spec(&placed_node, false);
    assert_eq!(ghost_overlay_rows.len(), placed_rows.len());
    for (ghost_row, placed_row) in ghost_overlay_rows.iter().zip(placed_rows.iter()) {
        assert_eq!(ghost_row["text"], placed_row["text"]);
        assert_eq!(ghost_row["layout"], placed_row["layout"]);
        assert_eq!(ghost_row["align"], placed_row["align"]);
    }
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    host.retire_cold();
}

#[test]
fn rebuild_dag_preserves_ghost_overlay_at_micro() {
    let mut host = host_with_test_bridge();
    host.set_viewport(1280, 800, 1.0);
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "brep.sketch2d.circle".into(),
        extension: "brep".into(),
        name: "Sketch Circle".into(),
        abbreviation: "Circle".into(),
        icon: "emoji:⚪️".into(),
        summary: "Sketched circle profile".into(),
        inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
        outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
        ..Default::default()
    }]));
    host.dag.set_automatic_lod(false);
    host.dag.set_forced_draw_lod_label("micro");
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#, 12.0, 18.0).unwrap();
    host.rebuild_dag();
    assert!(host.dag.ghost_node().is_some());
    assert_eq!(host.draw_lod_label(), "micro");
    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
    assert_eq!(ghost_rows.len(), 3);
    host.retire_cold();
}

#[test]
fn ghost_widget_paint_scene_smoke() {
    let mut host = host_with_test_bridge();
    host.set_viewport(800, 600, 1.0);
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 10.0, 20.0).unwrap();
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 800, 600, 1.0);
    host.retire_cold();
}

#[test]
fn selection_and_preview_state_round_trip() {
    let mut host = FlowHost::default();
    host.set_selection_json(r#"["slider","add"]"#);
    let selected: Vec<String> = serde_json::from_str(&host.selected_widget_ids_json()).unwrap();
    assert_eq!(selected, vec!["slider", "add"]);
    host.set_hover(Some("add"));
    assert_eq!(host.hovered_widget_id().as_deref(), Some("add"));
    host.set_preview_off_json(r#"["add"]"#);
    assert_eq!(host.preview_off_widget_ids(), vec!["add"]);
    host.toggle_preview("add").unwrap();
    assert!(host.preview_off_widget_ids().is_empty());
    host.retire_cold();
}

#[test]
fn channel_hover_and_selection_round_trip_at_detail_lod() {
    let mut host = host_with_test_bridge();
    host.dag.set_automatic_lod(false);
    host.dag.set_forced_draw_lod_label("detail");
    host.set_hover_channel(Some("add"), Some("a"));
    let hovered: dag::DagChannelRef = crate::os_pack::json::from_json_str(&host.hovered_channel_json()).unwrap();
    assert_eq!(hovered.widget_id, "add");
    assert_eq!(hovered.port, "a");
    assert_eq!(hovered.direction, "in");
    host.set_selected_channels_json(r#"[{"widgetId":"add","port":"a","direction":"in"}]"#);
    let selected: Vec<dag::DagChannelRef> = crate::os_pack::json::from_json_str(&host.selected_channels_json()).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].widget_id, "add");
    assert_eq!(selected[0].port, "a");
    host.retire_cold();
}

#[test]
fn drag_merge_node_preserves_single_fixture_widget() {
    let mut host = host_with_test_bridge();
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "dictionary.merge".into(),
        extension: "dictionary".into(),
        name: "Merge".into(),
        abbreviation: "Merge".into(),
        icon: "emoji:🔀️".into(),
        summary: "Merge".into(),
        inputs: vec![],
        outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
        variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
        ..Default::default()
    }]));
    let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 120.0, 80.0).unwrap();
    host.set_viewport(800, 600, 1.0);
    let merge = host.dag.fixture.nodes.iter().find(|n| n.id == merge_id).expect("merge").clone();
    let grab = Point::new(merge.x, merge.y);
    let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, grab);
    host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
    host.pointer_move_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
    host.pointer_up_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
    assert_eq!(host.fixture.widgets.iter().filter(|w| widget_id_for(w) == merge_id).count(), 1);
    assert_eq!(host.dag.fixture.nodes.iter().filter(|n| n.id == merge_id).count(), 1);
    let moved = host.fixture.layout.get(&merge_id).expect("merge layout");
    assert!((moved.x - merge.x).abs() > 1.0);
    host.retire_cold();
}

#[test]
fn ghost_widget_cleared_on_pointer_down_and_add_widget() {
    let mut host = host_with_test_bridge();
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 12.0, 18.0).unwrap();
    host.set_viewport(800, 600, 1.0);
    host.pointer_down_screen(120.0, 120.0, 0, false, false, false, false);
    assert!(host.ghost_node.is_none());
    host.set_ghost_widget(r#"{"kind":"inputSlider","label":"Number"}"#, 0.0, 0.0).unwrap();
    let _ = host.add_widget(r#"{"kind":"inputSlider","label":"Number"}"#, 40.0, 40.0).unwrap();
    assert!(host.ghost_node.is_none());
    assert_eq!(host.fixture.widgets.iter().filter(|w| widget_id_for(w).starts_with("slider")).count(), 2);
    assert_eq!(host.dag.fixture.nodes.iter().filter(|n| n.id == "slider").count(), 1);
    host.retire_cold();
}

#[test]
fn delete_selection_removes_widget_from_fixture() {
    let mut host = host_with_test_bridge();
    host.dag.set_selection(&["slider".into()]);
    host.delete_selection().unwrap();
    assert!(host.fixture.widgets.iter().all(|w| widget_id_for(w) != "slider"));
    assert!(host.dag.fixture.nodes.iter().all(|n| n.id != "slider"));
    host.retire_cold();
}

#[test]
fn node_drag_proximity_skips_wired_cut_inputs_in_flow() {
    use canvas::camera::{world_to_screen, Camera, Viewport};
    use canvas::Point;
    let mut host = FlowHost::default();
    host.set_viewport(1280, 800, 1.0);
    host.fixture.widgets = vec![
        Widget::Neuron { id: "sphere".into(), neuron_kind: "brep.prim3d.sphere".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
        Widget::Neuron { id: "torus".into(), neuron_kind: "brep.prim3d.torus".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
        Widget::Neuron { id: "cut".into(), neuron_kind: "brep.bool.cut".into(), params: Dictionary::new(), input_ports: vec!["a".into(), "b".into()], output_ports: vec![], preview: true },
    ];
    host.fixture.synapses = vec![
        SynapseSpec { id: "e1".into(), from: "sphere".into(), to: "cut".into(), from_port: "solid".into(), to_port: "a".into() },
        SynapseSpec { id: "e2".into(), from: "torus".into(), to: "cut".into(), from_port: "solid".into(), to_port: "b".into() },
    ];
    host.fixture.layout.insert("sphere".into(), WidgetLayout { x: 0.0, y: -60.0 });
    host.fixture.layout.insert("torus".into(), WidgetLayout { x: 0.0, y: 60.0 });
    host.fixture.layout.insert("cut".into(), WidgetLayout { x: 240.0, y: 0.0 });
    let solid_out = vec![InputSpec::named("S", "Sld", "solid", "Solid")];
    host.set_neuron_kind_infos_json(&crate::os_pack::json::to_json_string(&vec![
        NeuronKindInfo {
            id: "brep.prim3d.sphere".into(),
            extension: "brep".into(),
            name: "Sphere".into(),
            abbreviation: "Sphere".into(),
            icon: "emoji:⚪️".into(),
            summary: "Sphere".into(),
            inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
            outputs: solid_out.clone(),
            ..Default::default()
        },
        NeuronKindInfo {
            id: "brep.prim3d.torus".into(),
            extension: "brep".into(),
            name: "Torus".into(),
            abbreviation: "Torus".into(),
            icon: "emoji:🛢️".into(),
            summary: "Torus".into(),
            inputs: vec![InputSpec::number_default("major", 2.0, NUMBER_OPS), InputSpec::number_default("minor", 0.5, NUMBER_OPS)],
            outputs: solid_out.clone(),
            ..Default::default()
        },
        NeuronKindInfo {
            id: "brep.bool.cut".into(),
            extension: "brep".into(),
            name: "Cut".into(),
            abbreviation: "Cut".into(),
            icon: "emoji:🔗️".into(),
            summary: "Cut".into(),
            inputs: vec![InputSpec::requires("a", &["geometry"]), InputSpec::requires("b", &["geometry"])],
            outputs: solid_out,
            ..Default::default()
        },
    ]));
    host.rebuild_dag();
    host.dag.set_proximity_distance(160.0);
    host.dag.set_automatic_lod(false);
    host.dag.set_forced_draw_lod_label("normal");
    assert_eq!(host.dag.engine.edges.len(), 2, "synapses should load as engine edges");
    let cut = host.dag.fixture.nodes.iter().find(|node| node.id == "cut").expect("cut");
    let grab = Point::new(cut.x, cut.y);
    let cam = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, grab);
    host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
    host.pointer_move_screen(screen.x - 180.0, screen.y, false, false, false);
    assert!(host.dag.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges");
    host.pointer_up_screen(screen.x - 180.0, screen.y, false, false, false);
    assert_eq!(host.dag.engine.edges.len(), 2);
    assert_eq!(host.fixture.synapses.len(), 2);
    host.retire_cold();
}

#[test]
fn dag_bridge_keeps_same_named_brep_input_and_output_distinct() {
    let mut host = FlowHost::default();
    host.fixture.widgets = vec![
        Widget::Neuron { id: "extrude".into(), neuron_kind: "brep.solid.extrude".into(), params: Dictionary::new(), input_ports: vec!["wire".into(), "vector".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "brep".into(), neuron_kind: "brep.brep".into(), params: Dictionary::new(), input_ports: vec!["brep".into(), "vertex".into(), "edge".into(), "face".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "get".into(), neuron_kind: "list.get".into(), params: Dictionary::new(), input_ports: vec!["list".into(), "index".into(), "wrap".into()], output_ports: vec!["0".into()], preview: true },
    ];
    host.fixture.synapses = vec![
        SynapseSpec { id: "e112".into(), from: "extrude".into(), to: "brep".into(), from_port: "solid".into(), to_port: "brep".into() },
        SynapseSpec { id: "e113".into(), from: "brep".into(), to: "get".into(), from_port: "brep".into(), to_port: "list".into() },
    ];
    host.fixture.layout.insert("extrude".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.fixture.layout.insert("brep".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.fixture.layout.insert("get".into(), WidgetLayout { x: 400.0, y: 0.0 });
    host.set_neuron_kind_infos_json(&crate::os_pack::json::to_json_string(&vec![
        NeuronKindInfo {
            id: "brep.solid.extrude".into(),
            extension: "brep".into(),
            name: "Extrude".into(),
            abbreviation: "Extr".into(),
            icon: "emoji:⬆️".into(),
            summary: "Extrude".into(),
            inputs: vec![InputSpec::requires("wire", &["geometry"]), InputSpec::requires("vector", &["vector"])],
            outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "brep.brep".into(),
            extension: "brep".into(),
            name: "Brep".into(),
            abbreviation: "Brep".into(),
            icon: "emoji:🧊️".into(),
            summary: "Brep".into(),
            inputs: vec![InputSpec::requires("brep", &["brep.brep"]), InputSpec::list("vertex", &["brep.brep"]), InputSpec::list("edge", &["brep.brep"]), InputSpec::list("face", &["brep.brep"])],
            outputs: vec![InputSpec::named("B", "Brp", "brep", "Brep")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "list.get".into(),
            extension: "list".into(),
            name: "Get".into(),
            abbreviation: "Get".into(),
            icon: "emoji:📋️".into(),
            summary: "Get".into(),
            inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
            outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
            ..Default::default()
        },
    ]));
    host.rebuild_dag();
    let incoming = host.dag.engine.edges.get(&112).expect("incoming brep edge");
    let outgoing = host.dag.engine.edges.get(&113).expect("outgoing brep edge");
    let incoming_target = host.dag.engine.handles.get(&incoming.target).expect("incoming target");
    let outgoing_source = host.dag.engine.handles.get(&outgoing.source).expect("outgoing source");
    assert_eq!(incoming_target.role, HandleRole::Target);
    assert_eq!(outgoing_source.role, HandleRole::Source);
    host.retire_cold();
}

#[test]
fn delete_selection_removes_selected_edge_from_fixture() {
    let mut host = host_with_test_bridge();
    let synapse_count_before = host.fixture.synapses.len();
    assert!(synapse_count_before > 0);
    let edge_id = *host.dag.engine.edges.keys().next().expect("edge");
    host.dag.engine.selection.edge_ids.insert(edge_id);
    assert!(host.has_selection());
    host.delete_selection().unwrap();
    assert!(host.fixture.synapses.len() < synapse_count_before);
    assert!(!host.has_selection());
    host.retire_cold();
}

#[test]
fn delete_selection_removes_edge_selected_by_synapse_id_domain() {
    let mut host = host_with_test_bridge();
    let before = host.fixture.synapses.len();
    host.dag.set_selection_domains_json(r#"{"nodes":[],"edges":["s1"],"🐙️handles":[]}"#);
    assert!(host.has_selection(), "synapse id s1 must map into engine edge selection");
    host.delete_selection().unwrap();
    assert!(host.fixture.synapses.len() < before);
    assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
    host.retire_cold();
}

#[test]
fn align_selection_left_aligns_selected_widget_layout() {
    let mut host = host_with_test_bridge();
    host.move_widget("slider", -120.0, 20.0).unwrap();
    host.move_widget("add", 180.0, -40.0).unwrap();
    host.dag.set_selection(&["slider".into(), "add".into()]);
    host.align_selection("alignLeft").unwrap();
    let slider = host.dag.fixture.nodes.iter().find(|node| node.id == "slider").expect("slider");
    let add = host.dag.fixture.nodes.iter().find(|node| node.id == "add").expect("add");
    let slider_left = slider.x - slider.width * 0.5;
    let add_left = add.x - add.width * 0.5;
    assert!((slider_left - add_left).abs() < 1e-6, "left edges should match after alignLeft");
    assert!(host.fixture.layout.contains_key("slider"));
    assert!(host.fixture.layout.contains_key("add"));
    host.retire_cold();
}

#[test]
fn add_input_port_inserts_variadic_slot() {
    let mut host = host_with_test_bridge();
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "dictionary.merge".into(),
        extension: "dictionary".into(),
        name: "Merge".into(),
        abbreviation: "Merge".into(),
        icon: "emoji:🔀️".into(),
        summary: "Merge".into(),
        inputs: vec![],
        outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
        variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
        ..Default::default()
    }]));
    let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 0.0, 0.0).unwrap();
    host.add_input_port(&merge_id, 1).unwrap();
    let widget = host.fixture.widgets.iter().find(|widget| widget_id_for(widget) == merge_id).expect("merge");
    let Widget::Neuron { input_ports, .. } = widget else { panic!("neuron") };
    assert_eq!(input_ports.len(), 3);
    host.retire_cold();
}

#[test]
fn add_output_port_inserts_variadic_get_slot() {
    let mut host = host_with_test_bridge();
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "list.get".into(),
        extension: "list".into(),
        name: "Get".into(),
        abbreviation: "Get".into(),
        icon: "emoji:📋️".into(),
        summary: "Reads consecutive values by index".into(),
        inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
        outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
        variadic_output: Some(neural::VariadicSpec { slot_key: "value".into(), min: 1, max: None }),
        ..Default::default()
    }]));
    let get_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"list.get"}"#, 0.0, 0.0).unwrap();
    let node = host.dag.fixture.nodes.iter().find(|node| node.id == get_id).expect("get");
    let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
    assert_eq!(labels, vec!["i"]);
    host.add_output_port(&get_id, 1).unwrap();
    let widget = host.fixture.widgets.iter().find(|widget| widget_id_for(widget) == get_id).expect("get");
    let Widget::Neuron { output_ports, .. } = widget else { panic!("neuron") };
    assert_eq!(output_ports.len(), 2);
    let node = host.dag.fixture.nodes.iter().find(|node| node.id == get_id).expect("get");
    let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
    assert_eq!(labels, vec!["i", "i+1"]);
    host.retire_cold();
}

#[test]
fn add_widget_with_explicit_id() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","id":"custom_slider","value":2.0}"#, 0.0, 0.0).unwrap();
    assert_eq!(id, "custom_slider");
    host.retire_cold();
}

#[test]
fn insert_between_rewires_downstream_and_connects_anchor() {
    let mut host = host_with_test_bridge();
    let mid = host.add_widget(r#"{"kind":"neuron","id":"mid","neuronKind":"math.passThrough"}"#, 120.0, 0.0).unwrap();
    host.insert_between("slider", "number", &mid, "number", "number").unwrap();
    assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "mid"));
    assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "mid" && synapse.to == "add"));
    assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "add" && synapse.to == "preview"));
    assert!(!host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "add"));
    host.retire_cold();
}

#[test]
fn insert_between_preserves_existing_mid_inputs() {
    let mut host = host_with_test_bridge();
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 120.0, 0.0).unwrap();
    host.connect_ports("slider", "number", &variable_id, "width").unwrap();
    host.insert_between("slider", "number", &variable_id, "width", "width").unwrap();
    assert!(host.fixture.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == variable_id && synapse.to_port == "width"));
    assert!(!host.fixture.synapses.iter().any(|synapse| synapse.from == variable_id && synapse.to == variable_id));
    host.retire_cold();
}

#[test]
fn make_space_shifts_widgets_right_of_anchor() {
    let mut host = host_with_test_bridge();
    host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.fixture.layout.insert("preview".into(), WidgetLayout { x: 400.0, y: 0.0 });
    host.rebuild_dag();
    host.make_space("slider", 100.0, 0.0).unwrap();
    assert!((host.fixture.layout.get("slider").expect("slider").x - 0.0).abs() < 1e-6);
    assert!((host.fixture.layout.get("add").expect("add").x - 300.0).abs() < 1e-6);
    assert!((host.fixture.layout.get("preview").expect("preview").x - 500.0).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn set_neuron_params_merges_into_eval_input() {
    let mut host = host_with_test_bridge();
    let preview_synapse = host.fixture.synapses.iter().find(|synapse| synapse.from == "add" && synapse.to == "preview").map(|synapse| synapse.id.clone()).expect("preview synapse");
    host.disconnect(&preview_synapse).unwrap();
    let id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough"}"#, 100.0, 0.0).unwrap();
    host.connect_ports(&id, "number", "preview", "").unwrap();
    host.set_neuron_params(&id, r#"{"number":{"$schema":"number","value":7.5}}"#).unwrap();
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "7.5");
    host.retire_cold();
}

#[test]
fn cluster_ports_from_contract() {
    let inner = Tree {
        neurons: vec![
            Neuron::with_kind("in_a", INPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("a".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
            Neuron::with_kind("out_sum", OUTPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("sum".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
        ],
        synapses: vec![],
    };
    let widget = Widget::Cluster { id: "cluster".into(), name: "Add cluster".into(), tree: inner, flow: FlowGui::default() };
    let (inputs, outputs, _, _) = widget_io_ports(&widget, &[], &HashMap::new());
    assert_eq!(inputs.len(), 1);
    assert_eq!(outputs.len(), 1);
    assert_eq!(inputs[0].id, "a");
    assert_eq!(outputs[0].id, "sum");
}

#[test]
fn variable_relay_evaluates_through_flow_host() {
    let mut host = host_with_test_bridge();
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 0.0, 0.0).unwrap();
    let slider_id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":4.0}"#, -200.0, 0.0).unwrap();
    host.connect_ports(&slider_id, "number", &variable_id, "width").unwrap();
    let eval_json = host.evaluate().expect("evaluate");
    let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
    let width = parsed.get(&variable_id).and_then(|entry| entry.get("out")).and_then(|out| out.get("width")).expect("variable width output");
    assert_eq!(width.get("$schema").and_then(|value| value.as_str()), Some("number"));
    host.retire_cold();
}

#[test]
fn collapse_uses_variable_name_as_cluster_input_port() {
    let mut host = host_with_test_bridge();
    host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 100.0, 0.0).unwrap();
    host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.fixture.synapses.retain(|synapse| synapse.from != "slider" || synapse.to != "add");
    host.connect_ports("slider", "number", &variable_id, "width").unwrap();
    host.connect_ports(&variable_id, "width", "add", "a").unwrap();
    host.rebuild_dag();
    let cluster_id = host.collapse_selection(&[variable_id.clone(), "add".into()]).unwrap();
    let cluster = host
        .fixture
        .widgets
        .iter()
        .find_map(|widget| match widget {
            Widget::Cluster { id, tree, .. } if id == &cluster_id => Some(tree.clone()),
            _ => None,
        })
        .expect("cluster");
    let (inputs, _) = cluster.contract();
    assert!(inputs.iter().any(|port| port.name == "width"));
    host.explode_cluster(&cluster_id).unwrap();
    assert!(host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Variable { name, .. } if name == "width")));
    host.retire_cold();
}

#[test]
fn collapse_then_explode_round_trips() {
    let mut host = host_with_test_bridge();
    host.fixture.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.fixture.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.rebuild_dag();
    let cluster_id = host.collapse_selection(&["slider".into(), "add".into()]).unwrap();
    assert!(host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { id, .. } if id == &cluster_id)));
    host.explode_cluster(&cluster_id).unwrap();
    assert!(host.fixture.widgets.iter().any(|widget| widget_id_for(widget).starts_with(&format!("{cluster_id}/"))));
    assert!(!host.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { .. })));
    host.retire_cold();
}

#[test]
fn rectangle_extrude_fixture_port_labels_follow_draw_lod() {
    let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "label": "Width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "label": "Height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "label": "Distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
    let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
    let mut host = FlowHost::from_fixture(fixture);
    host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
    host.set_viewport(1280, 800, 1.0);
    host.fixture.camera.zoom = 1.0;
    host.rebuild_dag();
    let mut port_texts = |lod: &str| -> Vec<String> {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(lod);
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        raw["labels"].as_array().expect("labels").iter().filter(|row| row["kind"] == "port").filter_map(|row| row["text"].as_str().map(str::to_string)).collect()
    };
    let normal = port_texts("normal");
    assert!(normal.iter().any(|text| text.ends_with("wid")), "normal ports: {normal:?}");
    assert!(normal.iter().any(|text| text.ends_with("wir")), "normal ports: {normal:?}");
    let detail = port_texts("detail");
    assert!(detail.iter().any(|text| text.ends_with("width")), "detail ports: {detail:?}");
    assert!(detail.iter().any(|text| text.ends_with("wire")), "detail ports: {detail:?}");
    let micro = port_texts("micro");
    assert!(micro.iter().any(|text| text.ends_with("RectangleWire")), "micro ports: {micro:?}");
    assert!(micro.iter().any(|text| text.ends_with("ExtrudedSolid")), "micro ports: {micro:?}");
}

#[test]
fn rectangle_extrude_fixture_evaluates_solid_output() {
    let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 140, "y": -60, "zoom": 2.2 },
  "widgets": [
    { "kind": "inputSlider", "id": "width", "label": "Width", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "height", "label": "Height", "value": 2, "min": 0.1, "max": 10, "step": 0.1 },
    { "kind": "inputSlider", "id": "distance", "label": "Distance", "value": 3, "min": 0.1, "max": 10, "step": 0.1 },
    {
      "kind": "neuron",
      "id": "rect",
      "neuronKind": "brep.curve.rectangle",
      "params": {},
      "input_ports": ["width", "height"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "vector",
      "neuronKind": "math.vector",
      "params": {},
      "input_ports": ["x", "y", "z"],
      "preview": false
    },
    {
      "kind": "neuron",
      "id": "extrude",
      "neuronKind": "brep.solid.extrude",
      "params": {},
      "input_ports": ["wire", "vector"],
      "preview": true
    },
    {
      "kind": "neuron",
      "id": "volume",
      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
    let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
    let mut host = FlowHost::from_fixture(fixture);
    host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
    let eval_json = host.evaluate().expect("evaluate");
    let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
    let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
    assert_eq!(solid.get("$schema").and_then(|v| v.as_str()), Some("geometry"));
    assert_eq!(solid.get("kind").and_then(|v| v.as_str()), Some("solid"));
}

#[test]
fn hexagonal_mushroom_fixture_reports_extruded_solid_output() {
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 94.75581571737445, "y": -97.50833134679668, "zoom": 1.7844325616011099 },
  "widgets": [
    { "kind": "inputSlider", "id": "height", "label": "Column Height", "value": 6.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "m" },
    { "kind": "inputSlider", "id": "radius", "label": "Profile Radius", "value": 0.5, "min": 0.1, "max": 2.0, "step": 0.05, "unit": "m" },
    { "kind": "inputSlider", "id": "sides", "label": "Side Count", "value": 6.0, "min": 3.0, "max": 12.0, "step": 1.0 },
    { "kind": "neuron", "id": "profile", "neuronKind": "brep.curve.polygon", "params": {}, "input_ports": ["radius", "sides"], "preview": false },
    { "kind": "neuron", "id": "extrusion-axis", "neuronKind": "math.vector", "params": {}, "input_ports": ["x", "y", "z"], "preview": false },
    { "kind": "neuron", "id": "extrude", "neuronKind": "brep.solid.extrude", "params": {}, "input_ports": ["wire", "vector"], "preview": true },
    { "kind": "outputPreview", "id": "column-preview", "preview": {}, "expanded": [] }
  ],
  "synapses": [
    { "id": "e1", "from": "height", "to": "extrusion-axis", "fromPort": "number", "toPort": "z" },
    { "id": "e2", "from": "radius", "to": "profile", "fromPort": "number", "toPort": "radius" },
    { "id": "e3", "from": "sides", "to": "profile", "fromPort": "number", "toPort": "sides" },
    { "id": "e4", "from": "profile", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "column-preview", "fromPort": "solid", "toPort": "" }
  ],
  "layout": {
    "height": { "x": -197.1913555449187, "y": -102.70789997839545 },
    "radius": { "x": -156.03796288966, "y": -177.3373596163105 },
    "sides": { "x": -156.43467044109153, "y": -155.28679730672846 },
    "profile": { "x": -64.49671116929301, "y": -163.40310309861746 },
    "extrusion-axis": { "x": -65.26327021036892, "y": -116.45687403531778 },
    "extrude": { "x": 34.842068675720895, "y": -154.18083645790136 },
    "column-preview": { "x": 237.4197774877085, "y": -103.14518978933415 }
  }
}
"#;
    let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
    let mut host = FlowHost::from_fixture(fixture);
    host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
    let eval_json = host.evaluate().expect("evaluate");
    let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
    let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
    assert_eq!(solid.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"));
    assert_eq!(solid.get("kind").and_then(serde_json::Value::as_str), Some("solid"));
    let handle = solid.get("handle").and_then(serde_json::Value::as_str).expect("solid handle");
    assert!(handle.starts_with("solid-"));
    let mesh = crate::tessellate_geometry(handle, 0.05).expect("solid mesh");
    assert!(!mesh.positions.is_empty());
    assert!(mesh.indices.len() >= 3);
}

#[test]
fn compiled_wire_literal_includes_operator_kinds() {
    let host = host_with_test_bridge();
    let text = host.compiled_wire_literal();
    assert!(text.contains("core.number"));
    assert!(text.contains("math.add"));
    host.retire_cold();
}

#[test]
fn flow_fixture_to_form_spec_maps_input_widgets() {
    use self::forms_bridge::flow_fixture_to_form_spec;
    let fixture = FlowFixture::default();
    let spec = flow_fixture_to_form_spec(&fixture);
    let kinds: Vec<&str> = spec.steps[0].blocks.iter().map(|question| question.kind.as_str()).collect();
    assert!(kinds.contains(&"slider"));
}

#[test]
fn apply_generation_values_to_fixture_patches_slider_value() {
    use self::forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec};
    let fixture = FlowFixture::default();
    let spec = flow_fixture_to_form_spec(&fixture);
    let slider_id = spec.steps[0].blocks.iter().find(|question| question.kind == "slider").map(|question| question.id.clone()).expect("slider question");
    let fixture_json = crate::os_pack::json::to_json_string(&fixture);
    let mut values = crate::os_pack::json::Object::new();
    values.insert(slider_id.clone(), crate::os_pack::json::Value::Number(8.0.into()));
    let patched = apply_generation_values_to_fixture(&fixture_json, &values);
    let reparsed = crate::os_pack::json::parse(&patched).expect("patched json");
    let slider = reparsed.get("widgets").and_then(|widgets| widgets.as_array()).and_then(|widgets| widgets.iter().find(|widget| widget.get("id").and_then(|id| id.as_str()) == Some(slider_id.as_str()))).expect("slider widget");
    assert_eq!(slider.get("value").and_then(|value| value.as_f64()), Some(8.0));
}
