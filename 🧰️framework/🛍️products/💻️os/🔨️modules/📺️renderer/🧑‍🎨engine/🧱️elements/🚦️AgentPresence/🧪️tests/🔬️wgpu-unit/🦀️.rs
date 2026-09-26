//! 🧪️ wgpu `🚦️AgentPresence` unit tests — the frame→indicator mapping, checked against the React
//! twin's own `agentPresenceTone` truth table and its `os.agent.presence.*` copy.

use super::*;
use crate::agent_bridge::GatewayToShell;
use ui_wgpu::wgpu::Theme;

fn presence(active: bool, label: &str) -> AgentBridgePresence {
    AgentBridgePresence { active, label: label.into(), invocation_id: None }
}

#[test]
fn every_status_presence_pair_maps_to_the_react_tone() {
    let idle = presence(false, "");
    let busy = presence(true, "compile");
    let table = [
        (AgentBridgeStatus::Disabled, &idle, AgentPresenceTone::Disconnected),
        (AgentBridgeStatus::Closed, &idle, AgentPresenceTone::Disconnected),
        (AgentBridgeStatus::Closed, &busy, AgentPresenceTone::Disconnected),
        (AgentBridgeStatus::Connecting, &idle, AgentPresenceTone::Connecting),
        (AgentBridgeStatus::Reconnecting, &busy, AgentPresenceTone::Connecting),
        (AgentBridgeStatus::Open, &busy, AgentPresenceTone::Working),
        (AgentBridgeStatus::Open, &idle, AgentPresenceTone::Connected),
        (AgentBridgeStatus::Unavailable, &busy, AgentPresenceTone::Blocked),
        (AgentBridgeStatus::Incompatible(crate::agent_bridge::AgentBridgeVersionMismatch { gateway: 2, shell: 1 }), &idle, AgentPresenceTone::Blocked),
    ];
    for (status, presence, expected) in table {
        assert_eq!(agent_presence_tone(status, presence), expected, "{status:?} + active={}", presence.active);
    }
}

#[test]
fn a_presence_frame_moves_the_indicator_from_connected_to_working_and_back() {
    let mut bridge = crate::agent_bridge::AgentBridgeState::default();
    bridge.apply_frame(GatewayToShell::Welcome { bridge_version: 1, connection: "c".into(), principal: "p".into() }, 0.0);
    assert_eq!(agent_presence_tone(bridge.status, &bridge.presence), AgentPresenceTone::Connected);
    bridge.apply_frame(GatewayToShell::AgentPresence { active: true, label: "claude-code".into(), invocation_id: Some("inv-1".into()) }, 1.0);
    assert_eq!(agent_presence_tone(bridge.status, &bridge.presence), AgentPresenceTone::Working);
    bridge.apply_frame(GatewayToShell::AgentPresence { active: false, label: String::new(), invocation_id: None }, 2.0);
    assert_eq!(agent_presence_tone(bridge.status, &bridge.presence), AgentPresenceTone::Connected);
    bridge.apply_frame(GatewayToShell::Bye { reason: "shutdown".into() }, 3.0);
    assert_eq!(agent_presence_tone(bridge.status, &bridge.presence), AgentPresenceTone::Disconnected);
}

#[test]
fn a_dropped_socket_forgets_the_working_label() {
    let mut bridge = crate::agent_bridge::AgentBridgeState::default();
    bridge.apply_frame(GatewayToShell::Welcome { bridge_version: 1, connection: "c".into(), principal: "p".into() }, 0.0);
    bridge.apply_frame(GatewayToShell::AgentPresence { active: true, label: "compile".into(), invocation_id: None }, 1.0);
    bridge.note_socket_closed();
    assert_eq!(agent_presence_tone(bridge.status, &bridge.presence), AgentPresenceTone::Connecting);
    assert!(!bridge.presence.active, "a closed socket must not keep claiming the agent is working");
}

#[test]
fn the_four_tones_take_four_distinct_theme_tokens() {
    let theme = Theme::dark();
    let colors = [AgentPresenceTone::Working, AgentPresenceTone::Connected, AgentPresenceTone::Connecting, AgentPresenceTone::Disconnected].map(|tone| agent_presence_color(tone, &theme));
    assert_eq!(colors[0], theme.success);
    assert_eq!(colors[1], theme.accent);
    assert_eq!(colors[2], theme.warning);
    assert_eq!(colors[3], theme.text_muted);
    assert_eq!(agent_presence_color(AgentPresenceTone::Blocked, &theme), theme.error);
}

#[test]
fn status_text_follows_the_react_precedence_in_both_locales() {
    let busy = presence(true, "compile");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Reconnecting, &busy, Locale::En), "Reconnecting to agent…");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Connecting, &busy, Locale::En), "Connecting to agent…");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Open, &busy, Locale::En), "Agent working: compile");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Open, &presence(false, ""), Locale::En), "Agent idle");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Closed, &busy, Locale::En), "Agent disconnected");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Open, &busy, Locale::De), "Agent aktiv: compile");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Closed, &busy, Locale::De), "Agent getrennt");
    let incompatible = AgentBridgeStatus::Incompatible(crate::agent_bridge::AgentBridgeVersionMismatch { gateway: 2, shell: 1 });
    assert_eq!(agent_presence_text(AgentBridgeStatus::Unavailable, &busy, Locale::En), "The AI client's bridge does not answer; restart the AI client to connect again");
    assert_eq!(agent_presence_text(AgentBridgeStatus::Unavailable, &busy, Locale::De), "Die Brücke des KI-Clients antwortet nicht; starte den KI-Client neu, um erneut zu verbinden");
    assert_eq!(agent_presence_text(incompatible, &busy, Locale::En), "The AI client uses bridge version 2, this shell version 1; update the older one");
    assert_eq!(agent_presence_text(incompatible, &busy, Locale::De), "Der KI-Client nutzt Brückenversion 2, diese Oberfläche Version 1; aktualisiere die ältere");
}

#[test]
fn the_tone_string_matches_the_react_data_attribute_vocabulary() {
    assert_eq!(AgentPresenceTone::Connected.as_str(), "connected");
    assert_eq!(AgentPresenceTone::Working.as_str(), "working");
    assert_eq!(AgentPresenceTone::Connecting.as_str(), "connecting");
    assert_eq!(AgentPresenceTone::Disconnected.as_str(), "disconnected");
    assert_eq!(AgentPresenceTone::Blocked.as_str(), "blocked");
}
