//! 🧭️ Render steps for navbar, footer, overlays and notices.
//!
//! Second render half of `impl ShellState`: the navbar, overlay controls, navbar cluster, tutorial bar, footer,
//! overlay, context menu, tooltip, agent chat header, transient notice, plugin install and document opening
//! steps and the space administration plan.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;
