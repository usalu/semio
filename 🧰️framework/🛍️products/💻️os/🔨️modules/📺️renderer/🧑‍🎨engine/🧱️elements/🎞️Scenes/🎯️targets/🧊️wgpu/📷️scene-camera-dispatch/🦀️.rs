//! 📷️ Scene camera dispatch authority plus pointer edge, scroll, action and list-row helpers.

use super::*;

//#region SceneRuntime
//#region SceneCameraDispatch
pub(crate) const SCENE_CAMERA_DISPATCH_CAPACITY: usize = 256;
const SCENE_CAMERA_ID_BYTE_CAPACITY: usize = 256;

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct SceneCameraOwnerMetadata {
    document_id: ui_contract::UiNodeId,
    window_generation: u64,
    pub(crate) component_generation: u64,
    pub(crate) kind: ui_wgpu::wgpu::SurfaceKind,
    pub(crate) node: ui_wgpu::wgpu::NodeId,
    positional_key: Option<(u32, u32)>,
}

impl SceneCameraOwnerMetadata {
    fn from_owner(owner: &crate::element_contracts::ScenePointerTarget) -> Self {
        Self { document_id: owner.document_id, window_generation: owner.window_generation, component_generation: owner.component_generation, kind: owner.kind, node: owner.node, positional_key: match &owner.key { ui_wgpu::wgpu::NodeKey::Explicit(_) => None, ui_wgpu::wgpu::NodeKey::Positional(a, b) => Some((*a, *b)) } }
    }
}

type SceneCameraRecord = crate::camera_storage::Record<Option<SceneCameraOwnerMetadata>>;

pub(crate) fn camera_owner_matches(record: &SceneCameraRecord, owner: &crate::element_contracts::ScenePointerTarget) -> bool {
    let Some(metadata) = record.metadata else { return false };
    metadata.document_id == owner.document_id && metadata.window_generation == owner.window_generation && metadata.component_generation == owner.component_generation && metadata.kind == owner.kind
        && record.text(2) == owner.host_id && record.text(3) == owner.window_id && record.text(4) == owner.surface_id
        && match &owner.key { ui_wgpu::wgpu::NodeKey::Explicit(key) => metadata.positional_key.is_none() && record.text(5) == key, ui_wgpu::wgpu::NodeKey::Positional(a, b) => metadata.positional_key == Some((*a, *b)) }
}

fn camera_owns_surface(record: &SceneCameraRecord) -> bool {
    let current_matches = SCENE_STATE.with(|cell| {
        let states = cell.borrow();
        let current = states.get(record.text(0)).and_then(|state| state.canvas_camera_owner.as_ref());
        match (current, record.metadata) {
            (Some(current), Some(_)) => camera_owner_matches(record, current),
            (None, None) => true,
            _ => false,
        }
    });
    if !current_matches { return false; }
    let Some(metadata) = record.metadata else { return true };
    if crate::interpreter::ui_document_close_pending_for(record.text(3)) || scene_host_retiring(record.text(2)) { return false; }
    let witness = ui_wgpu::wgpu::reconcile::UiComponentSceneWitness {
        document_id: metadata.document_id,
        host_id: record.text(2),
        window_id: record.text(3),
        window_generation: metadata.window_generation,
        component_generation: metadata.component_generation,
        key: match metadata.positional_key { Some((variant, ordinal)) => ui_wgpu::wgpu::NodeKeyRef::Positional(variant, ordinal), None => ui_wgpu::wgpu::NodeKeyRef::Explicit(record.text(5)) },
        kind: metadata.kind,
        surface_id: record.text(4),
    };
    crate::interpreter::component_scene_is_presented(&witness)
}

pub(crate) fn remove_scene_camera_deadline(host: &str) {
    let result = SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| {
        let mut deadlines = cell.borrow_mut();
        let grant = deadlines.received_grant()?;
        deadlines.remove(host, grant)
    });
    if result.is_err() { SCENE_CAMERA_DISPATCH_FAULT.with(|fault| *fault.borrow_mut() = Some("scene camera removal credits exceeded")); }
}

