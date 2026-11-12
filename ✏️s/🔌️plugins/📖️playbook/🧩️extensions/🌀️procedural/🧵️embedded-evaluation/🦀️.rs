//! 🧵️ Caller-funded flow evaluation and tessellation for embedded questions.
use super::*;

/// 🧭️ Maps the retained operator refusal to its original turn failure, keeping the already paid receipt.
fn operator_turn_failure(failure: neural_engine::OperatorWorkFailure) -> OriginalTurnFailure {
    match failure {
        neural_engine::OperatorWorkFailure::Original(original) => original,
        neural_engine::OperatorWorkFailure::Evaluation { primary, retained, receipt_refusal } => {
            let mut turn = OriginalTurnFailure::from(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, primary.to_string()));
            turn.effects = retained;
            turn.receipt_refusal = receipt_refusal;
            turn
        }
    }
}

/// 🧭️ Maps the native geometry refusal to its original turn failure, keeping its physical effects and receipt refusal.
fn geometry_turn_failure(failure: flow::geometry::GeometryWorkFailure) -> OriginalTurnFailure {
    let mut turn = OriginalTurnFailure::from(failure.primary);
    turn.effects = failure.work.retained;
    turn.receipt_refusal = failure.receipt_refusal;
    turn
}
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure};
use semio_framework_value::retained_clone::RetainedCloneStep;

const EVALUATION_PROGRESS: &[u8] = br#"{"en":"Evaluating Flow","de":"Flow wird ausgewertet"}"#;
const GEOMETRY_PROGRESS: &[u8] = br#"{"en":"Preparing Geometry","de":"Geometrie wird vorbereitet"}"#;

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct PreviewCache {
    pub key: String,
    pub rows: Vec<(String, Option<String>)>,
    pub meshes: Vec<String>,
    pub instances: Vec<String>,
    handles: Vec<(String, String)>,
}

pub(super) struct EmbeddedFlowWork {
    owner: ArtifactInstanceOperationOwnerHandle,
    host: Option<Box<FlowHost>>,
    close: flow::FlowHostCloseSlot,
    cache: Option<PreviewCache>,
    retiring_cache: Option<PreviewCache>,
    cache_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<PreviewCache>>,
    identity: Option<flow::geometry::GeometryWorkIdentity>,
    index: usize,
    phase: u8,
    spatial: bool,
    tessellating: bool,
}

impl EmbeddedFlowWork {
    pub(super) fn new(owner: ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { owner, host: None, close: Default::default(), cache: None, retiring_cache: None, cache_close: None, identity: None, index: 0, phase: 0, spatial: false, tessellating: false }
    }

    fn demand(&self, copy: usize, closing: bool) -> Result<RetirementDemand, ValueError> {
        if closing {
            if self.tessellating {
                let (_, handle) = &self.cache.as_ref().unwrap().handles[self.index];
                return self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| owner.session.cancel_tessellation_demands(self.identity.unwrap(), handle, 0.05, copy).map_err(module_value_fault)).map_err(|_| ValueError::invariant("embedded tessellation cancellation owner is unavailable"));
            }
            if self.host.is_some() { return Ok(flow::FlowHostCloseSlot::intake_demand()); }
            if !self.close.terminal_is_empty() { return self.close.next_demand(copy); }
            return preview_close_demands(self.cache.is_some() || self.retiring_cache.is_some(), &self.cache_close, copy);
        }
        if self.phase == 0 { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<FlowHost>(), depth: 1, ..Default::default() }); }
        if self.phase == 1 { return self.host.as_ref().unwrap().next_evaluation_demands(copy); }
        if self.phase == 3 && self.index < self.cache.as_ref().unwrap().handles.len() {
            let (_, handle) = &self.cache.as_ref().unwrap().handles[self.index];
            return self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| owner.session.tessellation_demands(self.identity.unwrap(), handle, 0.05, copy).map_err(module_value_fault)).map_err(|_| ValueError::invariant("embedded tessellation owner is unavailable"));
        }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
}

