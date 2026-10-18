//! 🏁️ Closing state and quarantine of the board gestures: the closing state methods, the world content quarantine step and the brush state methods. See [`super`].

use super::*;

impl BoardClosingState {
    pub fn advance_opaque_scene_retirement_step(&self) -> bool {
        let Some(token) = self.opaque_scene_retirement.get() else {
            return true;
        };
        match semio_framework_canvas::advance_opaque_scene_retirement(token, 1, 4096) {
            OpaqueSceneRetirementStep::Blocked | OpaqueSceneRetirementStep::Pending { .. } => false,
            OpaqueSceneRetirementStep::Complete { .. } => {
                self.opaque_scene_retirement.set(None);
                true
            }
            OpaqueSceneRetirementStep::Fault => {
                self.opaque_scene_fault.set(true);
                false
            }
        }
    }

    pub fn publish_opaque_scene_retirement(&self, token: OpaqueSceneRetirementToken, scene: Scene) {
        assert!(self.opaque_scene_retirement.get().is_none(), "board retains at most one exact opaque scene retirement");
        semio_framework_canvas::publish_opaque_scene_retirement(token, scene);
        self.opaque_scene_retirement.set(Some(token));
    }

    pub fn opaque_scene_faulted(&self) -> bool {
        self.opaque_scene_fault.get()
    }

    pub fn push_close_string(&mut self, value: String) {
        let index = usize::from(self.close_string_len);
        assert!(index < self.close_strings.len(), "board close string scratch is schema bounded");
        self.close_strings[index] = Some(value);
        self.close_string_len += 1;
    }

    pub fn push_close_optional_string(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.push_close_string(value);
        }
    }

    pub fn close_scratch_step(&mut self) -> bool {
        if self.close_node_handles.is_some() {
            let handle = self.close_node_handles.as_mut().and_then(Vec::pop);
            if let Some(handle) = handle {
                self.push_close_string(handle.handle_kind);
                return true;
            }
            self.close_node_handles = None;
            return true;
        }
        if self.close_string_len > 0 {
            self.close_string_len -= 1;
            drop(self.close_strings[usize::from(self.close_string_len)].take());
            return true;
        }
        false
    }
}

pub fn quarantine_world_content_step(scene_state: &mut BoardSceneState, closing: &mut BoardClosingState) -> bool {
    if closing.opaque_scene_retirement.get().is_some() {
        closing.advance_opaque_scene_retirement_step();
        return false;
    }
    let mut cache = scene_state.world_content_cache.borrow_mut();
    if cache.is_none() {
        return true;
    }
    let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
        closing.opaque_scene_fault.set(true);
        return false;
    };
    let WorldContentCache { scene, .. } = cache.take().expect("world content cache was witnessed occupied");
    closing.publish_opaque_scene_retirement(token, scene);
    false
}

impl BoardBrushState {
    pub(super) fn brush_slot_hit_radius_world(&self) -> f64 {
        (self.brush_node_size * 0.5).max(1.0)
    }

    pub(super) fn brush_effective_suggestion_offset(&self) -> f64 {
        if self.brush_alt_pressed || self.brush_slot_suggestions_active {
            self.suggestion_offset
        } else {
            0.0
        }
    }
}
