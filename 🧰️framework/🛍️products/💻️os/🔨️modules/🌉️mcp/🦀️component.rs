//! 🌉️ Semio OS MCP gateway library root — dual-era JSON-RPC/MCP protocol core over stdio (packet
//! `P1a-protocol-core`). Downstream packets (P1b HTTP transport, P2 catalog, P6 actions/policy)
//! implement `crate::{ToolRegistry, ResourceRegistry, PromptRegistry, GatewayBackend}` against the
//! real plugin host; THIS crate has zero dependency on it (`📓️design-decisions.md` D8, this packet's
//! brief §2.6 — verified by the absence of `semio-framework*`/plugin/channel/actor deps in this
//! module's own `Cargo.toml`). Every public item from the `⚠️errors`/`🧬️schema`/`🧭️protocol`/
//! `🚚️transport` facets is re-exported flat at this crate root for ergonomic downstream use.

//#region 🔖️Facets
pub use crate::errors::*;
pub use crate::protocol::*;
pub use crate::schema::*;
pub use crate::transport::*;
//#endregion 🔖️Facets

//#region 🔖️StdioEntrypoint
/// ⚙️ Options `📦️bin.rs`'s `stdio` subcommand parses off argv (`semio-os-mcp stdio [--folder <dir>]
/// [--principal <id>] [--scopes a,b]`) — stored but not yet consumed by anything OS-specific: no real
/// `GatewayBackend` is wired in P1a, so these become real constructor inputs in a later packet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StdioOptions {
    pub folder: Option<String>,
    pub principal: Option<String>,
    pub scopes: Vec<String>,
}

/// 🚪️ Boots a [`McpServer::with_defaults`] and serves it over the REAL process stdin/stdout/stderr
/// until the client closes stdin (EOF) or a hard io error occurs. `bin.rs`'s entire `stdio` mode is
/// this one call — all logic lives here, in the lib, per this packet's brief §2.5.
pub fn run_stdio(_options: StdioOptions) -> Result<(), GatewayError> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut transport = StdioTransport::new(stdin.lock(), stdout.lock(), stderr.lock());
    let mut server = McpServer::with_defaults();
    transport.serve(&mut server)
}
//#endregion 🔖️StdioEntrypoint

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    #[test]
    fn stdio_options_default_to_empty() {
        let options = StdioOptions::default();
        assert!(options.folder.is_none());
        assert!(options.principal.is_none());
        assert!(options.scopes.is_empty());
    }

    #[test]
    fn every_facet_re_export_is_reachable_from_the_crate_root() {
        let _code: GatewayErrorCode = GatewayErrorCode::Internal;
        let _tools = InMemoryToolRegistry::new();
        let _resources = InMemoryResourceRegistry::new();
        let _prompts = InMemoryPromptRegistry::new();
        let _backend = NullBackend;
        let _server = McpServer::with_defaults();
        assert_eq!(SUPPORTED_PROTOCOL_VERSIONS[0], "2026-07-28");
    }
}
//#endregion 🧪️Tests
