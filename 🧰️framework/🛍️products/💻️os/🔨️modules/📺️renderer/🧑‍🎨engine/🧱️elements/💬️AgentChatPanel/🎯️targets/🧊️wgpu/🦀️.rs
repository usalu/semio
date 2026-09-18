//! 💬️ wgpu twin of the `💬️AgentChatPanel` element (`🟦️.tsx`) — the OS shell's agent dock: a header
//! carrying the panel title and the `🚦️AgentPresence` indicator, over the LIVE agent conversation.
//!
//! 🧩️ Split of ownership, stated plainly: this module owns the header band (chrome, which the wgpu
//! shell paints directly) and the empty-transcript line it shows while the feed has nothing in it.
//! The feed itself is a `UiNode` tree the shell assembles from
//! `crate::agent_bridge::AgentBridgeState::conversation` — `🐚️Shell/🎯️targets/🧊️wgpu`'s
//! `build_agent_chat_ui`/`agent_chat_entry_node` — because panel content in this renderer is
//! exclusively `UiNode`/`UiTree` rendered by the fixed-credit `MountedLayout` engine, never an
//! arbitrary subtree like React's `Tree` `emptyState` escape hatch admits.
//!
//! 🌉️ Its rows come from the same bridge frames React's panel reads: `GatewayToShell::AgentToolCall`
//! and `AgentToolResult` from the gateway's own `tools/call` dispatch, `ApprovalRequested`/
//! `ApprovalResolved` from its approval gate, and `ShellToGateway::AgentMessage` for every turn the
//! human types back. Nothing here is generated locally.

use crate::agent_bridge::{agent_label, AgentBridgePresence, AgentBridgeStatus};
use crate::agent_presence::{agent_presence_color, agent_presence_text, agent_presence_tone, AGENT_PRESENCE_DOT_PX, AGENT_PRESENCE_GAP_PX};
use ui_wgpu::wgpu::{Locale, Rect, Rgba, Theme};

//#region 🔖️AgentChatPanel
/// 🆔️ The chat panel tab this element fills — `framework.chat` on the React side.
pub const AGENT_CHAT_PANEL_ID: &str = "framework.chat";

pub fn agent_chat_panel_title(locale: Locale) -> String {
    agent_label("Chat", "Chat", locale)
}

/// 💬️ Shown while the conversation has nothing in it — React's `os.agent.chat.empty`, word for word.
pub fn agent_chat_empty_text(locale: Locale) -> String {
    agent_label(
        "No agent activity yet. Messages you send appear here, along with every tool the agent runs.",
        "Noch keine Agent-Aktivität. Gesendete Nachrichten erscheinen hier, ebenso jedes vom Agent ausgeführte Werkzeug.",
        locale,
    )
}

/// 📐️ The header band at the top of the chat panel — `border-b px-single py-single` on the React
/// side, i.e. one control-height row with the standard padding.
pub fn agent_chat_header_rect(panel: Rect, theme: &Theme) -> Rect {
    Rect::new(panel.x, panel.y, panel.w, theme.control_height + theme.padding_standard)
}

/// 📐️ Where the presence dot sits — right-aligned in the header, as `justify-between` puts it.
/// `text_width` is the measured (or estimated) width of the status text it precedes.
pub fn agent_chat_presence_dot_rect(header: Rect, theme: &Theme, text_width: f32) -> Rect {
    let right = header.x + header.w - theme.padding_standard;
    let dot_x = right - text_width - AGENT_PRESENCE_GAP_PX - AGENT_PRESENCE_DOT_PX;
    Rect::new(dot_x.max(header.x + theme.padding_standard), header.y + (header.h - AGENT_PRESENCE_DOT_PX) * 0.5, AGENT_PRESENCE_DOT_PX, AGENT_PRESENCE_DOT_PX)
}

/// 🧾️ Everything the shell's paint step needs for one frame of this panel's header, resolved in
/// one pure call so the paint step itself stays a flat sequence of draw commands.
pub struct AgentChatHeaderPlan {
    pub header: Rect,
    pub title: String,
    pub status_text: String,
    pub dot_color: Rgba,
    pub dot: Rect,
}

/// 💬️ Resolves this frame's header: title on the left, presence dot + status text on the right.
/// `text_width` is the measured width of `status_text`; callers that have not measured yet pass a
/// monospace estimate, the same approximation `render_chrome_tooltip` already uses.
pub fn plan_agent_chat_header(panel: Rect, status: AgentBridgeStatus, presence: &AgentBridgePresence, theme: &Theme, locale: Locale) -> AgentChatHeaderPlan {
    let header = agent_chat_header_rect(panel, theme);
    let status_text = agent_presence_text(status, presence, locale);
    let text_width = status_text.chars().count() as f32 * theme.font_size_small * 0.6;
    let tone = agent_presence_tone(status, presence);
    AgentChatHeaderPlan { header, title: agent_chat_panel_title(locale), status_text, dot_color: agent_presence_color(tone, theme), dot: agent_chat_presence_dot_rect(header, theme, text_width) }
}
//#endregion 🔖️AgentChatPanel
