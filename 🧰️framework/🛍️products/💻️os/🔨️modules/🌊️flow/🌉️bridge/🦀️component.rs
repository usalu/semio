//! 🌉️ Flow eval bridge and channel-eval helpers.

use neural_engine as neural;

use std::collections::{BTreeSet, HashMap};

use neural::{
    cluster_operator_info, Atom, ChannelSpec, Dictionary, EvalChannels, EvalError, Neuron, OperatorInfo, Value as NeuralValue,
    INPUT_KIND, OUTPUT_KIND,
};

use crate::artifact::*;
use crate::host::*;


// #region 🔖️EvalBridge
#[cfg(target_arch = "wasm32")]
pub(crate) fn parse_bridge_dictionary_json(result_json: &str) -> Result<Dictionary, EvalError> {
    if let Ok(dict) = serde_json::from_str::<Dictionary>(result_json) {
        return Ok(dict);
    }
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(result_json) {
        if let Some(err) = value.get("error").and_then(|v| v.as_str()) {
            return Err(EvalError::InvalidInput(err.into()));
        }
    }
    Err(EvalError::InvalidInput("invalid bridge response".into()))
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct EvalBridge {
    pub(crate) cb: js_sys::Function,
}

#[cfg(target_arch = "wasm32")]
impl EvalBridge {
    pub(crate) fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        use wasm_bindgen::JsValue;
        let input_json = serde_json::to_string(input).map_err(|e| EvalError::InvalidInput(e.to_string()))?;
        let result = self.cb.call2(&JsValue::NULL, &JsValue::from_str(kind_id), &JsValue::from_str(&input_json)).map_err(|_| EvalError::InvalidInput("bridge call failed".into()))?;
        let result_json = result.as_string().ok_or_else(|| EvalError::InvalidInput("bridge did not return string".into()))?;
        parse_bridge_dictionary_json(&result_json)
    }
}

/// 🔌️ Native eval-bridge callback: operator kind id + input dictionary in, evaluated dictionary or `EvalError` out.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) type EvalBridgeFn = dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Send;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct EvalBridge {
    pub(crate) cb: Box<EvalBridgeFn>,
}

#[cfg(not(target_arch = "wasm32"))]
impl EvalBridge {
    pub(crate) fn evaluate(&self, kind_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        (self.cb)(kind_id, input)
    }
}
// #endregion 🔖️EvalBridge

// #region 🔖️ChannelEval
fn neural_value_to_json(value: &NeuralValue) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

fn dictionary_to_json_object(dict: &Dictionary) -> serde_json::Map<String, serde_json::Value> {
    dict.keys().map(|key| (key.clone(), neural_value_to_json(dict.get(key).expect("key came from dict.keys(), so get(key) cannot miss")))).collect()
}

fn input_ports_json(dict: &Dictionary, kind_info: Option<&OperatorInfo>) -> serde_json::Map<String, serde_json::Value> {
    let mut ports = serde_json::Map::new();
    if let Some(info) = kind_info {
        if let Some(variadic) = &info.variadic_input {
            if let Some(slots) = dict.get(&variadic.slot_key).and_then(|value| value.as_dictionary()) {
                for key in slots.keys() {
                    if let Some(value) = slots.get(key) {
                        ports.insert(key.clone(), neural_value_to_json(value));
                    }
                }
            }
        }
        for port in &info.inputs {
            if port.name == "*" {
                continue;
            }
            if let Some(value) = dict.get(&port.name) {
                ports.insert(port.name.clone(), neural_value_to_json(value));
            }
        }
        return ports;
    }
    dictionary_to_json_object(dict)
}

fn output_ports_json(dict: &Dictionary) -> serde_json::Map<String, serde_json::Value> {
    let mut ports = serde_json::Map::new();
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            ports.insert(key.clone(), serde_json::to_value(value).unwrap_or(serde_json::Value::Null));
        }
    }
    ports
}

