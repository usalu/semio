//! 🧱️ Learning program and geometry ownership of the procedural module: the authored program, the flow widget helpers and the geometry handle owner. See [`super`].

use super::*;
use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::sqlite::snapshot::text;

//#region 🔖️Constants
/// 🧱️ The authored hexagonal-column learning program consumed by this playbook module.
pub(super) const HEX_COLUMN_LEARNING_PROGRAM_JSON: &str = r#"{
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
//#endregion 🔖️Constants

//#region 🔖️Payload
pub(super) fn learning_program_json(slug: &str) -> Option<&'static str> {
    match slug {
        "hexagonal-mushroom-column" => Some(HEX_COLUMN_LEARNING_PROGRAM_JSON),
        _ => None,
    }
}

pub(super) fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

pub(super) fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => Value::Number(*number).to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

pub(super) fn ui_admit<T, E>(result: Result<T, E>) -> UiAssemblyResult<T> {
    result.map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "procedural module UI admission failed"))
}

pub(super) fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> {
    ui_admit(Label::try_from(value.as_ref()))
}

pub(super) fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural module text admission failed"))
}

pub(super) fn text_node(value: impl AsRef<str>) -> UiAssemblyResult<BuiltNode> {
    ui_admit(text(ui_label(value)?).try_build())
}

pub(super) fn value_text(value: &str) -> UiAssemblyResult<UiValue> {
    Ok(UiValue::Text(ui_text(value)?))
}

pub(super) fn value_map(entries: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut entries: Vec<_> = entries.into_iter().collect();
    entries.sort_unstable_by(|left, right| left.0.cmp(right.0));
    let mut map = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural module action map admission failed"))?;
    for (key, value) in entries {
        ui_admit(map.push(key.into(), value))?;
    }
    Ok(UiValue::Map(map.finish()))
}

pub(super) fn bind_control(builder: impl Into<BuiltNode>, id: &str, label: &str, payload: &ModuleRenderPayload, command: &str, args: UiValue, trigger: Trigger) -> UiAssemblyResult<BuiltNode> {
    let mut node = builder.into();
    node.key = ui_text(id)?;
    node.accessibility.label = Some(ui_label(label)?);
    node.disabled = !payload.interactive;
    let action = UiActionId::try_v1(&payload.controller_id, command).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural module action admission failed"))?;
    ui_admit(node.bindings.try_push(ActionBinding { trigger, action, args: Some(args), capability: None }))?;
    Ok(node)
}
//#endregion 🔖️Payload

//#region 🔖️Preview
pub(super) fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

