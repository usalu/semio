use super::*;

fn vectors() -> Value { serde_json::from_str(include_str!("🔣️.json")).expect("shared rooted ticket vectors") }

#[test]
fn rooted_ticket_services_preserve_independent_shared_lifecycles_and_drafts() {
    let data = vectors();
    for value in data["invalidSegments"].as_array().unwrap() { assert!(!drafts::segment(value.as_str().unwrap())); }
    for value in data["validSegments"].as_array().unwrap() { assert!(drafts::segment(value.as_str().unwrap())); }
    let stores = [MemoryTicketStore::new(), MemoryTicketStore::new()];
    let tracker = NullIssueTracker;
    let events = RecordingEventSink::new();
    let clock = FixedClock::new(data["clock"]["year"].as_i64().unwrap(), data["clock"]["month"].as_i64().unwrap(), data["clock"]["day"].as_i64().unwrap(), data["clock"]["stamp"].as_str().unwrap());
    let layouts = [TicketLayout::new("/scope/a/.🧬semio/🦑️repo"), TicketLayout::new("/scope/b/.🧬semio/🦑️repo")];
    let metadata = InteractionMetadata { author: "fixture-author".into(), system: "fixture-system".into(), checkpoint: "fixture-checkpoint".into() };
    let services: Vec<_> = stores.iter().zip(layouts.iter()).map(|(store, layout)| TicketService::new(layout.clone(), store, &tracker, &clock, &events).with_interaction_metadata(metadata.clone())).collect();
    let request: TicketOpenRequest = serde_json::from_value(data["open"].clone()).unwrap();
    let mut opened = Vec::new();
    for (index, service) in services.iter().enumerate() {
        let draft = join_path(service.layout.repo_meta_dir(), "📝️drafts/seed/note.md");
        stores[index].seed_file(&draft, data["roots"][index]["draft"].as_str().unwrap());
        opened.push(service.open(&request).expect("rooted open"));
        assert!(!stores[index].exists(&dir_name(&draft)));
    }
    let other_before = stores[1].snapshot();
    let mut value = data["change"].clone(); value["id"] = Value::String(opened[0].id.clone());
    let change: TicketChangeRequest = serde_json::from_value(value).unwrap();
    let changed = services[0].change(&change).expect("rooted change");
    let changed_id = TicketId::parse(&changed.id).unwrap();
    let ticket = services[0].read(&changed_id).unwrap();
    assert_eq!(ticket.slug, data["expected"]["slug"].as_str().unwrap());
    assert_eq!(ticket.description, data["expected"]["prompt"].as_str().unwrap());
    assert_eq!(ticket.interactions.last().unwrap().prompt, ticket.description);
    let before = stores[0].snapshot();
    let invalid = TicketChangeRequest { id: changed.id.clone(), title: Some("Forbidden Rename".into()), client: Some("not-a-client".into()), ..Default::default() };
    assert!(services[0].change(&invalid).is_err()); assert_eq!(stores[0].snapshot(), before);
    let mut value = data["close"].clone(); value["id"] = Value::String(changed.id);
    let close: TicketCloseRequest = serde_json::from_value(value).unwrap();
    let closed = services[0].close(&close).expect("rooted close");
    stores[0].seed_file(&join_path(layouts[0].repo_meta_dir(), "📝️drafts/again/note.md"), "second\n");
    let mut value = data["reopen"].clone(); value["id"] = Value::String(closed.id);
    let reopen: TicketReopenRequest = serde_json::from_value(value).unwrap();
    let reopened = services[0].reopen(&reopen).expect("rooted reopen");
    let id = TicketId::parse(&reopened.id).unwrap();
    let ticket = services[0].read(&id).unwrap();
    assert_eq!(ticket.status.as_str(), data["expected"]["status"].as_str().unwrap());
    assert_eq!(serde_json::json!(ticket.interactions.iter().map(|value| &value.kind).collect::<Vec<_>>()), data["expected"]["kinds"]);
    for interaction in &ticket.interactions { assert_eq!(interaction.author, metadata.author); assert_eq!(interaction.system, metadata.system); assert_eq!(interaction.checkpoint, metadata.checkpoint); }
    for (index, name) in data["expected"]["draftFiles"].as_array().unwrap().iter().enumerate() {
        assert_eq!(stores[0].read(&join_path(&ticket.folder_path, name.as_str().unwrap())).unwrap(), if index == 0 { data["roots"][0]["draft"].as_str().unwrap() } else { "second\n" });
    }
    assert_eq!(stores[1].snapshot(), other_before);
    services[0].delete(&id, true).expect("rooted delete");
    assert!(!stores[0].exists(&ticket.folder_path)); assert_eq!(stores[1].snapshot(), other_before);
}