/// 🪙️ Receives the exact host bootstrap grant before any camera storage operation.
pub(crate) fn install_scene_camera_authority(grant: semio_framework_job::RetainedCloneGrant) -> Result<(), semio_framework_value::ValueError> {
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().install(grant))
}

/// 🧹️ The original host closes global deadlines with the same incoming five-axis authority.
pub(crate) fn close_scene_camera_storage(grant: semio_framework_job::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, semio_framework_value::ValueError> {
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().close_step(grant))
}

pub(crate) fn scene_camera_storage_is_empty() -> bool {
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow().terminal_is_empty())
}

/// 🕒️ Settles the original Canvas identity 120 ms after its latest mutation.
pub(crate) fn schedule_scene_camera_dispatch(host_id: &str, surface_id: &str) {
    let result = SCENE_STATE.with(|states| {
        let states = states.borrow();
        let owner = states.get(host_id).and_then(|state| state.canvas_camera_owner.as_ref());
        let (metadata, text) = match owner {
            Some(owner) => (Some(SceneCameraOwnerMetadata::from_owner(owner)), [host_id, surface_id, owner.host_id.as_str(), owner.window_id.as_str(), owner.surface_id.as_str(), match &owner.key { ui_wgpu::wgpu::NodeKey::Explicit(key) => key.as_str(), ui_wgpu::wgpu::NodeKey::Positional(_, _) => "" }]),
            None => (None, [host_id, surface_id, "", "", "", ""]),
        };
        SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| {
            let mut deadlines = cell.borrow_mut();
            let grant = deadlines.received_grant()?;
            deadlines.schedule(text, metadata, crate::app_now_ms() + 120.0, grant)
        })
    });
    if result.is_err() { SCENE_CAMERA_DISPATCH_FAULT.with(|fault| *fault.borrow_mut() = Some("scene camera deadline credits exceeded")); }
}

