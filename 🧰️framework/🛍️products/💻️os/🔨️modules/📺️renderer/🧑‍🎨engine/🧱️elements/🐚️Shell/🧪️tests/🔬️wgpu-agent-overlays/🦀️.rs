//! 🧪️ WGPU-RENDERER-REACT-PARITY packet W1j — the shell-side half of the three overlays React has
//! and this shell did not: the transient notice banner, the agent approvals modal, and the agent
//! presence indicator in the chat panel header. The element-side halves (frame codec, tone map,
//! summary parsing, layout) are covered by each element's own `🧪️tests/🔬️wgpu-unit`.

use super::*;
use crate::agent_approvals::{approval_decision_control_id, APPROVALS_CLOSE_CONTROL_ID};
use crate::agent_bridge::{ApprovalDecision, GatewayToShell, ShellToGateway};

//#region 🧯️TransientNotice
#[test]
fn a_notice_replaces_the_previous_one_rather_than_queueing() {
    let mut chrome = ShellChromeBuildState::default();
    chrome.show_transient_notice("first", semio_framework::Severity::Info, Some("a"), 0.0);
    chrome.show_transient_notice("second", semio_framework::Severity::Error, Some("b"), 10.0);
    let notice = chrome.transient_notice.as_ref().expect("one notice is showing");
    assert_eq!(notice.message, "second", "React's showTransientNotice replaces, it never queues");
    assert_eq!(notice.severity, semio_framework::Severity::Error);
    assert_eq!(notice.code.as_deref(), Some("b"));
}

#[test]
fn a_notice_auto_clears_exactly_at_the_four_second_deadline() {
    let mut chrome = ShellChromeBuildState::default();
    chrome.show_transient_notice("rejected", semio_framework::Severity::Warning, None, 1_000.0);
    assert!(chrome.expire_transient_notice(1_000.0 + TRANSIENT_NOTICE_AUTO_DISMISS_MS - 1.0), "still showing one millisecond before the deadline");
    assert!(!chrome.expire_transient_notice(1_000.0 + TRANSIENT_NOTICE_AUTO_DISMISS_MS), "cleared at the deadline");
    assert!(chrome.transient_notice.is_none());
}

#[test]
fn a_notice_is_dismissible_before_its_deadline() {
    let mut chrome = ShellChromeBuildState::default();
    chrome.show_transient_notice("read-only", semio_framework::Severity::Info, None, 0.0);
    chrome.dismiss_transient_notice();
    assert!(!chrome.expire_transient_notice(1.0));
}

#[test]
fn the_replacement_restarts_the_auto_dismiss_clock() {
    let mut chrome = ShellChromeBuildState::default();
    chrome.show_transient_notice("first", semio_framework::Severity::Info, None, 0.0);
    chrome.show_transient_notice("second", semio_framework::Severity::Info, None, 3_900.0);
    assert!(chrome.expire_transient_notice(4_000.0), "the second notice gets its own full 4 s, not the first's remainder");
    assert!(!chrome.expire_transient_notice(7_900.0));
}

#[test]
fn every_severity_takes_a_themed_tone_and_error_and_fatal_share_the_destructive_one() {
    use semio_framework::Severity;
    let theme = Theme::dark();
    let info = transient_notice_tone(Severity::Info, &theme);
    let warning = transient_notice_tone(Severity::Warning, &theme);
    let error = transient_notice_tone(Severity::Error, &theme);
    let fatal = transient_notice_tone(Severity::Fatal, &theme);
    assert_eq!(info.0, theme.border_normal);
    assert_eq!(warning.0, theme.warning);
    assert_eq!(error, fatal, "TRANSIENT_NOTICE_TONE_CLASS gives error and fatal the same destructive chrome");
    assert_eq!(error.0, theme.error);
    assert_ne!(info.1, warning.1);
}

#[test]
fn the_banner_sits_top_centre_eight_pixels_below_the_navbar() {
    let theme = Theme::dark();
    let notice = ShellTransientNotice { message: "your change was rejected".into(), severity: semio_framework::Severity::Error, code: None, shown_at_ms: 0.0 };
    let banner = transient_notice_rect(&notice, "Close", 1280.0, &theme);
    assert_eq!(banner.y, theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP);
    assert!((banner.x + banner.w * 0.5 - 640.0).abs() < 0.001, "horizontally centred like React's left-1/2 -translate-x-1/2");
    let close = transient_notice_close_rect(banner, "Close", &theme);
    assert!(close.x + close.w <= banner.x + banner.w, "the close control stays inside the banner");
    assert!(close.x > banner.x);
}

