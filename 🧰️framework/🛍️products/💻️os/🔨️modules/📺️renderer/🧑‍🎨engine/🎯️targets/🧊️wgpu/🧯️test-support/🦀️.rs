//! 🧯️ Source-text law support of the renderer tests: reads a module tree at run time instead of embedding it.
//!
//! 🧭️ Implemented by WP3 of `semio-framework-os-renderer-wgpu`: `read_module_tree_source` and `read_module_tree_files` replace the `include_str!` source embeds and the `*_SOURCE` consts of the renderer tests. Native only; no dependencies.

#[path = "🔤️rust-tokens/🦀️.rs"]
pub mod rust_tokens;

#[path = "🌳️module-tree/🦀️.rs"]
pub mod module_tree;

#[path = "📜️repo-text/🦀️.rs"]
pub mod repo_reader;

pub use module_tree::{inspect_module_children, read_module_tree_files, read_module_tree_source, slice_module_tree_source, ModuleChild, ModuleVia};
pub use repo_reader::{engine_canvas_source, engine_root, interpreter_source, renderer_source, repo_bytes, repo_module_tree, repo_module_trees, repo_root, repo_text, repo_texts, scenes_source, shell_source};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
