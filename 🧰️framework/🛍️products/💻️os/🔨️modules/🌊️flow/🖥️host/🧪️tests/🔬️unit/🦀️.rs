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
use semio_framework_artifact_flow_flow::{flow_host_snapshot_operations, FlowEnvelope};
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
//#region 🔌️PortSides
/// 🔌️ The catalogue-wide port-side law: one operator's INPUT ids and OUTPUT ids are disjoint,
/// because `"{nodeId}@{portId}"` is the only public name a wire endpoint has. Driven over the LIVE
/// first-party catalogue — every operator every shipped extension registers — so a new operator
/// cannot reintroduce the ambiguity that made a press on `extrusion-axis`'s output `z` begin a wire
/// from its INPUT `z`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
///
/// @see `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧫️fixtures/🔌️port-sides/🔣️.json`
const PORT_SIDES_FIXTURE_JSON: &str = include_str!("../../../../🧠️neural/⚙️engine/🧫️fixtures/🔌️port-sides/🔣️.json");

fn port_sides_fixture() -> serde_json::Value {
    serde_json::from_str(PORT_SIDES_FIXTURE_JSON).expect("port sides fixture")
}

fn catalogue_channel_ids(operator: &serde_json::Value, side: &str) -> Vec<String> {
    operator[side].as_array().map(|channels| channels.iter().filter_map(|channel| channel["name"].as_str().map(str::to_string)).collect()).unwrap_or_default()
}

#[test]
fn no_operator_in_the_catalogue_declares_one_port_id_on_both_sides() {
    let fixture = port_sides_fixture();
    let wildcard = fixture["wildcardMarker"].as_str().expect("wildcardMarker");
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let operators = catalogue.as_array().expect("neuron kind infos array");
    assert!(operators.len() >= 100, "the first-party catalogue must be live, not a stub: {} operators", operators.len());
    let mut checked = 0usize;
    let mut offenders: Vec<String> = Vec::new();
    for operator in operators {
        let id = operator["id"].as_str().unwrap_or_default();
        let inputs = catalogue_channel_ids(operator, "inputs");
        let outputs = catalogue_channel_ids(operator, "outputs");
        let both: Vec<String> = inputs.iter().filter(|input| input.as_str() != wildcard && outputs.contains(input)).cloned().collect();
        if !both.is_empty() {
            offenders.push(format!("{id}: {both:?}"));
        }
        checked += 1;
    }
    assert!(offenders.is_empty(), "these operators name one port id on both sides, so \"{{nodeId}}@{{portId}}\" is ambiguous for them: {offenders:#?}");
    println!("[DEBUG] port-side law checked {checked} first-party operators");
}

#[test]
fn every_named_port_side_row_is_the_catalogue_the_extensions_register() {
    let fixture = port_sides_fixture();
    let suffix = fixture["suffix"].as_str().expect("suffix");
    let rows = fixture["rows"].as_array().expect("rows");
    assert!(rows.len() >= 12, "the port-side law needs the collided families AND the controls");
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let operators = catalogue.as_array().expect("neuron kind infos array");
    for row in rows {
        let id = row["operator"].as_str().expect("row operator");
        let live = operators.iter().find(|operator| operator["id"].as_str() == Some(id)).unwrap_or_else(|| panic!("{id} is not in the live catalogue"));
        let expected_inputs: Vec<String> = row["inputs"].as_array().expect("inputs").iter().filter_map(|name| name.as_str().map(str::to_string)).collect();
        let expected_outputs: Vec<String> = row["outputs"].as_array().expect("outputs").iter().filter_map(|name| name.as_str().map(str::to_string)).collect();
        assert_eq!(catalogue_channel_ids(live, "inputs"), expected_inputs, "{id} inputs");
        assert_eq!(catalogue_channel_ids(live, "outputs"), expected_outputs, "{id} outputs");
        for output in &expected_outputs {
            if let Some(plain) = output.strip_suffix(suffix) {
                assert!(expected_inputs.iter().any(|input| input == plain), "{id}: `{output}` carries the suffix, so `{plain}` must be one of its own inputs — the suffix is not decoration");
            }
        }
    }
}

#[test]
fn a_renamed_output_keeps_the_display_name_it_always_had() {
    let catalogue: serde_json::Value = serde_json::from_str(&fixture_kind_infos_json()).expect("neuron kind infos json");
    let operators = catalogue.as_array().expect("neuron kind infos array");
    let vector = operators.iter().find(|operator| operator["id"].as_str() == Some("math.vector")).expect("math.vector");
    let output = vector["outputs"].as_array().expect("outputs").iter().find(|channel| channel["name"].as_str() == Some("vectorOut")).expect("vectorOut");
    assert_eq!(output["fullName"].as_str(), Some("Vector"), "the rename is an identity, not a label");
    assert_eq!(output["abbreviation"].as_str(), Some("vec"));
    assert_eq!(output["code"].as_str(), Some("VE"));
}
//#endregion 🔌️PortSides

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
    let node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == widget_id).expect("node");
    let (wx, wy) = dag::slider_track_center(node).expect("slider track");
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, Point::new(wx, wy));
    (screen.x, screen.y)
}

#[test]
fn default_fixture_maps_widgets_to_native_dag_kinds() {
    let host = host_with_test_bridge();
    let slider = host.dag.host_snapshot.nodes.iter().find(|n| n.id == "slider").expect("slider");
    assert!(matches!(slider.kind, DagNodeKind::Slider { .. }));
    assert_eq!(slider.height, slider_widget_height());
    let add = host.dag.host_snapshot.nodes.iter().find(|n| n.id == "add").expect("add");
    assert!(matches!(add.kind, DagNodeKind::Computation { .. }));
    assert_eq!(slider.width, add.width, "all components should share one width");
    assert_eq!(slider.width, computation_node_width(&slider.name, &[], &[]));
    let preview = host.dag.host_snapshot.nodes.iter().find(|n| n.id == "preview").expect("preview");
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
    replay.replace_host_snapshot(host.host_snapshot.clone());
    session.install_baseline_into(&mut replay);
    let pending = replay.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()));
    assert!(!pending.contains(&"slider".to_string()));
    session.retire_cold();
    replay.retire_cold();
    host.retire_cold();
}

#[test]
fn flow_eval_session_seeds_its_retained_neural_cache() {
    let session = FlowEvalSession::new();
    let expected = Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(42.0)));
    let output_json = crate::os_pack::json::to_json_string(&expected);
    session.seed_node_cache(17, &output_json).unwrap();
    let seeded = session.neural_cache().get(17);
    assert_eq!(seeded, Some(expected.clone()));
    seeded.retire_cold();
    expected.retire_cold();
    session.retire_cold();
}

/// 🧵️ Builds a two-computable-node chain (`add` -> `pass`, replacing `add`'s direct link to
/// `preview`) on top of the default fixture, for tests that need more than one node to step
/// through with a budgeted `evaluate_step`.
fn host_with_two_node_chain() -> (FlowHost, String) {
    let mut host = host_with_test_bridge();
    let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 240.0, 0.0).unwrap();
    host.connect_ports("add", "sum", &pass_id, "number").unwrap();
    host.connect_ports(&pass_id, "number", "preview", "").unwrap();
    let stale_link = host.host_snapshot.synapses.iter().find(|s| s.from == "add" && s.to == "preview").map(|s| s.id.clone());
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
    while session.tick(&mut host, None) {}
    assert!(!session.pending());
    assert_eq!(host.preview_text(), "12");
    host.set_slider_value("slider", 20.0);
    assert!(session.sync(&host));
    assert!(session.pending());
    host.set_slider_value("slider", 30.0);
    assert!(!session.sync(&host), "a chain is already scheduled — sync must not arm a redundant second one");
    assert!(session.pending(), "the in-flight chain is still the one that will pick up 30");
    while session.tick(&mut host, None) {}
    assert_eq!(host.preview_text(), "30", "converges on the latest value, not the superseded intermediate one");
    session.retire_cold();
    host.retire_cold();
}

