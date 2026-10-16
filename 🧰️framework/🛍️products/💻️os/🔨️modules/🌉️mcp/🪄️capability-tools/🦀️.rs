//! 🪄️ Capability tools of the catalog crate: Capability catalog types and compiler, deterministic search, conformance and plugin descriptor discovery.

use crate::catalog::{CapabilityAudience, CapabilityDefinition, CapabilityOwner};
use semio_framework_os_mcp_foundation::reports::SearchHit;
use semio_framework_os_mcp_wire::protocol::Tool;

//#region 🔖️Tools

pub fn to_schema_search_hit(capability: &CapabilityDefinition, score: f64) -> SearchHit {
    let (plugin_id, app_id) = match &capability.owner {
        CapabilityOwner::Plugin { plugin_id, app_id, .. } => (plugin_id.clone(), app_id.clone().unwrap_or_default()),
        CapabilityOwner::Framework => ("framework".to_string(), String::new()),
        CapabilityOwner::Os => ("os".to_string(), String::new()),
        CapabilityOwner::Shell => ("shell".to_string(), String::new()),
        CapabilityOwner::Gateway => ("gateway".to_string(), String::new()),
        CapabilityOwner::Extension { extension_id } => (extension_id.clone(), String::new()),
    };
    let audience = match capability.audience {
        CapabilityAudience::Agent => "agent",
        CapabilityAudience::Input => "input",
        CapabilityAudience::Chrome => "chrome",
    };
    SearchHit { capability_id: capability.id.to_string(), title: capability.title.clone(), description: capability.description.clone(), score, plugin_id, app_id, audience: audience.to_string(), artifact_kind: capability.artifact_kind.clone().unwrap_or_default() }
}

/// 🔧️ Projects one `CapabilityDefinition` onto the MCP `Tool` shape — the single place a tool's
/// title/description/schemas are read back off the catalog, shared by every facet that registers
/// direct tools (`🗿️artifact`, `💡️inference`, `🖥️ui`) so no facet re-derives them.
pub fn tool_from_capability(capability: &CapabilityDefinition, tool_name: &str) -> Tool {
    let mut tool = Tool::new(tool_name, capability.input_schema.clone());
    tool.title = Some(capability.title.clone());
    tool.description = Some(capability.description.clone());
    tool.output_schema = Some(capability.output_schema.clone());
    tool
}

//#endregion 🔖️Tools