#[test]
fn the_banner_never_overflows_a_narrow_viewport() {
    let theme = Theme::dark();
    let notice = ShellTransientNotice { message: "x".repeat(400), severity: semio_framework::Severity::Info, code: None, shown_at_ms: 0.0 };
    let banner = transient_notice_rect(&notice, "Close", 320.0, &theme);
    assert!(banner.x >= 0.0 && banner.x + banner.w <= 320.0);
}

#[test]
fn dispatch_faults_are_classified_the_way_reacts_three_call_sites_classify_them() {
    use semio_framework::Severity;
    let (message, severity, code) = classify_dispatch_fault_notice("guest refused: viewer.read-only", Locale::En);
    assert_eq!(severity, Severity::Info);
    assert_eq!(code, Some(VIEWER_READ_ONLY_FAULT_CODE));
    assert!(message.contains("read-only viewer"));

    let (_, severity, code) = classify_dispatch_fault_notice("mutation.rejected: conflicting edit", Locale::En);
    assert_eq!(severity, Severity::Error);
    assert_eq!(code, Some(MUTATION_REJECTED_FAULT_CODE));

    let (message, severity, code) = classify_dispatch_fault_notice("surface render failed", Locale::En);
    assert_eq!(severity, Severity::Error);
    assert_eq!(code, None);
    assert_eq!(message, "surface render failed");

    let (message, _, _) = classify_dispatch_fault_notice("viewer.read-only", Locale::De);
    assert!(message.contains("schreibgeschützter"), "no default language — the German copy is real");
}

#[test]
fn a_failed_dispatch_raises_a_banner_instead_of_the_persistent_error_line() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.note_dispatch_fault("guest refused: viewer.read-only");
    let notice = shell.transient_notice().expect("a banner is showing");
    assert_eq!(notice.severity, semio_framework::Severity::Info);
    assert_eq!(notice.code.as_deref(), Some(VIEWER_READ_ONLY_FAULT_CODE));
    assert!(shell.error.is_none(), "a transient fault must not park in the persistent error line");
}
//#endregion 🧯️TransientNotice

//#region ✅️AgentApprovals
#[test]
fn a_simulated_approval_requested_frame_opens_the_modal() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let summary = r#"{"capabilityId":"artifact.mutate","diffSummary":"+3 −1","risk":"high","requestedBy":"agent:local"}"#;
    shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: summary.into() }.encode()).expect("frame decodes");
    assert!(shell.chrome_build.agent.has_pending_approvals());
    shell.chrome_build.agent_approvals.observe(&shell.chrome_build.agent.pending_approvals);
    assert!(shell.chrome_build.agent_approvals.is_open(&shell.chrome_build.agent.pending_approvals));
}

#[test]
fn the_modal_paints_the_capability_diff_and_risk_of_every_parked_request() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let summary = r#"{"capabilityId":"artifact.mutate","diffSummary":"+3 −1","risk":"high","requestedBy":"agent:local"}"#;
    shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: summary.into() }.encode()).expect("frame decodes");
    let theme = Theme::dark();
    let ops = shell.agent_approvals_paint_ops(1280.0, 800.0, &theme);
    let painted: Vec<&str> = ops
        .iter()
        .filter_map(|op| match op {
            AgentApprovalPaintOp::Text { value, .. } => Some(value.as_str()),
            _ => None,
        })
        .collect();
    let joined = painted.join(" | ");
    assert!(joined.contains("artifact.mutate"), "capability: {joined}");
    assert!(joined.contains("+3 −1"), "change summary: {joined}");
    assert!(joined.contains("agent:local"), "requested by: {joined}");
    assert!(joined.contains("High"), "risk badge: {joined}");
    for decision in ApprovalDecision::ALL {
        let id = approval_decision_control_id("appr_1", decision);
        assert!(ops.iter().any(|op| matches!(op, AgentApprovalPaintOp::Hit { control_id, .. } if control_id == &id)), "{decision:?} button is hit-testable");
    }
    assert!(ops.iter().any(|op| matches!(op, AgentApprovalPaintOp::Hit { control_id, .. } if control_id == APPROVALS_CLOSE_CONTROL_ID)));
}

