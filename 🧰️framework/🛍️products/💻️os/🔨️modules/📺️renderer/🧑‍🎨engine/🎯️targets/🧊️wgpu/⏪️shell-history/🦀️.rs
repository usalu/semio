//! ⏪️ HistoryState component of the shell: time travel and local folders.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏪️history`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

use semio_framework_os_renderer_wgpu_shell_async::{HOST_STORAGE_SNAPSHOT_MAX_BYTES, debug_log};
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_shell_async::FilePrefsFlushState;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_async::PrefsStore;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_host_base::host_io_call;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏪️history/🧬️component/🦀️.rs"]
mod staged_history__component;
pub use staged_history__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏪️history/⏪️time-travel/🦀️.rs"]
mod staged_history__time_travel;
pub use staged_history__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏪️history/📎️local-folders/🦀️.rs"]
mod staged_history__local_folders;
pub use staged_history__local_folders::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏪️history/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_history__storage_prefs_and_i18n;
pub use staged_history__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏪️history/🔖️identity-and-types/🦀️.rs"]
mod staged_history__identity_and_types;
pub use staged_history__identity_and_types::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
