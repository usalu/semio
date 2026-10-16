//! 🧯️ Source-text law support of the plugin SDK tests: reads a module tree at run time instead of embedding it.
//!
//! 🧭️ Implemented by WP3 of `semio-framework-plugin`: `read_module_tree_source` and `read_module_tree_files` replace the `include_str!` source embeds of the plugin tests (a run-time read of the module tree below one root file). Native only; no dependencies. The same algorithm backs the renderer's `test-support`; both are checked against the TypeScript `readModuleTreeSource`.

#[path = "🔤️rust-tokens/🦀️.rs"]
pub mod rust_tokens;

#[path = "🌳️module-tree/🦀️.rs"]
pub mod module_tree;

pub use module_tree::{inspect_module_children, read_module_tree_files, read_module_tree_source, slice_module_tree_source, ModuleChild, ModuleVia};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
