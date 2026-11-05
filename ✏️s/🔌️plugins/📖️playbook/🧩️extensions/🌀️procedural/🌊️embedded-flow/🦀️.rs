//! 🌊️ Authored flow question controls and schema admission.
use super::*;

pub(super) const MAX_FLOW_BYTES: usize = 16_384;
pub(super) const MAX_FLOW_WIDGETS: usize = 32;
pub(super) const MAX_FLOW_SYNAPSES: usize = 64;
pub(super) const MAX_FLOW_DEPTH: usize = 32;

fn valid_depth(value: &Value, depth: usize) -> bool {
    depth <= MAX_FLOW_DEPTH && match value {
        Value::Object(object) => object.iter().all(|(_, value)| valid_depth(value, depth + 1)),
        Value::Array(array) => array.iter().all(|value| valid_depth(value, depth + 1)),
        _ => true,
    }
}

pub(super) fn authored_flow_snapshot(payload: &ModuleRenderPayload) -> UiAssemblyResult<FlowHostSnapshot> {
    let authored = params_as_json(&payload.authored_params);
    let current = params_as_json(&payload.params);
    let source = if authored.as_object().is_some() { authored.get("__flow") } else { current.get("__flow") }.ok_or_else(|| PluginAssemblyError::new("procedural.flow-missing", "an authored flow is required"))?;
    let json = source.as_str().map(str::to_owned).unwrap_or_else(|| json_to_string(source));
    if json.len() > MAX_FLOW_BYTES { return Err(PluginAssemblyError::new("procedural.flow-limit", "authored flow exceeds 16384 UTF-8 bytes")); }
    let raw = parse_json(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginAssemblyError::new("procedural.flow-invalid", error.to_string()))?;
    let invalid = || PluginAssemblyError::new("procedural.flow-invalid", "a bounded flow.host_snapshot with unique widget identities and connected synapses is required");
    let widgets = raw.get("widgets").and_then(Value::as_array).ok_or_else(invalid)?;
    let synapses = raw.get("synapses").and_then(Value::as_array).ok_or_else(invalid)?;
    if raw.get("schema").and_then(Value::as_str) != Some("flow.host_snapshot") || widgets.len() > MAX_FLOW_WIDGETS || synapses.len() > MAX_FLOW_SYNAPSES || !valid_depth(&raw, 0) { return Err(invalid()); }
    let mut ids = std::collections::BTreeSet::new();
    for widget in widgets {
        let id = widget.get("id").and_then(Value::as_str).ok_or_else(invalid)?;
        if id.is_empty() || id.len() > 128 || id == "__flow" || !ids.insert(id) || widget.get("label").and_then(Value::as_str).is_some_and(|label| label.len() > semio_ui_contract::UI_TEXT_MAX_BYTES) { return Err(invalid()); }
    }
    let mut edges = std::collections::BTreeSet::new();
    for synapse in synapses {
        let id = synapse.get("id").and_then(Value::as_str).ok_or_else(invalid)?;
        let from = synapse.get("from").and_then(Value::as_str).ok_or_else(invalid)?;
        let to = synapse.get("to").and_then(Value::as_str).ok_or_else(invalid)?;
        if id.is_empty() || id.len() > 128 || !edges.insert(id) || !ids.contains(from) || !ids.contains(to) { return Err(invalid()); }
    }
    semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginAssemblyError::new("procedural.flow-invalid", error.to_string()))
}

pub(super) fn input_initial_values(snapshot: &FlowHostSnapshot, params: &Value) -> Value {
    let mut values = Map::new();
    for widget in &snapshot.widgets {
        let (id, initial) = match widget {
            Widget::InputSlider { id, value, .. } => (id, Value::from(*value)),
            Widget::InputNote { id, text } => (id, Value::from(text.clone())),
            Widget::InputImage { id, src } => (id, Value::from(src.clone())),
            _ => continue,
        };
        values.insert(id.clone(), params.get(id).cloned().unwrap_or(initial));
    }
    Value::Object(values)
}

