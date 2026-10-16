//! 🏘️ Hub workspace: the hub connection book and retained workspace tree, the Space administration sheet chrome and the canvas peer presence overlay.
//!
//! 🧭️ Native and wasm32. The module files stay in their element trees; this crate root mounts them as the public modules `hub_connection`, `space_administration` and `canvas_presence` and keeps the former root names `hub_sign_in` and `space_browser` as imports of the hub contracts so the module bodies resolve `crate::hub_sign_in::…` unchanged.

#[cfg(test)]
use semio_framework_os_renderer_wgpu_doors::directory_door;
use semio_framework_os_renderer_wgpu_hub_contracts::{hub_sign_in, space_browser};

#[path = "../../../🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod hub_connection;

#[path = "../../../🧱️elements/🛂️SpaceAdministration/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod space_administration;

#[path = "../../../🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod canvas_presence;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
