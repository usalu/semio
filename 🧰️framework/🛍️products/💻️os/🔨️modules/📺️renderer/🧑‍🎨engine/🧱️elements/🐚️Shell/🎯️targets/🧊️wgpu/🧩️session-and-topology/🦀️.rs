//! 🧩️ Shell lifecycle, part one: IO, session, boot and window topology.
//!
//! First half of the shell lifecycle `impl ShellState`: shell IO, session and boot, dock sync, window
//! instance ids, window-topology publication and settle faults.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;