impl ArtifactCommandWork<ModuleApp> for EmbeddedFlowWork {
    fn tool_id(&self) -> &'static str { ACTION_EVALUATE_FLOW }

    fn extent(&self, _: &Command, _: &ModuleRenderPayload, _: &InteractionState, _: Option<&ArtifactOwnedToolJobContext<ModuleApp>>) -> Option<usize> { Some(1) }

    fn terminal_frame_release_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<Self>()) }

    fn work_demands(&self, _: &ArtifactCommandInputs<'_, ModuleApp>, copy: usize) -> Result<RetirementDemand, ValueError> { self.demand(copy, false) }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ModuleApp>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<ModuleApp>, OriginalTurnFailure> {
        let fuel_before = cx.fuel_remaining();
        if cx.is_cancelled() || cx.should_yield() { return Ok(ArtifactCommandWorkStep::Pending { stage: "flow", preview: EVALUATION_PROGRESS }); }
        if let Some(cache) = &self.cache {
            let current = self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| Ok(owner.active_preview_key == cache.key)).map_err(OriginalTurnFailure::from)?;
            if !current { return Err(ValueError::invariant("embedded flow inputs changed before completion").into()); }
        }
        match self.phase {
            0 => {
                let Command::EvaluateEmbeddedFlow { payload_json, body_key } = input.command else { return Err(ValueError::invariant("embedded flow command identity mismatch").into()); };
                if payload_json.len() > 65_536 { return Err(ValueError::invariant("embedded flow payload exceeds 65536 UTF-8 bytes").into()); }
                let payload: ModuleRenderPayload = semio_framework_pack_json::from_json_str(payload_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| ValueError::invariant("embedded flow payload is invalid"))?;
                let snapshot = if body_key == BODY_PREVIEW {
                    let source = learning_program_json(&payload.example_id).ok_or_else(|| ValueError::invariant("authored learning program is unavailable"))?;
                    semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| ValueError::invariant("authored learning program is invalid"))?
                } else {
                    if !matches!(body_key.as_str(), BODY_FLOW_PREVIEW | BODY_PROCEDURAL_PREVIEW) { return Err(ValueError::invariant("embedded flow body is invalid").into()); }
                    embedded_flow::authored_flow_snapshot(&payload).map_err(|_| ValueError::invariant("authored flow document is invalid"))?
                };
                self.spatial = body_key != BODY_FLOW_PREVIEW;
                let mut host = self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| { if owner.active_preview_key.is_empty() { owner.active_preview_key = payload_json.clone(); } owner.host(snapshot) }).map_err(OriginalTurnFailure::from)?;
                let source = host.host_snapshot.clone();
                apply_flow_params(&mut host, &source, &effective_params(&payload));
                source.retire_cold();
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: std::mem::size_of::<FlowHost>(), ..Default::default() })?;
                self.host = Some(Box::new(host));
                self.identity = Some(flow::geometry::GeometryWorkIdentity { operation: cx.operation().0, generation: cx.generation().0 });
                self.cache = Some(PreviewCache { key: payload_json.clone(), rows: Vec::new(), meshes: Vec::new(), instances: Vec::new(), handles: Vec::new() });
                self.phase = 1;
                cx.consume_fuel(1);
            }
            1 => {
                if matches!(self.host.as_mut().unwrap().evaluate_step(cx).map_err(operator_turn_failure)?, flow::FlowEvaluationStep::Complete) {
                    self.phase = 2;
                    self.index = 0;
                }
            }
            2 => {
                let host = self.host.as_ref().unwrap();
                if let Some(widget) = host.host_snapshot.widgets.get(self.index) {
                    if matches!(widget, Widget::OutputPreview { .. } | Widget::Neuron { preview: true, .. }) {
                        let id = widget_id(widget);
                        let channels = host.current_channels.as_deref().or(host.previous_channels.as_deref());
                        let dictionary = channels.and_then(|channels| {
                            if matches!(widget, Widget::OutputPreview { .. }) { channels.inputs.get(id).or_else(|| channels.outputs.get(id)) }
                            else { channels.outputs.get(id).or_else(|| channels.inputs.get(id)) }
                        });
                        let cache = self.cache.as_mut().unwrap();
                        cache.rows.push((id.into(), dictionary.and_then(channel_output)));
                        if self.spatial {
                            if let Some(dictionary) = dictionary {
                                let mut remaining = 128;
                                let handle = dictionary.iter().take(128).find_map(|(_, value)| geometry_in_value(value, 0, &mut remaining));
                                if let Some(handle) = handle { cache.handles.push((id.into(), handle)); }
                            }
                        }
                    }
                    self.index += 1;
                    cx.consume_fuel(1);
                } else { self.index = 0; self.phase = 3; }
            }
            3 => {
                let cache = self.cache.as_mut().unwrap();
                if let Some((id, handle)) = cache.handles.get(self.index) {
                    self.tessellating = true;
                    let step = self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| Ok(owner.session.tessellate_step(self.identity.unwrap(), handle, 0.05, cx))).map_err(OriginalTurnFailure::from)?.map_err(geometry_turn_failure)?;
                    match step.0 {
                        flow::geometry::GeometryStep::Ready(mesh) => {
                            let mesh_id = format!("eval-{id}");
                            cache.meshes.push(json_to_string(&semio_framework_pack_json::json!({ "id": mesh_id, "data": semio_framework_pack_json::to_json_string(&mesh) })));
                            cache.instances.push(json_to_string(&semio_framework_pack_json::json!({ "id": id, "meshId": mesh_id, "position": [0.0, 0.0, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0], "label": id, "selected": false, "hovered": false })));
                            self.tessellating = false;
                            self.index += 1;
                        }
                        flow::geometry::GeometryStep::Working { .. } => {}
                        _ => return Err(ValueError::invariant("embedded tessellation did not produce a mesh").into()),
                    }
                } else {
                    let handles = cache.handles.iter().map(|(_, handle)| handle.clone()).collect::<Vec<_>>();
                    self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| { owner.retain_current(&handles); Ok(()) }).map_err(OriginalTurnFailure::from)?;
                    self.phase = 4;
                }
            }
            4 => {
                let cache = self.cache.take().unwrap();
                self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| {
                    self.retiring_cache = owner.preview.replace(cache);
                    owner.preview_phase = 3;
                    Ok(())
                }).map_err(OriginalTurnFailure::from)?;
                self.phase = 5;
                return Ok(ArtifactCommandWorkStep::Complete(Emit { ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Emit::default() }));
            }
            _ => return Err(ValueError::invariant("embedded flow evaluation already completed").into()),
        }
        if let Some(cache) = &self.cache {
            self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| {
                if owner.active_preview_key == cache.key { owner.preview_phase = if self.phase >= 3 { 2 } else { 1 }; if cx.fuel_remaining() < fuel_before { owner.preview_units += 1; } }
                Ok(())
            }).map_err(OriginalTurnFailure::from)?;
        }
        Ok(ArtifactCommandWorkStep::Progress { stage: if self.phase == 3 { "tessellation" } else { "evaluation" }, preview: if self.phase == 3 { GEOMETRY_PROGRESS } else { EVALUATION_PROGRESS } })
    }

    fn begin_close(&mut self) {
        if self.phase < 5 {
            if let Some(cache) = &self.cache { let _ = self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| { if owner.active_preview_key == cache.key { owner.preview_phase = 4; } Ok(()) }); }
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demand(0, true)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demand(copy, true)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demand(0, true)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demand(0, true)?.depth) }

    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        let progress = if self.tessellating {
            let (_, handle) = &self.cache.as_ref().unwrap().handles[self.index];
            let step = self.owner.with_mut::<ModuleGeometryOwner, _>(|owner| Ok(owner.session.cancel_tessellation_step(self.identity.unwrap(), handle, 0.05, cx))).map_err(OriginalTurnFailure::from)?.map_err(geometry_turn_failure)?;
            if matches!(step.0, RetainedCloneStep::Complete(_)) { self.tessellating = false; }
            return Ok(InteractiveJobCloseStep::Pending { progress: step.1.retained });
        } else if self.host.is_some() {
            self.close.retain(&mut self.host, grant)?
        } else if !self.close.terminal_is_empty() {
            funded(cx, grant, |effect| self.close.close_step(effect).map(|step| step.progress()))?
        } else {
            let source = if self.cache.is_some() { &mut self.cache } else { &mut self.retiring_cache };
            funded(cx, grant, |effect| close_preview_cache(source, &mut self.cache_close, effect))?
        };
        if progress != RetainedCloneProgress::default() { cx.consume_fuel(1); }
        cx.consume_retained(progress)?;
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } })
    }

    fn terminal_is_empty(&self) -> bool { self.host.is_none() && self.close.terminal_is_empty() && self.cache.is_none() && self.retiring_cache.is_none() && self.cache_close.is_none() && !self.tessellating }
}

