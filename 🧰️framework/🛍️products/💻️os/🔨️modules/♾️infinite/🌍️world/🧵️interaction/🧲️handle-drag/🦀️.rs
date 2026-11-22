//! 🧲️ World3d handle drag arm: the authority arm that drives the handle gesture of `semio-os-infinite-world-handle-drag` from the pointer intents of a wgpu surface.

use super::*;
use semio_os_infinite_world_handle_drag::{publish_world3d_handle_action, world3d_handle_drag_descriptor, world3d_handle_drag_parameter, world3d_handle_drag_step, world3d_handle_pick, world3d_handle_targets, WorldHandleBindings, WorldHandleDragEvent, WorldHandleDragState, WorldHandleModifiers, WorldHandleTarget, WORLD_HANDLE_PICK_RADIUS_PX};

struct WorldHandleSession {
    state: WorldHandleDragState,
    origin: [f64; 3],
    axis: [f64; 3],
}

/// 🧲️ The handle drag of one surface: the handles of the last painted frame, the bindings of its document node and the drag in flight.
#[derive(Default)]
pub struct WorldHandleDrag {
    targets: Vec<WorldHandleTarget>,
    bindings: WorldHandleBindings,
    session: Option<WorldHandleSession>,
}

impl WorldHandleDrag {
    /// 🛑️ Whether a handle drag owns the pointer, so no other arm of the authority may take its moves and release.
    pub fn is_armed(&self) -> bool {
        self.session.is_some()
    }

    fn ray(session_state: &WorldSessionState, camera_state: &WorldCameraState, x: f32, y: f32) -> ([f64; 3], [f64; 3]) {
        let viewport = render_pick_viewport(session_state);
        let camera = camera_state.orbit.to_camera();
        let (origin, direction) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, x - viewport.x, y - viewport.y, viewport.w, viewport.h);
        ([f64::from(origin.x), f64::from(origin.y), f64::from(origin.z)], [f64::from(direction.x), f64::from(direction.y), f64::from(direction.z)])
    }

    fn parameter(session_state: &WorldSessionState, camera_state: &WorldCameraState, x: f32, y: f32, origin: [f64; 3], axis: [f64; 3]) -> Option<f64> {
        let (ray_origin, ray_direction) = Self::ray(session_state, camera_state, x, y);
        world3d_handle_drag_parameter(ray_origin, ray_direction, origin, axis)
    }

    fn ground_point(session_state: &WorldSessionState, camera_state: &WorldCameraState, x: f32, y: f32) -> [f64; 3] {
        world3d_ground_plane_pick(session_state, camera_state, x, y, 0.0).map_or([0.0; 3], |point| [f64::from(point[0]), f64::from(point[1]), f64::from(point[2])])
    }
}

impl WorldAuthorityState {
    /// 🛑️ Whether a handle drag is in flight, so a host key (Escape) knows there is something to abort.
    pub fn handle_drag_armed(&self) -> bool {
        self.interaction_authority.as_ref().is_some_and(|authority| authority.handles.is_armed())
    }

    /// 🧲️ Stages the handles of the frame just painted: the visible axis-carrying markers of the surface's annotation layer, or none.
    pub fn stage_handle_scene(&mut self, layer: Option<&semio_ui_declaration::ui::World3dAnnotationLayer>, view_proj: &[f32; 16], width: f32, height: f32) {
        let Some(authority) = self.interaction_authority.as_mut() else { return };
        let carries_handle = |layer: &semio_ui_declaration::ui::World3dAnnotationLayer| layer.items.iter().any(|item| matches!(item, semio_ui_declaration::ui::World3dAnnotation::Marker(marker) if marker.axis.is_some()));
        let targets = layer.filter(|layer| carries_handle(layer)).map(|layer| world3d_handle_targets(layer, view_proj, width, height)).unwrap_or_default();
        if authority.handles.targets != targets {
            authority.handles.targets = targets;
        }
    }

    /// 🔗️ Stages the bindings of the surface's document node; a node that binds none of the three moments releases a drag in flight.
    pub fn stage_handle_bindings(&mut self, bindings: WorldHandleBindings) {
        let Some(authority) = self.interaction_authority.as_mut() else { return };
        if authority.handles.bindings == bindings {
            return;
        }
        if !bindings.binds_any() {
            authority.handles.session = None;
        }
        authority.handles.bindings = bindings;
    }
}

