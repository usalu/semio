//! 🔎️ Introspection of the UI engine: the dump and statistics surface, the chrome ledger with the chrome accessibility projection, and the mesh and board2d statistics.
//!
//! 🧭️ Native and wasm32: the diagnostic dumps the browser host reads and the chrome input ledger the shell publishes into. The module files keep their place in the interpreter tree and are mounted here as public modules.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🦀️.rs"]
pub mod introspection;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🎯️chrome-ledger/🦀️.rs"]
pub mod chrome_ledger;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧱️mesh-and-board2d-stats/🦀️.rs"]
pub mod mesh_and_board2d_stats;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
