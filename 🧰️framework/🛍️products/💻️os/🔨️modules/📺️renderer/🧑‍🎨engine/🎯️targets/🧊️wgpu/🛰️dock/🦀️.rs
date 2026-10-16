//! 🛰️ Dock: the window tree, the stack chrome, split resize and the mode layouts of the wgpu shell, the Rust twin of React's `Mode` component and the `WindowLayoutNode` helpers.
//!
//! 🧭️ Native and wasm32. The module file stays in the `🛰️Dock` element tree; this crate root mounts it as `dock` and re-exports its surface. The shell-bound unit tests are mounted by the renderer hub until the shell test bins exist.

#[path = "../../../🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs"]
mod dock;
pub use dock::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
