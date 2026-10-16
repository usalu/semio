//! 🧲️ PluginState component of the shell with panel records and projections.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate semio_framework_os_kernel as protocol;
use semio_framework_os_renderer_wgpu_shell_async::debug_log;
use semio_framework_os_renderer_wgpu_shell_surfaces::SurfaceState;
#[cfg(not(target_arch = "wasm32"))]
use protocol::os_directory::client::{DOCUMENT_HTTP_PORT_TOPIC, DocumentHttpPortDeclarationV1};
use semio_framework::{AppDefinition, PanelTabDefinition};
use semio_framework_async::CancelToken;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_host_base::host_io_call;
use semio_framework_os_renderer_wgpu_program_bridge::{PluginHostConfig, ProgramBridgeEntry};
use semio_framework_value::FromValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/🧬️component/🦀️.rs"]
mod staged_plugins__component;
pub use staged_plugins__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_plugins__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_plugins__space_admin_and_settle;
pub use staged_plugins__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/📨️command-dispatch/🦀️.rs"]
mod staged_plugins__command_dispatch;
pub use staged_plugins__command_dispatch::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/🔖️identity-and-types/🦀️.rs"]
mod staged_plugins__identity_and_types;
pub use staged_plugins__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/🐚️shell-parent/🦀️.rs"]
mod staged_plugins__shell_parent;
pub use staged_plugins__shell_parent::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/🧩️session-and-topology/🦀️.rs"]
mod staged_plugins__session_and_topology;
pub use staged_plugins__session_and_topology::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🧲️plugins/🧾️panel-records/🦀️.rs"]
mod staged_plugins__panel_records;
pub use staged_plugins__panel_records::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