#[test]
fn draft_preflight_refuses_escapes_symlinks_and_preserves_partial_moves() {
    let store = MemoryTicketStore::new(); let tracker = NullIssueTracker; let clock = FixedClock::new(26, 10, 21, "now"); let events = RecordingEventSink::new();
    let service = TicketService::new(TicketLayout::new("/scope/a/.🧬semio/🦑️repo"), &store, &tracker, &clock, &events);
    let base: TicketOpenRequest = serde_json::from_value(vectors()["open"].clone()).unwrap();
    for draft in ["../escape", "/absolute", "a/../b", "a//b", "a\\b", "C:escape"] {
        let request = TicketOpenRequest { draft: draft.into(), ..base.clone() }; assert!(service.open(&request).is_err(), "{draft}");
    }
    let draft_root = join_path(service.layout.repo_meta_dir(), "📝️drafts/seed");
    store.seed_symlink(&join_path(&draft_root, "link"));
    assert!(service.open(&base).is_err()); assert!(store.exists(&join_path(&draft_root, "link")));
    store.remove_tree(&draft_root).unwrap();
    store.seed_file(&join_path(&draft_root, "a.md"), "first"); store.seed_file(&join_path(&draft_root, "b.md"), "second");
    let target = service.layout.ticket_dir(&TicketId::new(26, 10, 21, "SCOPED-TICKET"));
    store.fail_write(&join_path(&target, "b.md"), "second move refused");
    assert_eq!(service.open(&base).unwrap_err().message, "second move refused");
    assert_eq!(store.read(&join_path(&target, "a.md")).unwrap(), "first");
    assert_eq!(store.read(&join_path(&draft_root, "b.md")).unwrap(), "second");
    assert!(store.is_directory(&draft_root));
}

#[test]
fn remote_title_failure_warns_and_remote_delete_failure_preserves_local_ticket() {
    let store = MemoryTicketStore::new(); let clock = FixedClock::new(26, 10, 21, "now"); let events = RecordingEventSink::new();
    let tracker = RecordedIssueTracker::new(IssueTrackerScript { failures: vec!["update_issue_title".into(), "delete_issue".into()], ..Default::default() });
    let service = TicketService::new(TicketLayout::new("/scope/a/.🧬semio/🦑️repo"), &store, &tracker, &clock, &events);
    let mut request: TicketOpenRequest = serde_json::from_value(vectors()["open"].clone()).unwrap(); request.draft.clear();
    let opened = service.open(&request).unwrap(); let id = TicketId::parse(&opened.id).unwrap();
    let mut ticket = service.read(&id).unwrap(); ticket.management = Some(TicketManagementData { issue: "https://example.test/issues/1".into() }); service.save(&ticket).unwrap();
    let changed = service.change(&TicketChangeRequest { id: opened.id, title: Some("Renamed Ticket".into()), ..Default::default() }).unwrap();
    assert_eq!(changed.warnings.len(), 1); assert!(tracker.calls().iter().any(|call| call.starts_with("update_issue_title|")));
    let id = TicketId::parse(&changed.id).unwrap(); let before = store.snapshot();
    assert!(service.delete(&id, false).is_err()); assert_eq!(store.snapshot(), before);
    service.delete(&id, true).unwrap(); assert!(!store.exists(&service.layout.ticket_dir(&id)));
}

#[test]
fn rooted_ticket_service_rejects_foreign_document_identity_and_invalid_ids_before_effects() {
    let store = MemoryTicketStore::new(); let tracker = NullIssueTracker; let clock = FixedClock::new(26, 10, 21, "now"); let events = RecordingEventSink::new();
    let service = TicketService::new(TicketLayout::new("/scope/a/.🧬semio/🦑️repo"), &store, &tracker, &clock, &events);
    let mut request: TicketOpenRequest = serde_json::from_value(vectors()["open"].clone()).unwrap(); request.draft.clear();
    let opened = service.open(&request).unwrap(); let id = TicketId::parse(&opened.id).unwrap();
    let before = store.snapshot(); let mut ticket = service.read(&id).unwrap(); ticket.json_path = "/scope/b/foreign.json".into(); assert!(service.save(&ticket).is_err()); assert_eq!(store.snapshot(), before);
    for (year, month, day, slug) in [(100, 1, 1, "T"), (26, 0, 1, "T"), (26, 1, 32, "T"), (26, 1, 1, "../ESCAPE"), (26, 1, 1, "CON")] {
        let invalid = TicketId::new(year, month, day, slug); assert!(service.read(&invalid).is_err()); assert!(service.delete(&invalid, true).is_err()); assert_eq!(store.snapshot(), before);
    }
    store.remove_file(&service.layout.document_path(&id)).unwrap(); store.seed_symlink(&service.layout.document_path(&id)); assert!(service.read(&id).is_err());
}