/// ⚖️ LAW: the CHAIN ledger stays live for the whole of an evaluation, including at the hop
/// boundaries where both finer ledgers are structurally empty, and its node census only ever grows.
///
/// 🩸️ This is the law the whole progress defect needed. A window parked on an extension answer is
/// marked `owed`, not `armed`, and `tick_scheduled` is false — so `FlowEvalSession::pending`, the
/// tessellation ledger and the budgeted-eval ledger ALL report nothing, and the status a preview
/// published at that moment said `phase: "idle", inFlight: 0, ratio: 1.0` while the kernel was busy.
/// Measured on 6118 as 54 byte-identical publications across one 23 s evaluation
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md`).
#[test]
fn the_chain_ledger_is_live_at_a_hop_boundary_and_its_census_only_grows() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    let mut session = FlowEvalSession::new();
    session.capture_baseline_from(&host);
    let settled = session.preview_chain_status();
    assert!(!settled.working && settled.in_flight == 0, "a quiesced session owes nothing");
    assert!((settled.ratio() - 1.0).abs() < 1e-9, "a quiesced session is complete, not at zero");

    host.set_slider_value("slider", 12.0);
    assert!(session.sync(&host), "a changed slider arms the chain");
    let armed = session.preview_chain_status();
    assert!(armed.working && armed.nodes_total > 0, "an armed chain publishes a census to measure against");
    assert!(armed.nodes_done < armed.nodes_total, "an armed chain has not settled every node");

    // ⏳️ THE HOP BOUNDARY: the window is waiting on an extension answer, so nothing is armed and no
    // finer ledger holds a row. Only the latch knows, and the chain ledger is what reads it.
    session.arm_window_tick("preview-1");
    session.begin_window_tick("preview-1");
    session.note_window_extensions_in_flight("preview-1", 1);
    session.note_window_tick_outcome("preview-1", true);
    let parked = session.preview_chain_status();
    assert!(!session.window_tick_is_armed("preview-1"), "a parked window is owed, never armed — the state that made every ledger read idle");
    assert_eq!(session.preview_tessellate_status().in_flight, 0, "the tessellation ledger is empty at a hop boundary");
    assert_eq!(session.preview_eval_status().in_flight, 0, "the budgeted-eval ledger is empty at a hop boundary");
    assert!(parked.working, "the chain ledger still reports the work the other two cannot see");
    assert_eq!(parked.in_flight, 1, "one extension answer is outstanding");

    let mut ratios = vec![parked.ratio()];
    while session.tick(&mut host, None) {
        ratios.push(session.preview_chain_status().ratio());
    }
    session.settle_window_extension("preview-1");
    session.note_window_tick_outcome("preview-1", false);
    ratios.push(session.preview_chain_status().ratio());
    for pair in ratios.windows(2) {
        assert!(pair[1] >= pair[0] - 1e-9, "the census ratio went backwards: {ratios:?}");
    }
    let done = session.preview_chain_status();
    assert!(!done.working && done.in_flight == 0, "a settled chain owes nothing again");
    assert!((done.ratio() - 1.0).abs() < 1e-9, "a settled chain reports complete");
    eprintln!("[DEBUG] flow chain ledger: parked={parked:?} ratios={ratios:?} settled={done:?}");
    session.retire_cold();
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
    replay.replace_host_snapshot(host.host_snapshot.clone());
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
    replay.replace_host_snapshot(host.host_snapshot.clone());
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
    let fan_out: Vec<_> = host.host_snapshot.synapses.iter().filter(|s| s.from == "add" && s.from_port == "sum").collect();
    assert_eq!(fan_out.len(), 2);
    assert!(fan_out.iter().any(|s| s.to == "preview"));
    assert!(fan_out.iter().any(|s| s.to == pass_id));
    host.retire_cold();
}

#[test]
fn connect_ports_replaces_existing_incoming_on_same_input() {
    let mut host = host_with_test_bridge();
    assert!(host.host_snapshot.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
    let note_id = host.add_widget(r#"{"kind":"inputNote","id":"note","text":"2"}"#, -120.0, 0.0).unwrap();
    host.connect_ports(&note_id, "text", "add", "a").unwrap();
    let incoming_a: Vec<_> = host.host_snapshot.synapses.iter().filter(|s| s.to == "add" && s.to_port == "a").collect();
    assert_eq!(incoming_a.len(), 1);
    assert_eq!(incoming_a[0].from, note_id);
    assert!(!host.host_snapshot.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
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
    channels.retire_cold();
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
    outputs.retire_cold();
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
    channels.retire_cold();
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
    dict.retire_cold();
}

#[test]
fn preview_scalar_content_from_number_dict() {
    let dict = Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(3.0)));
    assert!(matches!(
        dag_preview_content_from_dict(&dict),
        DagPreviewContent::Scalar { text } if text == "3"
    ));
    dict.retire_cold();
}

#[test]
fn image_input_seed_and_preview_content() {
    let mut host = host_with_test_bridge();
    let png = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
    host.host_snapshot.widgets.push(Widget::InputImage { id: "image".into(), src: png.into() });
    host.rebuild_dag();
    let node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == "image").expect("image node");
    assert!(matches!(node.kind, DagNodeKind::Image { .. }));
    let seeds = host.build_seeds();
    assert_eq!(seeds.get("image").and_then(|d| d.get("image")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("dataUrl")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some(png));
    seeds.retire_cold();
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
    let slider_node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == "slider").expect("slider").clone();
    let DagNodeKind::Slider { .. } = slider_node.kind else {
        panic!("expected slider kind");
    };
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    let value = host
        .host_snapshot
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id.as_str() == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(value > 3.0);
    host.retire_cold();
}

/// 📐️ A world point projected through the host's own camera — the projection every screen pointer
/// method is addressed in.
fn world_screen_point(host: &FlowHost, wx: f64, wy: f64) -> (f64, f64) {
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, Point::new(wx, wy));
    (screen.x, screen.y)
}

/// 🫥️ A world point no node's rect covers — where a press grabs nothing at all.
fn empty_canvas_world_point(host: &FlowHost) -> (f64, f64) {
    let point = (0.0, 260.0);
    for node in &host.dag.host_snapshot.nodes {
        let covered = (point.0 - node.x).abs() <= node.width * 0.5 + 8.0 && (point.1 - node.y).abs() <= node.height * 0.5 + 8.0;
        assert!(!covered, "{} covers the point this law needs empty", node.id);
    }
    point
}

fn gesture_answer(host: &mut FlowHost) -> (usize, bool) {
    let answer: serde_json::Value = serde_json::from_str(&host.take_graph_edits_json()).expect("gesture answer json");
    (answer["operations"].as_array().expect("operations array").len(), answer["hostSnapshotChanged"].as_bool().expect("hostSnapshotChanged flag"))
}

/// 🪶 LAW: a pointer gesture that changed nothing answers with nothing — no narrow operation AND no
/// fixture commit — so a renderer reading that answer dispatches nothing at all.
///
/// 🩸️ The renderer read an empty `operations` as "fall back to the whole-fixture commit", so EVERY
/// plain click on the graph canvas dispatched a `setFixture` `nodeGraphEdit`: a retained command per
/// click, and — since a landed document mutation owes every attached preview a fresh evaluation — a
/// `flowEvalTick` pair on a shell nobody touched. Measured as the `keyboard-verbs` `baseline` row's
/// `invoked: ["nodeGraphEdit","flowEvalTick","flowEvalTick"]` on an idle, fully settled preview
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`).
#[test]
fn a_gesture_that_changed_nothing_answers_no_operations_and_no_fixture_commit() {
    let mut host = host_with_test_bridge();
    host.set_viewport(800, 600, 1.0);
    let (wx, wy) = empty_canvas_world_point(&host);
    let (sx, sy) = world_screen_point(&host, wx, wy);
    let positions_before: Vec<(String, f64, f64)> = host.dag.host_snapshot.nodes.iter().map(|node| (node.id.clone(), node.x, node.y)).collect();
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_up_screen(sx, sy, false, false, false);
    let (operations, fixture_changed) = gesture_answer(&mut host);
    println!("[DEBUG] quiet click answer operations={operations} hostSnapshotChanged={fixture_changed}");
    assert_eq!(operations, 0, "a click that wired nothing journals no narrow operation");
    assert!(!fixture_changed, "a click that moved no widget, no synapse and no layout owes no fixture commit");
    let positions_after: Vec<(String, f64, f64)> = host.dag.host_snapshot.nodes.iter().map(|node| (node.id.clone(), node.x, node.y)).collect();
    assert_eq!(positions_after, positions_before, "the click must not have moved the graph either");
    host.retire_cold();
}

