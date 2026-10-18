//! 🗺️ DAG node conversion of the flow snapshot: the property bag conversions, the widget properties and operator kind, the widget to DAG node conversion and the port endpoint parser. See [`super`].

use super::*;

// #region 🔖️Document
/// 🌉️ `PropertyValue` is `#[serde(untagged)]`, a shape the `ToValue`/`FromValue` derive does not
/// support, so this hand-walks the `DslValue` tree `Dictionary::to_value` already produces.
fn property_value_from_dsl(value: semio_framework_value::DslValue) -> PropertyValue {
    match value {
        semio_framework_value::DslValue::Null => PropertyValue::Null,
        semio_framework_value::DslValue::Bool(b) => PropertyValue::Bool(b),
        semio_framework_value::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
        semio_framework_value::DslValue::String(s) => PropertyValue::String(s),
        semio_framework_value::DslValue::Array(items) => PropertyValue::Array(items.into_iter().map(property_value_from_dsl).collect()),
        semio_framework_value::DslValue::Object(entries) => PropertyValue::Object(entries.into_iter().map(|(key, entry)| (key, property_value_from_dsl(entry))).collect()),
        semio_framework_value::DslValue::Bytes(bytes) => PropertyValue::Array(bytes.into_iter().map(|byte| PropertyValue::Number(f64::from(byte))).collect()),
    }
}

fn property_bag_from_dictionary(dict: &Dictionary) -> PropertyBag {
    match semio_framework_value::ToValue::to_value(dict) {
        semio_framework_value::DslValue::Object(entries) => entries.into_iter().map(|(key, entry)| (key, property_value_from_dsl(entry))).collect(),
        _ => PropertyBag::default(),
    }
}

fn widget_operator_kind(widget: &Widget) -> Option<String> {
    match widget {
        Widget::Neuron { neuron_kind, .. } => Some(neuron_kind.clone()),
        Widget::InputSlider { .. } => Some("core.number".into()),
        Widget::InputNote { .. } => Some("core.text".into()),
        Widget::InputImage { .. } => Some("core.image".into()),
        Widget::Variable { .. } => Some("core.variable".into()),
        Widget::Cluster { .. } => Some(CLUSTER_KIND.into()),
        _ => None,
    }
}

fn widget_properties(widget: &Widget, kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> PropertyBag {
    match widget {
        Widget::Neuron { neuron_kind, params, output_ports, .. } => {
            let mut bag = property_bag_from_dictionary(params);
            if !output_ports.is_empty() {
                bag.insert("count".into(), PropertyValue::Number(output_ports.len() as f64));
            } else if let Some(spec) = kind_infos.get(neuron_kind).and_then(|info| info.variadic_output.as_ref()) {
                bag.insert("count".into(), PropertyValue::Number(spec.min as f64));
            }
            bag
        }
        Widget::InputSlider { value, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("value".into(), PropertyValue::Number(*value));
            bag
        }
        Widget::InputNote { text, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("value".into(), PropertyValue::String(text.clone()));
            bag
        }
        Widget::InputImage { src, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("dataUrl".into(), PropertyValue::String(src.clone()));
            bag
        }
        Widget::Variable { name, schema, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("name".into(), PropertyValue::String(name.clone()));
            bag.insert("schema".into(), PropertyValue::String(schema.clone()));
            bag
        }
        Widget::Cluster { name, tree, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("name".into(), PropertyValue::String(name.clone()));
            bag.insert("clusterTree".into(), cluster_tree_property(tree));
            bag
        }
        _ => PropertyBag::new(),
    }
}

pub fn widget_to_dag_node(widget: &Widget, index: usize, layout: &OrderedMap<WidgetLayout>, synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>, size: (f64, f64)) -> DagNodeSpec {
    let id = match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id.clone(),
    };
    let (width, height) = size;
    let (x, y) = layout.get(&id).map_or(((index as f64) * 200.0, 0.0), |p| (p.x, p.y));
    let (name, abbreviation, icon) = widget_display_meta(widget, kind_infos);
    let mut node = match widget {
        Widget::Neuron { id: neuron_id, neuron_kind, params, input_ports, output_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) = neuron_io_layout(neuron_id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos);
            DagNodeSpec::computation(id, &name, &abbreviation, icon, inputs, outputs, variadic_inputs, variadic_outputs, x, y, width, height)
        }
        Widget::InputSlider { value, min, max, step, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Slider { min: *min, max: *max, step: *step, value: *value, output: io_port_typed(IoPortSpec::named("N", "Num", "number", "Number"), neural::VALUE_TYPE_NUMBER) },
        },
        Widget::InputNote { text, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Note { text: text.clone(), output: io_port_typed(IoPortSpec::named("T", "Txt", "text", "Text"), neural::VALUE_TYPE_TEXT) } }
        }
        Widget::InputImage { src, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Image { src: src.clone(), output: IoPortSpec::named("I", "Img", "image", "Image") } }
        }
        Widget::OutputPreview { preview, expanded, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Preview { content: dag_preview_content_from_dict(preview), expanded: expanded.iter().cloned().collect(), input: IoPortSpec::named("", "", "", "PreviewInput") },
        },
        Widget::OutputAction { action, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Action { label: action.clone(), input: IoPortSpec::named("", "", "", "ActionInput") } }
        }
        Widget::OutputExport { format, .. } => {
            let (display_name, abbreviation, _) = export_widget_display_meta(format);
            DagNodeSpec {
                id,
                name: display_name,
                abbreviation,
                icon,
                x,
                y,
                width,
                height,
                operator_kind: None,
                properties: PropertyBag::new(),
                kind: DagNodeKind::Export { label: format.to_uppercase(), format: format.clone(), input: IoPortSpec::named("", "", "", "ExportInput") },
            }
        }
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            DagNodeSpec::computation(id, name, &abbreviation, icon, inputs, outputs, false, false, x, y, width, height)
        }
        Widget::Cluster { id: cluster_id, name: cluster_name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(cluster_id, cluster_name, tree, synapses);
            DagNodeSpec::cluster(id, &name, &abbreviation, icon, inputs, outputs, x, y, width, height)
        }
    };
    node.operator_kind = widget_operator_kind(widget);
    node.properties = widget_properties(widget, kind_infos);
    node
}

pub fn parse_port_endpoint(endpoint: &str, default_port: &str) -> (String, String) {
    if let Some((widget_id, port_id)) = endpoint.split_once('@') {
        return (widget_id.to_string(), port_id.to_string());
    }
    (endpoint.to_string(), default_port.to_string())
}
// #endregion 🔖️Document
