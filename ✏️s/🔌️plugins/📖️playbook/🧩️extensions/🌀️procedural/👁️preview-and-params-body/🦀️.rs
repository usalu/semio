//! 👁️ Preview and parameters of the procedural module: the evaluated preview payload and body, the media buttons and the parameter controls. See [`super`].

use super::*;

//#region 🔖️Preview
pub(super) fn evaluated_preview_payload(owner: &mut ModuleGeometryOwner, host_snapshot: &FlowHostSnapshot, params: &Value) -> (String, String) {
    let mut host = match owner.host(host_snapshot.clone()) {
        Ok(host) => host,
        Err(_) => return ("[]".into(), "[]".into()),
    };
    apply_flow_params(&mut host, host_snapshot, params);
    let evaluated = host.evaluate();
    let eval_json = evaluated.unwrap_or_default();
    let eval: Value = parse_json(&eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::json!({}));
    let mut preview_handles = Vec::new();
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &host_snapshot.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let Some(handle) = geometry_handle_for_widget(&eval, &id) else {
            continue;
        };
        preview_handles.push(handle.clone());
        let mesh_id = format!("eval-{id}");
        if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            let tessellated = owner.session.tessellate_geometry(&handle, 0.05);
            if let Ok(data) = tessellated {
                meshes.push(semio_framework_pack_json::json!({ "id": mesh_id, "data": data }));
            }
        }
        if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            instances.push(semio_framework_pack_json::json!({
                "id": id,
                "meshId": mesh_id,
                "position": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
                "selected": false,
                "hovered": false,
            }));
        }
    }
    if let Ok(handles) = imported_geometry_handles(owner, params) {
        for (index, handle) in handles.iter().enumerate() {
            if let Ok(data) = owner.session.tessellate_geometry(handle, 0.05) {
                let id = format!("import-{index}");
                meshes.push(semio_framework_pack_json::json!({ "id": id, "data": data }));
                instances.push(semio_framework_pack_json::json!({ "id": id, "meshId": id, "position": [0.0, 0.0, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0], "label": id, "selected": false, "hovered": false }));
            }
        }
    }
    owner.retain_current(&preview_handles);
    host.retire_cold();
    if meshes.is_empty() {
        let fallback = semio_framework_pack_json::json!([{ "id": PREVIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(PREVIEW_FALLBACK_MESH_KIND) }]);
        let fallback_instances = semio_framework_pack_json::json!([{
            "id": "preview",
            "meshId": PREVIEW_FALLBACK_MESH_KIND,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": "preview",
            "selected": false,
            "hovered": false,
        }]);
        return (json_to_string(&fallback), json_to_string(&fallback_instances));
    }
    (json_to_string(&Value::Array(meshes)), json_to_string(&Value::Array(instances)))
}

pub(super) fn render_preview_body(owner: &mut ModuleGeometryOwner, payload: &ModuleRenderPayload) -> UiAssemblyResult<BuiltNode> {
    let slug = if payload.example_id.is_empty() { "hexagonal-mushroom-column" } else { payload.example_id.as_str() };
    let Some(snapshot_json) = learning_program_json(slug) else {
        return text_node(format!("Unknown learning example: {slug}"));
    };
    let host_snapshot: FlowHostSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| FlowHostSnapshot::default());
    let params = params_as_json(&payload.params);
    let (meshes_json, instances_json) = evaluated_preview_payload(owner, &host_snapshot, &params);
    host_snapshot.retire_cold();
    scene_surface(PREVIEW_SURFACE, SurfaceKind::World3d, &world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("single", &[], None), &WorldSunConfig::default()))
}
//#endregion 🔖️Preview

//#region 🔖️MediaExport
/// 🧵️ Collects every distinct brep geometry handle exposed by the learning program's preview-flagged widgets, evaluated against the current param overrides — same eval pass as `evaluated_preview_payload`, minus the tessellation step.
pub(super) fn evaluated_preview_geometry_handles(owner: &mut ModuleGeometryOwner, host_snapshot: &FlowHostSnapshot, params: &Value) -> Vec<String> {
    let mut host = match owner.host(host_snapshot.clone()) {
        Ok(host) => host,
        Err(_) => return Vec::new(),
    };
    apply_flow_params(&mut host, host_snapshot, params);
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = parse_json(&eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::json!({}));
    let mut handles: Vec<String> = Vec::new();
    for widget in &host_snapshot.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        if let Some(handle) = geometry_handle_for_widget(&eval, &id) {
            if !handles.contains(&handle) {
                handles.push(handle);
            }
        }
    }
    owner.retain_current(&handles);
    host.retire_cold();
    handles
}

fn media_button(payload: &ModuleRenderPayload, format: &str, import: bool) -> UiAssemblyResult<BuiltNode> {
    let verb = if import { "Import" } else { "Export" };
    let icon = if import { "import" } else { "export" };
    let label = format!("{verb} {}", format.to_uppercase());
    bind_control(
        button(ui_label(&label)?).icon(ui_text(icon)?),
        &format!("playbook-module.{icon}.{format}"),
        &label,
        payload,
        if import { ACTION_IMPORT_SOLID } else { ACTION_EXPORT_SOLID },
        value_map([("format", value_text(format)?)])?,
        Trigger::Activate,
    )
}

