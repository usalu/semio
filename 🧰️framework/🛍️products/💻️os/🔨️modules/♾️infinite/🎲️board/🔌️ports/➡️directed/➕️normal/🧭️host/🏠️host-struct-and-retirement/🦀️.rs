//! 🏠️ Host struct and retirement of the normal board host: the canvas content impl, the host struct, its retirement owner and the node move byte count. See [`super`].

use super::*;

impl semio_framework_canvas::canvas_content::CanvasContent for BoardHost {
    fn build_scene(&self) -> Scene {
        self.build_vector_scene()
    }

    fn clear_color(&self) -> Color {
        self.style.canvas_theme.raster_clear
    }
}

// #endregion board_host

pub struct BoardHost {
    /// 🎬️ Entities, camera, viewport and the cached world content of one board.
    pub scene: BoardSceneState,
    /// 🎯️ Selection, preselect, hover and highlight state with the active utility.
    pub focus: BoardFocusState,
    /// 🎨️ Style catalogs, themes, LOD and grid policy.
    pub style: BoardStyleState,
    /// 🖌️ Brush candidates, previews and suggestion state.
    pub brush: BoardBrushState,
    /// 🧩️ The outgoing board event queue and its emit dedupe signatures.
    pub outbox: BoardOutboxState,
    /// 👆️ Transform, region and link gesture state.
    pub gestures: BoardGesturesState,
    /// 🌿️ Pending pointer commit and delete operations.
    pub commit: BoardCommitState,
    /// 🧹️ Close and opaque scene retirement state.
    pub closing: BoardClosingState,
}

// #region board_host
pub struct BoardHostRetirement {
    pub host: std::mem::ManuallyDrop<BoardHost>,
    pub released: bool,
}

impl BoardHostRetirement {
    pub fn new(host: BoardHost) -> Self {
        Self { host: std::mem::ManuallyDrop::new(host), released: false }
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if self.released {
            return true;
        }
        if !self.host.close_nonopaque_step(context) {
            return false;
        }
        assert!(self.host.nonopaque_terminal_is_empty(), "BoardHost nonopaque terminal witness precedes shallow release");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.host) };
        self.released = true;
        true
    }

    pub fn terminal_nonopaque_is_empty(&self) -> bool {
        self.released
    }
}

impl Drop for BoardHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.released, "BoardHostRetirement must reach terminal-empty before release");
    }
}

pub(crate) fn board_node_move_owned_bytes(id: &str, x: f64, y: f64) -> Option<usize> {
    if id.len() > BOARD_EVENT_KEY_BYTE_CAPACITY || !x.is_finite() || !y.is_finite() {
        return None;
    }
    17usize.checked_add(board_json_string_bytes(id))?.checked_add(x.to_string().len())?.checked_add(y.to_string().len())?.checked_add(id.len())
}
