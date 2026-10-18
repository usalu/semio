//! 🧩️ Widget descriptors of the flow snapshot: the widget descriptor, its explicit and generated ids and the widget construction from a descriptor. See [`super`].

use super::*;

// #region 🔖️Document
#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum WidgetDescriptor {
    Neuron {
        #[serde(rename = "neuronKind")]
        #[value(rename = "neuronKind")]
        neuron_kind: String,
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
    },
    InputSlider {
        label: String,
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        value: Option<f64>,
        #[serde(default)]
        #[value(default)]
        min: Option<f64>,
        #[serde(default)]
        #[value(default)]
        max: Option<f64>,
        #[serde(default)]
        #[value(default)]
        step: Option<f64>,
    },
    InputNote {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        text: Option<String>,
    },
    InputImage {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
    },
    OutputPreview {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
    },
    OutputAction {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        action: String,
    },
    OutputExport {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        format: String,
    },
    Variable {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        name: Option<String>,
        #[serde(default)]
        #[value(default)]
        schema: Option<String>,
    },
}

pub fn descriptor_explicit_id(descriptor: &WidgetDescriptor) -> Option<String> {
    let id = match descriptor {
        WidgetDescriptor::Neuron { id, .. }
        | WidgetDescriptor::InputSlider { id, .. }
        | WidgetDescriptor::InputNote { id, .. }
        | WidgetDescriptor::InputImage { id }
        | WidgetDescriptor::OutputPreview { id }
        | WidgetDescriptor::OutputAction { id, .. }
        | WidgetDescriptor::OutputExport { id, .. }
        | WidgetDescriptor::Variable { id, .. } => id.clone(),
    }?;
    let trimmed = id.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn generated_widget_id<'a>(descriptor: &WidgetDescriptor, existing_ids: impl IntoIterator<Item = &'a str>) -> (String, u64) {
    let prefix = match descriptor {
        WidgetDescriptor::Neuron { neuron_kind, .. } => neuron_kind.replace('.', "_"),
        WidgetDescriptor::InputSlider { .. } => "slider".into(),
        WidgetDescriptor::InputNote { .. } => "note".into(),
        WidgetDescriptor::InputImage { .. } => "image".into(),
        WidgetDescriptor::Variable { .. } => "variable".into(),
        WidgetDescriptor::OutputPreview { .. } => "preview".into(),
        WidgetDescriptor::OutputAction { .. } => "action".into(),
        WidgetDescriptor::OutputExport { .. } => "export".into(),
    };
    let id_prefix = format!("{prefix}_");
    let used = existing_ids.into_iter().filter_map(|id| id.strip_prefix(&id_prefix)?.parse::<u64>().ok()).collect::<HashSet<_>>();
    let mut serial = 2_u64;
    while used.contains(&serial) {
        serial = serial.checked_add(1).expect("the finite Flow widget set must leave a generated identifier");
    }
    (format!("{prefix}_{serial}"), serial)
}

/// 🧾️ The params an inserted operator records (design §20.9): every declared input's default literal, in declaration
/// order, read off the operator kind at authoring time — so the record is self-describing and a fold decides an input's
/// existence and literal type from the snapshot alone, never from a runtime registry. Data inputs without a default stay
/// absent; an unknown kind records nothing.
pub fn default_neuron_params_from_info(kind_info: Option<&OperatorInfo>) -> Dictionary {
    kind_info.map_or_else(Dictionary::new, |info| info.inputs.iter().filter_map(|input| input.default.as_ref().map(|value| (input.name.as_str(), value))).fold(Dictionary::new(), |params, (name, value)| params.insert(name, value.clone())))
}

pub fn widget_from_descriptor_with_info(descriptor: &WidgetDescriptor, id: String, kind_info: Option<&OperatorInfo>) -> Widget {
    match descriptor {
        WidgetDescriptor::Neuron { neuron_kind, .. } => {
            Widget::Neuron { id, neuron_kind: neuron_kind.clone(), params: default_neuron_params_from_info(kind_info), input_ports: default_neuron_input_ports_from_info(&[], kind_info), output_ports: default_neuron_output_ports_from_info(&[], kind_info), preview: true }
        }
        WidgetDescriptor::InputSlider { label, value, min, max, step, .. } => {
            let (value, min, max, step) = resolve_input_slider_fields(*value, *min, *max, *step);
            Widget::InputSlider { id, label: label.clone(), value, min, max, step }
        }
        WidgetDescriptor::InputNote { text, .. } => Widget::InputNote { id, text: text.clone().unwrap_or_default() },
        WidgetDescriptor::InputImage { .. } => Widget::InputImage { id, src: String::new() },
        WidgetDescriptor::OutputPreview { .. } => Widget::OutputPreview { id, preview: Dictionary::new(), expanded: OrderedSet::new() },
        WidgetDescriptor::OutputAction { action, .. } => Widget::OutputAction { id, action: if action.is_empty() { "log".into() } else { action.clone() } },
        WidgetDescriptor::OutputExport { format, .. } => Widget::OutputExport { id, format: if format.is_empty() { default_export_format() } else { format.clone() } },
        WidgetDescriptor::Variable { name, schema, .. } => {
            Widget::Variable { id, name: name.clone().filter(|value| !value.trim().is_empty()).unwrap_or_else(default_variable_name), schema: schema.clone().filter(|value| !value.trim().is_empty()).unwrap_or_else(default_variable_schema) }
        }
    }
}

pub fn widget_from_descriptor(descriptor: &WidgetDescriptor, id: String, kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Widget {
    let kind_info = match descriptor {
        WidgetDescriptor::Neuron { neuron_kind, .. } => kind_infos.get(neuron_kind),
        _ => None,
    };
    widget_from_descriptor_with_info(descriptor, id, kind_info)
}
// #endregion 🔖️Document
