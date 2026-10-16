//! 🔬️ Test host of the renderer hub. The renderer I/O laws drive the native entry point and the kernel component reader, so they cannot live in `host-base`.
//! The host re-exports the module under test, so the law keeps its `use super::*;` and its module path while the law's file stays with its element.

pub(crate) use super::*;

#[path = "../../../../../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs"]
mod tests;
