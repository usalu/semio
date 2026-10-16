//! ⏳️ Shell detached tasks, pool futures, runners and import and export transfers.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

use semio_framework_os_config::opening_config::{UiLocale as OsUiLocale, UiPreferences};
use semio_framework_ui_locale::{Locale, Terminology};
use std::collections::HashMap;
use ui_wgpu::wgpu::{DrawList, Rect, Theme, UtilityNode, push_chrome_group_border};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/📨️command-dispatch/🦀️.rs"]
mod staged_async__command_dispatch;
pub use staged_async__command_dispatch::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_async__chrome_overlays_and_tour;
pub use staged_async__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/🧩️session-and-topology/🦀️.rs"]
mod staged_async__session_and_topology;
pub use staged_async__session_and_topology::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_async__storage_prefs_and_i18n;
pub use staged_async__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_async__chrome_paint_and_navbar;
pub use staged_async__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_async__dock_model_and_measures;
pub use staged_async__dock_model_and_measures::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