pub enum SceneCameraDispatchStep { Pending, Action(ActionDescriptor), Complete, Fault(&'static str) }

pub struct SceneCameraDispatchCursor {
    pub(crate) entries: crate::camera_storage::DirectoryCursor<Option<SceneCameraOwnerMetadata>>,
    pub(crate) now_ms: f64,
    pub(crate) fault: Option<&'static str>,
}

impl SceneCameraDispatchCursor {
    pub fn begin(now_ms: f64, grant: semio_framework_job::RetainedCloneGrant) -> Result<(Self, semio_framework_job::RetainedCloneProgress), semio_framework_value::ValueError> {
        let fault_copy = std::mem::size_of::<Option<&'static str>>();
        if grant.maximum_copy_bytes < fault_copy { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "camera fault header exceeds original copy authority")); }
        let (entries, mut progress) = SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().begin(semio_framework_job::RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - fault_copy, ..grant }))?;
        progress.copied_bytes += fault_copy;
        let fault = SCENE_CAMERA_DISPATCH_FAULT.with(|cell| cell.borrow_mut().take());
        Ok((Self { entries, now_ms, fault }, progress))
    }

    fn restore(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> Result<crate::camera_storage::RestoreStep, semio_framework_value::ValueError> {
        SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().restore(&mut self.entries, grant))
    }

    pub fn step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> Result<(SceneCameraDispatchStep, semio_framework_job::RetainedCloneProgress), semio_framework_value::ValueError> {
        use semio_framework_job::RetainedCloneProgress;
        use semio_framework_value::RetainedCloneStep;
        if self.fault.is_some() {
            if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < std::mem::size_of::<Option<&'static str>>() { return Ok((SceneCameraDispatchStep::Pending, RetainedCloneProgress::default())); }
            let fault = self.fault.take().unwrap();
            return Ok((SceneCameraDispatchStep::Fault(fault), RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<Option<&'static str>>(), ..Default::default() }));
        }
        if self.entries.has_retiring() {
            let progress = self.entries.close_retiring(grant)?.progress();
            return Ok((SceneCameraDispatchStep::Pending, progress));
        }
        let Some((_, record)) = self.entries.peek() else {
            let step = self.entries.close_backing(grant)?;
            return Ok((if matches!(step, RetainedCloneStep::Complete(_)) { SceneCameraDispatchStep::Complete } else { SceneCameraDispatchStep::Pending }, step.progress()));
        };
        if record.processed || record.terminal_is_empty() {
            return Ok((SceneCameraDispatchStep::Pending, self.entries.close_current(grant)?.progress()));
        }
        let valid = camera_owns_surface(record) && SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow().get(record.text(0)).is_none());
        if valid && record.at_ms > self.now_ms {
            match self.restore(grant)? {
                crate::camera_storage::RestoreStep::Returned { progress } => return Ok((SceneCameraDispatchStep::Pending, progress)),
                crate::camera_storage::RestoreStep::Retained { progress } if progress.copied_items != 0 => return Ok((SceneCameraDispatchStep::Pending, progress)),
                crate::camera_storage::RestoreStep::Retained { .. } => {}
            }
        } else if valid {
            let progress = self.entries.mark_processed(grant)?;
            let record = self.entries.peek().unwrap().1;
            let action = SCENE_STATE.with(|cell| -> Result<Option<ActionDescriptor>, &'static str> {
                let states = cell.borrow();
                let Some(state) = states.get(record.text(0)) else { return Ok(None) };
                let Some(controller_id) = state.camera_dispatch_controller_id.as_ref() else { return Ok(None) };
                if controller_id.len() > SCENE_CAMERA_ID_BYTE_CAPACITY { return Err("scene camera action identifier exceeded fixed credits"); }
                Ok(Some(scene_camera_action(record.text(1), controller_id, state.viewport)))
            });
            return Ok((match action { Ok(Some(action)) => SceneCameraDispatchStep::Action(action), Ok(None) => SceneCameraDispatchStep::Pending, Err(fault) => SceneCameraDispatchStep::Fault(fault) }, progress));
        }
        Ok((SceneCameraDispatchStep::Pending, self.entries.mark_processed(grant)?))
    }

    pub fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_value::ValueError> {
        use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
        use semio_framework_value::RetainedCloneStep;
        if self.fault.is_some() {
            if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < std::mem::size_of::<Option<&'static str>>() { return Ok(InteractiveJobCloseStep::Pending { progress: Default::default() }); }
            self.fault = None;
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<Option<&'static str>>(), ..Default::default() } });
        }
        let result = (|| {
            if self.entries.has_retiring() { return self.entries.close_retiring(grant); }
            let Some((_, record)) = self.entries.peek() else { return self.entries.close_backing(grant); };
            if !record.processed && !record.terminal_is_empty() && camera_owns_surface(record) {
                match self.restore(grant)? {
                    crate::camera_storage::RestoreStep::Returned { progress } => return Ok(RetainedCloneStep::Progress(progress)),
                    crate::camera_storage::RestoreStep::Retained { progress } if progress.copied_items != 0 => return Ok(RetainedCloneStep::Progress(progress)),
                    crate::camera_storage::RestoreStep::Retained { .. } => {}
                }
            }
            self.entries.close_current(grant)
        })();
        match result {
            Ok(step) if self.terminal_is_empty() => Ok(InteractiveJobCloseStep::Complete { progress: step.progress() }),
            Ok(step) => Ok(InteractiveJobCloseStep::Pending { progress: step.progress() }),
            Err(error) => Err(error),
        }
    }

    pub fn terminal_is_empty(&self) -> bool { self.entries.terminal_is_empty() && self.fault.is_none() }
}

