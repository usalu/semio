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

//#region 💬️AgentChatTranscript
// 💬️ WGPU-RENDERER-REACT-PARITY packet W15e, audit item 4/10. CORRECTION to the audit's premise: the
// transcript itself is NOT missing — W2c built `build_agent_chat_ui` out of the live bridge
// conversation, so the panel body has been real `UiNode` rows since then, and the "free-content
// escape hatch" the audit asked to scope turned out not to be needed at all. What WAS missing is
// proven here: React's feed container identity, its two per-row data attributes, the newest-window
// paging its auto-scrolled `<ol>` produces, and Enter-to-send on the composer.

fn chat_feed_node(shell: &ShellState) -> UiNode {
    let body = shell.build_agent_chat_ui();
    let UiNode::Stack(panel) = body else { panic!("the chat panel body is a stack") };
    panel.children.into_iter().find(|child| matches!(child, UiNode::Stack(stack) if stack.id.as_deref() == Some("framework.chat.feed"))).expect("the feed is its own node under React's id")
}

fn chat_feed_rows(shell: &ShellState) -> Vec<UiNode> {
    let UiNode::Stack(feed) = chat_feed_node(shell) else { panic!("the feed is a stack") };
    feed.children
}

fn row_attributes(row: &UiNode) -> std::collections::HashMap<String, String> {
    let UiNode::Stack(stack) = row else { panic!("a transcript row is a stack") };
    let UiNode::Text(role) = stack.children.first().expect("a row leads with its role line") else { panic!("the role line is text") };
    role.data_attributes.clone().unwrap_or_default()
}

#[test]
fn an_empty_transcript_paints_reacts_own_empty_line_inside_the_feed() {
    let shell = ShellState::new(Vec::new(), String::new());
    let rows = chat_feed_rows(&shell);
    assert_eq!(rows.len(), 1, "the empty state is one line, in the feed, not loose in the panel");
    assert!(matches!(&rows[0], UiNode::Text(text) if text.value.as_str().contains("No agent activity yet")));
}

#[test]
fn every_row_carries_reacts_two_data_attributes() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.apply_agent_bridge_frame(&GatewayToShell::AgentToolCall { invocation_id: "inv_1".into(), tool_name: "artifact.mutate".into(), arguments: "{}".into() }.encode()).expect("frame decodes");
    shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "let me".into() }.encode()).expect("frame decodes");
    let rows = chat_feed_rows(&shell);
    assert_eq!(rows.len(), 2);
    assert_eq!(row_attributes(&rows[0]).get("data-semio-agent-chat-entry").map(String::as_str), Some("toolCall"));
    assert_eq!(row_attributes(&rows[0]).get("data-agent-chat-state").map(String::as_str), Some("running"));
    assert_eq!(row_attributes(&rows[1]).get("data-semio-agent-chat-entry").map(String::as_str), Some("approval"));
    assert_eq!(row_attributes(&rows[1]).get("data-agent-chat-state").map(String::as_str), Some("pending"));
}

#[test]
fn a_tool_result_settles_its_own_row_in_place_rather_than_opening_a_second_one() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.apply_agent_bridge_frame(&GatewayToShell::AgentToolCall { invocation_id: "inv_1".into(), tool_name: "artifact.mutate".into(), arguments: "{\"id\":1}".into() }.encode()).expect("frame decodes");
    shell.apply_agent_bridge_frame(&GatewayToShell::AgentToolResult { invocation_id: "inv_1".into(), tool_name: "artifact.mutate".into(), ok: false, summary: "refused".into() }.encode()).expect("frame decodes");
    let rows = chat_feed_rows(&shell);
    assert_eq!(rows.len(), 1, "one call, one row");
    assert_eq!(row_attributes(&rows[0]).get("data-agent-chat-state").map(String::as_str), Some("failed"));
}

#[test]
fn a_resolved_approval_reports_its_own_decision_state() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "mutate".into() }.encode()).expect("frame decodes");
    shell.apply_agent_bridge_frame(&GatewayToShell::ApprovalResolved { approval_id: "appr_1".into(), decision: ApprovalDecision::Once }.encode()).expect("frame decodes");
    assert_eq!(row_attributes(&chat_feed_rows(&shell)[0]).get("data-agent-chat-state").map(String::as_str), Some("resolved"));
}

#[test]
fn the_feed_paints_the_newest_window_the_way_reacts_auto_scrolled_list_shows_it() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    for index in 0..(AGENT_CHAT_VISIBLE_ENTRIES + 6) {
        shell.apply_agent_bridge_frame(&GatewayToShell::AgentToolCall { invocation_id: format!("inv_{index}"), tool_name: format!("tool-{index}"), arguments: String::new() }.encode()).expect("frame decodes");
    }
    let rows = chat_feed_rows(&shell);
    assert_eq!(rows.len(), AGENT_CHAT_VISIBLE_ENTRIES, "the window is bounded, not the conversation");
    assert_eq!(shell.chrome_build.agent.conversation.len(), AGENT_CHAT_VISIBLE_ENTRIES + 6, "nothing is discarded, only unpainted");
    // 🔚️ The LAST painted row is the newest entry, which is what a bottom-pinned feed shows.
    let UiNode::Stack(last) = rows.last().expect("a row") else { panic!("a row is a stack") };
    assert_eq!(last.id.as_deref(), Some(format!("framework.chat.entry.toolCall.inv_{}", AGENT_CHAT_VISIBLE_ENTRIES + 5).as_str()));
}