/// 🫳️ LAW's partner: a gesture that DID change content still asks for the fixture commit, because the
/// narrow vocabulary the screen path journals carries wires only — a node drag is content the guest
/// learns about no other way.
#[test]
fn a_drag_that_moved_a_node_answers_a_fixture_commit() {
    let mut host = host_with_test_bridge();
    host.set_viewport(800, 600, 1.0);
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add node").clone();
    let (sx, sy) = world_screen_point(&host, node.x, node.y - node.height * 0.25);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 60.0, sy + 40.0, false, false, false);
    host.pointer_up_screen(sx + 60.0, sy + 40.0, false, false, false);
    let (operations, fixture_changed) = gesture_answer(&mut host);
    println!("[DEBUG] node drag answer operations={operations} hostSnapshotChanged={fixture_changed}");
    assert_eq!(operations, 0, "the screen path journals wires only, so a move is not a narrow operation");
    assert!(fixture_changed, "a drag that moved a node owes the fixture commit that carries it");
    host.retire_cold();
}

#[test]
fn default_fixture_does_not_auto_layout() {
    let host = host_with_test_bridge();
    let slider = host.host_snapshot.layout.get("slider").expect("slider");
    let add = host.host_snapshot.layout.get("add").expect("add");
    let preview = host.host_snapshot.layout.get("preview").expect("preview");
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
        .host_snapshot
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id.as_str() == "slider" => Some(*value),
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
        .host_snapshot
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id.as_str() == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(slider > 3.0);
    host.retire_cold();
}

#[test]
fn reorganize_overwrites_saved_layout_left_to_right() {
    let mut host = host_with_test_bridge();
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.host_snapshot.layout.insert("preview".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.rebuild_dag();
    host.reorganize("").unwrap();
    let slider = host.host_snapshot.layout.get("slider").expect("slider layout");
    let add = host.host_snapshot.layout.get("add").expect("add layout");
    let preview = host.host_snapshot.layout.get("preview").expect("preview layout");
    assert!(add.x > slider.x);
    assert!(preview.x > add.x);
    host.retire_cold();
}

#[test]
fn fixture_json_round_trip() {
    let host = FlowHost::default();
    let json = host.host_snapshot_json().unwrap();
    let parsed = FlowHost::parse_host_snapshot_json(&json).unwrap();
    assert_eq!(parsed.schema, "flow.host_snapshot");
    parsed.retire_cold();
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
    channels.retire_cold();
    seeds.retire_cold();
    registry.retire_cold();
    document.retire_cold();
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

//#region 📷️CameraAuthority
/// 📷️ The SURFACE half of the node-graph camera-fit law. The geometry half
/// (`♾️infinite/🖼️canvas/🧪️tests/📷️camera-fit/🦀️.rs` and the renderer's TypeScript twin) pins WHICH
/// camera a fit computes; these rows pin that the surface actually publishes it. A flow surface
/// carries two camera copies — `FlowHost::fixture.camera`, the authority every projection and every
/// `nodeGraphViewport` publication reads, and `DagHost::fixture.camera`, the derived copy the paint
/// reads — and the wgpu `Fit graph` control published the first while the fit had only landed on the
/// second. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
///
/// @see `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json` — `surfaceRows`
const CAMERA_FIT_FIXTURE_JSON: &str = include_str!("../../../../♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json");

/// 🕸️ The hexagonal mushroom column's own graph, laid out the way the shipped example lays it out —
/// the graph whose nodes sit at negative surface x, which is what made the stale publication visible.
fn camera_law_column_host() -> FlowHost {
    let mut host = host_with_test_bridge();
    let mut layout = crate::OrderedMap::new();
    for (id, x, y) in [("height", -197.19, -102.70), ("radius", -156.03, -177.33), ("sides", -156.43, -155.28), ("profile", -64.49, -163.40), ("extrusion-axis", -65.26, -116.45), ("extrude", 34.84, -154.18)] {
        layout.insert(id.to_string(), WidgetLayout { x, y });
    }
    host.replace_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "height".into(), label: "Column Height".into(), value: 6.0, min: 0.0, max: 10.0, step: 0.5 },
            Widget::Neuron { id: "extrusion-axis".into(), neuron_kind: "math.vector".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::Neuron { id: "extrude".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true },
        ],
        synapses: vec![],
        layout,
    });
    host
}

fn camera_fit_surface_rows() -> Vec<serde_json::Value> {
    let document: serde_json::Value = serde_json::from_str(CAMERA_FIT_FIXTURE_JSON).expect("camera fit fixture");
    document["surfaceRows"].as_array().expect("surfaceRows").clone()
}

#[test]
fn a_fitted_flow_surface_publishes_the_camera_it_computed() {
    let rows = camera_fit_surface_rows();
    assert!(rows.len() >= 4, "the surface half of the camera-fit law needs more than a happy path");
    for row in &rows {
        let name = row["name"].as_str().expect("row name");
        let width = row["viewport"]["width"].as_u64().expect("width") as u32;
        let height = row["viewport"]["height"].as_u64().expect("height") as u32;
        let mut host = camera_law_column_host();
        host.set_viewport(width, height, 1.0);
        host.set_camera(row["camera"]["x"].as_f64().expect("x"), row["camera"]["y"].as_f64().expect("y"), row["camera"]["zoom"].as_f64().expect("zoom"));
        let before = host.camera();

        let fitted = host.fit_camera_to_content();
        assert_eq!(fitted, row["expect"]["fits"].as_bool().expect("fits"), "{name}: the fit must report whether it framed anything");
        let published = host.camera();
        assert_ne!(published, before, "{name}: a fit that reports success must move the published camera off the pre-fit one");

        let painted = [host.dag.host_snapshot.camera.x, host.dag.host_snapshot.camera.y, host.dag.host_snapshot.camera.zoom];
        if row["expect"]["publishedEqualsPainted"].as_bool().unwrap_or(false) {
            assert_eq!(published, painted, "{name}: the published camera and the painted camera are one camera");
        }
        if row["expect"]["publishedEqualsFitRule"].as_bool().unwrap_or(false) {
            let content = host.dag.content_world_bounds().expect("the column graph has content");
            let rule = canvas::camera::fit_camera(&content, &Viewport { width, height, dpr: 1.0 }, canvas::camera::CONTENT_FIT_PADDING_PX);
            assert_eq!(published, [rule.x, rule.y, rule.zoom], "{name}: the published camera is the shared fit rule's answer, not a neighbouring copy");
        }
        if let Some(expected) = row["expect"]["coverageAfterFit"].as_f64() {
            assert!((host.dag.camera_content_coverage() - expected).abs() <= 1e-9, "{name}: a fitted camera frames the whole graph");
        }
        if row["expect"]["idempotent"].as_bool().unwrap_or(false) {
            host.fit_camera_to_content();
            assert_eq!(host.camera(), published, "{name}: fitting twice must not drift the camera");
        }
        host.retire_cold();
    }
}