pub(super) fn render_cached_preview(owner: &mut ModuleGeometryOwner, payload: &ModuleRenderPayload, body_key: &str, labels: &ModuleLabels) -> UiAssemblyResult<BuiltNode> {
    let key = semio_framework_pack_json::to_json_string(payload);
    if owner.active_preview_key != key { owner.active_preview_key = key.clone(); owner.preview_phase = 0; owner.preview_units = 0; }
    let cache = owner.preview.as_ref().filter(|cache| cache.key == key);
    let busy = matches!(owner.preview_phase, 1 | 2);
    let mut body = column();
    let child = ModuleRenderPayload { controller_id: MODULE_APP_ID.into(), interactive: !busy, ..payload.clone() };
    body = ui_admit(body.try_child(bind_control(button(ui_label(labels.run_flow.as_str())?), "embedded-flow.run", labels.run_flow.as_str(), &child, ACTION_EVALUATE_FLOW, value_map([("payloadJsonChunks", ui_admit(UiValue::try_text_chunks(&key, 65_536))?), ("bodyKey", value_text(body_key)?)])?, Trigger::Activate)?))?;
    let child = ModuleRenderPayload { controller_id: MODULE_APP_ID.into(), interactive: busy, ..payload.clone() };
    body = ui_admit(body.try_child(bind_control(button(ui_label(labels.cancel_flow.as_str())?), "embedded-flow.cancel", labels.cancel_flow.as_str(), &child, "cancelTypedOperation", value_map([])?, Trigger::Activate)?))?;
    let status = match owner.preview_phase { 1 => labels.evaluating_flow.as_str(), 2 => labels.tessellating_flow.as_str(), 3 => labels.flow_ready.as_str(), 4 => labels.flow_stopped.as_str(), _ => labels.run_flow_hint.as_str() };
    let status = if busy { format!("{status} ({})", owner.preview_units) } else { status.to_owned() };
    if busy {
        let mut indicator: BuiltNode = semio_ui_contract::progress(owner.preview_units as f64, ui_label(&status)?).into();
        indicator.key = ui_text("embedded-flow.progress")?;
        indicator.accessibility.label = Some(ui_label(&status)?);
        body = ui_admit(body.try_child(indicator))?;
    }
    let mut status_node = text_node(status)?;
    status_node.key = ui_text("embedded-flow.status")?;
    body = ui_admit(body.try_child(status_node))?;
    if let Some(cache) = cache {
        if body_key == BODY_FLOW_PREVIEW {
            for (id, value) in &cache.rows {
                let mut row = ui_admit(field(ui_label(id)?).try_id(format!("embedded-flow.output.{id}")))?;
                row = ui_admit(row.try_child(text_node(value.as_deref().unwrap_or(labels.output_unavailable.as_str()))?))?;
                body = ui_admit(body.try_child(ui_admit(row.try_build())?))?;
            }
            if cache.rows.is_empty() { body = ui_admit(body.try_child(text_node(labels.no_outputs.as_str())?))?; }
        } else {
            let meshes = format!("[{}]", cache.meshes.join(","));
            let instances = format!("[{}]", cache.instances.join(","));
            body = ui_admit(body.try_child(scene_surface(PREVIEW_SURFACE, SurfaceKind::World3d, &world3d_scene(world3d_default_camera(), meshes, instances, world3d_selection_json("single", &[], None), &WorldSunConfig::default()))?))?;
        }
    }
    ui_admit(body.try_build())
}

