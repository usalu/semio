//! 🎹️ CommandState component of the shell: search, find, staged commands and keybindings.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎹️commands`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, ShellInputState, ShellPaletteKind};
use semio_framework_os_renderer_wgpu_shell_prefs::{PrefsState, dsl_value_as_json};
use semio_framework_os_renderer_wgpu_shell_windows::WindowState;
use semio_framework::IconName;
use semio_framework_os_renderer_wgpu_contracts::{ShellFindItem, ShellFindItems};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::Label;
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::HashMap;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_input::{HitKind, InputState};
use semio_framework_ui_declaration::ui::{UiButtonNode, UiInputNode, UiNode, UiPresence, UiStackNode, UiTextNode, UiToggleNode};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎹️commands/🧬️component/🦀️.rs"]
mod staged_commands__component;
pub use staged_commands__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎹️commands/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_commands__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎹️commands/🧩️session-and-topology/🦀️.rs"]
mod staged_commands__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎹️commands/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_commands__window_actions_and_commands;
pub use staged_commands__window_actions_and_commands::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
