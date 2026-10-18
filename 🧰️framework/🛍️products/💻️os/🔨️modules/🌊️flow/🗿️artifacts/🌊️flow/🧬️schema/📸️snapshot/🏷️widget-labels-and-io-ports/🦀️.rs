//! 🏷️ Widget labels and IO ports of the flow snapshot: the widget label and display meta, the variable and cluster layouts and the default neuron and widget IO ports. See [`super`].

use super::*;

// #region 🔖️Document
pub fn widget_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { neuron_kind, .. } => neuron_kind.clone(),
        Widget::InputSlider { label, .. } => label.clone(),
        Widget::InputNote { .. } => "Note".into(),
        Widget::InputImage { .. } => "Image".into(),
        Widget::Variable { name, .. } => name.clone(),
        Widget::OutputPreview { .. } => "Preview".into(),
        Widget::OutputAction { action, .. } => action.clone(),
        Widget::OutputExport { format, .. } => format.to_uppercase(),
        Widget::Cluster { name, .. } => {
            if name.is_empty() {
                "Cluster".into()
            } else {
                name.clone()
            }
        }
    }
}

pub fn widget_display_meta(widget: &Widget, kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> (String, String, String) {
    match widget {
        Widget::Neuron { neuron_kind, .. } => kind_infos.get(neuron_kind).map_or_else(
            || {
                let (name, abbreviation) = normalize_node_display(neuron_kind, neuron_kind);
                (name, abbreviation, String::new())
            },
            |info| (info.name.clone(), info.abbreviation.clone(), info.icon.clone()),
        ),
        Widget::InputSlider { label, .. } => (label.clone(), label.clone(), "emoji:🎚️".into()),
        Widget::InputNote { .. } => ("Note".into(), "Note".into(), "emoji:📝️".into()),
        Widget::InputImage { .. } => ("Image".into(), "Image".into(), "emoji:🖼️".into()),
        Widget::Variable { name, .. } => (name.clone(), name.chars().take(3).collect::<String>(), "emoji:🔣️".into()),
        Widget::OutputPreview { .. } => ("Preview".into(), "Preview".into(), "emoji:👁️".into()),
        Widget::OutputAction { action, .. } => {
            let title = if action.is_empty() { "Action" } else { action.as_str() };
            let (name, abbreviation) = normalize_node_display(title, title);
            (name, abbreviation, "emoji:⚡️".into())
        }
        Widget::OutputExport { format, .. } => export_widget_display_meta(format),
        Widget::Cluster { name, .. } => {
            let title = if name.is_empty() { "Cluster" } else { name.as_str() };
            let (display_name, abbreviation) = normalize_node_display(title, title);
            (display_name, abbreviation, "emoji:🧩️".into())
        }
    }
}

fn variable_port_label(name: &str) -> (String, String, String) {
    let code: String = name.chars().take(1).collect::<String>().to_uppercase();
    let abbrev: String = if name.len() <= 3 { name.to_string() } else { name.chars().take(3).collect() };
    let code = if code.is_empty() { "V".into() } else { code };
    let abbrev = if abbrev.is_empty() { "val".into() } else { abbrev };
    (code, abbrev, name.to_string())
}

pub fn variable_io_ports(name: &str, schema: &str) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    let (code, abbrev, full_name) = variable_port_label(name);
    let mut input = IoPortSpec::named(&code, &abbrev, name, &full_name);
    input.value_type = Some(schema.to_string());
    let mut output = IoPortSpec::named(&code, &abbrev, name, &full_name);
    output.value_type = Some(schema.to_string());
    (vec![input], vec![output])
}

pub fn cluster_io_layout(cluster_id: &str, name: &str, tree: &Tree, synapses: &[SynapseSpec]) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    let info = cluster_operator_info(cluster_id, name, tree);
    let inputs = info.inputs.iter().map(|spec| input_spec_to_port(spec, &Dictionary::new(), is_port_connected(synapses, cluster_id, &spec.name))).collect();
    let outputs = info.outputs.iter().map(channel_spec_to_output_port).collect();
    (inputs, outputs)
}

fn neural_value_to_dsl_value(value: &NeuralValue) -> semio_framework_value::DslValue {
    semio_framework_value::ToValue::to_value(value)
}

/// 🔤️ The port type a node-graph surface publishes for a channel: the DECLARED value schemas,
/// comma-joined, never the operator-id list `operators` holds. `None` is an undeclared port, which
/// `Registry::channel_compatible` leaves connectable.
fn channel_spec_value_type(spec: &ChannelSpec) -> Option<String> {
    if spec.value_types.is_empty() {
        None
    } else {
        Some(spec.value_types.join(","))
    }
}

fn is_port_connected(synapses: &[SynapseSpec], neuron_id: &str, port_id: &str) -> bool {
    synapses.iter().any(|syn| syn.to == neuron_id && syn.to_port == port_id)
}

/// 🔤️ Stamps a port type onto a hand-built widget port, so a slider, a note or an image declares the
/// same value schema a registered operator channel would.
pub(super) fn io_port_typed(mut port: IoPortSpec, value_type: &str) -> IoPortSpec {
    port.value_type = Some(value_type.to_string());
    port
}

fn channel_spec_to_output_port(spec: &ChannelSpec) -> IoPortSpec {
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary, connected: bool) -> IoPortSpec {
    let value = params.get(&spec.name).or(spec.default.as_ref()).map(neural_value_to_dsl_value);
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.value = value;
    port.connected = Some(connected);
    port.cardinality = spec.cardinality.symbol();
    port
}

