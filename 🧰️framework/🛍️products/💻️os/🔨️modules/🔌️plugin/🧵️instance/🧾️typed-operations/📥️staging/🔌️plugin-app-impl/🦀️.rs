//! 🎺️ Staged slice of the `plugin-app-impl` band of the mounted artifact instance: its `instance-typed-ops` members (WP6 step S0 of the instance family).
//!
//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.
use super::*;
use crate::tool_jobs::tool_run::TOOL_RUN_TRACE_DELTA_BYTES;

/// ⏳️ Rounds one child publication spends reclaiming the retirements ahead of it before it waits instead.
/// ♻️ Returned document roots a publication leaves to the maintenance rotation: only beyond this many does a publication
/// unit drain them on its critical path. A one-item publication returns exactly one pre-edit root, so it never walks the
/// document; a multi-item batch drains before its second item reads a base, which keeps it bounded to the staged root,
/// the base it reads and this many returned roots (the 816-op fill OOM), never to one root per folded item.
pub(super) const PUBLICATION_RETURNED_ROOT_ALLOWANCE: usize = 1;