/// 🕒️ A Canvas2d/Paint2d `setCamera` action from a bare surface id/controller id/viewport — same
/// `{surfaceId, camera: {x, y, zoom}}` shape `ink_set_camera_action` already uses for `InkCanvas`'s own
/// camera dispatch (this crate's one other camera-from-viewport builder), reused here since neither
/// `Paint2dHost`'s React source nor this repo's own Ink precedent key it any other way.
pub(crate) fn scene_camera_action(surface_id: &str, controller_id: &str, viewport: Viewport) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: "setCamera".into(), args: crate::action_args_json!({ "surfaceId": surface_id, "camera": { "x": viewport.x, "y": viewport.y, "zoom": viewport.zoom } }) }
}

#[cfg(test)]
include!("../../../🧪️tests/🧊️wgpu-standalone/🦀️.rs");
//#endregion SceneCameraDispatch

/** 🖱️ Cheap read of a surface's pointer edge-detection fields, avoiding a full `SceneSurfaceState` clone. `pub(crate)` so `interpreter::apply_scene_ui_command` (the real per-event `UiCommand::Scene` handler — the sole caller of `handle_scene_pointer_button`/`handle_scene_pointer_move` now, `RenderEntry`'s own once-per-render-frame `apply_scene_wheel`/`apply_scene_pointer` having been deleted once every generic-fallback surface was proven reachable through this path) can read `pointer_was_down`/`last_pointer_pos` to derive `handle_scene_pointer_move`'s `down`/drag-delta parameters. */
pub(crate) fn scene_pointer_edge_state(surface_id: &str) -> (bool, f32, f32) {
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).map(|state| (state.pointer_was_down, state.last_pointer_pos.0, state.last_pointer_pos.1)).unwrap_or((false, 0.0, 0.0)))
}

/** 🖱️ Records `surface_id`'s latest known pointer position — the write half of `scene_pointer_edge_state`, `pub(crate)` for the same reason (see that fn's own doc comment). */
pub(crate) fn set_scene_last_pointer_pos(surface_id: &str, x: f32, y: f32) {
    mutate_scene_state(surface_id, |state| {
        state.last_pointer_pos = (x, y);
    });
}

pub(crate) fn scroll_key(surface_id: &str, suffix: &str) -> String {
    format!("{surface_id}.{suffix}")
}

pub(crate) fn scroll_offset(surface_id: &str, suffix: &str) -> f32 {
    let key = scroll_key(surface_id, suffix);
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).and_then(|state| state.scroll_offsets.get(&key).copied()).unwrap_or(0.0)).max(0.0)
}

pub(crate) fn set_scroll_offset(surface_id: &str, suffix: &str, value: f32) {
    let key = scroll_key(surface_id, suffix);
    mutate_scene_state(surface_id, |state| {
        state.scroll_offsets.insert(key, value.max(0.0));
    });
}

