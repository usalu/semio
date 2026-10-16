//! 🎬️ Shell actions, part one: persistence bindings, sync, hub projection and action dispatch.
//!
//! `ShellTurn`, persistence bindings, the sync pill, hub projection and check-in, invocation history,
//! sync actions and `dispatch_action`.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;