pub fn default_neuron_input_ports(kind: &str, input_ports: &[String], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Vec<String> {
    default_neuron_input_ports_from_info(input_ports, kind_infos.get(kind))
}

pub(super) fn default_neuron_input_ports_from_info(input_ports: &[String], kind_info: Option<&OperatorInfo>) -> Vec<String> {
    if !input_ports.is_empty() {
        return input_ports.to_vec();
    }
    if let Some(spec) = kind_info.and_then(|info| info.variadic_input.as_ref()) {
        return (0..spec.min).map(|index| index.to_string()).collect();
    }
    if let Some(info) = kind_info {
        if !info.inputs.is_empty() && info.inputs[0].name != "*" {
            return info.inputs.iter().map(|entry| entry.name.clone()).collect();
        }
    }
    vec![]
}

fn variadic_output_label(index: usize) -> String {
    if index == 0 {
        "i".into()
    } else {
        format!("i+{index}")
    }
}

pub fn default_neuron_output_ports(kind: &str, output_ports: &[String], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Vec<String> {
    default_neuron_output_ports_from_info(output_ports, kind_infos.get(kind))
}

pub(super) fn default_neuron_output_ports_from_info(output_ports: &[String], kind_info: Option<&OperatorInfo>) -> Vec<String> {
    if !output_ports.is_empty() {
        return output_ports.to_vec();
    }
    if let Some(spec) = kind_info.and_then(|info| info.variadic_output.as_ref()) {
        return (0..spec.min).map(|index| index.to_string()).collect();
    }
    vec![]
}

fn build_variadic_output_ports(neuron_kind: &str, output_ports: &[String], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Vec<IoPortSpec> {
    let ports = default_neuron_output_ports(neuron_kind, output_ports, kind_infos);
    let output_spec = kind_infos.get(neuron_kind).and_then(|info| info.outputs.first());
    ports
        .iter()
        .enumerate()
        .map(|(index, port_id)| {
            let label = variadic_output_label(index);
            let mut port = if let Some(spec) = output_spec {
                let mut entry = channel_spec_to_output_port(spec);
                entry.id = port_id.clone();
                entry.label = label.clone();
                entry.code = label.clone();
                entry.abbreviation = label;
                entry.full_name = if index == 0 { "IndexValue".into() } else { format!("IndexValuePlus{index}") };
                entry
            } else {
                IoPortSpec::simple(&label, &label)
            };
            port.id = port_id.clone();
            port
        })
        .collect()
}

fn neuron_output_ports(neuron_kind: &str, output_ports: &[String], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> (Vec<IoPortSpec>, bool) {
    let info = kind_infos.get(neuron_kind);
    let has_variadic_output = info.and_then(|entry| entry.variadic_output.as_ref()).is_some();
    let outputs = if has_variadic_output { build_variadic_output_ports(neuron_kind, output_ports, kind_infos) } else { info.map(|entry| entry.outputs.iter().map(channel_spec_to_output_port).collect::<Vec<_>>()).unwrap_or_default() };
    (outputs, has_variadic_output)
}

pub fn neuron_io_layout(
    neuron_id: &str,
    neuron_kind: &str,
    input_ports: &[String],
    output_ports: &[String],
    params: &Dictionary,
    synapses: &[SynapseSpec],
    kind_infos: &HistoryFoldIndex<String, OperatorInfo>,
) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    let info = kind_infos.get(neuron_kind);
    let (outputs, has_variadic_output) = neuron_output_ports(neuron_kind, output_ports, kind_infos);
    if let Some(_spec) = info.and_then(|entry| entry.variadic_input.as_ref()) {
        let ports = default_neuron_input_ports(neuron_kind, input_ports, kind_infos);
        let inputs = ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                IoPortSpec { id: port_id.clone(), label: port_id.clone(), connected: Some(connected), ..Default::default() }
            })
            .collect();
        return (inputs, outputs, true, has_variadic_output);
    }
    if let Some(entry) = info {
        if !entry.inputs.is_empty() && entry.inputs[0].name != "*" {
            let inputs = entry.inputs.iter().map(|spec| input_spec_to_port(spec, params, is_port_connected(synapses, neuron_id, &spec.name))).collect();
            return (inputs, outputs, false, has_variadic_output);
        }
    }
    if !input_ports.is_empty() {
        let inputs = input_ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                if let Some(spec) = info.and_then(|entry| entry.inputs.iter().find(|channel| channel.name == *port_id)) {
                    return input_spec_to_port(spec, params, connected);
                }
                let mut port = IoPortSpec::simple(port_id, port_id);
                port.connected = Some(connected);
                port
            })
            .collect();
        return (inputs, outputs, false, has_variadic_output);
    }
    (vec![], outputs, false, has_variadic_output)
}

pub fn widget_io_ports(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => neuron_io_layout(id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos),
        Widget::InputSlider { .. } => (vec![], vec![io_port_typed(IoPortSpec::named("N", "Num", "number", "Number"), neural::VALUE_TYPE_NUMBER)], false, false),
        Widget::InputNote { .. } => (vec![], vec![io_port_typed(IoPortSpec::named("T", "Txt", "text", "Text"), neural::VALUE_TYPE_TEXT)], false, false),
        Widget::InputImage { .. } => (vec![], vec![IoPortSpec::named("I", "Img", "image", "Image")], false, false),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            (inputs, outputs, false, false)
        }
        Widget::OutputPreview { .. } => (vec![IoPortSpec::named("", "", "", "PreviewInput")], vec![], false, false),
        Widget::OutputAction { .. } => (vec![IoPortSpec::named("", "", "", "ActionInput")], vec![], false, false),
        Widget::OutputExport { .. } => (vec![IoPortSpec::named("", "", "", "ExportInput")], vec![], false, false),
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            (inputs, outputs, false, false)
        }
    }
}
// #endregion 🔖️Document