impl WorldInteractionAuthority {
    /// 🧲️ The handle drag arm: a left press on a handle of a surface whose node binds `delta`/`commit`/`abort` arms a drag and is consumed here, so neither the gumball, the marquee nor any camera plan also takes it; while armed, the left moves, the release and a host cancel are this arm's alone. `None` leaves the intent to the other arms.
    ///
    /// 🚦️ A dispatch that finds no output credit leaves the drag and the intent untouched and answers `OutputBlocked`, so the same intent retries.
    pub(crate) fn step_handle_drag(&mut self, session_state: &WorldSessionState, camera_state: &WorldCameraState, intent: &WorldInteractionIntent, input: &mut semio_ui_wgpu_input::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Option<WorldInteractionAuthorityStep> {
        if !self.handles.bindings.binds_any() {
            return None;
        }
        let modifiers = WorldHandleModifiers { shift: intent.shift, alt: intent.alt, ctrl: intent.ctrl || intent.meta };
        let left = intent.button == 0;
        let (x, y) = (intent.x, intent.y);
        let event = match intent.phase {
            WorldInteractionPhase::PointerButton if left && intent.down && self.handles.session.is_none() => {
                if !world_pick_rect(session_state).contains(x, y) {
                    return None;
                }
                let viewport = render_pick_viewport(session_state);
                let index = world3d_handle_pick(&self.handles.targets, f64::from(x - viewport.x), f64::from(y - viewport.y), WORLD_HANDLE_PICK_RADIUS_PX)?;
                let target = self.handles.targets[index].clone();
                let s = WorldHandleDrag::parameter(session_state, camera_state, x, y, target.origin, target.axis)?;
                self.handles.session = Some(WorldHandleSession { state: world3d_handle_drag_step(None, WorldHandleDragEvent::Press { handle: target.id, s }).state?, origin: target.origin, axis: target.axis });
                self.queue.retire_front(intent.generation);
                context.consume_fuel(1);
                return Some(WorldInteractionAuthorityStep::Complete);
            }
            WorldInteractionPhase::PointerMove if left && intent.down => {
                let session = self.handles.session.as_ref()?;
                match WorldHandleDrag::parameter(session_state, camera_state, x, y, session.origin, session.axis) {
                    Some(s) => WorldHandleDragEvent::Move { s, modifiers, at: WorldHandleDrag::ground_point(session_state, camera_state, x, y) },
                    None => {
                        self.queue.retire_front(intent.generation);
                        context.consume_fuel(1);
                        return Some(WorldInteractionAuthorityStep::Complete);
                    }
                }
            }
            WorldInteractionPhase::PointerButton if left && !intent.down => {
                let session = self.handles.session.as_ref()?;
                let at = WorldHandleDrag::ground_point(session_state, camera_state, x, y);
                match WorldHandleDrag::parameter(session_state, camera_state, x, y, session.origin, session.axis) {
                    Some(s) => WorldHandleDragEvent::Release { s, modifiers, at },
                    None => WorldHandleDragEvent::Cancel { modifiers, at },
                }
            }
            WorldInteractionPhase::Cancel => {
                let session = self.handles.session.as_ref()?;
                WorldHandleDragEvent::Cancel { modifiers: WorldHandleModifiers::default(), at: session.origin }
            }
            _ => return None,
        };
        let session = self.handles.session.as_ref()?;
        let step = world3d_handle_drag_step(Some(&session.state), event);
        if let Some((trigger, payload)) = step.dispatch.as_ref() {
            if let Some(descriptor) = world3d_handle_drag_descriptor(&self.handles.bindings, *trigger, payload) {
                match publish_world3d_handle_action(input, &descriptor) {
                    Ok(()) => {}
                    Err(semio_ui_action::BoundedActionFault::ItemCredits | semio_ui_action::BoundedActionFault::ByteCredits) => return Some(WorldInteractionAuthorityStep::OutputBlocked),
                    Err(_) => {
                        self.faulted = true;
                        return Some(WorldInteractionAuthorityStep::Fault);
                    }
                }
            }
        }
        match (step.state, self.handles.session.as_mut()) {
            (Some(state), Some(session)) => session.state = state,
            _ => self.handles.session = None,
        }
        self.queue.retire_front(intent.generation);
        context.consume_fuel(1);
        Some(WorldInteractionAuthorityStep::Complete)
    }
}
