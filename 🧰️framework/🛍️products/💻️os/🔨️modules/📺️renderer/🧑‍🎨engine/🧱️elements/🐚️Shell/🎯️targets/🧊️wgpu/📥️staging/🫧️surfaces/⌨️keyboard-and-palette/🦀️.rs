//! 🫧️ `shell-surfaces` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl SettleState {
    /// 🫀️ Arms the runtime-owned settle lane — the settle twin of [`Self::owe_refresh`], and the ONE
    /// way a producer says "there is a chain here to converge". Declaring is all a producer may do:
    /// the frame loop is the only caller of [`Self::settle_pump_step`], so nothing converges inside a
    /// boot, a dispatch or a pointer handler any more.
    pub fn owe_settle(&mut self) {
        self.settle_pump.owed = true;
    }
}

impl SurfaceState {
    /// ⏳️ Every live World3d surface whose own published status says its producer is working, with
    /// that status — the one input both halves of a pump step read.
    pub(super) fn live_compute_surfaces(&self) -> impl Iterator<Item = (String, World3dComputeStatus)> + '_ {
        self.world3d_status.iter().filter(|(surface_id, _)| self.world3d_states.contains_key(surface_id.as_str())).filter_map(|(surface_id, json)| {
            let status = world3d_compute_status(Some(json.as_str()));
            status.computing.then(|| (surface_id.clone(), status))
        })
    }

    pub(crate) fn sync_world3d_catalogue_drop_preview(&mut self, x: f32, y: f32, drag_data: &HashMap<String, String>) {
        let Some((object_kind, mesh_url)) = crate::engine_canvas::puzzle3d_catalogue_drag_payload(drag_data) else {
            self.clear_world3d_catalogue_drop_previews();
            return;
        };
        let mesh_url_ref = mesh_url.as_deref();
        for state in self.world3d_states.values_mut() {
            if state.bounds.contains(x, y) {
                world3d_update_catalogue_drop_preview(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, x, y, &object_kind, mesh_url_ref);
            } else {
                world3d_clear_catalogue_drop_preview(&mut state.gumball);
            }
        }
    }

    pub(super) fn clear_world3d_catalogue_drop_previews(&mut self) {
        for state in self.world3d_states.values_mut() {
            world3d_clear_catalogue_drop_preview(&mut state.gumball);
        }
    }

    pub(super) fn world3d_catalogue_drop_action(&self, x: f32, y: f32, drag_data: &HashMap<String, String>) -> Option<ActionDescriptor> {
        let (object_kind, _mesh_url) = crate::engine_canvas::puzzle3d_catalogue_drag_payload(drag_data)?;
        for state in self.world3d_states.values() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            let Some(origin) = world3d_catalogue_drop_origin(&state.session, &state.camera, &state.scene, x, y) else {
                return None;
            };
            return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "addObjectKind".into(), args: crate::action_args_json!({ "objectKind": object_kind, "origin": origin }) });
        }
        None
    }
}

//#endregion ShellInput