pub(super) fn render_question_control(question: &PlaybookBlock, value: &Value, payload: &ModuleRenderPayload, parent_window: Option<(&str, &str)>) -> UiAssemblyResult<BuiltNode> {
    let key = question.id.as_str();
    let mut ids = UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural question ids admission failed"))?;
    ui_admit(ids.push(value_text(&payload.question_id)?))?;
    let blueprint = payload.surface == "blueprint";
    let mut args = vec![("questionIds", UiValue::List(ids.finish())), ("field", value_text(if blueprint { "param" } else { "tryParam" })?), ("paramKey", value_text(key)?), ("key", value_text(&payload.question_id)?)];
    if let Some((id, kind)) = parent_window {
        args.extend([("windowId", value_text(id)?), ("windowKindId", value_text(kind)?)]);
    }
    let args = value_map(args)?;
    let control: BuiltNode = match question.kind.as_str() {
        "text" | "longText" | "number" => {
            let kind = match question.kind.as_str() {
                "longText" => InputKind::LongText,
                "number" => InputKind::Number,
                _ => InputKind::Text,
            };
            let mut input = input(kind).value(ui_text(json_string_value(value))?);
            if let Some(placeholder) = &question.placeholder {
                input = input.placeholder(ui_label(placeholder)?);
            }
            input.into()
        }
        "slider" => slider(json_f64_value(value)).min(question.min.unwrap_or(0.0)).max(question.max.unwrap_or(100.0)).step(question.step.unwrap_or(1.0)).into(),
        "boolean" => toggle(value.as_bool().unwrap_or(false)).icon(ui_text("check")?).into(),
        _ => return text_node(format!("Unsupported param kind: {}", question.kind)),
    };
    let control = bind_control(control, &format!("playbook-module.{key}.input"), &question.label, payload, if blueprint { "patchQuestions" } else { "setTryValue" }, args, Trigger::Change)?;
    let field = ui_admit(field(ui_label(&question.label)?).try_id(format!("playbook-module.{key}")))?;
    ui_admit(ui_admit(field.try_child(control))?.try_build())
}

pub(super) fn render_params_body(payload: &ModuleRenderPayload, labels: &ModuleLabels, parent_window: Option<(&str, &str)>, embedded: bool) -> UiAssemblyResult<BuiltNode> {
    let slug = if payload.example_id.is_empty() { "hexagonal-mushroom-column" } else { payload.example_id.as_str() };
    let Some(snapshot_json) = learning_program_json(slug) else {
        return text_node(format!("Unknown learning example: {slug}"));
    };
    let host_snapshot: FlowHostSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginAssemblyError::new("procedural.learning-program", error.to_string()))?;
    let spec = flow_host_snapshot_to_form_spec(&host_snapshot, &PlaybookFlowFormProducer);
    host_snapshot.retire_cold();
    let values: Map = params_as_json(&payload.params).as_object().cloned().unwrap_or_default();
    let Some(step) = spec.steps.first() else {
        return text_node(labels.no_flow_inputs.as_str());
    };
    let values_dsl: semio_framework_artifact_playbook_playbook::PlaybookValues = values.iter().map(|(key, value)| (key.to_string(), json_to_dsl_value(value))).collect();
    let visible = visible_blocks(step, &values_dsl);
    semio_framework_value::FromValue::retire_decoded(values_dsl);
    let mut column = column();
    if visible.is_empty() {
        column = ui_admit(column.try_child(text_node(labels.no_procedural_parameters.as_str())?))?;
    }
    for question in visible {
        let value = values.get(&question.id).cloned().unwrap_or_else(|| question.default.as_ref().map(json_from_dsl_value).unwrap_or(Value::Null));
        column = ui_admit(column.try_child(render_question_control(question, &value, payload, parent_window)?))?;
    }
    for format in SOLID_MEDIA_FORMATS.into_iter().filter(|_| !embedded) {
        column = ui_admit(column.try_child(media_button(payload, format, false)?))?;
        column = ui_admit(column.try_child(media_button(payload, format, true)?))?;
    }
    ui_admit(column.try_build())
}

pub(super) fn embedded_payload(input: &str, body_key: &str) -> UiAssemblyResult<(ModuleRenderPayload, Option<(String, String)>)> {
    let error = || PluginAssemblyError::new("procedural.extension-input", "invalid embedded surface input");
    let props = parse_json(input, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| error())?;
    if props.get("bodyKey").and_then(Value::as_str) != Some(body_key) { return Err(error()); }
    let json = props.get("paramsJson").and_then(Value::as_str).ok_or_else(error)?;
    let payload: ModuleRenderPayload = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| error())?;
    let target = parse_json(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| error())?;
    let window = match (target.get("windowId").and_then(Value::as_str), target.get("windowKindId").and_then(Value::as_str)) {
        (None, None) => None,
        (Some(id), Some(kind)) if !id.is_empty() && !kind.is_empty() && payload.surface == "try" => Some((id.to_string(), kind.to_string())),
        _ => return Err(error()),
    };
    if payload.question_id.is_empty() || payload.controller_id.is_empty() || !matches!(payload.surface.as_str(), "blueprint" | "try") { return Err(error()); }
    Ok((payload, window))
}
//#endregion 🔖️Params
