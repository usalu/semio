//! ⏱️ Quiet slice of the reactor turn: the one rule that decides how long a turn keeps stepping work that moves nothing the host must see. See [`super`].

/// ⏳️ Executing microseconds one turn may spend on back-to-back steps without host-bound output; one selection took 10 048 one-step turns.
pub(super) const QUIET_TURN_SLICE_US:u64=8_000;

/// ⏱️ Whether a turn may take one more quiet step: no host-bound output was added, nothing asks the turn to yield and the executing clock, which is `None` once it was lost, is still inside [`QUIET_TURN_SLICE_US`].
///
/// 🔗️ Shared by the typed-operation, restored-document-load and command-exchange loops of the turn, so every quiet loop closes on the same three signals.
pub(super) fn quiet_slice_open(host_bound_grew:bool,yielding:bool,executing_us:Option<u64>)->bool{!host_bound_grew&&!yielding&&executing_us.is_some_and(|us|us<QUIET_TURN_SLICE_US)}

/// 🚪️ A quiet opening can step again only while it is pending and its original ledger advanced.
pub(super) fn opening_slice_open(ready: bool, progressed: bool, cancelled: bool, yielding: bool, executing_us: Option<u64>) -> bool {
    progressed && !cancelled && quiet_slice_open(ready, yielding, executing_us)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod quiet_slice_tests;
