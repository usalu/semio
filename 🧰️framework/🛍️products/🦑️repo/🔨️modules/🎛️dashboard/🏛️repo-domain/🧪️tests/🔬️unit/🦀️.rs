use super::*;
use std::fs;

#[test]
fn repo_action_shared_vectors_validate_and_project_selected_values() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🏛️repo-action/🔣️.json")).unwrap();
    for vector in fixture["valid"].as_array().unwrap() {
        let action: RepoAction = serde_json::from_value(vector["action"].clone()).unwrap();
        assert!(action.validate().is_ok(), "{action:?}");
        assert_eq!(serde_json::to_value(action.go_argv()).unwrap(), vector["argv"]);
        assert_eq!(action_key(&action), vector["key"].as_str().unwrap());
        assert_eq!(action.mutates(), vector["mutating"].as_bool().unwrap());
    }
    for vector in fixture["invalid"].as_array().unwrap() {
        assert!(serde_json::from_value::<RepoAction>(vector.clone()).map(|action| action.validate().is_err()).unwrap_or(true), "{vector}");
    }
}

#[test]
fn repo_action_ticket_codec_preserves_the_shared_interaction_with_serde_json() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🎫️tickets/🧪️tests/🧫️fixtures/📄️ticket-document-codec/📄️documents.json")).unwrap();
    let text = fixture["documents"][0]["text"].as_str().unwrap();
    let ticket = tickets::decode_ticket_document(text).unwrap();
    let encoded = tickets::encode_ticket_document(&ticket);
    let own: serde_json::Value = serde_json::from_str(&encoded).unwrap();
    let independent = serde_json::to_value(&ticket).unwrap();
    assert_eq!(own, independent);
    assert_eq!(tickets::decode_ticket_document(&encoded).unwrap().interactions, ticket.interactions);
    assert_eq!(own["interactions"][0]["prompt"], "Continue the exact human request 🦑 <keep> & verify");
}

#[test]
fn draft_actions_execute_isolated_filesystem_lifecycle_and_reject_escape() {
    let root = temp_root("draft");
    let start = RepoAction::DraftStart { title: "A Human Draft".into() };
    let notes = root.join(".🧬semio/🦑️repo/✍️notes/A-HUMAN-DRAFT");
    assert!(start.execute(&root).contains("created draft A-HUMAN-DRAFT"));
    assert!(notes.is_dir());
    assert!(start.execute(&root).contains("already exists"));
    assert!(RepoAction::DraftDiscard { slug: "../escape".into() }.execute(&root).contains("invalid"));
    assert!(notes.is_dir());
    assert!(RepoAction::DraftDiscard { slug: "A-HUMAN-DRAFT".into() }.execute(&root).contains("discarded draft"));
    assert!(!notes.exists());
    assert!(RepoAction::DraftStart { title: "🦑".into() }.execute(&root).contains("invalid"));
    assert!(RepoAction::DraftStart { title: "A".repeat(1024) }.admit_root(&root).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn reopen_action_records_the_human_prompt_and_event_without_management() {
    let root = temp_root("reopen");
    let store = tickets::FileTicketStore::new();
    let tracker = tickets::RecordedIssueTracker::new(tickets::IssueTrackerScript::default());
    let clock = tickets::FixedClock::new(26, 9, 23, "2026-09-23 12:00:00");
    let events = tickets::RecordingEventSink::new();
    let service = tickets::TicketService::new(layout(&root), &store, &tracker, &clock, &events);
    let outcome = service.open(&tickets::TicketOpenRequest { emoji: "🦑".into(), title: "Human Reopen".into(), prompt: "Initial request".into(), goal: "TEST".into(), client: DASHBOARD_CLIENT.into(), no_management: true, ..Default::default() }).unwrap();
    service.close(&tickets::TicketCloseRequest { id: outcome.id.clone(), no_management: true, bulk: true, ..Default::default() }).unwrap();
    let prompt = "Continue the exact human request 🦑";
    service.reopen(&ticket_reopen_request(&outcome.id, prompt)).unwrap();
    let ticket = service.read(&tickets::TicketId::parse(&outcome.id).unwrap()).unwrap();
    assert_eq!(ticket.status, TicketStatus::Open);
    assert_eq!(ticket.interactions.last().unwrap().prompt, prompt);
    assert!(tracker.calls().is_empty());
    assert!(events.recorded().iter().any(|event| event.kind == semio_framework_repo_events::TICKET_REOPEN_ENDED && event.payload.contains(prompt)));
    assert!(RepoAction::TicketReopen { id: outcome.id, prompt: prompt.into() }.execute_result(&root).is_err());
    fs::remove_dir_all(root).unwrap();
}

fn temp_root(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join(format!("semio-repo-domain-{name}-{nanos}"));
    fs::create_dir_all(&dir).expect("create temp root");
    dir
}

#[test]
fn repo_actions_run_in_process_against_the_domain_crates() {
    let tmp = temp_root("in-process");
    let catalog = RepoAction::StatutesCatalog.execute(&tmp);
    assert!(catalog.lines().count() > 1, "statute catalog is empty: {catalog}");
    assert!(RepoAction::TreeStatute.execute(&tmp).contains('\n'));
    assert!(RepoAction::TreeTerritory.execute(&tmp).contains('\n'));
    assert_eq!(RepoAction::GoalsList.execute(&tmp), "");
    assert!(RepoAction::TicketShow { id: "26/09/06/ABSENT".into() }.execute(&tmp).contains("ticket not found"));
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn repo_action_carries_its_go_argv() {
    assert_eq!(RepoAction::GoalsTree.go_argv(), vec!["goal", "tree"]);
    assert_eq!(RepoAction::TreeMonorepo.go_argv(), vec!["tree", "monorepo"]);
    assert_eq!(RepoAction::Analyze { scope: "rust".into(), policy: None }.go_argv(), vec!["analyze", "rust"]);
    assert_eq!(RepoAction::TicketShow { id: "26/09/06/X".into() }.go_argv(), vec!["ticket", "show", "26/09/06/X"]);
    assert_eq!(RepoAction::TicketClose { id: "26/09/06/X".into() }.go_argv(), vec!["ticket", "close", "26/09/06/X", "--summary", DASHBOARD_CLOSE_SUMMARY, "--no-management"]);
}

#[test]
fn the_go_binary_lives_in_the_marked_cache() {
    let root = Path::new("/workspace");
    assert!(go_binary_path(root).starts_with(root.join(REPO_BIN_CACHE_DIR)));
}

#[test]
fn every_scenario_of_the_repo_domain_feature_is_proved_by_a_test() {
    dashboard_core::testkit::assert_proved(include_str!("../../../🧪️tests/🏛️repo-domain/🥒️.feature"), &[include_str!("🦀️.rs"), include_str!("../../../🎮️registry/🧪️tests/🔬️unit/🦀️.rs")], &[
        ("Repo actions run in process against the domain crates", &["repo_actions_run_in_process_against_the_domain_crates"]),
        ("A repo action carries the argument list of the Go implementation", &["repo_action_carries_its_go_argv"]),
        ("Every action has a distinct key the registry grammar accepts", &["every_action_has_a_distinct_key_the_registry_grammar_accepts"]),
        ("The Go binary lives in the marked cache", &["the_go_binary_lives_in_the_marked_cache"]),
    ]);
}
