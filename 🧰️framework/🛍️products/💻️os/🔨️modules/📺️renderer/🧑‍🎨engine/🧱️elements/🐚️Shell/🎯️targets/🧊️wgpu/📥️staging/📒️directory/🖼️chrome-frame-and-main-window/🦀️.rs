//! 📒️ `shell-directory` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl DirectoryState {
    /// 👕️ Every board window of the attached session as the presence wire names it — camera, size and
    /// the world point under the pointer (`crate::canvas_presence::board_presence_view`) — plus the
    /// active utility of the board under the pointer, React's `publishLocalActiveToolV1`.
    pub fn board_presence_views(&self, surfaces: &SurfaceState) -> (Vec<PresenceWindowView>, Option<String>) {
        let mut active_tool = None;
        let views = surfaces
            .board2d_states
            .iter()
            .filter_map(|(host_id, surface)| {
                let (camera, utility) = semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_presence_state(host_id)?;
                let view = semio_framework_os_renderer_wgpu_hub_workspace::canvas_presence::board_presence_view(&surface.window_id, surface.bounds, camera, self.presence_pointer);
                if view.pointer.is_some() {
                    active_tool = utility;
                }
                Some(view)
            })
            .collect();
        (views, active_tool)
    }

    /// 👥️ Every board window's peer overlays from the verified roster: the other actors' cursors,
    /// viewports and marks this shell paints over its boards (`crate::canvas_presence::board_peer_overlays`,
    /// React's `CanvasPresenceOverlayV1`). Nothing before the hub named this connection's own actor.
    pub fn board_peer_overlays(&self, surfaces: &SurfaceState) -> Vec<(Rect, semio_framework_os_renderer_wgpu_hub_workspace::canvas_presence::BoardPeerOverlays)> {
        let Some((actor, color)) = self.presence_self.as_ref() else { return Vec::new() };
        surfaces.board2d_states
            .iter()
            .filter_map(|(host_id, surface)| {
                let (camera, _) = semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_presence_state(host_id)?;
                let overlays = semio_framework_os_renderer_wgpu_hub_workspace::canvas_presence::board_peer_overlays(&self.presence_peers, &surface.window_id, surface.bounds, camera, actor, *color, &format!("board/{}", surface.window_id));
                (!overlays.cursors.is_empty() || !overlays.marks.is_empty()).then_some((surface.bounds, overlays))
            })
            .collect()
    }
}

//#endregion ShellChrome

