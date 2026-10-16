//! 🛂️ The renderer's two host doors: the duplex socket door (verbs, frames, pages) and the one directory transport seam the identity, command and Space administration lanes are parameterized over.
//!
//! 🧭️ Native and wasm32. The module files stay in the target tree; this crate root mounts them as the public modules `socket_door` and `directory_door`.

extern crate semio_framework_os_kernel as store_sync;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_host_base::host_io_call;
use semio_framework_os_renderer_wgpu_host_base::spawn_app_task;

#[path = "../🔌️socket-door/🦀️.rs"]
pub mod socket_door;

#[path = "../📇️directory-door/🦀️.rs"]
pub mod directory_door;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
