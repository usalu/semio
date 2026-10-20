//! 🧭️ The `repo_cli` facade: mounts the declared command tree and the process entry, and re-exports the entry points the binaries and the
//! language-agnostic test adapters use. The event stream, the command framework, the tree source, the verbs and the MCP repository are
//! producer crates this crate depends on.

//#region 🧭️RepoCli

#[path = "🌳️command-tree/🦀️.rs"]
mod command_tree;

//#region 🚦️Verbs
/// 🚦️ One handler per verb, each a thin front for its domain owner.
pub use semio_framework_repo_cli_verbs as verbs;
//#endregion 🚦️Verbs

#[path = "🚀️run-and-projection/🦀️.rs"]
mod run_and_projection;
pub use run_and_projection::{command_paths, export_records_json, graphql_roundtrip_json, hook_verb_dispatch_json, mcp_conversation, projection, render_stream_json, run, test_verb_lines_json, usage};

//#endregion 🧭️RepoCli

#[path = "🚪️entrypoint/🦀️.rs"]
pub mod entrypoint;