#[test]
fn the_composer_sends_on_enter_the_way_reacts_textarea_does() {
    let shell = ShellState::new(Vec::new(), String::new());
    let UiNode::Stack(panel) = shell.build_agent_chat_ui() else { panic!("the chat panel body is a stack") };
    let draft = panel
        .children
        .iter()
        .find_map(|child| match child {
            UiNode::Input(input) if input.id == "framework.chat.draft" => Some(input.clone()),
            _ => None,
        })
        .expect("the composer is in the panel");
    assert_eq!(draft.commit.as_deref(), Some("enter"));
    let submit = draft.on_submit.as_ref().expect("Enter dispatches a verb");
    assert_eq!(submit.action, "sendChatDraft", "Enter and the Send button are one code path");
    assert_eq!(submit.controller_id, "framework");
}

#[test]
fn the_chat_composer_projects_its_explicit_localized_accessible_name_without_using_the_placeholder() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/💬️chat-input-accessibility/🔣️.json")).expect("chat accessibility fixture");
    for row in fixture["cases"].as_array().expect("chat cases") {
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.locale_id = row["locale"].as_str().expect("locale").into();
        if row["bridge"] == "open" {
            shell.apply_agent_bridge_frame(&GatewayToShell::Welcome { bridge_version: 1, connection: "fixture".into(), principal: "agent".into() }.encode()).expect("open bridge fixture");
        }
        let records = panel_ui_records(FRAMEWORK_CHAT_PANEL_ID, &shell.build_agent_chat_ui()).expect("Chat projects to retained records");
        let draft = records.iter().find(|record| record.key.as_str().ends_with("/framework.chat.draft")).expect("retained Chat draft");
        let expected_name = row["accessibleName"].as_str().expect("accessible name");
        let expected_placeholder = row["placeholder"].as_str().expect("placeholder");
        assert_ne!(expected_name, expected_placeholder, "the fixture refuses placeholder-as-name compatibility");
        assert_eq!(draft.accessibility.label.as_ref().map(|label| label.0.as_str()), Some(expected_name), "{}:{} explicit accessible label", row["locale"], row["bridge"]);
        assert_eq!(draft.disabled, !row["enabled"].as_bool().expect("enabled"));
    }
}
//#endregion 💬️AgentChatTranscript

//#region 🛂️SpaceAdministrationSheet
/// 🪪️ The minimal signed-in identity the administration lane gates on — `open_space_administration`
/// refuses outright without one, so every sheet law needs exactly this and nothing more.
fn test_identity() -> Identity {
    Identity { user_id: "user-1".into(), email: "user-1@example.test".into(), display_name: "User One".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 0 }
}

// 🛂️ WGPU-RENDERER-REACT-PARITY packet W15e, audit item 3. The operation and the control set have
// been live on both targets since W1e; these are the laws of the CHROME nobody had written, and of
// the one authority rule the chrome must not break.

#[test]
fn no_operation_means_no_sheet_exactly_as_react_mounts_nothing() {
    let shell = ShellState::new(Vec::new(), String::new());
    assert!(shell.space_administration_plan().is_none());
}

#[test]
fn opening_without_an_identity_mounts_nothing_rather_than_an_empty_sheet() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.open_space_administration("space-1");
    assert!(shell.space_administration_plan().is_none(), "administration needs a signed-in identity, and says so by showing nothing");
}

#[test]
fn the_close_control_retires_the_operation() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.identity = Some(test_identity());
    shell.open_space_administration("space-1");
    assert!(shell.space_administration_plan().is_some(), "an identity plus a space id mounts the sheet");
    shell.resolve_space_administration_control(crate::space_administration::SPACE_ADMINISTRATION_CLOSE_CONTROL_ID);
    assert!(shell.space_administration_plan().is_none());
}

#[test]
fn the_sheet_leads_with_reacts_own_title_and_the_operations_live_status() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.identity = Some(test_identity());
    shell.open_space_administration("space-7");
    let plan = shell.space_administration_plan().expect("a sheet");
    assert_eq!(plan.space_id, "space-7");
    assert_eq!(plan.title, "Space administration");
    assert_eq!(plan.status, shell_space_administration_status(ShellSpaceAdministrationPhaseV1::Loading, false), "the status line is the phase, not a row");
    assert!(plan.rows.iter().all(|row| row.control_id != "os.space-administration.status"), "the status control is lifted out of the roster");
}

#[test]
fn the_invite_copy_control_is_withheld_rather_than_offered_and_silently_losing_the_capability() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.identity = Some(test_identity());
    shell.open_space_administration("space-1");
    let plan = shell.space_administration_plan().expect("a sheet");
    assert!(plan.rows.iter().all(|row| row.control_id != "os.space-administration.invite.copy"));
}
//#endregion 🛂️SpaceAdministrationSheet