#[test]
fn a_flow_surface_projects_screen_points_with_the_camera_it_published() {
    let mut host = camera_law_column_host();
    host.set_viewport(483, 814, 1.0);
    host.set_camera(94.75581571737445, -97.50833134679668, 1.7844325616011099);
    assert!(host.fit_camera_to_content());
    let published = host.camera();
    let content = host.dag.content_world_bounds().expect("the column graph has content");
    let rule = canvas::camera::fit_camera(&content, &Viewport { width: 483, height: 814, dpr: 1.0 }, canvas::camera::CONTENT_FIT_PADDING_PX);
    let centre = host.world_from_screen(483.0 / 2.0, 814.0 / 2.0);
    assert!((centre.0 - rule.x).abs() <= 1e-6 && (centre.1 - rule.y).abs() <= 1e-6, "the viewport centre must project through the FITTED camera, not through the one the document shipped with");
    assert_eq!(published, [rule.x, rule.y, rule.zoom]);
    host.retire_cold();
}
//#endregion 📷️CameraAuthority

#[test]
fn replace_fixture_preserves_kind_infos_and_named_input_ports() {
    let mut host = host_with_test_bridge();
    host.replace_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![Widget::Neuron { id: "add".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true }],
        synapses: vec![],
        layout: crate::OrderedMap::new(),
    });
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add node");
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
    let extras = flow_backed_node_graph_extras(&host.host_snapshot, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
    assert!(extras.host_snapshot_json.as_ref().is_some_and(|json| json.contains("flow.host_snapshot")));
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

    let fixture = FlowHostSnapshot::default();
    let extras = flow_backed_node_graph_extras(&fixture, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
    let scene = ui_wgpu::wgpu::NodeGraphScene {
        editable: Some(true),
        capabilities_json: extras.capabilities_json,
        lod_json: extras.lod_json,
        host_snapshot_json: extras.host_snapshot_json,
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
    host.replace_host_snapshot(<FlowHostSnapshot as crate::os_store::ArtifactDsl>::parse_dsl(include_str!("../../../📚️examples/🗣️.dsl.semio")).expect("fixture"));
    assert!(!host.dag.host_snapshot.edges.is_empty(), "synapses should become dag edges");
    let add = host.dag.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add node");
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
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == id).expect("export node");
    assert!(matches!(node.kind, DagNodeKind::Export { .. }));
    host.retire_cold();
}

/// ↩️ Exercises the standard `crate::os_store::ArtifactStore<FlowHostSnapshot, FlowMutation>` undo/redo
/// mechanism directly (the same one `FlowHost::undo`/`redo` are built on) — add a widget, undo,
/// confirm it's gone, redo, confirm it's back — in place of the old test's direct assertions on a
/// hand-rolled `Vec<FlowHostSnapshot>` snapshot stack.
#[semio_framework_async_macros::async_test]
async fn undo_redo_add_widget() {
    let mut host = host_with_test_bridge();
    let fixture_before = host.host_snapshot.clone();
    let count_before = fixture_before.widgets.len();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"undo me"}"#, 42.0, 42.0).unwrap();
    assert_eq!(host.host_snapshot.widgets.len(), count_before + 1);

    let operations = flow_host_snapshot_operations(&fixture_before, &host.host_snapshot).expect("wire-representable flow fixture");
    assert!(!operations.is_empty(), "add_widget must diff into vcs operations");

    let envelope: FlowEnvelope = create_document_envelope(FLOW_DOCUMENT_SCHEMA, "test", fixture_before, None);
    let mut store = FlowStore::new(envelope).await.expect("valid flow store fixture");
    // 🔐️ `ArtifactStore::new` installs NO owner catalog, and `reserve_edit_history_slot` refuses every
    // `Apply` without one — `edit history insertion requires its exact mutation retirement factory`.
    // The refusal then drops the replayed projection on the error path, so the FIRST thing this law
    // saw was `ordered-map root must be explicitly retired before drop` from inside `apply_command`,
    // never the validation that caused it. The flow host installs the same catalog on its own history
    // store (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    store.install_document_store_owners_exact(FlowHostSnapshot::member_store_owners());
    store.dispatch(ArtifactCommand::Apply { mutations: operations, description: None }).await.expect("apply add-widget operations");
    let applied = store.snapshot().expect("projection");
    assert_eq!(applied.widgets.len(), count_before + 1);
    applied.retire_cold();

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    let after_undo = store.snapshot().expect("projection");
    assert_eq!(after_undo.widgets.len(), count_before);
    assert!(!after_undo.widgets.iter().any(|w| widget_id_for(w) == id));
    after_undo.retire_cold();

    store.dispatch(ArtifactCommand::Redo).await.expect("redo");
    let after_redo = store.snapshot().expect("projection");
    assert!(after_redo.widgets.iter().any(|w| widget_id_for(w) == id));
    after_redo.retire_cold();
    semio_framework_artifact_flow_flow::retire_flow_store_cold(store);
    host.retire_cold();
}

#[test]
fn camera_change_does_not_create_undo_step() {
    let mut host = host_with_test_bridge();
    let camera_before = host.host_snapshot.camera.clone();
    host.set_camera(camera_before.x + 50.0, camera_before.y - 30.0, camera_before.zoom * 1.5);
    assert!(!host.can_undo());
    let id = host.add_widget(r#"{"kind":"inputNote","text":"x"}"#, 0.0, 0.0).unwrap();
    assert!(host.can_undo());
    assert!(host.undo());
    assert_eq!(host.host_snapshot.camera.x, camera_before.x + 50.0);
    assert_eq!(host.host_snapshot.camera.y, camera_before.y - 30.0);
    assert!((host.host_snapshot.camera.zoom - camera_before.zoom * 1.5).abs() < 1e-9);
    assert!(!host.host_snapshot.widgets.iter().any(|w| widget_id_for(w) == id));
    host.retire_cold();
}

#[test]
fn replace_fixture_preserves_live_camera() {
    let mut host = host_with_test_bridge();
    host.set_camera(120.0, -45.0, 1.75);
    host.replace_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![Widget::InputNote { id: "note".into(), text: "hello".into() }],
        synapses: vec![],
        layout: crate::OrderedMap::new(),
    });
    assert_eq!(host.host_snapshot.camera.x, 120.0);
    assert_eq!(host.host_snapshot.camera.y, -45.0);
    assert!((host.host_snapshot.camera.zoom - 1.75).abs() < 1e-9);
    assert!(host.host_snapshot.widgets.iter().any(|w| widget_id_for(w) == "note"));
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
        let next = merged.merge(slot);
        std::mem::replace(&mut merged, next).retire_cold();
    }
    Ok(channel_output("dictionary", merged))
}

