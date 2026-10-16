//! 📄️ `shell-docs` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl DocumentState {
    //#region 🎯️RetainedPointerRouting
    pub fn media_owner_for_session(&self, session: &ActiveSession) -> Option<semio_framework_os_renderer_wgpu_media_slots::PresentedMediaOwner> {
        let parent_document_id = self.app_document_identities.document(&(session.plugin_id.clone(), session.instance_id))?.to_string();
        Some(semio_framework_os_renderer_wgpu_media_slots::PresentedMediaOwner { plugin_id: session.plugin_id.clone(), controller_id: session.app.controller_id.clone(), app_instance_id: session.instance_id, parent_document_id })
    }
    //#endregion 🎯️RetainedPointerRouting
}

//#endregion ShellInput