/// 🎬️ One scene-addressed action descriptor: the surface's own controller id, the action name and its
/// JSON args projected into the DSL. The ONE builder every scene lane shares — `⚙️EngineCanvas`'s
/// standalone test lane calls it too rather than keeping a second copy
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
pub(crate) fn scene_action(scene: &UiComponentSceneNode, action: &str, args: semio_framework::DslValue) -> ActionDescriptor {
    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.into(), args: Some(args) }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn now_ms() -> f64 {
    web_sys::window().and_then(|window| window.performance()).map(|perf| perf.now()).unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn now_ms() -> f64 {
    0.0
}

/// 🎨️ The theme the most recent scene paint ran under. The pointer/wheel path (`passive_scene_*`)
/// only ever receives a `&UiComponentSceneNode` and a rect — never the frame's `Theme` — yet every
/// list surface's row geometry is theme-derived (`control_height`, `padding_standard`), so a hit
/// resolved against `Theme::default()` would land on a different row than the one painted. Recorded
/// once per scene paint (phase 2 of `render_component_scene_step`) and read back here.
#[cfg(target_arch = "wasm32")]
thread_local! {
    static SCENE_INPUT_THEME: RefCell<Option<Theme>> = const { RefCell::new(None) };
}
#[cfg(not(target_arch = "wasm32"))]
static SCENE_INPUT_THEME: WorkerCell<Option<Theme>> = WorkerCell::new();

pub(crate) fn remember_scene_theme(theme: &Theme) {
    SCENE_INPUT_THEME.with(|cell| {
        *cell.borrow_mut() = Some(*theme);
    });
}

pub(crate) fn scene_input_theme() -> Theme {
    SCENE_INPUT_THEME.with(|cell| *cell.borrow()).unwrap_or_default()
}

/// 📏️ Byte credits one `DslValue` argument payload needs inside a bounded action — every object key
/// and string scalar it carries. Numbers/bools/nulls are inline `FlatValue`s and cost no text bytes.
fn dsl_value_bytes(value: &semio_framework::DslValue) -> usize {
    match value {
        semio_framework::DslValue::String(text) => text.len(),
        semio_framework::DslValue::Array(values) => values.iter().map(dsl_value_bytes).sum(),
        semio_framework::DslValue::Object(entries) => entries.iter().map(|(key, value)| key.len().saturating_add(dsl_value_bytes(value))).sum(),
        _ => 0,
    }
}

/// 📨️ Publishes one already-built `ActionDescriptor` through the bounded action queue — the single
/// ingress every list surface's pointer handler uses, so a row click, a header sort and a palette
/// press all pay the same exact item/byte credits.
pub(crate) fn write_scene_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, descriptor: &ActionDescriptor) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = scene_action_bytes(descriptor)?;
    let mut batch = input.reserve_actions(1, bytes)?;
    write_scene_action_to_batch(&mut batch, descriptor, bytes)?;
    batch.publish()
}

pub(crate) fn scene_action_bytes(descriptor: &ActionDescriptor) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    ui_wgpu::wgpu::checked_action_string_bytes(&[descriptor.controller_id.as_str(), descriptor.action.as_str()])?.checked_add(descriptor.args.as_ref().map_or(0, dsl_value_bytes)).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
}

pub(crate) fn write_scene_action_to_batch(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, descriptor: &ActionDescriptor, bytes: usize) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    batch.action(&descriptor.controller_id, &descriptor.action, bytes, |builder| match descriptor.args.as_ref() {
        Some(args) => builder.value(None, args),
        None => {
            builder.begin_object(None)?;
            builder.end_container()
        }
    })
}

/// 🎟️ Pre-admits a list surface's per-row draw backing on the layer the caller most recently opened.
/// Rows are culled to the visible band before they are drawn, so the reservation is bounded by the
/// surface's own height rather than by its payload's row count — a ten-thousand-row table reserves
/// exactly what a screenful costs.
const LIST_ROW_ITEM_BUDGET: usize = 64;
const LIST_SURFACE_ITEM_CEILING: usize = 16 * 1024;

pub(crate) fn reserve_list_rows(ctx: &mut FrameworkWidgetContext<'_>, rows: usize) -> bool {
    ctx.draw.try_reserve_retained_items(rows.saturating_mul(LIST_ROW_ITEM_BUDGET).min(LIST_SURFACE_ITEM_CEILING)).is_ok()
}

/// 📐️ How many rows of `row_h` a surface of `bounds` can show at once, plus the partial row at each
/// edge — the reservation and the paint cull agree on this count.
pub(crate) fn list_visible_row_capacity(bounds: Rect, row_h: f32) -> usize {
    ((bounds.h / row_h.max(1.0)).ceil().max(0.0) as usize).saturating_add(2)
}

/// 🖱️ The control id the pointer last resolved inside this surface — the hover authority for every
/// list kind. The retained hit registry cannot carry it: a `ComponentScene` leaf registers exactly
/// ONE entry (`retained_scene_hit`), minted AFTER this paint stages its own rows, so
/// `InputState::hovered_id` always answers the surface itself and never one of its rows.
pub(crate) fn scene_hovered_control_id(surface_id: &str) -> Option<String> {
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).and_then(|state| state.hovered_control_id.clone()))
}

//#endregion SceneRuntime
