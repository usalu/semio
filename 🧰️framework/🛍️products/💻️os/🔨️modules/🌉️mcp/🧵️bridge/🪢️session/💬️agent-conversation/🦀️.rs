//! 💬️ Agent conversation of the bridge session: the conversation record and its turn operations. See [`super`].

use super::*;

/// 💬️ The gateway's own emitter for the agent-conversation frames — the ONE place `tools/call` [Design notes](README.md#source-notes-5).
pub struct AgentConversation {
    pub bridge: BridgeSlot,
    pub label: String,
    pub(crate) next_invocation: AtomicU64,
}

impl AgentConversation {
    /// 🏗️ `label` is the connected client's own name (e.g. `claude-code`) — what
    /// `GatewayToShell::AgentPresence` already publishes and what the panel shows as the speaker.
    pub fn new(bridge: BridgeSlot, label: impl Into<String>) -> Self {
        Self { bridge, label: label.into(), next_invocation: AtomicU64::new(1) }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn publish(&self, frame: GatewayToShell) {
        if let Some(handle) = self.bridge.get() {
            let _ = handle.broadcast(frame);
        }
    }

    /// 🛠️ Announces one tool call and returns its invocation id. Emitted BEFORE the handler runs, so [Design notes](README.md#source-notes-37).
    pub fn begin_tool_call(&self, tool_name: &str, arguments: &serde_json::Value) -> String {
        let invocation_id = format!("inv_{}", self.next_invocation.fetch_add(1, Ordering::Relaxed));
        job_registry().begin_with_id(invocation_id.clone(), "toolCall");
        job_registry().report_progress(&invocation_id, 0.0, Some(tool_name.to_string()));
        let rendered = if arguments.is_null() { String::new() } else { truncate_conversation_text(&serde_json::to_string(arguments).unwrap_or_default()) };
        self.publish(GatewayToShell::AgentToolCall { invocation_id: invocation_id.clone(), tool_name: tool_name.to_string(), arguments: rendered });
        self.publish(GatewayToShell::AgentPresence { active: true, label: self.label.clone(), invocation_id: Some(invocation_id.clone()) });
        invocation_id
    }

    /// 🧾️ Closes the invocation `begin_tool_call` opened, then returns presence to idle.
    ///
    /// 🛑️ A call the human cancelled settles as `Cancelled`, not as whatever the handler happened to
    /// return; `finish` is a no-op once terminal, so a cancel that already closed the record wins.
    pub fn finish_tool_call(&self, invocation_id: &str, tool_name: &str, ok: bool, summary: &str) {
        let registry = job_registry();
        if registry.is_cancel_requested(invocation_id) {
            registry.mark_cancelled(invocation_id);
        } else if ok {
            registry.succeed(invocation_id, serde_json::json!({ "tool": tool_name }));
        } else {
            registry.fail(invocation_id, GatewayError::new(GatewayErrorCode::Internal, summary.to_string()));
        }
        self.publish(GatewayToShell::AgentToolResult { invocation_id: invocation_id.to_string(), tool_name: tool_name.to_string(), ok, summary: truncate_conversation_text(summary) });
        self.publish(GatewayToShell::AgentPresence { active: false, label: self.label.clone(), invocation_id: None });
    }
}
//#endregion 💬️AgentConversation
