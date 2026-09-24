use super::*;
use semio_framework_repo_providers::{ManagementProviders, NullManagementProvider};

fn layout() -> TicketLayout {
    TicketLayout::new("/repo/.🧬semio/🦑️repo")
}

fn service<'a>(store: &'a MemoryTicketStore, tracker: &'a ManagementProviders, clock: &'a FixedClock, events: &'a RecordingEventSink) -> TicketService<'a, MemoryTicketStore, ManagementProviders, FixedClock, RecordingEventSink> {
    TicketService::new(layout(), store, tracker, clock, events)
}

#[test]
fn ticket_id_round_trips_through_both_spellings() {
    let id = TicketId::new(26, 9, 6, "SOME-TICKET");
    assert_eq!(id.id(), "26/09/06/SOME-TICKET");
    assert_eq!(id.rel_path(), "🎆️26/🌙️09/☀️06/SOME-TICKET");
    assert_eq!(TicketId::parse(&id.id()).unwrap(), id);
    assert_eq!(TicketId::parse(&id.rel_path()).unwrap(), id);
    assert_eq!(TicketId::parse("26/09/06/PARENT/CHILD").unwrap().slug, "PARENT/CHILD");
}

#[test]
fn a_title_becomes_an_upper_kebab_slug() {
    assert_eq!(ticket_slug_from_title("Tree Text Short IDs").unwrap(), "TREE-TEXT-SHORT-I-DS");
    assert!(ticket_slug_from_title("   ").is_err());
}

#[test]
fn the_encoder_html_escapes_the_way_go_does() {
    assert_eq!(go_json_string(">e-"), "\"\\u003ee-\"");
    assert_eq!(go_json_string("<e-"), "\"\\u003ce-\"");
    assert_eq!(go_json_string("a&b"), "\"a\\u0026b\"");
}

#[test]
fn a_document_without_a_status_is_refused() {
    let error = decode_ticket_document("{\"title\":\"x\"}").unwrap_err();
    assert_eq!(error.message, "ticket status must be explicitly \"open\" or \"closed\"");
}

#[test]
fn unknown_members_are_dropped_on_re_encoding() {
    let source = "{\n  \"title\": \"T\",\n  \"status\": \"open\",\n  \"note\": \"kept nowhere\"\n}";
    let ticket = decode_ticket_document(source).unwrap();
    assert_eq!(encode_ticket_document(&ticket), "{\n  \"title\": \"T\",\n  \"status\": \"open\"\n}");
}

#[test]
fn an_open_close_reopen_round_trip_emits_three_events() {
    let store = MemoryTicketStore::new();
    let tracker = ManagementProviders::Null(NullManagementProvider);
    let clock = FixedClock::new(26, 9, 6, "2026-09-06 12:00:00");
    let events = RecordingEventSink::new();
    let service = service(&store, &tracker, &clock, &events);
    let opened = service
        .open(&TicketOpenRequest { emoji: "🎫️".to_string(), title: "Some Ticket".to_string(), prompt: "do it".to_string(), client: "claude-code".to_string(), goal: "🎯g".to_string(), no_issue: true, no_management: true, ..Default::default() })
        .unwrap();
    assert_eq!(opened.id, "26/09/06/SOME-TICKET");
    let closed = service.close(&TicketCloseRequest { id: opened.id, summary: "done".to_string(), files: vec!["a/b.rs".to_string()], no_management: true, bulk: false }).unwrap();
    assert_eq!(closed.status, "closed");
    assert!(!store.exists(&service.layout.important_path(&TicketId::parse(&closed.id).unwrap())));
    let reopened = service.reopen(&TicketReopenRequest { id: closed.id, prompt: "again".to_string(), client: "claude-code".to_string(), no_management: true, ..Default::default() }).unwrap();
    assert_eq!(reopened.status, "open");
    assert_eq!(events.recorded().iter().map(|event| event.kind.clone()).collect::<Vec<String>>(), vec!["ticket.open.ended", "ticket.close.ended", "ticket.reopen.ended"]);
}

#[test]
fn a_failed_save_rolls_the_important_document_back() {
    let store = MemoryTicketStore::new();
    let tracker = ManagementProviders::Null(NullManagementProvider);
    let clock = FixedClock::new(26, 9, 6, "2026-09-06 12:00:00");
    let events = RecordingEventSink::new();
    let service = service(&store, &tracker, &clock, &events);
    let opened = service
        .open(&TicketOpenRequest { emoji: "🎫️".to_string(), title: "Rollback".to_string(), prompt: "p".to_string(), client: "claude-code".to_string(), goal: "🎯g".to_string(), no_issue: true, no_management: true, ..Default::default() })
        .unwrap();
    let id = TicketId::parse(&opened.id).unwrap();
    service.close(&TicketCloseRequest { id: opened.id.clone(), summary: "s".to_string(), files: vec!["a.rs".to_string()], no_management: true, bulk: false }).unwrap();
    store.fail_write(&service.layout.document_path(&id), "disk is full");
    let error = service.reopen(&TicketReopenRequest { id: opened.id, prompt: "again".to_string(), client: "claude-code".to_string(), no_management: true, ..Default::default() }).unwrap_err();
    assert_eq!(error.message, "disk is full");
    assert!(!store.exists(&service.layout.important_path(&id)));
    assert!(!store.exists(&service.layout.important_dir(&id)));
}

#[test]
fn a_hunk_stream_becomes_line_numbers() {
    let diff = "diff --git a/x.rs b/x.rs\n--- a/x.rs\n+++ b/x.rs\n@@ -1,2 +1,3 @@\n";
    let parsed = parse_diff_lines(diff);
    assert_eq!(parsed["x.rs"].removed, vec![1, 2]);
    assert_eq!(parsed["x.rs"].added, vec![1, 2, 3]);
}

#[test]
fn an_oversized_artifact_is_purged_but_the_document_is_not() {
    let store = MemoryTicketStore::new();
    let tracker = ManagementProviders::Null(NullManagementProvider);
    let clock = FixedClock::new(26, 9, 6, "2026-09-06 12:00:00");
    let events = RecordingEventSink::new();
    let service = service(&store, &tracker, &clock, &events);
    let id = TicketId::new(26, 9, 6, "PURGE");
    store.seed_file(&service.layout.document_path(&id), "{}");
    store.seed_sized_file(&join_path(&service.layout.ticket_dir(&id), "🗑️generated/huge.log"), OVERSIZED_FILE_BYTES + 1);
    store.seed_sized_file(&join_path(&service.layout.ticket_dir(&id), "small.txt"), 10);
    let report = service.purge_artifacts(&id).unwrap();
    assert_eq!(report.removed_files.len(), 1);
    assert!(store.exists(&service.layout.document_path(&id)));
    assert!(store.exists(&join_path(&service.layout.ticket_dir(&id), "small.txt")));
}