fn geometry_in_value(value: &neural_engine::Value, depth: usize, remaining: &mut usize) -> Option<String> {
    if depth > embedded_flow::MAX_FLOW_DEPTH || *remaining == 0 { return None; }
    *remaining -= 1;
    let dictionary = value.as_dictionary()?;
    if let Some(handle) = dictionary.get("handle").and_then(neural_engine::Value::as_atom).and_then(neural_engine::Atom::as_str).filter(|handle| is_brep_geometry_handle(handle)) { return Some(handle.into()); }
    dictionary.iter().take(*remaining).find_map(|(_, value)| geometry_in_value(value, depth + 1, remaining))
}

fn output_json(value: &neural_engine::Value, output: &mut String, depth: usize, remaining: &mut usize) -> bool {
    use neural_engine::{Atom, Value as ChannelValue};
    if depth > embedded_flow::MAX_FLOW_DEPTH || *remaining == 0 || output.len() > 512 { return false; }
    *remaining -= 1;
    match value {
        ChannelValue::Atom(atom) => {
            let text = match atom {
                Atom::Null => "null".into(),
                Atom::Boolean(value) => value.to_string(),
                Atom::Integer(value) => value.to_string(),
                Atom::Decimal(value) => { if !value.is_finite() { return false; } value.to_string() },
                Atom::String(value) => { if value.len() > 512 { return false; } json_to_string(&Value::from(value.clone())) }
            };
            output.push_str(&text);
        }
        ChannelValue::Dictionary(dictionary) => {
            if matches!(dictionary.schema(), Some("number" | "text" | "boolean")) {
                if let Some(value) = dictionary.get("value") { return output_json(value, output, depth + 1, remaining); }
            }
            let list = dictionary.schema() == Some("list");
            if list {
                let count = dictionary.keys().filter(|key| key.as_str() != "schema").take(129).count();
                if count > 128 { return false; }
                output.push('[');
                for index in 0..count {
                    if index > 0 { output.push(','); }
                    let Some(value) = dictionary.get(&index.to_string()) else { return false; };
                    if !output_json(value, output, depth + 1, remaining) { return false; }
                }
                output.push(']');
                return output.len() <= 512;
            }
            output.push('{');
            let mut first = true;
            for (key, value) in dictionary.iter().filter(|(key, _)| key.as_str() != "schema") {
                if !first { output.push(','); }
                first = false;
                if !list {
                    if key.len() > 512 { return false; }
                    output.push_str(&json_to_string(&Value::from(key.clone())));
                    output.push(':');
                }
                if !output_json(value, output, depth + 1, remaining) { return false; }
            }
            output.push(if list { ']' } else { '}' });
        }
    }
    output.len() <= 512
}

