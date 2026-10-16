//! 📨️ Shell actions, part two: command dispatch, hub workspace, documents and identity.
//!
//! `dispatch_command`, session and replay commands, space administration, hub workspace actions, document
//! services, directory event pumps, identity bootstrap, `apply_mutations`, extension records, plugin install
//! and `apply_shell_uri`.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;
