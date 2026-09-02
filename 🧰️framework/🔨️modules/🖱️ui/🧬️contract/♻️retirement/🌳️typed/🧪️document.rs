use super::*;

fn lease() -> UiDocumentLease {
    let record = UiNodeRecord {
        id: UiNodeId(1), key: crate::UiText::try_from_str(&"🌊".repeat(128)).unwrap(),
        component: crate::Component::Extension(crate::ExtensionProps { extension: crate::UiText::try_from_str("typed").unwrap(), props: serde_json::from_value(serde_json::json!({"nested":["Grüße"]})).unwrap() }),
        layout: Default::default(), style: Default::default(), activity: Default::default(), disabled: false,
        transition: None, accessibility: Default::default(), bindings: Default::default(), menu: None, children: Default::default(),
    };
    let mut builder = UiDocumentBuilder::try_new(91, SurfaceId::try_from("typed").unwrap(), UiRevision(1), Some(record.id), 0).unwrap();
    builder.try_push(record).unwrap();
    builder.finish().unwrap()
}

fn close(lease: &mut UiDocumentLease, grant: usize) -> usize {
    let mut bytes = 0;
    for _ in 0..100_000 {
        let step = lease.close_step_with_grant(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete { assert!(lease.terminal_is_empty()); return bytes; }
        if !step.progressed { std::thread::yield_now(); }
    }
    panic!("exact document did not finish");
}

#[test]
fn instance_lifetime_ui_document_aliases_preserve_complete_multibyte_readers() {
    for grant in [1, 64, 4096] {
        let mut original = lease();
        let mut alias = original.try_alias().unwrap();
        let mut page = alias.read_node_page(0).unwrap().unwrap().into_record();
        let wire = serde_json::to_value(&page).unwrap();
        assert_eq!(original.close_step_with_grant(0, grant).unwrap(), UiValueRetirementStep::default());
        assert_eq!(original.close_step_with_grant(1, 0).unwrap(), UiValueRetirementStep::default());
        assert!(!original.close_step_with_grant(1, grant).unwrap().complete);
        assert_eq!(alias.header().unwrap().node_count, 1);
        assert_eq!(close(&mut alias, grant), 522);
        assert_eq!(serde_json::to_value(&page).unwrap(), wire);
        assert_eq!(close(&mut original, grant), 0);
        let mut cursor = UiTypedRetirementCursor::default();
        let mut bytes = 0;
        for _ in 0..100_000 {
            let step = cursor.advance(&mut page, 1, grant).unwrap();
            bytes += step.released_bytes;
            if step.complete { break; }
        }
        assert!(cursor.terminal_is_empty());
        assert_eq!(bytes, 530);
    }
}

#[test]
fn instance_lifetime_ui_document_claim_survives_global_close_and_cancellation() {
    let mut owner = lease();
    let handle = owner.handle.unwrap();
    assert!(owner.close_step_with_grant(1, 1).unwrap().progressed);
    assert!(owner.claimed);
    for _ in 0..64 { close_ui_document_page_with_grant(1, 1).unwrap(); }
    assert!(with_ui_document_arena(|arena| arena.slot(handle).is_some_and(|slot| slot.retirement_claimed && slot.nodes.len() == 1)));
    assert!(matches!(owner.read_node_page(0), Err(UiDocumentLeaseError::Closing)));
    for _ in 0..40 { owner.close_step_with_grant(1, 1).unwrap(); }
    drop(owner);
    assert!(with_ui_document_arena(|arena| arena.slot(handle).is_some()) && DOCUMENT_HANDBACKS.has_slot_pending(handle.slot));
    for _ in 0..100_000 {
        let step = close_ui_document_page_with_grant(1, 1).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= 1);
        if !with_ui_document_arena(|arena| arena.active(handle)) { return; }
        if !step.progressed { std::thread::yield_now(); }
    }
    panic!("cancelled exact document retained its slot");
}

#[test]
fn instance_lifetime_ui_document_contention_preserves_claim_and_zero_progress() {
    for started in [false, true] {
        let mut owner = lease();
        if started { owner.close_step_with_grant(1, 1).unwrap(); }
        let before = (owner.handle, owner.released, owner.claimed);
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let holder = std::thread::spawn(move || {
            let _guard = UI_DOCUMENT_ARENA.lock().unwrap();
            ready_tx.send(()).unwrap();
            let _ = release_rx.recv_timeout(std::time::Duration::from_millis(200));
        });
        ready_rx.recv().unwrap();
        let blocked = owner.close_step_with_grant(1, 1).unwrap();
        let after = (owner.handle, owner.released, owner.claimed);
        let _ = release_tx.send(());
        holder.join().unwrap();
        close(&mut owner, 4096);
        assert_eq!(blocked, UiValueRetirementStep::default());
        assert_eq!(before, after);
    }
}

#[test]
fn instance_lifetime_ui_document_drop_hands_back_without_waiting_for_arena() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap();
    for started in [false, true] {
        let mut owner = lease();
        let handle = owner.handle.unwrap();
        if started { owner.close_step_with_grant(1, 1).unwrap(); }
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let released = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let holder_released = released.clone();
        let holder = std::thread::spawn(move || {
            let _guard = UI_DOCUMENT_ARENA.lock().unwrap();
            ready_tx.send(()).unwrap();
            let _ = release_rx.recv_timeout(std::time::Duration::from_millis(200));
            holder_released.store(true, std::sync::atomic::Ordering::Release);
        });
        ready_rx.recv().unwrap();
        drop(owner);
        let waited = released.load(std::sync::atomic::Ordering::Acquire);
        let _ = release_tx.send(());
        holder.join().unwrap();
        let retained = with_ui_document_arena(|arena| arena.active(handle));
        for _ in 0..100_000 {
            close_ui_document_page_with_grant(1, 4096).unwrap();
            if !with_ui_document_arena(|arena| arena.active(handle)) { break; }
        }
        assert!(retained);
        assert!(!with_ui_document_arena(|arena| arena.active(handle)));
        assert_eq!(waited, fixture["ownership"]["dropWaitsForArena"].as_bool().unwrap());
    }
}
