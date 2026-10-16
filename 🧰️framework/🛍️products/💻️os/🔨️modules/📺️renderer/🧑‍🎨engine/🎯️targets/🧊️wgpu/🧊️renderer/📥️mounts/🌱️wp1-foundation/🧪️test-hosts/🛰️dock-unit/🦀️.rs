//! 🛰️ Test host of the renderer hub. The dock unit laws name `ShellState` and the engine canvas, so they cannot live in `dock`.
//! The host re-exports the module under test, so the law keeps its `use super::*;` and its module path while the law's file stays with its element.

pub(crate) use crate::dock::*;

#[path = "../../../../../../../🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
