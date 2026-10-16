//! 📐️ Provider model: the provider error, the pending model vocabulary and the MCP client kind table.
//!
//! Holds the `🔖️ProviderError`, `📐️ModelPending`, `🪪️McpClientKind` regions of the crate root; the root re-exports its items.

use super::*;

// #region 🔖️ProviderError
/// ⚠️ Every provider failure, carrying the message the Go implementation formats verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderError {
    pub message: String,
}

impl ProviderError {
    /// 🆕️ Wraps a message.
    pub fn new(message: impl Into<String>) -> ProviderError {
        ProviderError { message: message.into() }
    }
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ProviderError {}

/// 🎯️ The crate's result alias.
pub type ProviderResult<T> = Result<T, ProviderError>;
// #endregion 🔖️ProviderError

// #region 📐️ModelPending
/// 🚚️ `ToolKind`, `HookEvent` and `HookResult` are model-level shapes. `semio-framework-repo-model`
/// does not export them yet (checked at ticket time: its `🦀️.rs` carries no `Kind`, `HookEvent` or
/// `HookResult`), so they live here and MUST move down to `🔨️modules/📐️model` the moment that crate
/// grows them; this crate then re-exports them for `🪝️hooks`.
/// 🏷️ A categorized tool execution kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolKind {
    Generic,
    Plan,
    CodeSearch,
    CodeEdit,
    Test,
    Build,
    Terminal,
}

impl ToolKind {
    /// 🔤️ The wire slug.
    pub fn as_str(self) -> &'static str {
        match self {
            ToolKind::Generic => "generic",
            ToolKind::Plan => "plan",
            ToolKind::CodeSearch => "code-search",
            ToolKind::CodeEdit => "code-edit",
            ToolKind::Test => "test",
            ToolKind::Build => "build",
            ToolKind::Terminal => "terminal",
        }
    }

    /// 🧭️ Parses a wire slug, defaulting to `Generic` exactly like the Go zero value.
    pub fn parse(raw: &str) -> ToolKind {
        match raw {
            "plan" => ToolKind::Plan,
            "code-search" => ToolKind::CodeSearch,
            "code-edit" => ToolKind::CodeEdit,
            "test" => ToolKind::Test,
            "build" => ToolKind::Build,
            "terminal" => ToolKind::Terminal,
            _ => ToolKind::Generic,
        }
    }
}

/// 📡️ A lifecycle event kind for hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HookEvent {
    VersionCheckpointStarting,
    VersionCheckpointEnded,
    VersionCheckinStarting,
    VersionCheckinEnded,
    VersionCheckoutStarting,
    VersionCheckoutEnded,
    AgentStarted,
    AgentEnded,
    AgentPromptSubmitting,
    AgentCompacting,
    AgentToolStarting,
    AgentToolEnded,
    AgentToolPlanUpdatingStarting,
    AgentToolPlanUpdatingEnded,
    AgentToolSearchStarting,
    AgentToolSearchEnded,
    AgentToolCodeEditStarting,
    AgentToolCodeEditEnded,
    AgentToolTestStarting,
    AgentToolTestEnded,
    AgentToolBuildStarting,
    AgentToolBuildEnded,
    AgentToolTerminalStarting,
    AgentToolTerminalEnded,
    AgentThinkingStarting,
    AgentThinkingEnded,
}

/// 📋️ Every valid hook event, in the Go declaration order.
pub const ALL_HOOK_EVENTS: [HookEvent; 26] = [
    HookEvent::VersionCheckpointStarting,
    HookEvent::VersionCheckpointEnded,
    HookEvent::VersionCheckinStarting,
    HookEvent::VersionCheckinEnded,
    HookEvent::VersionCheckoutStarting,
    HookEvent::VersionCheckoutEnded,
    HookEvent::AgentStarted,
    HookEvent::AgentEnded,
    HookEvent::AgentPromptSubmitting,
    HookEvent::AgentCompacting,
    HookEvent::AgentToolStarting,
    HookEvent::AgentToolEnded,
    HookEvent::AgentToolPlanUpdatingStarting,
    HookEvent::AgentToolPlanUpdatingEnded,
    HookEvent::AgentToolSearchStarting,
    HookEvent::AgentToolSearchEnded,
    HookEvent::AgentToolCodeEditStarting,
    HookEvent::AgentToolCodeEditEnded,
    HookEvent::AgentToolTestStarting,
    HookEvent::AgentToolTestEnded,
    HookEvent::AgentToolBuildStarting,
    HookEvent::AgentToolBuildEnded,
    HookEvent::AgentToolTerminalStarting,
    HookEvent::AgentToolTerminalEnded,
    HookEvent::AgentThinkingStarting,
    HookEvent::AgentThinkingEnded,
];