pub(super) fn is_brep_geometry_handle(handle: &str) -> bool {
    handle.len() == 64 && handle.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for (_, entry) in map.iter() {
                collect_geometry_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

pub(super) fn geometry_handle_for_widget(eval: &Value, widget_id: &str) -> Option<String> {
    let widget_eval = eval.get(widget_id)?;
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"))?;
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles.into_iter().next()
}

pub(super) fn apply_flow_params(host: &mut FlowHost, host_snapshot: &FlowHostSnapshot, params: &Value) {
    let Some(object) = params.as_object() else {
        return;
    };
    for (key, value) in object {
        if let Some(number) = value.as_f64() {
            host.set_slider_value(key, number);
        }
    }
    for widget in &host_snapshot.widgets {
        if let Widget::Neuron { id, .. } = widget {
            if let Some(params) = object.get(id).filter(|value| value.as_object().is_some()) {
                let _ = host.set_neuron_params(id, &json_to_string(params));
            }
        }
    }
}

/// 🌐️ Concrete geometry and operator authority retained by one procedural module instance.
pub(super) struct ModuleGeometryOwner {
    pub(super) session: SessionCapture,
    pub(super) registry: Option<SharedRegistry>,
    pub(super) registry_retirement: RegistryRetirement,
    closing: bool,
    pub(super) imported: Option<ModuleImportedGeometry>,
    stale: Vec<ModuleStaleGeometry>,
    retirement: ValueRetirement,
}

pub(super) struct ModuleImportedGeometry { pub(super) source: String, pub(super) handles: Vec<String> }

enum ModuleStaleGeometry { Text(String), Handles(Vec<String>) }

impl ModuleGeometryOwner {
    pub(super) fn new() -> Self {
        let session = Session::new();
        let mut registry = Registry::new();
        semio_s_plugin_flow_math::register(&mut registry);
        semio_s_plugin_flow_brep::register(&mut registry, &session);
        registry.finalize();
        let (registry, registry_retirement) = SharedRegistry::new(registry);
        Self { session: session.capture(), registry: Some(registry), registry_retirement, closing: false, imported: None, stale: Vec::new(), retirement: ValueRetirement::default() }
    }

    pub(super) fn retire_import_cache(&mut self) {
        if let Some(cache) = self.imported.take() {
            self.stale.push(ModuleStaleGeometry::Handles(cache.handles));
            self.stale.push(ModuleStaleGeometry::Text(cache.source));
        }
    }

    fn admit_stale(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        match self.stale.pop().expect("observed stale geometry owner") {
            ModuleStaleGeometry::Text(value) => self.retirement.text(value, grant).map_err(|(error, value)| { self.stale.push(ModuleStaleGeometry::Text(value)); error }),
            ModuleStaleGeometry::Handles(value) => self.retirement.push_strings(value, grant).map_err(|(error, value)| { self.stale.push(ModuleStaleGeometry::Handles(value)); error }),
        }
    }

    fn stale_demand(&self) -> RetirementDemand {
        RetirementDemand { capacity_bytes: ValueRetirement::domain_frame_birth_bytes(), depth: 1, ..Default::default() }
    }

    fn retirement_cursor_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: self.retirement.next_copy_byte_demand()?, capacity_bytes: self.retirement.next_capacity_byte_demand(body)?, release_bytes: self.retirement.next_release_byte_demand()?, depth: self.retirement.next_depth_demand()? })
    }

    pub(super) fn retain_current(&self, handles: &[String]) {
        let mut retained = handles.to_vec();
        if let Some(cache) = &self.imported {
            retained.extend(cache.handles.iter().cloned());
        }
        self.session.retain_geometry_handles(&retained);
    }

    pub(super) fn host(&self, snapshot: FlowHostSnapshot) -> Result<FlowHost, Fault> {
        let registry = self.registry.as_ref().ok_or_else(|| Fault::from("playbook.module.procedural.geometry-closed"))?;
        Ok(FlowHost::from_host_snapshot(snapshot).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY).with_operator_registry(registry.clone()).with_geometry_port(self.session.port()))
    }
}

impl ArtifactInstanceOperationOwner for ModuleGeometryOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing { return Ok(if self.stale.is_empty() { RetirementDemand::default() } else { self.stale_demand() }); }
        if !self.stale.is_empty() { return Ok(self.stale_demand()); }
        if !self.retirement.terminal_is_empty() { return self.retirement_cursor_demand(body); }
        if !self.registry_retirement.terminal_is_empty() {
            return Ok(RetirementDemand { copy_bytes: self.registry_retirement.next_copy_byte_demand()?, capacity_bytes: self.registry_retirement.next_capacity_byte_demand(body)?, release_bytes: self.registry_retirement.next_release_byte_demand()?, depth: self.registry_retirement.next_depth_demand()? });
        }
        Ok(RetirementDemand { copy_bytes: self.session.next_close_copy_byte_demand()?, capacity_bytes: self.session.next_close_capacity_byte_demand(body)?, release_bytes: self.session.next_close_release_byte_demand()?, depth: self.session.next_close_depth_demand()? })
    }

    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if !self.stale.is_empty() { return Ok(PluginLifecycleStep::Progress(self.admit_stale(grant)?)); }
        if self.retirement.terminal_is_empty() { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        let step = self.retirement.close_step(grant)?;
        Ok(PluginLifecycleStep::Progress(step.progress()))
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if !self.closing {
            self.closing = true;
            drop(self.registry.take());
            self.retire_import_cache();
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if !self.stale.is_empty() { return Ok(PluginLifecycleStep::Progress(self.admit_stale(grant)?)); }
        if !self.retirement.terminal_is_empty() { return self.maintenance_step(cx,grant); }
        let step = if !self.registry_retirement.terminal_is_empty() {
            self.registry_retirement.close_step(grant)
        } else {
            self.session.close_step(grant)
        }
        .and_then(|step| semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, self.registry_retirement.terminal_is_empty() && self.session.terminal_is_empty(), "original procedural geometry owner"))
        ?;
        Ok(PluginLifecycleStep::retained(step, self.terminal_is_empty()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.registry.is_none() && self.imported.is_none() && self.stale.is_empty() && self.retirement.terminal_is_empty() && self.registry_retirement.terminal_is_empty() && self.session.terminal_is_empty()
    }
}
//#endregion 🔖️Preview
