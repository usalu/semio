//! 🖐️ `shell-pointer` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
/// 📏️ Convergence rounds one `flush_deferred_actions` may spend re-entering the window bodies after
/// its actions and extension answers have quiesced.
///
/// ⚖️ A guest's preview job is driven BY its own window render: rendering `procedural-preview` is
/// what arms the next `flowEvalTick`, and each tick's extension answer arms exactly one more. One
/// refresh plus one flush therefore stops the moment the chain THAT render armed quiesces — measured
/// on 6118 as `meshingFaces 6/8` with the solid never published. React has no such hole because a
/// mutation re-renders the shell, which re-enters every window body and re-arms
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) const SHELL_SETTLE_ROUNDS: usize = 64;

/// 📏️ Steps between two settle-pump heartbeat traces. A converging edit alternates `Drained` and
/// `Crossed` every frame, so the trace reports class TRANSITIONS plus one heartbeat per this many
/// steps — enough for a probe to prove the lane is live and running, few enough to read.
pub(crate) const SHELL_SETTLE_HEARTBEAT_STEPS: u64 = 64;

/// 🫀️ Whether the frame loop owes the shell a settle step — the whole predicate, over the parts a
/// frame can see, so it is drivable without a live guest.
///
/// ⚖️ Derived from five independent terms, never from the arm alone: a step that forgot to re-arm
/// cannot park a live chain, and a chain nobody armed cannot pin the pump awake.
pub(crate) fn settle_pump_owes(settling: bool, has_session: bool, armed_work: bool, owed_scope: &UiDirtyScope, armed: bool, computing: bool) -> bool {
    if settling || !has_session {
        return false;
    }
    armed_work || !owed_scope.asks_for_nothing() || armed || computing
}

//#endregion ShellTypes

