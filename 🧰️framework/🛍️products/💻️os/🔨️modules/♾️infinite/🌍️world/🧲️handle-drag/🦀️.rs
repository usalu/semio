//! 🧲️ World3d handle drag: the drag parameter along a handle axis, the pick of a handle marker, the gesture state machine whose output is one `delta`, `commit` or `abort` binding input, and the publication of that input as a bounded action.
//!
//! The Rust twin of `🌐️World3dHost/🧲️handle-drag/🟦️.ts`; both reproduce `🌐️World3dHost/🧪️tests/🧫️fixtures/🧲️handle-drag.json`, whose parameter vectors an independent numpy solve also derives.

use semio_ui_declaration::layout::ActionDescriptor;

pub const WORLD_HANDLE_PICK_RADIUS_PX: f64 = 12.0;

/// 🧲️ One pickable handle of the last painted frame: its marker's id, its screen position in the viewport's own pixels, and the world axis line it drags along.
#[derive(Clone, Debug, PartialEq)]
pub struct WorldHandleTarget {
    pub id: String,
    pub screen: [f64; 2],
    pub origin: [f64; 3],
    pub axis: [f64; 3],
}

/// 🔗️ The actions the surface node binds to the three moments of a handle drag, each with its authored arguments; `None` for a moment the node does not bind.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorldHandleBindings {
    pub delta: Option<ActionDescriptor>,
    pub commit: Option<ActionDescriptor>,
    pub abort: Option<ActionDescriptor>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldHandleTrigger {
    Delta,
    Commit,
    Abort,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WorldHandleModifiers {
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
}

/// 📨️ The payload of one dispatch, the `handle` / `s` / `origin` / `modifiers` / `at` the shell's `Gesture::parse` reads.
#[derive(Clone, Debug, PartialEq)]
pub struct WorldHandleDragInput {
    pub handle: String,
    pub s: f64,
    pub origin: f64,
    pub modifiers: WorldHandleModifiers,
    pub at: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldHandleDragState {
    pub handle: String,
    pub origin: f64,
    pub s: f64,
    pub moved: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorldHandleDragEvent {
    Press { handle: String, s: f64 },
    Move { s: f64, modifiers: WorldHandleModifiers, at: [f64; 3] },
    Release { s: f64, modifiers: WorldHandleModifiers, at: [f64; 3] },
    Cancel { modifiers: WorldHandleModifiers, at: [f64; 3] },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldHandleDragStep {
    pub state: Option<WorldHandleDragState>,
    pub dispatch: Option<(WorldHandleTrigger, WorldHandleDragInput)>,
}

impl WorldHandleBindings {
    pub fn binds_any(&self) -> bool {
        self.delta.is_some() || self.commit.is_some() || self.abort.is_some()
    }

    pub fn action(&self, trigger: WorldHandleTrigger) -> Option<&ActionDescriptor> {
        match trigger {
            WorldHandleTrigger::Delta => self.delta.as_ref(),
            WorldHandleTrigger::Commit => self.commit.as_ref(),
            WorldHandleTrigger::Abort => self.abort.as_ref(),
        }
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn unit(a: [f64; 3]) -> Option<[f64; 3]> {
    let length = dot(a, a).sqrt();
    (length > 0.0).then(|| [a[0] / length, a[1] / length, a[2] / length])
}

/// 📐️ The signed distance `s` from `axis_origin` along `axis` of the point of the axis line closest to the pointer ray; `None` when the ray is parallel to the axis or either direction is degenerate.
pub fn world3d_handle_drag_parameter(ray_origin: [f64; 3], ray_direction: [f64; 3], axis_origin: [f64; 3], axis: [f64; 3]) -> Option<f64> {
    let (u, d) = (unit(ray_direction)?, unit(axis)?);
    let w = [axis_origin[0] - ray_origin[0], axis_origin[1] - ray_origin[1], axis_origin[2] - ray_origin[2]];
    let b = dot(d, u);
    let denominator = 1.0 - b * b;
    (denominator >= 1e-9).then(|| (b * dot(u, w) - dot(d, w)) / denominator)
}

/// 🎯️ The index of the nearest handle within `radius_px` of the pointer, ties to the first in scene order.
pub fn world3d_handle_pick(handles: &[WorldHandleTarget], x: f64, y: f64, radius_px: f64) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for (index, handle) in handles.iter().enumerate() {
        let distance = (handle.screen[0] - x).hypot(handle.screen[1] - y);
        if distance <= radius_px && best.is_none_or(|(_, nearest)| distance < nearest) {
            best = Some((index, distance));
        }
    }
    best.map(|(index, _)| index)
}

/// 🧾️ The gesture of one handle drag: a press arms it silently, a changed parameter streams `delta`, a release after movement commits once, a cancel after movement aborts, and a click without travel dispatches nothing.
#[allow(clippy::float_cmp)]
pub fn world3d_handle_drag_step(state: Option<&WorldHandleDragState>, event: WorldHandleDragEvent) -> WorldHandleDragStep {
    if let WorldHandleDragEvent::Press { handle, s } = event {
        return WorldHandleDragStep { state: Some(WorldHandleDragState { handle, origin: s, s, moved: false }), dispatch: None };
    }
    let Some(state) = state else { return WorldHandleDragStep { state: None, dispatch: None } };
    let input = |s: f64, modifiers: WorldHandleModifiers, at: [f64; 3]| WorldHandleDragInput { handle: state.handle.clone(), s, origin: state.origin, modifiers, at };
    match event {
        WorldHandleDragEvent::Press { .. } => unreachable!("a press returns before the armed state is read"),
        WorldHandleDragEvent::Move { s, .. } if s == state.s => WorldHandleDragStep { state: Some(state.clone()), dispatch: None },
        WorldHandleDragEvent::Move { s, modifiers, at } => WorldHandleDragStep { state: Some(WorldHandleDragState { s, moved: true, ..state.clone() }), dispatch: Some((WorldHandleTrigger::Delta, input(s, modifiers, at))) },
        WorldHandleDragEvent::Release { s, modifiers, at } => WorldHandleDragStep { state: None, dispatch: state.moved.then(|| (WorldHandleTrigger::Commit, input(s, modifiers, at))) },
        WorldHandleDragEvent::Cancel { modifiers, at } => WorldHandleDragStep { state: None, dispatch: state.moved.then(|| (WorldHandleTrigger::Abort, input(state.s, modifiers, at))) },
    }
}

/// 🧾️ The arguments one dispatch carries, as the map the shell's `Gesture::parse` reads.
pub fn world3d_handle_drag_arguments(input: &WorldHandleDragInput) -> semio_framework_value::DslValue {
    use semio_framework_value::DslValue;
    let flag = |name: &str, value: bool| (name.to_string(), DslValue::Bool(value));
    DslValue::Object(vec![
        ("handle".into(), DslValue::String(input.handle.clone())),
        ("s".into(), DslValue::float(input.s)),
        ("origin".into(), DslValue::float(input.origin)),
        ("modifiers".into(), DslValue::Object(vec![flag("shift", input.modifiers.shift), flag("alt", input.modifiers.alt), flag("ctrl", input.modifiers.ctrl)])),
        ("at".into(), DslValue::Array(input.at.iter().copied().map(DslValue::float).collect())),
    ])
}

/// 🌉️ The action one dispatch fires: the bound action with the gesture's arguments merged over its authored ones, exactly as a control's `UiIntentCommand` merges them; `None` for a moment the node does not bind.
pub fn world3d_handle_drag_descriptor(bindings: &WorldHandleBindings, trigger: WorldHandleTrigger, input: &WorldHandleDragInput) -> Option<ActionDescriptor> {
    let bound = bindings.action(trigger)?;
    let args = semio_ui_action::intent_payload(bound.args.as_ref(), &world3d_handle_drag_arguments(input), semio_ui_action::INTENT_VALUE_FIELD);
    Some(ActionDescriptor { controller_id: bound.controller_id.clone(), action: bound.action.clone(), args: Some(args) })
}

/// 📍️ The pickable handles of an annotation layer painted through `view_proj` onto a `width` x `height` viewport: the visible markers that carry an axis, in layer order.
pub fn world3d_handle_targets(layer: &semio_ui_declaration::ui::World3dAnnotationLayer, view_proj: &[f32; 16], width: f32, height: f32) -> Vec<WorldHandleTarget> {
    use semio_ui_declaration::ui::{project_world3d_annotations, World3dAnnotation};
    let projected = project_world3d_annotations(layer, view_proj, width, height);
    layer
        .items
        .iter()
        .zip(&projected)
        .filter_map(|(item, entry)| match (item, entry.marker) {
            (World3dAnnotation::Marker(marker), Some(screen)) if entry.visible => marker.axis.map(|axis| WorldHandleTarget { id: marker.id.clone(), screen: [screen.x, screen.y], origin: marker.position, axis }),
            _ => None,
        })
        .collect()
}

fn handle_argument_bytes(value: &semio_framework_value::DslValue) -> usize {
    use semio_framework_value::DslValue;
    match value {
        DslValue::String(text) => text.len(),
        DslValue::Array(values) => values.iter().map(handle_argument_bytes).sum(),
        DslValue::Object(entries) => entries.iter().map(|(key, value)| key.len().saturating_add(handle_argument_bytes(value))).sum(),
        _ => 0,
    }
}

/// 📤️ Publishes one handle action onto the surface's input queue as a single reserved batch; a credit shortage is the caller's `OutputBlocked`, never a drop.
pub fn publish_world3d_handle_action(input: &mut semio_ui_wgpu_input::InputState<ActionDescriptor>, descriptor: &ActionDescriptor) -> Result<(), semio_ui_action::BoundedActionFault> {
    let bytes = semio_ui_action::checked_action_string_bytes(&[descriptor.controller_id.as_str(), descriptor.action.as_str()])?.checked_add(descriptor.args.as_ref().map_or(0, handle_argument_bytes)).ok_or(semio_ui_action::BoundedActionFault::ByteCredits)?;
    let mut batch = input.reserve_actions(1, bytes)?;
    batch.action(&descriptor.controller_id, &descriptor.action, bytes, |builder| match descriptor.args.as_ref() {
        Some(args) => builder.value(None, args),
        None => {
            builder.begin_object(None)?;
            builder.end_container()
        }
    })?;
    batch.publish()
}