#[test]
fn each_decision_button_emits_the_matching_approval_resolved_frame() {
    for decision in ApprovalDecision::ALL {
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "translate".into() }.encode()).expect("frame decodes");
        shell.resolve_agent_approval_control(&approval_decision_control_id("appr_1", decision));
        assert!(!shell.chrome_build.agent.has_pending_approvals(), "{decision:?} clears the request");
        let outbound = shell.take_agent_bridge_outbox();
        assert_eq!(outbound.len(), 1, "{decision:?} emits exactly one frame");
        let frame = ShellToGateway::decode(&outbound[0]).expect("the emitted frame is on the wire format");
        assert_eq!(frame, ShellToGateway::Approval { approval_id: "appr_1".into(), decision, note: None });
    }
}

#[test]
fn dismissing_the_modal_leaves_the_request_parked() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "translate".into() }.encode()).expect("frame decodes");
    shell.chrome_build.agent_approvals.observe(&shell.chrome_build.agent.pending_approvals);
    shell.resolve_agent_approval_control(APPROVALS_CLOSE_CONTROL_ID);
    assert!(shell.chrome_build.agent.has_pending_approvals(), "dismissing is not deciding");
    assert!(!shell.chrome_build.agent_approvals.is_open(&shell.chrome_build.agent.pending_approvals));
    assert_eq!(shell.take_agent_bridge_outbox().len(), 0, "a dismissal never answers the gateway");
}

#[test]
fn a_plain_text_summary_still_lists_a_change_summary_line() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "translate selection".into() }.encode()).expect("frame decodes");
    let theme = Theme::dark();
    let ops = shell.agent_approvals_paint_ops(1280.0, 800.0, &theme);
    assert!(ops.iter().any(|op| matches!(op, AgentApprovalPaintOp::Text { value, .. } if value.contains("translate selection"))));
}
//#endregion ✅️AgentApprovals

//#region 🚦️AgentPresence
#[test]
fn presence_frames_move_the_shells_own_indicator() {
    use crate::agent_presence::AgentPresenceTone;
    let mut shell = ShellState::new(Vec::new(), String::new());
    assert_eq!(shell.agent_presence_tone(), AgentPresenceTone::Disconnected);
    shell.apply_agent_bridge_frame(&GatewayToShell::Welcome { bridge_version: 1, connection: "c".into(), principal: "p".into() }.encode()).expect("frame decodes");
    assert_eq!(shell.agent_presence_tone(), AgentPresenceTone::Connected);
    shell.apply_agent_bridge_frame(&GatewayToShell::AgentPresence { active: true, label: "compile".into(), invocation_id: None }.encode()).expect("frame decodes");
    assert_eq!(shell.agent_presence_tone(), AgentPresenceTone::Working);
}

#[test]
fn the_chat_header_plan_puts_the_dot_and_status_inside_the_panel() {
    let theme = Theme::dark();
    let panel = Rect::new(900.0, 40.0, 320.0, 700.0);
    let plan = crate::agent_chat_panel::plan_agent_chat_header(panel, crate::agent_bridge::AgentBridgeStatus::Open, &crate::agent_bridge::AgentBridgePresence::default(), &theme, Locale::En);
    assert_eq!(plan.title, "Chat");
    assert_eq!(plan.status_text, "Agent idle");
    assert_eq!(plan.dot_color, theme.accent);
    assert!(plan.dot.x >= panel.x && plan.dot.x + plan.dot.w <= panel.x + panel.w);
    assert!(plan.header.y == panel.y && plan.header.w == panel.w);
}
//#endregion 🚦️AgentPresence

//#region 💡️Tooltip
#[test]
fn the_tooltip_delay_matches_the_react_chrome_control_hint() {
    assert_eq!(CHROME_TOOLTIP_DELAY_MS, 400.0, "CHROME_CONTROL_TOOLTIP_DELAY_MS in 💡️ChromeControlHint/🟦️.tsx");
    let hover = ChromeTooltipHover { control_id: "nav.help".into(), anchor_x: 0.0, anchor_y: 0.0, started_ms: 1_000.0 };
    assert!(!chrome_tooltip_ready(&hover, 1_399.0));
    assert!(chrome_tooltip_ready(&hover, 1_400.0));
}
//#endregion 💡️Tooltip
