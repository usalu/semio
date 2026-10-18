//! 🔎️ Inference dependency and evaluate JSON of the wasm extension glue: the canonical geometry inference ancestry and the JSON operator and function evaluation. See [`super`].

use super::*;

/// 🧬️ Canonical source and wiring ancestry of one geometry inference, excluding presentation data.
pub fn flow_inference_dependency_json(snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, neuron_id: &str) -> String {
    use semio_framework_artifact_flow_flow::Widget;
    use std::collections::BTreeMap;
    let mut ids = std::collections::BTreeSet::new();
    let mut pending = vec![neuron_id];
    while let Some(id) = pending.pop() {
        if !ids.insert(id) { continue; }
        pending.extend(snapshot.synapses.iter().filter(|edge| edge.to == id).map(|edge| edge.from.as_str()));
    }
    let widgets: BTreeMap<_, _> = snapshot.widgets.iter().filter(|widget| ids.contains(semio_framework_artifact_flow_flow::widget_id_for(widget))).map(|widget| {
        let value = match widget {
            Widget::Neuron { neuron_kind, params, .. } => DslValue::object([("operatorId".into(), neuron_kind.to_value()), ("params".into(), params.to_value())]),
            Widget::InputSlider { value, .. } => DslValue::object([("number".into(), value.to_value())]),
            Widget::InputNote { text, .. } => DslValue::object([("text".into(), text.to_value())]),
            Widget::InputImage { src, .. } => DslValue::object([("source".into(), src.to_value())]),
            Widget::Variable { name, schema, .. } => DslValue::object([("name".into(), name.to_value()), ("schema".into(), schema.to_value())]),
            Widget::Cluster { tree, .. } => tree.to_value(),
            _ => DslValue::Null,
        };
        (semio_framework_artifact_flow_flow::widget_id_for(widget).to_string(), value)
    }).collect();
    let synapses: BTreeMap<_, _> = snapshot.synapses.iter().filter(|edge| ids.contains(edge.to.as_str())).map(|edge| (edge.id.clone(), edge.to_value())).collect();
    semio_framework_pack_json::to_json_string(&DslValue::object([
        ("widgets".into(), DslValue::object(widgets)), ("synapses".into(), DslValue::object(synapses)),
    ]))
}

// #region 🔖️Evaluate
/// 🧮️ Evaluates an operator and returns JSON dictionary or `{ "error": ... }`.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match semio_framework_pack_json::from_json_str(input_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(d) => d,
        Err(err) => return semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let input = ColdOwner::new(match registry.operator_info(kind_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    });
    match registry.dispatch(kind_id, &input) {
        Ok(out) => semio_framework_pack_json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}

/// 🧮️ Evaluates a neural tree as a function and returns the out dictionary JSON or `{ "error": ... }`.
pub fn evaluate_function_json(registry: &Registry, tree_json: &str, in_dict_json: &str) -> String {
    let tree: neural_engine::Tree = match semio_framework_pack_json::from_json_str(tree_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(tree) => tree,
        Err(err) => return semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let tree = ColdOwner::new(tree);
    let in_dict: Dictionary = match semio_framework_pack_json::from_json_str(in_dict_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(dict) => dict,
        Err(err) => return semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let evaluator = neural_engine::Evaluator::new(registry);
    let in_dict = ColdOwner::new(in_dict);
    match evaluator.evaluate_function(&tree, &in_dict) {
        Ok(out) => semio_framework_pack_json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}
// #endregion 🔖️Evaluate