#[test]
fn variadic_merge_evaluates_port_routed_inputs() {
    let mut host = FlowHost::from_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
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
    std::mem::take(&mut host.outputs).retire_cold();
    host.evaluate_internal();
    let preview = host
        .host_snapshot
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
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == id).expect("node");
    assert_eq!(node.name, "Add");
    assert_eq!(node.abbreviation, "Add");
    assert_eq!(node.icon, "emoji:➕️");
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_explicit_range() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":10.2,"min":10.2,"max":15.0,"step":0.1}"#, 0.0, 0.0).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 10.2).abs() < 1e-6);
    assert!((min - 10.2).abs() < 1e-6);
    assert!((max - 15.0).abs() < 1e-6);
    assert!((step - 0.1).abs() < 1e-6);
    let node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
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
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputNote { text, .. } = widget else {
        panic!("expected note widget");
    };
    assert_eq!(text, "some text");
    let node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
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
    let node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
    let origin_x = node.x - node.width * 0.5 + 4.0;
    host.begin_note_edit(&id, origin_x + 40.0, node.y);
    host.note_insert_text("!");
    host.note_commit_edit();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputNote { text, .. } = widget else {
        panic!("expected note widget");
    };
    assert_eq!(text, "hi!");
    assert!(host.undo());
    let Widget::InputNote { text: restored, .. } = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget") else {
        panic!("expected note widget");
    };
    assert_eq!(restored, "hi");
    host.retire_cold();
}

#[test]
fn wheel_screen_zoom_gesture_changes_zoom() {
    let mut host = host_with_test_bridge();
    let z0 = host.host_snapshot.camera.zoom;
    host.wheel_screen(400.0, 300.0, 0.0, -10.0, true);
    assert_ne!(host.host_snapshot.camera.zoom, z0);
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
    assert_eq!(direct.host_snapshot.camera, planned.host_snapshot.camera);

    let stale = planned.plan_wheel(320.0, 240.0, 0.0, -10.0, true);
    planned.pointer_down_screen(10.0, 10.0, 0, false, false, false, true);
    let replacement = planned.host_snapshot.camera.clone();
    assert!(!planned.commit_wheel(stale));
    assert_eq!(planned.host_snapshot.camera, replacement);
    planned.retire_cold();
    direct.retire_cold();
}

#[test]
fn set_note_text_keeps_uniform_component_width() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
    let short_w = host.dag.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node").width;
    host.set_note_text(&id, "a much longer note string");
    let node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
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
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
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
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 1.3).abs() < 1e-6);
    assert!((min - 0.0).abs() < 1e-6);
    assert!((max - 10.0).abs() < 1e-6);
    assert!((step - 0.1).abs() < 1e-6);
    let node = host.dag.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
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
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
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
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
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
    let placed_width = host.dag.host_snapshot.nodes.iter().find(|node| node.id == placed_id).expect("placed").width;
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
    let merge = host.dag.host_snapshot.nodes.iter().find(|n| n.id == merge_id).expect("merge").clone();
    let grab = Point::new(merge.x, merge.y);
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, grab);
    host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
    host.pointer_move_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
    host.pointer_up_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
    assert_eq!(host.host_snapshot.widgets.iter().filter(|w| widget_id_for(w) == merge_id).count(), 1);
    assert_eq!(host.dag.host_snapshot.nodes.iter().filter(|n| n.id == merge_id).count(), 1);
    let moved = host.host_snapshot.layout.get(&merge_id).expect("merge layout");
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
    assert_eq!(host.host_snapshot.widgets.iter().filter(|w| widget_id_for(w).starts_with("slider")).count(), 2);
    assert_eq!(host.dag.host_snapshot.nodes.iter().filter(|n| n.id == "slider").count(), 1);
    host.retire_cold();
}

#[test]
fn delete_selection_removes_widget_from_host_snapshot() {
    let mut host = host_with_test_bridge();
    host.dag.set_selection(&["slider".into()]);
    host.delete_selection().unwrap();
    assert!(host.host_snapshot.widgets.iter().all(|w| widget_id_for(w) != "slider"));
    assert!(host.dag.host_snapshot.nodes.iter().all(|n| n.id != "slider"));
    host.retire_cold();
}

#[test]
fn node_drag_proximity_skips_wired_cut_inputs_in_flow() {
    use canvas::camera::{world_to_screen, Camera, Viewport};
    use canvas::Point;
    let mut host = FlowHost::default();
    host.set_viewport(1280, 800, 1.0);
    host.host_snapshot.replace_widgets(vec![
        Widget::Neuron { id: "sphere".into(), neuron_kind: "brep.prim3d.sphere".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
        Widget::Neuron { id: "torus".into(), neuron_kind: "brep.prim3d.torus".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
        Widget::Neuron { id: "cut".into(), neuron_kind: "brep.bool.cut".into(), params: Dictionary::new(), input_ports: vec!["a".into(), "b".into()], output_ports: vec![], preview: true },
    ]);
    host.host_snapshot.synapses = vec![
        SynapseSpec { id: "e1".into(), from: "sphere".into(), to: "cut".into(), from_port: "solid".into(), to_port: "a".into() },
        SynapseSpec { id: "e2".into(), from: "torus".into(), to: "cut".into(), from_port: "solid".into(), to_port: "b".into() },
    ];
    host.host_snapshot.layout.insert("sphere".into(), WidgetLayout { x: 0.0, y: -60.0 });
    host.host_snapshot.layout.insert("torus".into(), WidgetLayout { x: 0.0, y: 60.0 });
    host.host_snapshot.layout.insert("cut".into(), WidgetLayout { x: 240.0, y: 0.0 });
    let solid_out = vec![InputSpec::named("S", "Sld", "solid", "Solid")];
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![
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
    let cut = host.dag.host_snapshot.nodes.iter().find(|node| node.id == "cut").expect("cut");
    let grab = Point::new(cut.x, cut.y);
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, grab);
    host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
    host.pointer_move_screen(screen.x - 180.0, screen.y, false, false, false);
    assert!(host.dag.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges");
    host.pointer_up_screen(screen.x - 180.0, screen.y, false, false, false);
    assert_eq!(host.dag.engine.edges.len(), 2);
    assert_eq!(host.host_snapshot.synapses.len(), 2);
    host.retire_cold();
}

#[test]
fn dag_bridge_keeps_same_named_brep_input_and_output_distinct() {
    let mut host = FlowHost::default();
    host.host_snapshot.replace_widgets(vec![
        Widget::Neuron { id: "extrude".into(), neuron_kind: "brep.solid.extrude".into(), params: Dictionary::new(), input_ports: vec!["wire".into(), "vector".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "brep".into(), neuron_kind: "brep.brep".into(), params: Dictionary::new(), input_ports: vec!["brep".into(), "vertex".into(), "edge".into(), "face".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "get".into(), neuron_kind: "list.get".into(), params: Dictionary::new(), input_ports: vec!["list".into(), "index".into(), "wrap".into()], output_ports: vec!["0".into()], preview: true },
    ]);
    host.host_snapshot.synapses = vec![
        SynapseSpec { id: "e112".into(), from: "extrude".into(), to: "brep".into(), from_port: "solid".into(), to_port: "brep".into() },
        SynapseSpec { id: "e113".into(), from: "brep".into(), to: "get".into(), from_port: "brep".into(), to_port: "list".into() },
    ];
    host.host_snapshot.layout.insert("extrude".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.host_snapshot.layout.insert("brep".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.host_snapshot.layout.insert("get".into(), WidgetLayout { x: 400.0, y: 0.0 });
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![
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
fn delete_selection_removes_selected_edge_from_host_snapshot() {
    let mut host = host_with_test_bridge();
    let synapse_count_before = host.host_snapshot.synapses.len();
    assert!(synapse_count_before > 0);
    let edge_id = *host.dag.engine.edges.keys().next().expect("edge");
    host.dag.engine.selection.edge_ids.insert(edge_id);
    assert!(host.has_selection());
    host.delete_selection().unwrap();
    assert!(host.host_snapshot.synapses.len() < synapse_count_before);
    assert!(!host.has_selection());
    host.retire_cold();
}

#[test]
fn delete_selection_removes_edge_selected_by_synapse_id_domain() {
    let mut host = host_with_test_bridge();
    let before = host.host_snapshot.synapses.len();
    host.dag.set_selection_domains_json(r#"{"nodes":[],"edges":["s1"],"🐙️handles":[]}"#);
    assert!(host.has_selection(), "synapse id s1 must map into engine edge selection");
    host.delete_selection().unwrap();
    assert!(host.host_snapshot.synapses.len() < before);
    assert!(!host.host_snapshot.synapses.iter().any(|synapse| synapse.id == "s1"));
    host.retire_cold();
}

#[test]
fn align_selection_left_aligns_selected_widget_layout() {
    let mut host = host_with_test_bridge();
    host.move_widget("slider", -120.0, 20.0).unwrap();
    host.move_widget("add", 180.0, -40.0).unwrap();
    host.dag.set_selection(&["slider".into(), "add".into()]);
    host.align_selection("alignLeft").unwrap();
    let slider = host.dag.host_snapshot.nodes.iter().find(|node| node.id == "slider").expect("slider");
    let add = host.dag.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add");
    let slider_left = slider.x - slider.width * 0.5;
    let add_left = add.x - add.width * 0.5;
    assert!((slider_left - add_left).abs() < 1e-6, "left edges should match after alignLeft");
    assert!(host.host_snapshot.layout.contains_key("slider"));
    assert!(host.host_snapshot.layout.contains_key("add"));
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
    let widget = host.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == merge_id).expect("merge");
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
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == get_id).expect("get");
    let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
    assert_eq!(labels, vec!["i"]);
    host.add_output_port(&get_id, 1).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == get_id).expect("get");
    let Widget::Neuron { output_ports, .. } = widget else { panic!("neuron") };
    assert_eq!(output_ports.len(), 2);
    let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == get_id).expect("get");
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
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "mid"));
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "mid" && synapse.to == "add"));
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "add" && synapse.to == "preview"));
    assert!(!host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "add"));
    host.retire_cold();
}