pub(super) fn render_flow_params(payload: &ModuleRenderPayload, labels: &ModuleLabels, parent_window: Option<(&str, &str)>) -> UiAssemblyResult<BuiltNode> {
    let params = effective_params(payload);
    let authored = params_as_json(&payload.authored_params);
    let source_params = if authored.as_object().is_some() { &authored } else { &params };
    let mut body = column();
    if payload.surface == "blueprint" {
        let source = source_params.get("__flow").map(|value| value.as_str().map(str::to_owned).unwrap_or_else(|| json_to_string(value))).unwrap_or_default();
        let mut scene = semio_plugin::TextEditorScene::base(source, Some("json".into()), None);
        scene.settings_json = Some(json_to_string(&semio_framework_pack_json::json!({
            "readOnly": !payload.interactive,
            "commit": "change",
            "editControllerId": payload.controller_id,
            "editAction": "patchQuestions",
            "editArgument": "valueJsonChunks",
            "editEncoding": "json-chunks",
            "editMaximumBytes": BLUEPRINT_VALUE_MAXIMUM_BYTES,
            "editArguments": { "questionIds": [payload.question_id], "field": "param", "paramKey": "__flow", "key": payload.question_id },
            "label": labels.flow_document.as_str()
        })));
        let editor = scene_surface("embedded-flow.source", SurfaceKind::TextEditor, &scene)?;
        let wrapper = ui_admit(field(ui_label(labels.flow_document.as_str())?).try_id("embedded-flow.source.field"))?;
        body = ui_admit(body.try_child(ui_admit(ui_admit(wrapper.try_child(editor))?.try_build())?))?;
    }
    let snapshot = match authored_flow_snapshot(payload) {
        Ok(snapshot) => snapshot,
        Err(_) => {
            let label = if source_params.get("__flow").is_some() { labels.flow_document_invalid.as_str() } else { labels.flow_document_hint.as_str() };
            return ui_admit(ui_admit(body.try_child(text_node(label)?))?.try_build());
        }
    };
    let initial = input_initial_values(&snapshot, &params);
    let input_nodes = snapshot.widgets.iter().filter_map(|widget| match widget {
        Widget::InputSlider { .. } => Some(2),
        Widget::InputNote { .. } => Some(4),
        Widget::InputImage { .. } => Some(4),
        _ => None,
    }).sum::<usize>();
    if 1 + usize::from(payload.surface == "blueprint") * 4 + input_nodes > semio_ui_contract::TREE_WINDOW_BODY_NODE_BUDGET || json_to_string(&initial).len() > 16_384 {
        snapshot.retire_cold();
        return ui_admit(ui_admit(body.try_child(text_node(labels.flow_inputs_limit.as_str())?))?.try_build());
    }
    for widget in &snapshot.widgets {
        let (id, label, kind, range) = match widget {
            Widget::InputSlider { id, label, min, max, step, .. } => (id.as_str(), label.as_str(), "slider", Some((*min, *max, *step))),
            Widget::InputNote { id, .. } => (id.as_str(), id.as_str(), "longText", None),
            Widget::InputImage { id, .. } => (id.as_str(), id.as_str(), "text", None),
            _ => continue,
        };
        let mut question = empty_flow_question();
        question.id = id.into();
        question.label = label.into();
        question.kind = kind.into();
        if let Some((min, max, step)) = range { question.min = Some(min); question.max = Some(max); question.step = Some(step); }
        body = ui_admit(body.try_child(render_question_control(&question, initial.get(id).unwrap(), payload, parent_window, Some(&initial))?))?;
    }
    snapshot.retire_cold();
    ui_admit(body.try_build())
}

fn empty_flow_question() -> PlaybookBlock {
    PlaybookBlock { id: String::new(), label: String::new(), kind: "text".into(), description: None, required: None, placeholder: None, default: None, min: None, max: None, step: None, unit: None, text: None, options: None, fields: None, schema: None, src: None, accept: None, example_id: None, params: None, condition: None }
}
