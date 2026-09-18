//! 💬️ wgpu twin of the `💬️AgentChatPanel` element (`🟦️.tsx`, 30 lines) — the OS shell's chat dock:
//! a header carrying the panel title and the `🚦️AgentPresence` indicator, over the chat transcript.
//!
//! 🧩️ Scope, stated plainly: the header half is real here (it is chrome, and the wgpu shell paints
//! chrome directly), the transcript half is not. React's body is `BasicChatPanel`, an arbitrary
//! React subtree hosted inside a panel through `Tree`'s `emptyState` escape hatch
//! (`📌️ChromePanels/🟦️.tsx:1376-1391`). The wgpu panel pipeline has no such hatch — panel content is
//! exclusively `UiNode`/`UiTree` rendered by the fixed-credit `MountedLayout` engine — so a
//! transcript cannot be projected until the content-projection packet lands (shell audit §5,
//! recommendation 7, which names `RightPanelKind::Chat` as its own forcing case). The header
//! therefore also renders the empty-transcript line, so the panel is never blank.

use crate::agent_bridge::{agent_label, AgentBridgePresence, AgentBridgeStatus};
use crate::agent_presence::{agent_presence_color, agent_presence_text, agent_presence_tone, AGENT_PRESENCE_DOT_PX, AGENT_PRESENCE_GAP_PX};
use ui_wgpu::wgpu::{Locale, Rect, Rgba, Theme};

//#region 🔖️AgentChatPanel
/// 🆔️ The chat panel tab this element fills — `BasicChatPanel id="framework.chat"` on the React side.
pub const AGENT_CHAT_PANEL_ID: &str = "framework.chat";

pub fn agent_chat_panel_title(locale: Locale) -> String {
    agent_label("Chat", "Chat", locale)
}

/// 💬️ Shown while the transcript has nothing in it — and, today, whenever the transcript cannot be
/// projected at all (see the module docstring).
pub fn agent_chat_empty_text(locale: Locale) -> String {
    agent_label("No messages yet", "Noch keine Nachrichten", locale)
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