#[test]
fn insert_between_preserves_existing_mid_inputs() {
    let mut host = host_with_test_bridge();
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 120.0, 0.0).unwrap();
    host.connect_ports("slider", "number", &variable_id, "width").unwrap();
    host.insert_between("slider", "number", &variable_id, "width", "width").unwrap();
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == variable_id && synapse.to_port == "width"));
    assert!(!host.host_snapshot.synapses.iter().any(|synapse| synapse.from == variable_id && synapse.to == variable_id));
    host.retire_cold();
}

#[test]
fn make_space_shifts_widgets_right_of_anchor() {
    let mut host = host_with_test_bridge();
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.host_snapshot.layout.insert("preview".into(), WidgetLayout { x: 400.0, y: 0.0 });
    host.rebuild_dag();
    host.make_space("slider", 100.0, 0.0).unwrap();
    assert!((host.host_snapshot.layout.get("slider").expect("slider").x - 0.0).abs() < 1e-6);
    assert!((host.host_snapshot.layout.get("add").expect("add").x - 300.0).abs() < 1e-6);
    assert!((host.host_snapshot.layout.get("preview").expect("preview").x - 500.0).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn set_neuron_params_merges_into_eval_input() {
    let mut host = host_with_test_bridge();
    let preview_synapse = host.host_snapshot.synapses.iter().find(|synapse| synapse.from == "add" && synapse.to == "preview").map(|synapse| synapse.id.clone()).expect("preview synapse");
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
    widget.retire_cold();
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
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 100.0, 0.0).unwrap();
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.host_snapshot.synapses.retain(|synapse| synapse.from != "slider" || synapse.to != "add");
    host.connect_ports("slider", "number", &variable_id, "width").unwrap();
    host.connect_ports(&variable_id, "width", "add", "a").unwrap();
    host.rebuild_dag();
    let cluster_id = host.collapse_selection(&[variable_id.clone(), "add".into()]).unwrap();
    let cluster = host
        .host_snapshot
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
    assert!(host.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Variable { name, .. } if name == "width")));
    cluster.retire_cold();
    host.retire_cold();
}

#[test]
fn collapse_then_explode_round_trips() {
    let mut host = host_with_test_bridge();
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.rebuild_dag();
    let cluster_id = host.collapse_selection(&["slider".into(), "add".into()]).unwrap();
    assert!(host.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { id, .. } if id == &cluster_id)));
    host.explode_cluster(&cluster_id).unwrap();
    assert!(host.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget).starts_with(&format!("{cluster_id}/"))));
    assert!(!host.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { .. })));
    host.retire_cold();
}

#[test]
fn rectangle_extrude_fixture_port_labels_follow_draw_lod() {
    let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_host_snapshot_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.host_snapshot",
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
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vectorOut", "toPort": "vector" },
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
    let fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    let mut host = FlowHost::from_host_snapshot(fixture);
    host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
    host.set_viewport(1280, 800, 1.0);
    host.host_snapshot.camera.zoom = 1.0;
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
    drop(port_texts);
    host.retire_cold();
}

#[test]
fn rectangle_extrude_fixture_evaluates_solid_output() {
    let _guard = RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|error| error.into_inner());
    // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
    // handcrafted DSL (`crate::os_store::ArtifactDsl`) — inlined the same flow-fixture JSON this test actually
    // parses (`FlowHost::parse_host_snapshot_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.host_snapshot",
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
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vectorOut", "toPort": "vector" },
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
    let fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    let mut host = FlowHost::from_host_snapshot(fixture);
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
    // parses (`FlowHost::parse_host_snapshot_json`), decoupled from procedural's document format.
    let json = r#"{
  "schema": "flow.host_snapshot",
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
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vectorOut", "toPort": "vector" },
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
    let fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    let mut host = FlowHost::from_host_snapshot(fixture);
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
fn flow_host_snapshot_to_form_spec_maps_input_widgets() {
    use self::forms_bridge::flow_host_snapshot_to_form_spec;
    let fixture = FlowHostSnapshot::default();
    let spec = flow_host_snapshot_to_form_spec(&fixture);
    let kinds: Vec<&str> = spec.steps[0].blocks.iter().map(|question| question.kind.as_str()).collect();
    assert!(kinds.contains(&"slider"));
}

#[test]
fn apply_generation_values_to_host_snapshot_patches_slider_value() {
    use self::forms_bridge::{apply_generation_values_to_host_snapshot, flow_host_snapshot_to_form_spec};
    let fixture = FlowHostSnapshot::default();
    let spec = flow_host_snapshot_to_form_spec(&fixture);
    let slider_id = spec.steps[0].blocks.iter().find(|question| question.kind == "slider").map(|question| question.id.clone()).expect("slider question");
    let fixture_json = crate::os_pack::json::to_json_string(&fixture);
    let mut values = crate::os_pack::json::Object::new();
    values.insert(slider_id.clone(), crate::os_pack::json::Value::Number(8.0.into()));
    let patched = apply_generation_values_to_host_snapshot(&fixture_json, &values);
    let reparsed = crate::os_pack::json::parse(&patched).expect("patched json");
    let slider = reparsed.get("widgets").and_then(|widgets| widgets.as_array()).and_then(|widgets| widgets.iter().find(|widget| widget.get("id").and_then(|id| id.as_str()) == Some(slider_id.as_str()))).expect("slider widget");
    assert_eq!(slider.get("value").and_then(|value| value.as_f64()), Some(8.0));
}

/// 🔌️ A bridge whose `plugin.*` kinds live in a PLUGIN, not in this process — the exact shape a
/// contributed brep operator has from the guest's side.
fn test_extension_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
    if let Some(operator_id) = kind.strip_prefix("plugin.") {
        return Err(EvalError::PendingExtension { extension_id: "geometry".into(), operator_id: operator_id.into(), node_hash: neural::node_hash(kind, input) });
    }
    test_math_bridge(kind, input)
}

