//! 📒️ `shell-directory` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🆔️ The stable node id one conversation row carries — React's `entryElementId`
/// (`💬️AgentChatPanel/🟦️.tsx`), so a probe addresses a row by identity rather than by position.
pub fn agent_chat_entry_id(entry: &semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry) -> String {
        use semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry;
    let kind = match entry {
        AgentConversationEntry::UserMessage { .. } => "userMessage",
        AgentConversationEntry::ToolCall { .. } => "toolCall",
        AgentConversationEntry::Approval { .. } => "approval",
        AgentConversationEntry::AgentMessage { .. } => "agentMessage",
    };
    format!("framework.chat.entry.{kind}.{}", entry.id())
}

/// 🏷️ The translated role noun one conversation row leads with — React's
/// `os.agent.chat.{youRole,toolCallRole,approvalRole}`.
fn agent_chat_role_label(entry: &semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry, is_de: bool) -> &'static str {
        use semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry;
    match entry {
        AgentConversationEntry::UserMessage { .. } => shell_chrome_string("chat.youRole", is_de),
        AgentConversationEntry::ToolCall { .. } => shell_chrome_string("chat.toolCallRole", is_de),
        AgentConversationEntry::Approval { .. } => shell_chrome_string("chat.approvalRole", is_de),
        AgentConversationEntry::AgentMessage { .. } => shell_chrome_string("chat.agentRole", is_de),
    }
}

/// 🚥️ The translated state chip one conversation row shows to its right — React's
/// `os.agent.chat.{running,failed,succeeded,approvalPending}`, or the human's own decision once an
/// approval is resolved. A sent turn carries no chip, exactly as React's empty `state` renders none.
fn agent_chat_state_label(entry: &semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry, is_de: bool, locale: Locale) -> Option<String> {
        use semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::{AgentApprovalState, AgentConversationEntry, AgentReplyState, AgentToolCallState};
    match entry {
        AgentConversationEntry::UserMessage { .. } => None,
        AgentConversationEntry::AgentMessage { state, .. } => matches!(state, AgentReplyState::Streaming).then(|| shell_chrome_string("chat.replyStreaming", is_de).to_string()),
        AgentConversationEntry::ToolCall { state, .. } => Some(
            match state {
                AgentToolCallState::Running => shell_chrome_string("chat.running", is_de),
                AgentToolCallState::Cancelling => shell_chrome_string("chat.cancelling", is_de),
                AgentToolCallState::Failed => shell_chrome_string("chat.failed", is_de),
                AgentToolCallState::Ok => shell_chrome_string("chat.succeeded", is_de),
            }
            .to_string(),
        ),
        AgentConversationEntry::Approval { state, decision, .. } => match (state, decision) {
            (AgentApprovalState::Pending, _) => Some(shell_chrome_string("chat.approvalPending", is_de).to_string()),
            (AgentApprovalState::Resolved, Some(decision)) => Some(semio_framework_os_renderer_wgpu_agent_elements::agent_approvals::approvals_decision_label(*decision, locale)),
            (AgentApprovalState::Resolved, None) => None,
            (AgentApprovalState::Withdrawn(reason), _) => Some(semio_framework_os_renderer_wgpu_agent_elements::agent_approvals::approvals_withdrawal_label(*reason, locale)),
        },
    }
}

/// 🏷️ The row's own `kind` attribute — React's `data-semio-agent-chat-entry`, whose vocabulary is
/// `AgentConversationEntry`'s own discriminant spelled in camelCase.
pub fn agent_chat_entry_kind(entry: &semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry) -> &'static str {
        use semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry;
    match entry {
        AgentConversationEntry::UserMessage { .. } => "userMessage",
        AgentConversationEntry::ToolCall { .. } => "toolCall",
        AgentConversationEntry::Approval { .. } => "approval",
        AgentConversationEntry::AgentMessage { .. } => "agentMessage",
    }
}

