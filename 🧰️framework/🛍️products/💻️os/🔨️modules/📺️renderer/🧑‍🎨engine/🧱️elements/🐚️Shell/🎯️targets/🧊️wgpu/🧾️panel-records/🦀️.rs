//! 🧾️ Panel records assembler, tool-run panel and app-catalogue attempt.
//!
//! `panel_ui_records`, `PanelProjection`, the settings/staged-argument/agent-chat row helpers, the tool-run
//! panel reveal law and the app-catalogue attempt.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle
//#region ⏯️ToolRunPanel

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs"]
mod tool_run_panel_tests;
//#endregion ⏯️ToolRunPanel



//#endregion ShellLifecycle