/// 🔌️ The two-operator catalogue plus the two CONTRIBUTED kinds a wave law needs. Declared here
/// rather than folded into `test_kind_infos_json` so every other host law keeps indexing exactly the
/// catalogue it was written against.
fn test_extension_kind_infos_json() -> String {
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
            id: "plugin.left".into(),
            extension: "geometry".into(),
            name: "Left".into(),
            abbreviation: "Left".into(),
            icon: "emoji:⬅️".into(),
            summary: "Contributed left branch".into(),
            inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("O", "Out", "out", "Number")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "plugin.right".into(),
            extension: "geometry".into(),
            name: "Right".into(),
            abbreviation: "Right".into(),
            icon: "emoji:➡️".into(),
            summary: "Contributed right branch".into(),
            inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("O", "Out", "out", "Number")],
            ..Default::default()
        },
    ])
}

/// 🌊️ The default fixture plus two INDEPENDENT contributed nodes hanging off `add` — the flow-host
/// shape of one topological wave.
fn host_with_two_extension_siblings() -> FlowHost {
    let mut host = FlowHost::default();
    host.set_eval_bridge_fn(Box::new(test_extension_bridge));
    host.set_neuron_kind_infos_json(&test_extension_kind_infos_json());
    let left = host.add_widget(r#"{"kind":"neuron","id":"left","neuronKind":"plugin.left","params":{},"input_ports":[],"preview":false}"#, 240.0, 0.0).unwrap();
    let right = host.add_widget(r#"{"kind":"neuron","id":"right","neuronKind":"plugin.right","params":{},"input_ports":[],"preview":false}"#, 240.0, 80.0).unwrap();
    host.connect_ports("add", "sum", &left, "number").unwrap();
    host.connect_ports("add", "sum", &right, "number").unwrap();
    host
}

/// 📊️ The census entries of one status object, as `(widget id, status tag)` pairs.
fn census_entries(status_json: &str) -> Vec<(String, String)> {
    let value = crate::os_pack::json::parse(status_json).expect("census json");
    let object = value.as_object().expect("census object").clone();
    let mut entries: Vec<(String, String)> = object.iter().map(|(id, entry)| (id.to_string(), entry.get("status").and_then(crate::os_pack::json::Value::as_str).unwrap_or_default().to_string())).collect();
    entries.sort();
    entries
}

/// 📈 How many nodes of a census have SETTLED — the numerator [`FlowEvalSession::preview_chain_status`]
/// derives, stated here so a law can watch it move.
fn census_nodes_done(status_json: &str) -> usize {
    census_entries(status_json).into_iter().filter(|(_, status)| !matches!(status.as_str(), "queued" | "computing" | "stale")).count()
}

/// ⚖️ LAW: the node census ADVANCES as a chain walks — a node this evaluation has already
/// recomputed is `ok`, never `stale`.
///
/// 🩸️ `dirty` is measured against the chain's FROZEN baseline, which only advances when the chain
/// COMPLETES, so every node an evaluation touches stays dirty until the very last hop. Calling those
/// nodes `stale` made `nodes_done` the count of nodes the evaluation never touched — constant from
/// the first hop to the last. The published ratio was monotone and completely flat, then jumped to
/// `1.0`: a progress bar that only ever reads "nothing yet" and then "done"
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_node_census_advances_as_a_chain_walks_and_never_calls_a_recomputed_node_stale() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    host.set_slider_value("slider", 12.0);
    let armed = build_flow_status_json(&host, &host.pending_eval_widget_ids());
    assert_eq!(census_entries(&armed), [("add".to_string(), "computing".to_string()), ("pass".to_string(), "queued".to_string()), ("preview".to_string(), "ok".to_string()), ("slider".to_string(), "ok".to_string())]);

    let after_first = host.evaluate_step(EvalStepBudget::dispatches(1));
    let hop1 = build_flow_status_json(&host, &after_first);
    assert_eq!(census_entries(&hop1), [("add".to_string(), "ok".to_string()), ("pass".to_string(), "computing".to_string()), ("preview".to_string(), "ok".to_string()), ("slider".to_string(), "ok".to_string())], "the node this hop recomputed has SETTLED, whatever the frozen baseline still calls dirty");

    let after_second = host.evaluate_step(EvalStepBudget::dispatches(1));
    assert!(after_second.is_empty(), "two budget-one hops converge this chain");
    let hop2 = build_flow_status_json(&host, &after_second);
    let census = [census_nodes_done(&armed), census_nodes_done(&hop1), census_nodes_done(&hop2)];
    assert!(census[0] < census[1] && census[1] < census[2], "the census must GROW every hop, not merely refuse to shrink: {census:?}");
    eprintln!("[DEBUG] flow node census nodes_done per hop: {census:?}");
    host.retire_cold();
}

/// ⚖️ LAW: a coalesced tick parks a whole wave AND paints every member of it `computing` — the
/// census names what is outstanding at a plugin right now, never just the head of the remaining list.
#[test]
fn a_coalesced_tick_parks_a_whole_wave_and_paints_every_member_computing() {
    let mut host = host_with_two_extension_siblings();
    let remaining = host.evaluate_step(flow_eval_tick_budget(None));
    let parked: Vec<&str> = host.pending_extension_evals.iter().map(|pending| pending.neuron_id.as_str()).collect();
    assert_eq!(parked, ["left", "right"], "both ready contributed nodes park on the SAME hop");
    let census = census_entries(&build_flow_status_json(&host, &remaining));
    assert_eq!(census, [("add".to_string(), "ok".to_string()), ("left".to_string(), "computing".to_string()), ("preview".to_string(), "ok".to_string()), ("right".to_string(), "computing".to_string()), ("slider".to_string(), "ok".to_string())]);
    eprintln!("[DEBUG] flow wave census: parked={parked:?} census={census:?}");
    host.retire_cold();
}

/// ⚖️ LAW: cancellation is WAVE-SIZED — one gesture retires a whole coalesced wave, and every answer
/// that was already crossing when it landed arms nothing.
///
/// 🚨️ The cancel affordance must not get weaker as a hop carries more work. A wave of N parked
/// answers leaves N settles still to come; if any of them could re-arm the chain, a user's cancel
/// would be silently undone by the network (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn cancelling_a_coalesced_wave_retires_every_parked_answer_and_late_settles_arm_nothing() {
    let mut session = FlowEvalSession::new();
    session.arm_window_tick("preview-1");
    session.begin_window_tick("preview-1");
    session.note_window_extensions_in_flight("preview-1", 3);
    session.note_window_tick_outcome("preview-1", true);
    assert_eq!(session.window_extensions_in_flight("preview-1"), 3, "a three-wide wave is three answers outstanding");
    assert!(session.preview_chain_status().working, "a parked wave is live work");

    let retired = session.cancel_preview_evaluation("preview-1");
    assert_eq!(session.window_extensions_in_flight("preview-1"), 0, "one gesture retires the WHOLE wave, not one answer of it");
    assert!(session.preview_cancelled(), "the surface may publish the cancelled banner");
    assert!(!session.preview_chain_status().working, "a cancelled chain owes nothing");

    let rearms = (0..3).filter(|_| session.settle_window_extension("preview-1")).count();
    assert_eq!(rearms, 0, "every answer still crossing when the cancel landed arms nothing");
    assert!(!session.window_tick_is_armed("preview-1"), "a cancelled window stays unarmed");
    eprintln!("[DEBUG] wave cancel: retiredTessellations={retired} rearms={rearms}");
    session.retire_cold();
}