pub(crate) fn outputs_from_channel_eval_json(json: &str) -> HashMap<String, Dictionary> {
    let Ok(parsed) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json) else {
        return HashMap::new();
    };
    let mut outputs = HashMap::new();
    for (widget_id, entry) in parsed {
        let Some(out_ports) = entry.get("out").and_then(|value| value.as_object()) else {
            continue;
        };
        let mut dict = Dictionary::new();
        for (key, val) in out_ports {
            if let Ok(value) = serde_json::from_value::<NeuralValue>(val.clone()) {
                dict = dict.insert(key.clone(), value);
            }
        }
        if !dict.is_empty() {
            outputs.insert(widget_id, dict);
        }
    }
    outputs
}

pub(crate) fn inputs_from_channel_eval_json(json: &str) -> HashMap<String, Dictionary> {
    let Ok(parsed) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(json) else {
        return HashMap::new();
    };
    let mut inputs = HashMap::new();
    for (widget_id, entry) in parsed {
        let Some(in_ports) = entry.get("in").and_then(|value| value.as_object()) else {
            continue;
        };
        let mut dict = Dictionary::new();
        for (key, val) in in_ports {
            if let Ok(value) = serde_json::from_value::<NeuralValue>(val.clone()) {
                dict = dict.insert(key.clone(), value);
            }
        }
        if !dict.is_empty() {
            inputs.insert(widget_id, dict);
        }
    }
    inputs
}

pub(crate) fn preview_dict_from_connection(src: &Dictionary, from_port: &str, to_port: &str) -> Dictionary {
    let payload = if from_port.is_empty() {
        src.clone()
    } else if let Some(dict) = src.get(from_port).and_then(|value| value.as_dictionary()) {
        dict.clone()
    } else if let Some(value) = src.get(from_port) {
        Dictionary::new().insert(from_port, value.clone())
    } else {
        Dictionary::new()
    };
    if to_port.is_empty() {
        payload
    } else {
        Dictionary::new().insert(to_port, NeuralValue::Dictionary(payload))
    }
}

pub(crate) fn widget_operator_info(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> Option<OperatorInfo> {
    match widget {
        Widget::Neuron { neuron_kind, .. } => kind_infos.get(neuron_kind).cloned(),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            let info = OperatorInfo {
                id: "core.variable".into(),
                extension: "core".into(),
                name: name.clone(),
                abbreviation: name.chars().take(3).collect(),
                icon: "emoji:🔣️".into(),
                summary: "Named typed dictionary".into(),
                inputs: inputs.iter().map(|port| ChannelSpec::requires(&port.id, &[schema.as_str()])).collect(),
                outputs: outputs.iter().map(|port| ChannelSpec::provides(&port.id, vec![schema.clone()])).collect(),
                ..Default::default()
            };
            Some(info)
        }
        Widget::Cluster { id, name, tree, .. } => Some(cluster_operator_info(id, if name.is_empty() { "Cluster" } else { name }, tree)),
        _ => None,
    }
}

pub(crate) fn widget_to_inner_neuron(widget: &Widget) -> Option<Neuron> {
    match widget {
        Widget::Neuron { id, neuron_kind, params, .. } => Some(Neuron::with_kind(id, neuron_kind, params.clone())),
        Widget::InputSlider { id, value, .. } => Some(Neuron::with_kind(id, "core.number", Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(*value))))),
        Widget::InputNote { id, text } => Some(Neuron::with_kind(id, "core.text", Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))))),
        Widget::InputImage { id, src } => Some(Neuron::with_kind(id, "core.image", Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))))),
        Widget::Variable { id, name, schema } => Some(Neuron::with_kind(id, "core.variable", Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))).insert("schema", NeuralValue::Atom(Atom::String(schema.clone()))))),
        _ => None,
    }
}

pub(crate) fn contract_boundary_params(channel: &str, schema: &str) -> Dictionary {
    Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String(channel.into()))).insert("operators", NeuralValue::Atom(Atom::String(schema.into())))
}

fn boundary_schema_from_params(params: &Dictionary) -> String {
    params.get("operators").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("dictionary").to_string()
}

pub(crate) fn variable_widget_meta(widgets: &[Widget], id: &str) -> Option<(String, String)> {
    widgets.iter().find_map(|widget| match widget {
        Widget::Variable { id: widget_id, name, schema } if widget_id == id => Some((name.clone(), schema.clone())),
        _ => None,
    })
}

