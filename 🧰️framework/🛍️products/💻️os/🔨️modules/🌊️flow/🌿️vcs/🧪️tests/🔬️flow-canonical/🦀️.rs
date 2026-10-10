//! 🧪️ The borrowed Flow canonical JSON matches an independent serde_json oracle for every leaf shape.
use super::*;
use crate::os_store::{ArtifactCanonicalJson, ArtifactCanonicalJsonNode as N, ArtifactCanonicalJsonText, ArtifactCanonicalJsonValue as V, ArtifactPreparedOperationSource};
use neural::{Atom, Dictionary, Value as NeuralValue};
use serde_json::{json, Value};

fn render(value: V<'_>) -> Value {
    match value {
        V::Scalar(N::Null) => Value::Null,
        V::Scalar(N::Bool(value)) => json!(value),
        V::Scalar(N::U64(value)) => json!(value),
        V::Scalar(N::I64(value)) => json!(value),
        V::Scalar(N::F64(value)) => json!(value),
        V::Scalar(N::String(value)) => json!(value),
        V::Scalar(_) | V::Source(_) => panic!("flow canonical JSON borrows only plain scalars"),
        V::Array(values) => Value::Array(values.map(render).collect()),
        V::Object(fields) => Value::Object(fields.map(|(key, value)| (match key { ArtifactCanonicalJsonText::Contiguous(key) => key.to_string(), ArtifactCanonicalJsonText::Native(_) => panic!("flow keys are contiguous") }, render(value))).collect()),
    }
}

fn indexed(source: &FlowMutation, path: &mut Vec<usize>) -> Value {
    match source.canonical_json_node(path).expect("indexed canonical node") {
        N::Null => Value::Null,
        N::Bool(value) => json!(value),
        N::U64(value) => json!(value),
        N::I64(value) => json!(value),
        N::F64(value) => json!(value),
        N::String(value) => json!(value),
        N::Array(length) => Value::Array((0..length).map(|index| { path.push(index); let value = indexed(source, path); path.pop(); value }).collect()),
        N::Object(length) => Value::Object((0..length).map(|index| {
            let key = match source.canonical_json_key(path, index).expect("indexed canonical key") { ArtifactCanonicalJsonText::Contiguous(key) => key.to_string(), ArtifactCanonicalJsonText::Native(_) => panic!("flow keys are contiguous") };
            path.push(index);
            let value = indexed(source, path);
            path.pop();
            (key, value)
        }).collect()),
        _ => panic!("flow canonical JSON has only plain nodes"),
    }
}

fn canonical(mutation: &FlowMutation) -> Value {
    let borrowed = render(mutation.canonical_json_borrowed_root().expect("canonical traversal").expect("flow mutations borrow a root"));
    assert_eq!(borrowed, indexed(mutation, &mut Vec::new()), "the borrowed root and the indexed traversal are one document");
    assert!(mutation.canonical_json_node(&[usize::MAX]).is_err() && mutation.canonical_json_key(&[0], 9).is_err(), "unknown positions are refused");
    borrowed
}

fn synapse(id: &str) -> SynapseSpec {
    SynapseSpec { id: id.into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }
}

#[test]
fn every_leaf_shape_matches_the_independent_oracle() {
    assert_eq!(canonical(&FlowMutation::RemoveWidget(RemoveWidget { id: "w".into() })), json!({"mutation": "remove-widget", "payload": {"id": "w"}}));
    assert_eq!(canonical(&FlowMutation::MoveWidget(MoveWidget { id: "w".into(), to_index: 3 })), json!({"mutation": "move-widget", "payload": {"id": "w", "toIndex": 3}}));
    assert_eq!(canonical(&FlowMutation::AddSynapse(AddSynapse { index: 2, synapse: synapse("s") })), json!({"mutation": "add-synapse", "payload": {"index": 2, "synapse": {"id": "s", "from": "a", "to": "b", "fromPort": "out", "toPort": "in"}}}));
    assert_eq!(canonical(&FlowMutation::RemoveSynapse(RemoveSynapse { id: "s".into() })), json!({"mutation": "remove-synapse", "payload": {"id": "s"}}));
    assert_eq!(canonical(&FlowMutation::MoveSynapse(MoveSynapse { id: "s".into(), to_index: 1 })), json!({"mutation": "move-synapse", "payload": {"id": "s", "toIndex": 1}}));
    assert_eq!(canonical(&FlowMutation::ChangeSynapse(ChangeSynapse { id: "s".into(), synapse: synapse("s") })), json!({"mutation": "change-synapse", "payload": {"id": "s", "synapse": {"id": "s", "from": "a", "to": "b", "fromPort": "out", "toPort": "in"}}}));
    assert_eq!(
        canonical(&FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "w".into(), layout: Some(WidgetLayout { x: 1.5, y: 2.0 }) }, FlowLayoutEntry { id: "v".into(), layout: None }] })),
        json!({"mutation": "change-layout", "payload": {"entries": [{"id": "w", "layout": {"x": 1.5, "y": 2.0}}, {"id": "v", "layout": null}]}})
    );
}

#[test]
fn widget_payloads_borrow_every_owned_field() {
    let widget = Widget::Neuron { id: "n".into(), neuron_kind: "core.add".into(), params: Dictionary::new().insert("a", NeuralValue::Atom(Atom::Integer(1))).insert("b", NeuralValue::Atom(Atom::String("x".into()))), input_ports: vec!["a".into()], output_ports: vec!["out".into()], preview: true };
    let note = Widget::InputNote { id: "t".into(), text: "hello".into() };
    let changed = FlowMutation::ChangeWidget(ChangeWidget { id: "t".into(), widget: note });
    assert_eq!(canonical(&changed), json!({"mutation": "change-widget", "payload": {"id": "t", "widget": {"kind": "inputNote", "id": "t", "text": "hello"}}}));
    let added = FlowMutation::AddWidget(AddWidget { index: 0, widget });
    assert_eq!(
        canonical(&added),
        json!({"mutation": "add-widget", "payload": {"index": 0, "widget": {"kind": "neuron", "id": "n", "neuronKind": "core.add", "params": {"a": 1, "b": "x"}, "inputPorts": ["a"], "outputPorts": ["out"], "preview": true}}})
    );
    retire_flow_mutation(changed);
    retire_flow_mutation(added);
}

#[test]
fn the_wire_source_is_the_mutation_itself_as_canonical_json() {
    let mutation = FlowMutation::RemoveWidget(RemoveWidget { id: "w".into() });
    assert!(matches!(prepared_operation_wire_source(&mutation), Some(ArtifactPreparedOperationSource::CanonicalJson { header: b"flow", .. })));
}