/// 🚥️ The row's own `state` attribute — React's `data-agent-chat-state`: a tool call's or approval's
/// live state, and the literal `"sent"` for a human turn, which has none.
pub fn agent_chat_entry_state_attribute(entry: &semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry) -> &'static str {
        use semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::{AgentApprovalState, AgentConversationEntry, AgentReplyState, AgentToolCallState};
    match entry {
        AgentConversationEntry::UserMessage { .. } => "sent",
        AgentConversationEntry::AgentMessage { state, .. } => match state {
            AgentReplyState::Streaming => "streaming",
            AgentReplyState::Complete => "complete",
        },
        AgentConversationEntry::ToolCall { state, .. } => match state {
            AgentToolCallState::Running => "running",
            AgentToolCallState::Cancelling => "cancelling",
            AgentToolCallState::Ok => "ok",
            AgentToolCallState::Failed => "failed",
        },
        AgentConversationEntry::Approval { state, .. } => match state {
            AgentApprovalState::Pending => "pending",
            AgentApprovalState::Resolved => "resolved",
            AgentApprovalState::Withdrawn(_) => "withdrawn",
        },
    }
}

/// 💬️ One conversation row — the wgpu twin of React's `AgentChatEntry`: role noun, state chip, then
/// the body this kind carries (the human's text, the approval's summary, or the tool's name, its real
/// arguments and — once the result frame lands — its summary behind the `Result` noun).
///
/// 🏷️ The leading role line carries React's own two row attributes, so a probe (and the accessibility
/// mirror `dumpAccessibility` publishes) can tell a tool call from an approval, and a running call
/// from a failed one, WITHOUT reading the translated noun — which is what React's `li` does.
pub fn agent_chat_entry_node(entry: &semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry, is_de: bool, locale: Locale) -> UiNode {
        use semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry;
    let mut role = UiTextNode { presence: UiPresence::default(), value: Label::data(agent_chat_role_label(entry, is_de)), emphasize: Some(false), data_attributes: None, menu: None };
    role.data_attributes = Some(HashMap::from([("data-semio-agent-chat-entry".to_string(), agent_chat_entry_kind(entry).to_string()), ("data-agent-chat-state".to_string(), agent_chat_entry_state_attribute(entry).to_string())]));
    let mut rows = vec![UiNode::Text(role)];
    if let Some(state) = agent_chat_state_label(entry, is_de, locale) {
        rows.push(settings_text_row(&state));
    }
    if matches!(entry, AgentConversationEntry::ToolCall { state: semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentToolCallState::Running, .. }) {
        rows.push(UiNode::Button(UiButtonNode {
            id: Some(format!("framework.chat.cancel.{}", entry.id())),
            icon_id: IconName::Square,
            label: Label::data(shell_chrome_string("chat.cancel", is_de)),
            action: ActionDescriptor { controller_id: "framework".into(), action: "cancelToolCall".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "invocationId": entry.id().to_string() }) },
            style: Some(semio_framework_ui_declaration::ui::StyleSpec { variant: Some("ghost".into()), size: None, density: None }),
            presence: UiPresence::default(),
            menu: None,
        }));
    }
    match entry {
        AgentConversationEntry::UserMessage { text, .. } => rows.push(settings_text_row(text)),
        AgentConversationEntry::AgentMessage { text, .. } => rows.push(settings_text_row(text)),
        AgentConversationEntry::Approval { summary, .. } => rows.push(settings_text_row(summary)),
        AgentConversationEntry::ToolCall { tool_name, arguments, summary, .. } => {
            rows.push(settings_text_row(tool_name));
            if !arguments.is_empty() {
                rows.push(settings_text_row(arguments));
            }
            if let Some(summary) = summary {
                rows.push(settings_text_row(&format!("{}: {summary}", shell_chrome_string("chat.toolResultRole", is_de))));
            }
        }
    }
    UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some(agent_chat_entry_id(entry)), children: rows, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
}

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

