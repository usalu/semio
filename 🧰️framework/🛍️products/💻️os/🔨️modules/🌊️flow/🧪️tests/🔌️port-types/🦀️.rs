//! 🔌️ The port-type law of the flow graph, answered over the LIVE extension registry: every channel
//! the fixture names declares exactly the value schemas the fixture records, and every pair the
//! fixture lists is accepted or refused exactly as the fixture says.
//!
//! The oracle is `🧫️fixtures/🔌️port-types/🔣️.json` — not this file and not the TypeScript twin
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`),
//! which answers the same rows over the same declared types. A drift on either side fails on both.
//!
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

// #region 🔖️Imports
use semio_framework_os_flow::{port_value_types_compatible, widget_port_value_types, PortSide};
use semio_framework_artifact_flow_flow::{SynapseSpec, Widget};
use semio_framework_os_flow::{FlowHost, FlowHostRetirement};
use semio_framework_artifact_flow_flow::FlowFixture;
use std::collections::HashMap;
use std::sync::OnceLock;
// #endregion 🔖️Imports

// #region 🧰️Fixture
const FIXTURE: &str = include_str!("../../🧫️fixtures/🔌️port-types/🔣️.json");

struct Row {
    source: String,
    target: String,
    compatible: bool,
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("port-types fixture parses")
}

/// 🍄️ The defect's own graph, exactly as the example ships it.
fn hexagonal_mushroom_column() -> FlowFixture {
    let widgets = vec![
        Widget::Neuron { id: "profile".into(), neuron_kind: "brep.curve.polygon".into(), params: Default::default(), input_ports: vec!["radius".into(), "sides".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "extrusion-axis".into(), neuron_kind: "math.vector".into(), params: Default::default(), input_ports: vec!["x".into(), "y".into(), "z".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "extrude".into(), neuron_kind: "brep.solid.extrude".into(), params: Default::default(), input_ports: vec!["wire".into(), "vector".into()], output_ports: vec![], preview: true },
    ];
    let synapses = vec![
        SynapseSpec { id: "e4".into(), from: "profile".into(), from_port: "wire".into(), to: "extrude".into(), to_port: "wire".into() },
        SynapseSpec { id: "e5".into(), from: "extrusion-axis".into(), from_port: "vectorOut".into(), to: "extrude".into(), to_port: "vector".into() },
    ];
    FlowFixture { widgets, synapses, ..Default::default() }
}

/// 🧹️ `FlowFixture`'s ordered maps refuse to drop unretired, so a law that builds a real host walks
/// it through the same retirement ladder the session close walks.
fn retire(host: FlowHost) {
    let mut retirement = FlowHostRetirement::new(host);
    for _ in 0..1_000_000 {
        if retirement.close_page(64, 65_536).expect("flow host retirement") {
            return;
        }
    }
    panic!("flow host did not retire within bound");
}

fn complete_registration<T>(future: impl std::future::Future<Output = T>) -> T {
    match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("extension registration must not depend on external work"),
    }
}

/// 📇️ The first-party operator catalogue the eight examples draw from, built from the extension
/// crates' OWN registrations — the same declarations the app installs at runtime, with no global
/// registry state in between.
fn kind_infos() -> &'static HashMap<String, neural_engine::OperatorInfo> {
    static CACHE: OnceLock<(neural_engine::ColdOwner<neural_engine::Registry>, HashMap<String, neural_engine::OperatorInfo>)> = OnceLock::new();
    &CACHE
        .get_or_init(|| {
            let mut registry = neural_engine::ColdOwner::new(neural_engine::Registry::new());
            semio_s_plugin_flow_extension_primitive::register(&mut registry);
            semio_s_plugin_flow_extension_math::register(&mut registry);
            complete_registration(semio_s_plugin_flow_extension_brep::register(&mut registry));
            registry.finalize();
            let infos = registry.operator_infos().map(|info| (info.id.clone(), info.clone())).collect();
            (registry, infos)
        })
        .1
}

fn declared_types(reference: &str) -> Vec<String> {
    let (operator, port) = reference.split_once('@').expect("channel reference is operator@port");
    if let Some(widget) = operator.strip_prefix("widget.") {
        let widget = match widget {
            "inputSlider" => Widget::InputSlider { id: "s".into(), label: String::new(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 },
            other => panic!("unknown widget in fixture: {other}"),
        };
        let synapses: Vec<SynapseSpec> = Vec::new();
        let out = widget_port_value_types("s", port, PortSide::Output, std::slice::from_ref(&widget), &synapses, kind_infos());
        let inn = widget_port_value_types("s", port, PortSide::Input, std::slice::from_ref(&widget), &synapses, kind_infos());
        return if out.is_empty() { inn } else { out };
    }
    let info = kind_infos().get(operator).unwrap_or_else(|| panic!("operator {operator} is registered"));
    info.inputs
        .iter()
        .chain(info.outputs.iter())
        .find(|channel| channel.name == port)
        .unwrap_or_else(|| panic!("{operator} declares a channel named {port}"))
        .value_types
        .clone()
}
// #endregion 🧰️Fixture

// #region 🧪️Laws
/// 🔤️ Every channel the law reasons about declares exactly the value schemas the fixture records —
/// so a channel that silently loses its declaration (which would make it connectable to anything)
/// fails here, not in a user's graph.
#[test]
fn every_named_channel_declares_the_value_types_the_fixture_records() {
    let fixture = fixture();
    let channels = fixture["channels"].as_array().expect("channels array");
    assert!(channels.len() >= 30, "the fixture must cover the whole example catalogue, saw {}", channels.len());
    for channel in channels {
        let reference = channel["ref"].as_str().expect("ref");
        let expected: Vec<String> = channel["valueTypes"].as_array().expect("valueTypes").iter().map(|entry| entry.as_str().expect("value type").to_string()).collect();
        assert_eq!(declared_types(reference), expected, "{reference} declares different value types than the fixture records");
        assert!(!expected.is_empty(), "{reference} must declare at least one value type");
    }
}

/// 🔌️ Every pair the fixture lists is accepted or refused exactly as it says — the compatible rows
/// are the wires the eight shipped examples actually draw, so a rule that over-refuses fails here.
#[test]
fn every_fixture_pair_is_accepted_or_refused_as_the_fixture_says() {
    let fixture = fixture();
    let rows: Vec<Row> = fixture["rows"]
        .as_array()
        .expect("rows array")
        .iter()
        .map(|row| Row { source: row["source"].as_str().expect("source").to_string(), target: row["target"].as_str().expect("target").to_string(), compatible: row["compatible"].as_bool().expect("compatible") })
        .collect();
    assert!(rows.iter().any(|row| !row.compatible), "the fixture must carry refused pairs");
    assert!(rows.iter().any(|row| row.compatible), "the fixture must carry accepted pairs");
    for row in &rows {
        let source = declared_types(&row.source);
        let target = declared_types(&row.target);
        assert_eq!(port_value_types_compatible(&source, &target), row.compatible, "{} -> {} ({source:?} -> {target:?})", row.source, row.target);
    }
}

/// 🚫️ The defect itself: the vector output the hexagonal mushroom column wires into `extrude@vector`
/// may NOT land on `extrude@wire`, while the profile wire that belongs there still may.
#[test]
fn a_vector_output_is_refused_by_the_wire_input_that_accepts_the_profile() {
    let vector_out = declared_types("math.vector@vectorOut");
    let wire_in = declared_types("brep.solid.extrude@wire");
    let vector_in = declared_types("brep.solid.extrude@vector");
    let profile_out = declared_types("brep.curve.polygon@wire");
    assert!(!port_value_types_compatible(&vector_out, &wire_in), "a math.vector output must never reach a wire input");
    assert!(port_value_types_compatible(&vector_out, &vector_in), "the shipped vector wire must stay legal");
    assert!(port_value_types_compatible(&profile_out, &wire_in), "the shipped profile wire must stay legal");
}

/// 🍄️ The defect end to end, through the ONE door every node-graph surface dispatches into: the
/// hexagonal mushroom column's own graph refuses the drop, keeps the wire the drop would have
/// displaced, and adds no synapse — so nothing re-solves.
#[test]
fn the_flow_host_refuses_the_drop_and_keeps_the_displaced_wire() {
    let mut host = FlowHost::from_fixture_with_cache_and_infos(hexagonal_mushroom_column(), std::sync::Arc::new(neural_engine::NeuralCache::new()), std::sync::Arc::new(kind_infos().clone()));
    let before: Vec<(String, String, String, String)> = host.fixture.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.from_port.clone(), synapse.to.clone(), synapse.to_port.clone())).collect();
    let refusal = host.connect_ports("extrusion-axis", "vectorOut", "extrude", "wire");
    assert!(matches!(refusal, Err(semio_framework_os_flow::FlowCoreError::IncompatiblePortTypes { .. })), "the drop must be refused, got {refusal:?}");
    let after: Vec<(String, String, String, String)> = host.fixture.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.from_port.clone(), synapse.to.clone(), synapse.to_port.clone())).collect();
    assert_eq!(after, before, "a refused drop must leave the graph byte-identical");
    assert!(after.contains(&("profile".into(), "wire".into(), "extrude".into(), "wire".into())), "the profile wire the drop would have displaced must survive");
    assert!(host.connect_ports("extrusion-axis", "vectorOut", "extrude", "vector").is_err(), "that wire already exists");
    retire(host);
    let mut host = FlowHost::from_fixture_with_cache_and_infos(hexagonal_mushroom_column(), std::sync::Arc::new(neural_engine::NeuralCache::new()), std::sync::Arc::new(kind_infos().clone()));
    host.disconnect("e5").expect("the shipped vector wire is there to cut");
    host.connect_ports("extrusion-axis", "vectorOut", "extrude", "vector").expect("and the compatible pair reconnects");
    retire(host);
}

/// 🕊️ An undeclared channel stays connectable — the rule refuses only when BOTH sides declare, so a
/// third-party operator that has not adopted port types is never retro-refused.
#[test]
fn an_undeclared_channel_stays_connectable() {
    let geometry = vec!["geometry".to_string()];
    let undeclared: Vec<String> = Vec::new();
    assert!(port_value_types_compatible(&undeclared, &geometry));
    assert!(port_value_types_compatible(&geometry, &undeclared));
    assert!(port_value_types_compatible(&undeclared, &undeclared));
}
// #endregion 🧪️Laws
