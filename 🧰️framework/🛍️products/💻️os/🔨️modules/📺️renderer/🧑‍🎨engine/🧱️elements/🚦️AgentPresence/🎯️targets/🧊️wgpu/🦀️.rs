//! 🚦️ wgpu twin of the `🚦️AgentPresence` element (`🟦️.tsx`, 48 lines) — the small connected/
//! working/idle/disconnected indicator for the MCP agent session, driven by `🔗️AgentBridge`'s
//! `agentPresence` frames. Distinct from `👥️PresenceBar`, which is the HUMAN collaborator roster
//! and already has its own wgpu twin under `🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar`.
//!
//! [`agent_presence_tone`] is a byte-for-byte port of the React file's own pure `agentPresenceTone`
//! (the React twin factored it out for exactly this reason: "pure so it is directly unit-testable
//! without rendering"), and the four tones map onto shared `Theme` tokens rather than the React
//! file's literal Tailwind palette classes — `emerald/sky/amber/muted-foreground` are the light-mode
//! spellings of `success`/`accent`/`warning`/`text_muted`, which is what a themed renderer must read.

use crate::agent_bridge::{agent_label, AgentBridgePresence, AgentBridgeStatus};
use ui_wgpu::wgpu::{Locale, Rgba, Theme};

//#region 🔖️AgentPresence
/// 🚦️ The four visual states the indicator collapses the (status, presence) pair into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentPresenceTone {
    Connected,
    Working,
    Connecting,
    Disconnected,
    Blocked,
}

impl AgentPresenceTone {
    /// 🏷️ The `data-semio-agent-presence-tone` value the React twin stamps — the same string a
    /// wgpu-side probe reads off the registered control id, so one assertion covers both shells.
    pub fn as_str(self) -> &'static str {
        match self {
            AgentPresenceTone::Connected => "connected",
            AgentPresenceTone::Working => "working",
            AgentPresenceTone::Connecting => "connecting",
            AgentPresenceTone::Disconnected => "disconnected",
            AgentPresenceTone::Blocked => "blocked",
        }
    }
}

/// 🚦️ Dot tone for the current bridge status/presence combination — pure, so it is unit-testable
/// without a `DrawList`/`FontAtlas` fixture, exactly like its React original.
pub fn agent_presence_tone(status: AgentBridgeStatus, presence: &AgentBridgePresence) -> AgentPresenceTone {
    match status {
        AgentBridgeStatus::Unavailable | AgentBridgeStatus::Incompatible(_) => AgentPresenceTone::Blocked,
        AgentBridgeStatus::Disabled | AgentBridgeStatus::Closed => AgentPresenceTone::Disconnected,
        AgentBridgeStatus::Connecting | AgentBridgeStatus::Reconnecting => AgentPresenceTone::Connecting,
        AgentBridgeStatus::Open if presence.active => AgentPresenceTone::Working,
        AgentBridgeStatus::Open => AgentPresenceTone::Connected,
    }
}

/// 🎨️ The dot's fill, taken from the shared theme tokens rather than a literal palette.
pub fn agent_presence_color(tone: AgentPresenceTone, theme: &Theme) -> Rgba {
    match tone {
        AgentPresenceTone::Working => theme.success,
        AgentPresenceTone::Connected => theme.accent,
        AgentPresenceTone::Connecting => theme.warning,
        AgentPresenceTone::Disconnected => theme.text_muted,
        AgentPresenceTone::Blocked => theme.error,
    }
}

/// 🗣️ The status line beside the dot — the React twin's exact precedence: an in-flight
/// (re)connection names itself before the tone does, and `working` interpolates the invocation
/// label the way `os.agent.presence.working`'s `{{label}}` placeholder does.
pub fn agent_presence_text(status: AgentBridgeStatus, presence: &AgentBridgePresence, locale: Locale) -> String {
    match (status, agent_presence_tone(status, presence)) {
        (AgentBridgeStatus::Incompatible(mismatch), _) => agent_label("The AI client uses bridge version {{gateway}}, this shell version {{shell}}; update the older one", "Der KI-Client nutzt Brückenversion {{gateway}}, diese Oberfläche Version {{shell}}; aktualisiere die ältere", locale).replace("{{gateway}}", &mismatch.gateway.to_string()).replace("{{shell}}", &mismatch.shell.to_string()),
        (_, AgentPresenceTone::Blocked) => agent_label("The AI client's bridge does not answer; restart the AI client to connect again", "Die Brücke des KI-Clients antwortet nicht; starte den KI-Client neu, um erneut zu verbinden", locale),
        (AgentBridgeStatus::Reconnecting, _) => agent_label("Reconnecting to agent…", "Verbindung zum Agent wird wiederhergestellt…", locale),
        (AgentBridgeStatus::Connecting, _) => agent_label("Connecting to agent…", "Verbinde mit Agent…", locale),
        (_, AgentPresenceTone::Disconnected) => agent_label("Agent disconnected", "Agent getrennt", locale),
        (_, AgentPresenceTone::Working) => {
            let working = agent_label("Agent working", "Agent aktiv", locale);
            format!("{working}: {}", presence.label)
        }
        (_, AgentPresenceTone::Connected | AgentPresenceTone::Connecting) => agent_label("Agent idle", "Agent inaktiv", locale),
    }
}

/// ♿️ The indicator's accessible name (`role="status"` + `aria-label` on the React side), reused
/// here as the registered control id's tooltip title.
pub fn agent_presence_status_label(locale: Locale) -> String {
    agent_label("Agent status", "Agent-Status", locale)
}

/// 📐️ Dot diameter — `h-2 w-2` in the React twin.
pub const AGENT_PRESENCE_DOT_PX: f32 = 8.0;

/// 📐️ Dot→text gap — `gap-1.5` in the React twin.
pub const AGENT_PRESENCE_GAP_PX: f32 = 6.0;

/// 🆔️ The one control id the indicator registers, so a probe can hit-test it by name.
pub const AGENT_PRESENCE_CONTROL_ID: &str = "shell.agent.presence";
//#endregion 🔖️AgentPresence

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
