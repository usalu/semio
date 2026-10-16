//! 🎯️ World camera ledger: the live world camera rows and their diagnostic, lifted out of the chrome ledger for the `scene-state` crate of the renderer partition (plan section 5.2).
use super::*;

//#region 🎯️ChromeLedger
/// 🎥️ The LIVE orbit of every world surface that painted this frame, keyed by surface id — the
/// diagnostics twin of [`CHROME_LEDGER`], and for the same reason: `dumpMeshStats` reaches only
/// `UI_ENGINE`, while the orbit lives on the shell's `world3d_states`, so the shell has to hand it
/// over at the one point per frame where it holds both the surface id and the state.
pub static WORLD_CAMERA_LEDGER: WorkerCell<std::collections::BTreeMap<String, Value>> = WorkerCell::new(Default::default);

/** 🎥️ Records ONE world surface's live orbit. Called once per surface per painted frame from
 * `🎞️Scenes/🎯️targets/🧊️wgpu`'s world3d render, gated on runtime diagnostics exactly as the chrome
 * ledger is, so a production frame pays nothing. */
pub fn note_world3d_live_camera(surface_id: &str, camera: Value) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    WORLD_CAMERA_LEDGER.borrow_mut().insert(surface_id.to_string(), camera);
}

/// 🧹️ Transfers one exact component's diagnostic payload to its bounded retirement cursor.
pub fn take_world_camera_diagnostic(host_id: &str) -> Option<(String, Value)> {
    WORLD_CAMERA_LEDGER.borrow_mut().remove_entry(host_id)
}
//#endregion 🎯️ChromeLedger