impl HookEvent {
    /// 🔤️ The dotted event slug carried on the wire.
    pub fn as_str(self) -> &'static str {
        match self {
            HookEvent::VersionCheckpointStarting => "version.checkpoint.starting",
            HookEvent::VersionCheckpointEnded => "version.checkpoint.ended",
            HookEvent::VersionCheckinStarting => "version.checkin.starting",
            HookEvent::VersionCheckinEnded => "version.checkin.ended",
            HookEvent::VersionCheckoutStarting => "version.checkout.starting",
            HookEvent::VersionCheckoutEnded => "version.checkout.ended",
            HookEvent::AgentStarted => "agent.started",
            HookEvent::AgentEnded => "agent.ended",
            HookEvent::AgentPromptSubmitting => "agent.prompt.submitting",
            HookEvent::AgentCompacting => "agent.compacting",
            HookEvent::AgentToolStarting => "agent.tool.starting",
            HookEvent::AgentToolEnded => "agent.tool.ended",
            HookEvent::AgentToolPlanUpdatingStarting => "agent.tool.plan.updating.starting",
            HookEvent::AgentToolPlanUpdatingEnded => "agent.tool.plan.updating.ended",
            HookEvent::AgentToolSearchStarting => "agent.file.read.starting",
            HookEvent::AgentToolSearchEnded => "agent.file.read.ended",
            HookEvent::AgentToolCodeEditStarting => "agent.tool.code.edit.starting",
            HookEvent::AgentToolCodeEditEnded => "agent.tool.code.edit.ended",
            HookEvent::AgentToolTestStarting => "agent.tool.test.starting",
            HookEvent::AgentToolTestEnded => "agent.tool.test.ended",
            HookEvent::AgentToolBuildStarting => "agent.tool.build.starting",
            HookEvent::AgentToolBuildEnded => "agent.tool.build.ended",
            HookEvent::AgentToolTerminalStarting => "agent.tool.terminal.starting",
            HookEvent::AgentToolTerminalEnded => "agent.tool.terminal.ended",
            HookEvent::AgentThinkingStarting => "agent.thinking.starting",
            HookEvent::AgentThinkingEnded => "agent.thinking.ended",
        }
    }

    /// 🧭️ Resolves a dotted event slug.
    pub fn parse(raw: &str) -> Option<HookEvent> {
        ALL_HOOK_EVENTS.into_iter().find(|event| event.as_str() == raw)
    }
}

/// 🔶️ The outcome of a hook invocation: the two fields every Go `HookResult` exposes through its
/// interface, plus the event-specific payload `json.Marshal` would have emitted.
#[derive(Debug, Clone, PartialEq)]
pub struct HookResult {
    pub allowed: bool,
    pub message: String,
    /// 🧾️ The event-specific fields in the order the Go struct declares them. A `Map` would sort
    /// them alphabetically and `json.Marshal` does not, so the order is carried explicitly.
    pub extra: Vec<(String, Value)>,
    /// 🙈️ Whether the `message` member belongs to the event payload rather than to the base.
    ///
    /// Go's `HookResultAgentBase` declares `MessageID string \`json:"message,omitempty"\``, which
    /// shadows `HookResultBase.Message` on every agent result: the block reason stays readable
    /// through `GetMessage()` and never reaches the marshalled object. This flag reproduces that.
    pub message_shadowed: bool,
}

impl HookResult {
    /// 🆕️ A result carrying only the base fields.
    pub fn new(allowed: bool, message: impl Into<String>) -> HookResult {
        HookResult { allowed, message: message.into(), extra: Vec::new(), message_shadowed: false }
    }

    /// ➕️ Adds one event-specific field, replacing an earlier value in place.
    pub fn with(mut self, key: impl Into<String>, value: Value) -> HookResult {
        let key = key.into();
        match self.extra.iter_mut().find(|(name, _)| name == &key) {
            Some(slot) => slot.1 = value,
            None => self.extra.push((key, value)),
        }
        self
    }

    /// 🧾️ The fields `json.Marshal(result)` writes, in Go's declaration order: `allowed`, `message`
    /// when non-empty, then every event-specific field.
    pub fn fields(&self) -> Vec<(String, Value)> {
        let mut fields = vec![("allowed".to_string(), Value::Bool(self.allowed))];
        if !self.message.is_empty() && !self.message_shadowed {
            fields.push(("message".to_string(), Value::String(self.message.clone())));
        }
        fields.extend(self.extra.iter().cloned());
        fields
    }