fn is_variable_widget(widgets: &[Widget], id: &str) -> bool {
    widgets.iter().any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Variable { .. }))
}

pub(crate) fn boundary_variable_widget_ids(selected: &BTreeSet<String>, crossing: &[SynapseSpec], widgets: &[Widget]) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for synapse in crossing {
        let from_selected = selected.contains(&synapse.from);
        let to_selected = selected.contains(&synapse.to);
        if to_selected && !from_selected && is_variable_widget(widgets, &synapse.to) {
            ids.insert(synapse.to.clone());
        }
        if from_selected && !to_selected && is_variable_widget(widgets, &synapse.from) {
            ids.insert(synapse.from.clone());
        }
    }
    ids
}

pub(crate) fn unique_generated_boundary_name(prefix: &str, serial: &mut usize, used: &BTreeSet<String>) -> String {
    loop {
        *serial += 1;
        let candidate = format!("{prefix}{serial}");
        if !used.contains(&candidate) {
            return candidate;
        }
    }
}

fn dictionary_schema_at_port(output: &Dictionary, port: &str) -> Option<String> {
    let dict = if port.is_empty() { output.clone() } else { output.get(port).and_then(|value| value.as_dictionary()).cloned()? };
    dict.schema().map(str::to_string)
}

pub(crate) fn infer_port_schema(outputs: &HashMap<String, Dictionary>, kind_infos: &HashMap<String, OperatorInfo>, widgets: &[Widget], synapses: &[SynapseSpec], widget_id: &str, port: &str) -> String {
    if let Some(schema) = outputs.get(widget_id).and_then(|out| dictionary_schema_at_port(out, port)) {
        return schema;
    }
    if let Some(widget) = widgets.iter().find(|entry| widget_id_for(entry) == widget_id) {
        if let Widget::Variable { schema, .. } = widget {
            if !schema.is_empty() {
                return schema.clone();
            }
        }
        let (input_ports, output_ports, _, _) = widget_io_ports(widget, synapses, kind_infos);
        let port_spec = output_ports.iter().find(|entry| entry.id == port).or_else(|| input_ports.iter().find(|entry| entry.id == port));
        if let Some(spec) = port_spec {
            if let Some(value_type) = &spec.value_type {
                if let Some(first) = value_type.split(',').next() {
                    if !first.is_empty() && first != "value" {
                        return first.to_string();
                    }
                }
            }
        }
    }
    "dictionary".into()
}

/// 🧩️ Built-in "core.*" neuron kinds recognized when reconstructing a widget from an evaluated neuron.
enum CoreNeuronKind {
    Number,
    Text,
    Image,
    Variable,
}

impl CoreNeuronKind {
    fn parse(kind: &str) -> Option<Self> {
        match kind {
            "core.number" => Some(Self::Number),
            "core.text" => Some(Self::Text),
            "core.image" => Some(Self::Image),
            "core.variable" => Some(Self::Variable),
            _ => None,
        }
    }
}

pub(crate) fn neuron_to_exploded_widget(neuron: &Neuron) -> Widget {
    match neuron.kind.as_str() {
        INPUT_KIND => {
            Widget::Variable { id: neuron.id.clone(), name: neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).into(), schema: boundary_schema_from_params(&neuron.params) }
        }
        OUTPUT_KIND => {
            Widget::Variable { id: neuron.id.clone(), name: neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).into(), schema: boundary_schema_from_params(&neuron.params) }
        }
        kind => match CoreNeuronKind::parse(kind) {
            Some(CoreNeuronKind::Number) => {
                Widget::InputSlider { id: neuron.id.clone(), value: neuron.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).unwrap_or(3.0), min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP }
            }
            Some(CoreNeuronKind::Text) => Widget::InputNote { id: neuron.id.clone(), text: neuron.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("").into() },
            Some(CoreNeuronKind::Image) => Widget::InputImage { id: neuron.id.clone(), src: neuron.params.get("dataUrl").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("").into() },
            Some(CoreNeuronKind::Variable) => Widget::Variable {
                id: neuron.id.clone(),
                name: neuron.params.get("name").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("value").into(),
                schema: neuron.params.get("schema").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or("dictionary").into(),
            },
            None => Widget::Neuron { id: neuron.id.clone(), neuron_kind: neuron.kind.clone(), params: neuron.params.clone(), input_ports: vec![], output_ports: vec![], preview: neuron.kind != INPUT_KIND && neuron.kind != OUTPUT_KIND },
        },
    }
}

