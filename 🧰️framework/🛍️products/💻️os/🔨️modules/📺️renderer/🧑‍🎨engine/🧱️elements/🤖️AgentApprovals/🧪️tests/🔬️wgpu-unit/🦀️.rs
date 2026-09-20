//! 🧪️ wgpu `🤖️AgentApprovals` unit tests — summary parsing (ported truth table), the open/dismiss
//! rule, the control-id contract the shell's hit handler depends on, and the modal's geometry.

use super::*;
use crate::agent_bridge::{AgentBridgeState, GatewayToShell};
use ui_wgpu::wgpu::Theme;

/// 🧾️ The shared summary corpus — the same file the React `parseApprovalSummary` asserts against,
/// read at compile time so a moved fixture is a build error, not a skipped test.
const SUMMARY_FIXTURE: &str = include_str!("../../🧫️fixtures/🛡️summary/🔣️.json");

fn parked(id: &str, summary: &str) -> PendingAgentApproval {
    PendingAgentApproval { approval_id: id.into(), summary: summary.into(), requested_at_ms: 0.0 }
}

/// 🧾️ Every row of the shared fixture, parsed on THIS bank exactly as the React bank parses it.
#[test]
fn the_shared_summary_fixture_parses_identically_on_this_bank() {
    let fixture: serde_json::Value = serde_json::from_str(SUMMARY_FIXTURE).expect("the shared summary fixture is JSON");
    for row in ["rich", "plainText", "legacyWithoutTheNewFields"] {
        let case = &fixture[row];
        let parsed = parse_approval_summary(case["summary"].as_str().expect("every row carries a wire summary"));
        let expected = &case["parsed"];
        let text = |field: &str| expected[field].as_str().map(str::to_string);
        assert_eq!(parsed.capability_id, text("capabilityId"), "{row}: capabilityId");
        assert_eq!(parsed.capability_title, text("capabilityTitle"), "{row}: capabilityTitle");
        assert_eq!(parsed.description, text("description"), "{row}: description");
        assert_eq!(parsed.artifact_kind, text("artifactKind"), "{row}: artifactKind");
        assert_eq!(Some(parsed.diff_summary.clone()), text("diffSummary"), "{row}: diffSummary");
        assert_eq!(parsed.risk.map(|risk| risk.as_str().to_string()), text("risk"), "{row}: risk");
        assert_eq!(parsed.requested_by, text("requestedBy"), "{row}: requestedBy");
        assert_eq!(parsed.timeout_ms, expected["timeoutMs"].as_u64(), "{row}: timeoutMs");
    }
    assert_eq!(parse_approval_summary(fixture["unusableTimeout"]["summary"].as_str().expect("row")).timeout_ms, None, "a non-positive timeout is nothing to count down");
}

/// ⏱️ The countdown is a duration from arrival, never a wall clock, and it floors at zero.
#[test]
fn the_countdown_counts_from_arrival_and_never_goes_negative() {
    assert_eq!(approval_seconds_remaining(Some(120_000), 1_000.0, 1_000.0), Some(120));
    assert_eq!(approval_seconds_remaining(Some(120_000), 1_000.0, 61_000.0), Some(60));
    assert_eq!(approval_seconds_remaining(Some(120_000), 1_000.0, 999_000.0), Some(0));
    assert_eq!(approval_seconds_remaining(None, 1_000.0, 1_000.0), None);
}

#[test]
fn a_plain_text_summary_becomes_the_diff_summary() {
    let parsed = parse_approval_summary("translate selection by (1,0,0)");
    assert_eq!(parsed.diff_summary, "translate selection by (1,0,0)");
    assert!(parsed.capability_id.is_none());
    assert!(parsed.risk.is_none());
    assert!(parsed.requested_by.is_none());
}

