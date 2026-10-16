//! 🪝 `shell-window-actions` staging of `space-admin-and-settle`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellTypes
//#region 🧯️BootProgramSelection
/// 🚀️ Resolves a declared plugin and optional app at the boot composition boundary.
#[derive(Clone, Copy, Debug)]
pub struct BootSelectionRequest<'a> {
    pub plugin_id: &'a str,
    pub app_id: Option<&'a str>,
}

/// 👁️✏️ Projects the variant's anchor app onto the boot ROLE (`?role=viewer`, `SEMIO_APP_ROLE`,
/// `VITE_SEMIO_APP_ROLE`). A surface app id is `<dialect>#<role>` by construction
/// (`semio_framework::surface_app_id`), so the registry's `generation3d → …#editor` row names the
/// right DIALECT and the wrong role the moment a viewer boot is requested. This is the Rust twin of
/// React's `resolveBootPrimaryAppV1` (`🏛️ShellHost/🔀️surface-switch/🟦️.ts`) and follows its rule
/// exactly: a manifest that declares the sibling surface for the anchor's dialect wins, and a
/// manifest that does not keeps the anchor — a viewer request on an editor-only artifact is a
/// downgrade to the surface that exists, never a dead boot.
pub(super) fn project_boot_app_role<'a>(manifest: &'a semio_framework::PluginManifest, anchor: &'a AppDefinition, app_role: semio_framework::manifest::AppRole) -> &'a AppDefinition {
    if anchor.role == app_role {
        return anchor;
    }
    manifest.apps.iter().find(|app| app.role == app_role && app.dialect == anchor.dialect).unwrap_or(anchor)
}
//#endregion 🧯️BootProgramSelection
//#endregion ShellTypes

//#region ShellTypes
/// 📏️ Rounds one `flush_deferred_actions` may spend chasing the chain an extension answer
/// re-arms. The native preview chain converges in five ticks and six extension round trips per
/// preview window; this is generous enough for several windows and finite enough that a guest that
/// never settles is reported instead of spinning forever.
pub(crate) const SHELL_DEFERRED_CHAIN_ROUNDS: usize = 512;

/// 📏️ Pump steps ONE live producer may publish the same progress reading for before the pump stops
/// believing the guest will arm another hop and drives that run to a terminal state.
///
/// ⚖️ A step is one frame, so this is a wall-clock patience measured in frames rather than in
/// milliseconds: a solve that is genuinely advancing moves its `unitsDone`/`facesDone`/`inFlight`
/// witness and resets the count, and only a producer whose witness is frozen — `meshingFaces 36/56`
/// for 200 s on 6118, deaf to `setActiveExample` — ever reaches it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md` §8.2).
pub(crate) const SHELL_SETTLE_STALL_STEPS: u32 = 240;

/// 📏️ Terminal drives the pump may aim at ONE wedged surface before it leaves the run alone. The
/// abort is the producer's OWN published `cancelAction`, and the run it retires is commonly restarted
/// by the producer's own restart policy — so this is bounded to keep a producer that answers an abort
/// with a fresh wedge from becoming an abort storm.
pub(crate) const SHELL_SETTLE_TERMINAL_DRIVES: u32 = 3;

/// 🫀️ What the watchdog decided about ONE live producer this step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellSettleVerdict {
    /// 🫀️ Spend this step's crossing on the producer's own window body.
    Fund,
    /// 🧯 The witness has been frozen for [`SHELL_SETTLE_STALL_STEPS`] steps: drive the run to a
    /// terminal state through the producer's own published `cancelAction`.
    Terminal,
    /// 🏁️ The witness is frozen and there is nothing left to try — the producer published no way to
    /// end its own run, or the terminal-drive budget is spent. Crossings are not helping it, so the
    /// pump stops paying for them until the witness moves again.
    ///
    /// ⚖️ This is the term that keeps a producer which reports `computing` FOREVER from pinning the
    /// host at one guest crossing per frame. The generation3d preview does exactly that on the served
    /// guest — 560 consecutive samples of `computing:true phase:"idle" inFlight:0 ratio:1.0`, over a
    /// window in which nothing was evaluating at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-progress-visibility-2026-09-14.md` §3.1) — and a pump that believed it would never
    /// stand down.
    Stand,
}

/// ⏳️ The progress witness the wedge watchdog compares across frames — every counter a producer moves
/// while it is genuinely advancing, and nothing that changes on its own.
pub(super) fn settle_progress_signature(status: &World3dComputeStatus) -> String {
    format!("{}|{}|{}|{}|{}|{}", status.phase, status.units_done, status.units_total, status.faces_done, status.faces_total, status.in_flight)
}
//#endregion ShellTypes