fn channel_output(dictionary: &neural_engine::Dictionary) -> Option<String> {
    let mut remaining = 128;
    let value = if dictionary.len() == 1 { dictionary.iter().next().map(|(_, value)| value) } else { None };
    if let Some(value) = value {
        let value = value.as_dictionary().filter(|dictionary| matches!(dictionary.schema(), Some("number" | "text" | "boolean"))).and_then(|dictionary| dictionary.get("value")).unwrap_or(value);
        if let Some(text) = value.as_atom().and_then(neural_engine::Atom::as_str) { return (text.len() <= 512).then(|| text.to_owned()); }
        let mut output = String::new();
        return output_json(value, &mut output, 0, &mut remaining).then_some(output);
    }
    let mut output = String::from("{");
    for (index, (key, value)) in dictionary.iter().enumerate() {
        if index > 0 { output.push(','); }
        if key.len() > 512 { return None; }
        output.push_str(&json_to_string(&Value::from(key.clone())));
        output.push(':');
        if !output_json(value, &mut output, 0, &mut 128) { return None; }
    }
    output.push('}');
    (output.len() <= 512).then_some(output)
}

pub(super) fn preview_close_demands(original: bool, closing: &Option<semio_framework_value::retirement::controlled::ControlledRetirement<PreviewCache>>, copy: usize) -> Result<RetirementDemand, ValueError> {
    if let Some(cursor) = closing {
        if cursor.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: std::mem::size_of_val(closing), depth: 1, ..Default::default() }); }
        let mut demand = cursor.next_demand(copy)?;
        demand.depth += 1;
        return Ok(demand);
    }
    Ok(if original { RetirementDemand { copy_bytes: std::mem::size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<PreviewCache>>(), depth: 1, ..Default::default() } } else { RetirementDemand::default() })
}