pub(crate) fn build_channel_eval_json(fixture: &FlowFixture, channels: &EvalChannels, kind_infos: &HashMap<String, OperatorInfo>) -> String {
    let mut widgets = serde_json::Map::new();
    for widget in &fixture.widgets {
        let id = widget_id_for(widget);
        let operator_info = widget_operator_info(widget, kind_infos);
        let kind_info = operator_info.as_ref();
        let input_dict = match widget {
            Widget::Neuron { params, .. } => channels.inputs.get(id).cloned().unwrap_or_default().merge(params),
            _ => channels.inputs.get(id).cloned().unwrap_or_default(),
        };
        let output_dict = channels.outputs.get(id);
        let mut entry = serde_json::Map::new();
        entry.insert("in".into(), serde_json::Value::Object(input_ports_json(&input_dict, kind_info)));
        entry.insert("out".into(), serde_json::Value::Object(output_dict.map(output_ports_json).unwrap_or_default()));
        if let Some(output) = output_dict {
            if let Some(error) = output.get("error").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
                entry.insert("error".into(), serde_json::Value::String(error.to_string()));
            }
        }
        widgets.insert(id.to_string(), serde_json::Value::Object(entry));
    }
    serde_json::to_string(&widgets).unwrap_or_else(|_| "{}".into())
}

fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() {
        return false;
    }
    if ["vertex-", "edge-", "wire-", "face-", "shell-", "solid-", "compound-", "curve-", "surface-"].iter().any(|prefix| handle.starts_with(prefix)) {
        return true;
    }
    // Blake3 hex digests minted by `BrepKernel::mint` (no kind prefix).
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

fn collect_geometry_handles_from_value(value: &NeuralValue, handles: &mut Vec<String>) {
    if let Some(dict) = value.as_dictionary() {
        collect_geometry_handles_from_dictionary(dict, handles);
    }
}

pub(crate) fn collect_geometry_handles_from_dictionary(dict: &Dictionary, handles: &mut Vec<String>) {
    if let Some(handle) = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
        if is_brep_geometry_handle(handle) {
            handles.push(handle.to_string());
        }
    }
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            collect_geometry_handles_from_value(value, handles);
        }
    }
}

pub(crate) fn collect_live_geometry_handles_from_channels(channels: &EvalChannels) -> Vec<String> {
    let mut handles = Vec::new();
    for dict in channels.outputs.values().chain(channels.inputs.values()) {
        collect_geometry_handles_from_dictionary(dict, &mut handles);
    }
    handles.sort();
    handles.dedup();
    handles
}

fn collect_drawing_handles_from_value(value: &NeuralValue, handles: &mut Vec<String>) {
    if let Some(dict) = value.as_dictionary() {
        collect_drawing_handles_from_dictionary(dict, handles);
    }
}

fn collect_drawing_handles_from_dictionary(dict: &Dictionary, handles: &mut Vec<String>) {
    if let Some(handle) = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
        if handle.starts_with("drawing-") {
            handles.push(handle.to_string());
        }
    }
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            collect_drawing_handles_from_value(value, handles);
        }
    }
}

pub(crate) fn collect_live_drawing_handles_from_channels(channels: &EvalChannels) -> Vec<String> {
    let mut handles = Vec::new();
    for dict in channels.outputs.values().chain(channels.inputs.values()) {
        collect_drawing_handles_from_dictionary(dict, &mut handles);
    }
    handles.sort();
    handles.dedup();
    handles
}

pub(crate) fn is_global_eval_error_json(json: &str) -> bool {
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json) else {
        return true;
    };
    let Some(object) = parsed.as_object() else {
        return true;
    };
    object.len() == 1 && object.contains_key("error")
}
// #endregion 🔖️ChannelEval
