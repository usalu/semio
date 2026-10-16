//! 🫧️ `shell-surfaces` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
/// ⏳️ What one [`ShellState::settle_pump_step`] did — the frame loop traces it so a step that cost a
/// guest crossing can be told from one that only drained an already-armed action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellSettleStep {
    /// ♻️ The step executed armed work (deferred actions, extension answers, file opens) and took the
    /// refresh pass that work owed.
    Drained,
    /// 🫀️ Nothing was armed and a producer is still working, so the step spent ONE guest crossing on
    /// exactly the window bodies whose producers report `computing`.
    Crossed,
    /// 🧯 A producer published the same witness for [`SHELL_SETTLE_STALL_STEPS`] steps, so the step
    /// drove its run to a terminal state through the producer's own published `cancelAction`.
    Wedged,
    /// 🏁️ Nothing is armed and nothing is working — the chain has settled and the pump stands down.
    Quiescent,
}

/// ⏳️ One live producer's progress witness, as the pump watches it across frames.
#[derive(Default)]
pub struct ShellSettleWatch {
    pub signature: String,
    pub stalled_steps: u32,
    pub terminal_drives: u32,
    pub standing: bool,
}

/// 🫀️ The shell's half of the runtime-owned settle pump — the wgpu twin of React's
/// `createUiRefreshCoalescerV1` drain loop (`🛠️ShellHelpers/🟦️.tsx`).
///
/// React's shell never converges a chain inside one call either: a pass DECLARES what it still owes
/// and the lane's own loop runs the next pass, so the browser paints between them. This is that lane,
/// with the loop owned by the frame runtime instead of by a promise chain — `settle_boot` arms it and
/// returns, `🧊️renderer/🦀️.rs`'s `FrameDeferredWork::Settle` takes exactly one step per frame, and the
/// step's own verdict says whether another is owed.
///
/// Two things only this layer can own live here:
///
/// * **the guest crossing that funds a background solve.** A window body's render is also the turn
///   that pumps the guest's worker pool, which is why narrowing `refresh_ui` on `UiDirtyScope` alone
///   froze 14 of 16 examples mid-solve (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4). The pump makes
///   that crossing per FRAME, on exactly the surfaces whose producers report `computing`, so the
///   refresh may narrow without withdrawing compute from a solve the UI has nothing to do with.
/// * **the wedge watchdog.** A producer that stops arming hops leaves a non-terminal run standing, and
///   a non-terminal run refuses every restart — the preview then ignores `setActiveExample` forever
///   (`📓️wgpu-progress-visibility-2026-09-14.md` §8.2). The pump watches each producer's own witness
///   and, when it freezes, dispatches the producer's own published `cancelAction`, which is the one
///   gesture measured to unblock it (§7.1 of the same report).
#[derive(Default)]
pub struct ShellSettlePump {
    pub owed: bool,
    pub steps: u64,
    pub crossings: u64,
    pub traced: Option<ShellSettleStep>,
    pub watches: HashMap<String, ShellSettleWatch>,
    pub rendering: Option<ShellDetached<ShellRenderedRefresh>>,
}

//#endregion ShellTypes