/// 🪙️ Runs one close leaf inside the turn's own original effect; an unadmitted quote is zero progress that retries next turn.
pub(super) fn funded(cx: &mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant, run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError>) -> Result<RetainedCloneProgress, OriginalTurnFailure> {
    match cx.with_original_effect(grant, |_, effect| run(effect))? {
        Some((result, performed)) => { cx.consume_work(performed); Ok(result?) }
        None => Ok(Default::default()),
    }
}

pub(super) fn close_preview_cache(original: &mut Option<PreviewCache>, closing: &mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<PreviewCache>>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    let grant = effect.grant();
    let demand = preview_close_demands(original.is_some(), closing, grant.maximum_copy_bytes)?;
    if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(Default::default()); }
    if let Some(cursor) = closing {
        if cursor.terminal_is_empty() {
            *closing = None;
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
        }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = match cursor.step(grant.maximum_copy_bytes, effect) {
            semio_framework_value::RetainedWorkPoll::Pending { .. } => RetainedCloneStep::Progress(Default::default()),
            semio_framework_value::RetainedWorkPoll::Complete(turn) => turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?,
        };
        return Ok(semio_framework_value::retained_clone::admit_retained_clone_close(child, step, cursor.terminal_is_empty(), "embedded preview cache")?.progress());
    }
    if let Some(cache) = original.take() {
        *closing = Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain(cache));
        return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
    }
    Ok(Default::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_cache_close_respects_original_grants_and_reaches_terminal() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🌊️embedded-flow.json")).unwrap();
        let mut original = Some(PreviewCache { key: fixture["snapshot"].to_string(), rows: vec![("answer".into(), Some(fixture["expected"]["rows"][0]["value"].as_str().unwrap().into()))], meshes: vec!["[]".into()], instances: vec!["[]".into()], handles: Vec::new() });
        let mut closing = None;
        let mut turns = 0;
        while original.is_some() || closing.is_some() {
            let demand = preview_close_demands(original.is_some(), &closing, 32).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(32), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            let ownership = (original.is_some(), closing.is_some());
            let stalled = close_preview_cache(&mut original, &mut closing, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap();
            assert_eq!(stalled, RetainedCloneProgress::default());
            assert_eq!((original.is_some(), closing.is_some()), ownership);
            let progress = close_preview_cache(&mut original, &mut closing, grant).unwrap();
            assert!(progress.fits(grant));
            turns += 1;
            assert!(turns < 100_000, "quoted cache close must make bounded progress");
        }
        assert!(turns > 1);
    }
}

