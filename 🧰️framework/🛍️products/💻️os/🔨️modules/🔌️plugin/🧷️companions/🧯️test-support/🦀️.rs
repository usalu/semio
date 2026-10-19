//! 🧯️ Source-text law support of the plugin SDK tests: reads a module tree or a carved crate family at run time instead of embedding it.
//!
//! 🧭️ Implemented by WP3 of `semio-framework-plugin`: `read_module_tree_source` and `read_module_tree_files` replace the `include_str!` source embeds of the plugin tests (a run-time read of the module tree below one root file). Native only; no dependencies. The same algorithm backs the renderer's `test-support`; both are checked against the TypeScript `readModuleTreeSource`.

#[path = "🔤️rust-tokens/🦀️.rs"]
pub mod rust_tokens;

#[path = "🌳️module-tree/🦀️.rs"]
pub mod module_tree;

#[path = "👪️source-family/🦀️.rs"]
pub mod source_family;

pub use module_tree::{inspect_module_children, read_module_tree_files, read_module_tree_source, slice_module_tree_source, ModuleChild, ModuleVia};
pub use source_family::{list_source_family, read_source_family_source};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