/// ⚖️ LAW: a fold may only take over a hop the SCHEDULER would have dispatched — never invent one,
/// never resume a cancelled chain, and never start a walk a turn has no wall left for.
///
/// 🔁️ The inline continuation is the removal of the `flowEvalResolve` → `flowEvalTick` pair each
/// dependency level used to cost (`📓️flow-tick-coalescing-2026-09-14.md` §7 item 4). Its whole
/// safety argument is that its admission question IS the run job's own scheduling question
/// (`window_tick_owed`), so the chain's SHAPE cannot change — only who runs the hop.
#[test]
fn an_inline_continuation_is_admitted_exactly_where_the_run_job_would_have_dispatched_a_hop() {
    let mut session = FlowEvalSession::new();
    assert!(session.inline_continuation_admitted("preview-1", None, None), "a window that never ticked owes its first hop, so a fold may run it");

    session.arm_window_tick("preview-1");
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "a hop is already armed — a fold must not run a second one");

    session.begin_window_tick("preview-1");
    session.note_window_extensions_in_flight("preview-1", 2);
    session.note_window_tick_outcome("preview-1", true);
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "the FIRST answer of a two-wide wave leaves a sibling in flight — a fold must not run the next wave yet");

    assert!(!session.settle_window_extension("preview-1"), "the first settle of a fan-out arms nothing");
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "one answer is still outstanding");
    assert!(!session.settle_window_extension("preview-1"), "no source had asked for a re-arm while the wave was crossing");
    assert!(session.inline_continuation_admitted("preview-1", None, None), "the LAST answer of a wave holds a window that owes a hop nothing is chasing — exactly the run job's Dispatch branch");

    let armed = session.arm_window_tick("preview-1");
    assert!(armed, "the continuation CLAIMS the hop it takes over");
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "a claimed hop is not offered twice");

    session.begin_window_tick("preview-1");
    session.note_window_tick_outcome("preview-1", false);
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "a finished window owes nothing to continue");
    eprintln!("[DEBUG] inline continuation admission walked one two-wide wave");
    session.retire_cold();
}

/// ⚖️ LAW: a CANCELLED chain is never continued inline.
///
/// 🛑 `begin_window_tick` retires the `cancelled` banner, because work resuming is the one thing that
/// may. An inline continuation on a cancelled session would therefore not merely compute one wave too
/// many — it would UN-CANCEL the run the user stopped, from inside an answer that was already
/// crossing when they stopped it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn a_cancelled_chain_is_never_continued_inline_by_an_answer_that_was_already_crossing() {
    let mut session = FlowEvalSession::new();
    session.arm_window_tick("preview-1");
    session.begin_window_tick("preview-1");
    session.note_window_extensions_in_flight("preview-1", 2);
    session.note_window_tick_outcome("preview-1", true);

    session.cancel_preview_evaluation("preview-1");
    assert!(session.preview_cancelled(), "the gesture landed");
    for _ in 0..2 {
        session.settle_window_extension("preview-1");
        assert!(!session.inline_continuation_admitted("preview-1", None, None), "an answer that was already crossing may not continue a cancelled chain");
    }
    assert!(!session.window_tick_is_armed("preview-1"), "and it armed nothing either");
    eprintln!("[DEBUG] cancelled chain refused 2 inline continuations");
    session.retire_cold();
}

/// ⚖️ LAW: the wall a continuation runs under belongs to the TURN, not to the walk — a turn with less
/// than [`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`] left parks, and a continuation that is admitted
/// inherits the turn's own deadline instead of opening a fresh allowance.
///
/// 🚨️ Without both halves an inline chain stretches the interactive hold by one whole
/// [`FLOW_EVAL_TICK_ELAPSED_CEILING_US`] per wave — and `EvalStepBudget` always dispatches at least
/// one node before a deadline can stop it, so an admission with a sliver left overruns by a whole
/// operator rather than yielding.
#[test]
fn every_dag_walk_of_one_guest_turn_shares_one_wall_deadline_and_a_spent_turn_parks() {
    let turn_started_us = 1_000_000_u64;
    assert!(flow_eval_inline_continuation_fits(Some(turn_started_us), Some(turn_started_us)), "a turn that has spent nothing admits a continuation");
    let last_admitted_us = turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US - FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US;
    assert!(flow_eval_inline_continuation_fits(Some(turn_started_us), Some(last_admitted_us)), "a turn with exactly the reserve left still admits one");
    assert!(!flow_eval_inline_continuation_fits(Some(turn_started_us), Some(last_admitted_us + 1)), "one microsecond past the reserve parks");
    assert!(!flow_eval_inline_continuation_fits(Some(turn_started_us), Some(turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US * 4)), "a turn that already overran parks");
    assert!(flow_eval_inline_continuation_fits(None, Some(last_admitted_us)), "a walk that OPENS its own turn is not measured against anyone else's");
    assert!(flow_eval_inline_continuation_fits(Some(turn_started_us), None), "an uninstrumented process falls back to the node budget, exactly as flow_eval_tick_budget does");

    assert_eq!(flow_eval_tick_budget(Some(turn_started_us)).deadline_us(), Some(turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US), "an inline walk inherits the TURN's deadline");
    let own_turn = flow_eval_tick_budget(None).deadline_us();
    assert_ne!(own_turn, Some(turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US), "a walk that opens its own turn gets its own allowance");
    assert!(FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US > 0 && FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US < FLOW_EVAL_TICK_ELAPSED_CEILING_US, "the reserve is a slice of the allowance, not all of it and not none of it");
    eprintln!("[DEBUG] turn deadline: ceiling={FLOW_EVAL_TICK_ELAPSED_CEILING_US}us reserve={FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US}us ownTurnDeadline={own_turn:?}");
}

/// ⚖️ LAW: a chain that has settled its own census says so, and a chain that has published no census
/// yet does NOT — the two windows in which the run view legitimately knows more, and less, than the
/// evaluation it is pacing.
///
/// 🩸️ `computing` and the `toolRunAbort` affordance were ORed straight off `ToolRunView`, which is a
/// host artifact that reaches a surface on a render and therefore lags its own job. Once the inline
/// continuation let a chain finish inside an extension answer's turn, the guest went quiet with the
/// run's terminal state still unrendered — and the preview published `phase: "idle", ratio: 1.0,
/// nodesDone 7/7` beside `computing: true, cancellable: true`, for good: a spinner that outlived its
/// work and a Cancel for an evaluation with nothing left to cancel. Measured on 6024 in two picks
/// (`📓️flow-inline-continuation-2026-09-14.md`).
#[test]
fn a_settled_chain_census_outranks_a_lagging_run_view_and_an_empty_one_does_not() {
    let working = PreviewChainStatus { nodes_done: 3, nodes_total: 7, in_flight: 1, working: true };
    assert!(!working.settled(), "a chain with work in flight has not settled");

    let half = PreviewChainStatus { nodes_done: 3, nodes_total: 7, in_flight: 0, working: false };
    assert!(!half.settled(), "a quiesced chain whose census is incomplete has not settled either");

    let census_free = PreviewChainStatus { nodes_done: 0, nodes_total: 0, in_flight: 0, working: false };
    assert!(!census_free.settled(), "a chain that has published NO census may not silence the run: this is the window a gesture's own start lives in, before its first hop ran");

    let done = PreviewChainStatus { nodes_done: 7, nodes_total: 7, in_flight: 0, working: false };
    assert!(done.settled(), "nothing owed, nothing in flight, every published node accounted for");
    assert_eq!(done.units(), (7, 7), "and its published fraction is complete");

    let overshoot = PreviewChainStatus { nodes_done: 9, nodes_total: 7, in_flight: 0, working: false };
    assert!(overshoot.settled(), "a census that counted more than it declared is settled, never unsettled");
    eprintln!("[DEBUG] chain settled: working={} half={} censusFree={} done={}", working.settled(), half.settled(), census_free.settled(), done.settled());
}