#[test]
fn projected_draft_descendants_are_bounded_before_any_move_or_ticket_creation() {
    let store = MemoryTicketStore::new(); let tracker = NullIssueTracker; let clock = FixedClock::new(26, 10, 21, "now"); let events = RecordingEventSink::new();
    let layout = TicketLayout::new(format!("/scope/{}/meta", "r".repeat(130)));
    let service = TicketService::new(layout, &store, &tracker, &clock, &events);
    let mut request: TicketOpenRequest = serde_json::from_value(vectors()["open"].clone()).unwrap(); request.draft = "seed".into();
    let relative = format!("folder/{}", "x".repeat(80)); let source = join_path(service.layout.repo_meta_dir(), &format!("📝️drafts/seed/{relative}"));
    let id = TicketId::new(26, 10, 21, "SCOPED-TICKET"); let destination = join_path(&service.layout.ticket_dir(&id), &relative);
    assert!(source.encode_utf16().count() <= 256); assert!(service.layout.important_path(&id).encode_utf16().count() <= 256); assert!(destination.encode_utf16().count() > 256);
    store.seed_file(&source, "preserved"); let before = store.snapshot(); assert!(service.open(&request).is_err()); assert_eq!(store.snapshot(), before); assert!(!store.exists(&service.layout.ticket_dir(&id)));
}

#[test]
fn retitle_projected_descendants_are_bounded_before_renaming_the_ticket_tree() {
    let store = MemoryTicketStore::new(); let tracker = NullIssueTracker; let clock = FixedClock::new(26, 10, 21, "now"); let events = RecordingEventSink::new();
    let service = TicketService::new(TicketLayout::new("/scope/a/meta"), &store, &tracker, &clock, &events);
    let mut request: TicketOpenRequest = serde_json::from_value(vectors()["open"].clone()).unwrap(); request.draft.clear();
    let opened = service.open(&request).unwrap(); let id = TicketId::parse(&opened.id).unwrap();
    let relative = format!("{}/{}", "f".repeat(60), "x".repeat(80)); let source = join_path(&service.layout.ticket_dir(&id), &relative);
    let title = "r".repeat(120); let target_id = TicketId::new(26, 10, 21, ticket_slug_from_title(&title).unwrap());
    assert!(source.encode_utf16().count() <= 256); assert!(service.layout.important_path(&target_id).encode_utf16().count() <= 256); assert!(join_path(&service.layout.ticket_dir(&target_id), &relative).encode_utf16().count() > 256);
    store.seed_file(&source, "preserved"); let before = store.snapshot();
    assert!(service.change(&TicketChangeRequest { id: opened.id, title: Some(title), no_management: true, ..Default::default() }).is_err()); assert_eq!(store.snapshot(), before); assert!(!store.exists(&service.layout.ticket_dir(&target_id)));
}

#[test]
fn canonical_ticket_paths_preserve_shared_drive_and_unc_namespaces() {
    for row in vectors()["separatorCases"].as_array().unwrap() { let input = row["input"].as_str().unwrap(); assert_eq!(normalize_separators(input), row["normalized"].as_str().unwrap()); assert_eq!(dir_name(input), row["parent"].as_str().unwrap()); }
}

#[test]
fn rooted_ticket_document_round_trip_preserves_all_declared_state() {
    let data: Value = serde_json::from_str(include_str!("../../../../🎫️tickets/🧪️tests/🧫️fixtures/📄️ticket-document-codec/📄️documents.json")).unwrap();
    let source = data["documents"].as_array().unwrap().iter().find(|entry| entry["source"] == "DURABLE-TICKET-STATE").unwrap()["text"].as_str().unwrap();
    let expected: Value = serde_json::from_str(source).unwrap();
    let ticket = decode_ticket_document(source).unwrap();
    let first = encode_ticket_document(&ticket);
    assert_eq!(serde_json::from_str::<Value>(&first).unwrap(), expected);
    assert_eq!(serde_json::to_value(&ticket).unwrap(), expected);
    let second = encode_ticket_document(&decode_ticket_document(&first).unwrap());
    assert_eq!(first, second);
}