#[test]
fn a_rich_json_summary_yields_capability_diff_risk_and_requester() {
    let parsed = parse_approval_summary(r#"{"capabilityId":"artifact.mutate","diffSummary":"+3 −1","risk":"high","requestedBy":"agent:local"}"#);
    assert_eq!(parsed.capability_id.as_deref(), Some("artifact.mutate"));
    assert_eq!(parsed.diff_summary, "+3 −1");
    assert_eq!(parsed.risk, Some(ApprovalRisk::High));
    assert_eq!(parsed.requested_by.as_deref(), Some("agent:local"));
}

#[test]
fn json_without_any_rich_field_stays_the_plain_text_fallback() {
    let summary = r#"{"diffSummary":"only a diff"}"#;
    let parsed = parse_approval_summary(summary);
    assert_eq!(parsed.diff_summary, summary, "the React twin only takes the rich shape when one of capability/risk/requestedBy is present");
    assert!(parsed.capability_id.is_none());
}

#[test]
fn malformed_json_and_arrays_never_blank_the_dialog() {
    assert_eq!(parse_approval_summary("{not json").diff_summary, "{not json");
    assert_eq!(parse_approval_summary(r#"["a","b"]"#).diff_summary, r#"["a","b"]"#);
    assert_eq!(parse_approval_summary("").diff_summary, "");
}

#[test]
fn an_unknown_risk_word_is_dropped_rather_than_guessed() {
    let parsed = parse_approval_summary(r#"{"capabilityId":"c","risk":"catastrophic"}"#);
    assert_eq!(parsed.capability_id.as_deref(), Some("c"));
    assert_eq!(parsed.risk, None);
}

#[test]
fn the_modal_opens_purely_from_a_non_empty_queue() {
    let mut model = AgentApprovalsModel::default();
    assert!(!model.is_open(&[]));
    let queue = vec![parked("appr_1", "s")];
    model.observe(&queue);
    assert!(model.is_open(&queue));
}

#[test]
fn a_dismissed_modal_reopens_on_a_newly_arrived_request() {
    let mut model = AgentApprovalsModel::default();
    let one = vec![parked("appr_1", "s")];
    model.observe(&one);
    model.dismiss();
    assert!(!model.is_open(&one));
    let two = vec![parked("appr_1", "s"), parked("appr_2", "t")];
    model.observe(&two);
    assert!(model.is_open(&two), "a newly arrived approval must re-open the dialog");
}

#[test]
fn an_emptied_queue_closes_the_modal_without_a_dismissal() {
    let mut model = AgentApprovalsModel::default();
    let one = vec![parked("appr_1", "s")];
    model.observe(&one);
    assert!(model.is_open(&one));
    model.observe(&[]);
    assert!(!model.is_open(&[]));
}

#[test]
fn decision_control_ids_round_trip_for_every_decision() {
    for decision in ApprovalDecision::ALL {
        let id = approval_decision_control_id("appr_1", decision);
        assert_eq!(parse_approval_decision_control_id(&id), Some(("appr_1".to_string(), decision)));
    }
    assert_eq!(parse_approval_decision_control_id("shell.dialog.x.confirm"), None);
    assert_eq!(parse_approval_decision_control_id(APPROVALS_CLOSE_CONTROL_ID), None);
}

#[test]
fn a_simulated_frame_through_the_bridge_produces_a_modal_whose_buttons_resolve_it() {
    let mut bridge = AgentBridgeState::default();
    let mut model = AgentApprovalsModel::default();
    let summary = r#"{"capabilityId":"artifact.mutate","diffSummary":"+3 −1","risk":"medium","requestedBy":"agent:local"}"#;
    bridge.apply_encoded_frame(&GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: summary.into() }.encode(), 0.0).expect("frame decodes");
    model.observe(&bridge.pending_approvals);
    assert!(model.is_open(&bridge.pending_approvals));

    let parsed = parse_approval_summary(&bridge.pending_approvals[0].summary);
    assert_eq!(approval_row_line_count(&parsed), 4, "capability + diff + requestedBy + risk");

    let theme = Theme::dark();
    let heights = vec![approval_row_height(&parsed, &theme)];
    let modal = approvals_modal_rect(1280.0, 800.0, &heights, &theme);
    let list = approvals_list_rect(modal, &theme);
    assert!(modal.w <= APPROVALS_MODAL_WIDTH);
    assert!(list.y >= modal.y && list.y + list.h <= modal.y + modal.h + 0.01);

    let row = Rect::new(list.x, list.y, list.w, heights[0]);
    let buttons = approval_decision_rects(row, &theme);
    assert_eq!(buttons.map(|(decision, _)| decision), ApprovalDecision::ALL);
    for (_, rect) in buttons {
        assert!(rect.y >= row.y && rect.y + rect.h <= row.y + row.h + 0.01, "every decision button stays inside its row");
    }

    let (approval_id, decision) = parse_approval_decision_control_id(&approval_decision_control_id("appr_1", ApprovalDecision::Session)).expect("decision id parses");
    bridge.resolve_approval(&approval_id, decision, None);
    assert!(!bridge.has_pending_approvals());
    model.observe(&bridge.pending_approvals);
    assert!(!model.is_open(&bridge.pending_approvals));
}

#[test]
fn the_modal_never_exceeds_a_small_viewport() {
    let theme = Theme::dark();
    let heights = vec![200.0; 12];
    let modal = approvals_modal_rect(360.0, 240.0, &heights, &theme);
    assert!(modal.w <= 360.0 && modal.h <= 240.0);
    assert!(modal.x >= 0.0 && modal.y >= 0.0);
}

#[test]
fn risk_badges_take_three_distinct_theme_tokens_and_both_locales() {
    let theme = Theme::dark();
    assert_eq!(ApprovalRisk::Low.color(&theme), theme.accent);
    assert_eq!(ApprovalRisk::Medium.color(&theme), theme.warning);
    assert_eq!(ApprovalRisk::High.color(&theme), theme.error);
    assert_eq!(ApprovalRisk::High.label(Locale::En), "High");
    assert_eq!(ApprovalRisk::High.label(Locale::De), "Hoch");
    assert_eq!(approvals_decision_label(ApprovalDecision::Session, Locale::De), "Für Sitzung genehmigen");
}
