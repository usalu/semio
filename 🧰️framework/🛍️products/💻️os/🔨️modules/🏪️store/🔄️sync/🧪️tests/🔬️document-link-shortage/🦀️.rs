//! 🔌️ Language-agnostic document-link-shortage fixture — Rust runner. The same
//! `🏪️store/🧫️fixtures/document-link-shortage-v1/🔣️.json` drives the React twin
//! (`documentLinkTransition` in `🏪️store/🟦️.ts`), so the native actor, the browser actor and the React
//! worker cannot drift into three reconnect-and-expiry rules again (ticket 26/09/23 audit P2-2).

use super::{document_admission_refuses_access, document_link_terminal_message, DocumentLink, DocumentLinkEvent, DocumentLinkShortagePolicy, DocumentLinkStatus, DOCUMENT_LINK_ACCESS_REFUSED_STATUSES, DOCUMENT_LINK_SHORTAGE_POLICY};
use crate::os_directory::client::{DirectoryClientError, TransportError};

const FIXTURE: &str = include_str!("../../../🧫️fixtures/document-link-shortage-v1/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("document-link-shortage fixture json")
}

fn millis(value: &serde_json::Value, field: &str) -> u64 {
    value.get(field).and_then(serde_json::Value::as_u64).unwrap_or_else(|| panic!("fixture field {field}"))
}

fn state(value: &serde_json::Value) -> DocumentLink {
    match value["kind"].as_str() {
        Some("linked") => DocumentLink::Linked,
        Some("unlinked") => DocumentLink::Unlinked { since_ms: millis(value, "sinceMs"), backoff_ms: millis(value, "backoffMs"), retry_at_ms: millis(value, "retryAtMs") },
        Some("expired") => DocumentLink::Expired { since_ms: millis(value, "sinceMs"), at_ms: millis(value, "atMs") },
        Some("revoked") => DocumentLink::Revoked { at_ms: millis(value, "atMs") },
        other => panic!("fixture state kind {other:?}"),
    }
}

fn event(value: &serde_json::Value) -> DocumentLinkEvent {
    let now_ms = millis(value, "nowMs");
    match value["kind"].as_str() {
        Some("failed") => DocumentLinkEvent::Failed { now_ms },
        Some("restored") => DocumentLinkEvent::Restored { now_ms },
        Some("tick") => DocumentLinkEvent::Tick { now_ms },
        Some("refused") => DocumentLinkEvent::Refused { now_ms },
        other => panic!("fixture event kind {other:?}"),
    }
}

#[test]
fn the_kernel_policy_is_the_fixture_policy() {
    let policy = &fixture()["policy"];
    assert_eq!(
        DOCUMENT_LINK_SHORTAGE_POLICY,
        DocumentLinkShortagePolicy { reconnect_min_ms: millis(policy, "reconnectMinMs"), reconnect_max_ms: millis(policy, "reconnectMaxMs"), shortage_bound_ms: millis(policy, "shortageBoundMs") }
    );
    assert_eq!(DOCUMENT_LINK_SHORTAGE_POLICY.shortage_bound_ms, 2 * DOCUMENT_LINK_SHORTAGE_POLICY.reconnect_max_ms, "the bound admits at least two capped reconnect attempts");
}

#[test]
fn every_fixture_vector_walks_the_same_link_states() {
    let fixture = fixture();
    let mut steps = 0;
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let id = vector["id"].as_str().expect("vector id");
        let mut link = DocumentLink::opened(millis(vector, "openedAtMs"));
        assert!(link.retry_due(millis(vector, "openedAtMs")), "{id}: an opened link dials at once");
        for (index, step) in vector["steps"].as_array().expect("steps").iter().enumerate() {
            link = link.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, event(&step["event"]));
            let expect = &step["expect"];
            assert_eq!(link, state(&expect["state"]), "{id} step {index}");
            assert_eq!(link.status().code(), expect["status"].as_str().expect("status"), "{id} step {index}");
            assert_eq!(link.admits_local_edits(), expect["admitsLocalEdits"].as_bool().expect("admitsLocalEdits"), "{id} step {index}");
            assert_eq!(link.expires_at_ms(&DOCUMENT_LINK_SHORTAGE_POLICY), expect.get("expiresAtMs").and_then(serde_json::Value::as_u64), "{id} step {index}");
            steps += 1;
        }
    }
    assert!(steps >= 20, "the fixture walks every transition ({steps} steps)");
}

#[test]
fn every_shortage_status_speaks_the_fixture_texts_in_both_tongues() {
    let texts = &fixture()["texts"];
    for status in [DocumentLinkStatus::Reconnecting, DocumentLinkStatus::LinkExpired, DocumentLinkStatus::AccessRevoked] {
        let row = &texts[status.code()];
        assert_eq!(status.text(false), row["en"].as_str(), "{} en", status.code());
        assert_eq!(status.text(true), row["de"].as_str(), "{} de", status.code());
    }
    assert_eq!(DocumentLinkStatus::Linked.text(false), None);
}

#[test]
fn an_unlinked_link_needs_a_turn_at_its_retry_or_its_expiry() {
    let link = DocumentLink::Linked.apply(&DOCUMENT_LINK_SHORTAGE_POLICY, DocumentLinkEvent::Failed { now_ms: 1_000 });
    assert_eq!(link.next_deadline_ms(&DOCUMENT_LINK_SHORTAGE_POLICY), Some(1_000 + DOCUMENT_LINK_SHORTAGE_POLICY.reconnect_min_ms));
    let capped = DocumentLink::Unlinked { since_ms: 0, backoff_ms: DOCUMENT_LINK_SHORTAGE_POLICY.reconnect_max_ms, retry_at_ms: 70_000 };
    assert_eq!(capped.next_deadline_ms(&DOCUMENT_LINK_SHORTAGE_POLICY), Some(DOCUMENT_LINK_SHORTAGE_POLICY.shortage_bound_ms), "expiry wins over a later retry");
    assert_eq!(DocumentLink::Linked.next_deadline_ms(&DOCUMENT_LINK_SHORTAGE_POLICY), None);
}

#[test]
fn an_admission_the_hub_refuses_revokes_and_a_short_one_retries() {
    let statuses: Vec<u16> = fixture()["accessRefusedStatuses"].as_array().expect("accessRefusedStatuses").iter().map(|status| status.as_u64().expect("status") as u16).collect();
    assert_eq!(statuses, DOCUMENT_LINK_ACCESS_REFUSED_STATUSES);
    for status in statuses {
        assert!(document_admission_refuses_access(&DirectoryClientError::Http { status, body: String::new() }), "{status} revokes");
    }
    assert!(document_admission_refuses_access(&DirectoryClientError::Unauthorized));
    for status in [408_u16, 429, 500, 502, 503] {
        assert!(!document_admission_refuses_access(&DirectoryClientError::Http { status, body: String::new() }), "{status} is a shortage");
    }
    assert!(!document_admission_refuses_access(&DirectoryClientError::Transport(TransportError::Io("reset".into()))));
}

#[test]
fn a_terminal_link_names_its_status_as_the_fault_code() {
    let message = document_link_terminal_message("artifact-1", DocumentLinkStatus::LinkExpired);
    assert_eq!(message.code.0, "link-expired");
    assert_eq!(message.target, vec!["artifact-1".to_string()]);
    assert_eq!(message.message, DocumentLinkStatus::LinkExpired.text(false).expect("text"));
}
