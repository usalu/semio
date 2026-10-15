//! 🧭️ The repo command line and the production repository behind the repo MCP server: `repo_cli`
//! is the `semio-repo` binary's verb tree, `mcp_verb` the repository the `repo` stdio server and
//! the `mcp` verb both serve. Neither links a terminal user interface.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go

//#region 🧭️RepoCli

/// 🧭️ The repo command line: the event stream, its three renderers, the declared command tree and
/// one handler per verb, each a thin front for the domain crate that owns the aggregate. Twin of the
/// Go `cli` package.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go
#[path = "🧭️repo-cli/🦀️.rs"]
pub mod repo_cli;

//#endregion 🧭️RepoCli

//#region 📡️EventStream

#[path = "📡️event-stream/🦀️.rs"]
mod event_stream;

//#endregion 📡️EventStream

//#region 🔌️McpVerb

/// 🔌️ The production repository the repo MCP server serves: every tool reaches the domain owner the
/// matching verb reaches, every resource reads through the repository's GraphQL surface.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust/🦀️.rs
#[path = "🔌️mcp-verb/🦀️.rs"]
pub mod mcp_verb;

//#endregion 🔌️McpVerb