    /// 🧾️ The same fields as an unordered map, for a caller that only looks values up.
    pub fn payload(&self) -> Map<String, Value> {
        self.fields().into_iter().collect()
    }

    /// 🧾️ The JSON text `json.Marshal(result)` produces, field order included.
    pub fn to_json(&self) -> String {
        write_ordered_object(&self.fields())
    }
}

/// 🧾️ Writes an object whose members keep the given order, which `serde_json::Map` cannot.
pub fn write_ordered_object(fields: &[(String, Value)]) -> String {
    let members: Vec<String> = fields.iter().map(|(key, value)| format!("{}:{}", Value::String(key.clone()), value)).collect();
    format!("{{{}}}", members.join(","))
}

impl Serialize for HookResult {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let fields = self.fields();
        let mut map = serializer.serialize_map(Some(fields.len()))?;
        for (key, value) in &fields {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for HookResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let object = Map::<String, Value>::deserialize(deserializer)?;
        let allowed = object.get("allowed").and_then(Value::as_bool).unwrap_or_default();
        let message = object.get("message").and_then(Value::as_str).unwrap_or_default().to_string();
        let extra = object.into_iter().filter(|(key, _)| key != "allowed" && key != "message").collect();
        Ok(HookResult { allowed, message, extra, message_shadowed: false })
    }
}
// #endregion 📐️ModelPending

// #region 🪪️McpClientKind
/// 🪪️ Which IDE-native MCP surface is talking. Fix 7 of `📓️go-region-dependency-graph.md`: the enum is
/// editor identity, not MCP protocol, so it sits beside the editor providers that enumerate the same
/// IDE set and `🎫️tickets`/`🪝️hooks` reach it downward instead of importing `🔌️mcp`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpClientKind {
    Generic,
    Cursor,
    Kiro,
    Copilot,
    Claude,
    Codex,
}

impl McpClientKind {
    /// 🔤️ The wire slug.
    pub fn as_str(self) -> &'static str {
        match self {
            McpClientKind::Generic => "generic",
            McpClientKind::Cursor => "cursor",
            McpClientKind::Kiro => "kiro",
            McpClientKind::Copilot => "copilot",
            McpClientKind::Claude => "claude",
            McpClientKind::Codex => "codex",
        }
    }
}

/// 🧭️ Maps a CLI/MCP profile slug to an MCP server kind.
pub fn parse_mcp_client_kind(raw: &str) -> ProviderResult<McpClientKind> {
    match raw.trim().to_lowercase().as_str() {
        "" | "generic" | "client" => Ok(McpClientKind::Generic),
        "cursor" => Ok(McpClientKind::Cursor),
        "kiro" => Ok(McpClientKind::Kiro),
        "copilot" => Ok(McpClientKind::Copilot),
        "claude" => Ok(McpClientKind::Claude),
        "codex" => Ok(McpClientKind::Codex),
        _ => Err(ProviderError::new(format!("unknown mcp kind {:?} (expected client, cursor, copilot, claude, codex, or kiro)", raw))),
    }
}

/// 🪝️ Maps an MCP entry binary to the hook client id used by hook event resolution.
pub fn hook_client_for_mcp_kind(kind: McpClientKind) -> &'static str {
    match kind {
        McpClientKind::Cursor => "cursor-chat",
        McpClientKind::Kiro => "kiro-cli",
        McpClientKind::Copilot => "copilot-chat",
        McpClientKind::Claude => "claude-code",
        McpClientKind::Codex => "codex",
        McpClientKind::Generic => "",
    }
}

/// 🧾️ Maps a validated ticket client slug to an MCP surface kind.
pub fn mcp_kind_from_resolved_client(client: &str) -> McpClientKind {
    match client.trim() {
        "cursor-chat" | "cursor" => McpClientKind::Cursor,
        "kiro-cli" => McpClientKind::Kiro,
        "copilot-chat" => McpClientKind::Copilot,
        "claude-code" => McpClientKind::Claude,
        "codex" => McpClientKind::Codex,
        _ => McpClientKind::Generic,
    }
}

/// 🏷️ The MCP server identifier for the given kind.
pub fn mcp_server_name(kind: McpClientKind) -> &'static str {
    match kind {
        McpClientKind::Cursor | McpClientKind::Kiro | McpClientKind::Copilot | McpClientKind::Claude | McpClientKind::Codex | McpClientKind::Generic => "repo",
    }
}
// #endregion 🪪️McpClientKind
