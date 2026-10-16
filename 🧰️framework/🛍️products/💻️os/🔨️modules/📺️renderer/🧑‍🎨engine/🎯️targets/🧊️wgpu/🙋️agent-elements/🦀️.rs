//! 🙋️ Agent elements: the MCP agent bridge consumer (frame codec and state), its presence, approvals and chat panel renderers, and the socket transport that carries the bridge frames.
//!
//! 🧭️ Native and wasm32. The module files stay in their element trees; this crate root mounts them as the public modules `agent_bridge`, `agent_presence`, `agent_approvals`, `agent_chat_panel` and `agent_bridge_door`.

use semio_framework_os_renderer_wgpu_doors::socket_door;
use semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool;

#[path = "../../../🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_bridge;

#[path = "../../../🧱️elements/🚦️AgentPresence/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_presence;

#[path = "../../../🧱️elements/🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_approvals;

#[path = "../../../🧱️elements/💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_chat_panel;

#[path = "../🌉️agent-bridge-door/🦀️.rs"]
pub mod agent_bridge_door;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
