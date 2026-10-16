use super::*;

fn publish(authority: &mut TextEditAuthority, text: String, start: usize, end: usize) -> usize {
    authority.enqueue_owned(authority.generation(), text, start, end).unwrap();
    for _ in 0..256 {
        if let TextEditProgress::Published { caret } = authority.step(authority.generation(), 1, false).unwrap() {
            while authority.retirement_count != 0 {
                authority.step(authority.generation(), 1, false).unwrap();
            }
            return caret;
        }
    }
    panic!("edit did not publish");
}

#[test]
fn multi_megabyte_root_middle_paste_is_atomic_and_exact() {
    let mut authority = TextEditAuthority::new(TextRoot::default(), 7);
    for _ in 0..128 {
        let end = authority.len();
        publish(&mut authority, "a".repeat(TEXT_PAGE_BYTES), end, end);
    }
    for _ in 0..128 {
        let end = authority.len();
        publish(&mut authority, "b".repeat(TEXT_PAGE_BYTES), end, end);
    }
    let token = authority.begin(7, TEXT_INGRESS_BYTES, 2 * 1024 * 1024, 2 * 1024 * 1024).unwrap();
    for _ in 0..16 {
        authority.push(token, "x".repeat(TEXT_PAGE_BYTES)).unwrap();
    }
    authority.commit(token).unwrap();
    let mut published = None;
    for _ in 0..64 {
        if let TextEditProgress::Published { caret } = authority.step(7, 1, false).unwrap() {
            published = Some(caret);
            break;
        }
    }
    assert_eq!(published, Some(2 * 1024 * 1024 + TEXT_INGRESS_BYTES));
    authority.start_projection(2 * 1024 * 1024, TEXT_PROJECTION_BYTES).unwrap();
    let projected = loop {
        if let Some(projected) = authority.step_projection(1).unwrap() {
            break projected;
        }
    };
    assert_eq!(projected, "x".repeat(TEXT_PROJECTION_BYTES));
    assert_eq!(authority.undo(), Some(2 * 1024 * 1024));
    assert_eq!(authority.root().len(), 4 * 1024 * 1024);
}

#[test]
fn aggregate_credits_zero_budget_and_bounded_cancel_are_deterministic() {
    let mut authority = TextEditAuthority::new(TextRoot::default(), 1);
    let token = authority.begin(1, TEXT_INGRESS_BYTES, 0, 0).unwrap();
    assert_eq!(authority.begin(1, 1, 0, 0), Err(TextEditFault::ByteCredits));
    assert_eq!(authority.step(1, 0, false), Ok(TextEditProgress::Yield));
    for _ in 0..16 {
        authority.push(token, "x".repeat(TEXT_PAGE_BYTES)).unwrap();
    }
    authority.commit(token).unwrap();
    let mut turns = 0;
    while authority.cancel_step() {
        turns += 1;
        assert!(turns <= TEXT_PAGE_SLOTS + TEXT_ROOT_DEPTH);
    }
    assert_eq!(turns, 17);
    assert_eq!(authority.reserved_bytes(), 0);
}

#[test]
fn stale_generation_and_failed_page_admission_do_not_leak_credits() {
    let mut authority = TextEditAuthority::default();
    assert_eq!(authority.begin(2, 1, 0, 0), Err(TextEditFault::Generation));
    let token = authority.begin(1, 0, 0, 0).unwrap();
    for _ in 0..TEXT_PAGE_SLOTS {
        authority.push(token, String::new()).unwrap();
    }
    let before = authority.reserved_bytes();
    assert_eq!(authority.enqueue_owned(1, "z".to_string(), 0, 0), Err(TextEditFault::PageCredits));
    assert_eq!(authority.reserved_bytes(), before);
}

#[test]
fn close_drains_every_owned_page_and_root_incrementally() {
    let mut authority = TextEditAuthority::default();
    let token = authority.begin(1, TEXT_INGRESS_BYTES, 0, 0).unwrap();
    for _ in 0..16 {
        authority.push(token, "x".repeat(TEXT_PAGE_BYTES)).unwrap();
    }
    authority.commit(token).unwrap();
    let mut turns = 0;
    while !authority.close_step(1).unwrap() {
        turns += 1;
        assert!(turns < TEXT_PAGE_SLOTS + TEXT_ROOT_DEPTH);
    }
    assert_eq!(authority.reserved_bytes(), 0);
}

#[test]
fn slot_epoch_rejects_a_late_chunk_after_reuse() {
    let mut authority = TextEditAuthority::default();
    let stale = authority.begin(1, 1, 0, 0).unwrap();
    authority.abort(stale).unwrap();
    while authority.retire_one_operation() {}
    let current = authority.begin(1, 1, 0, 0).unwrap();
    assert_eq!(stale.slot, current.slot);
    assert_ne!(stale.epoch, current.epoch);
    assert_eq!(authority.push(stale, "x".to_string()), Err(TextEditFault::Protocol));
    authority.push(current, "y".to_string()).unwrap();
}

#[test]
fn unicode_boundaries_keep_projection_and_caret_valid() {
    let mut authority = TextEditAuthority::default();
    let caret = publish(&mut authority, "aé🚀z".to_string(), 0, 0);
    assert_eq!(caret, "aé🚀z".len());
    assert!(!authority.root().is_char_boundary(2).unwrap());
    assert_eq!(authority.start_projection(2, 4), Err(TextEditFault::Protocol));
    let middle = authority.root().previous_boundary("aé🚀".len()).unwrap();
    assert_eq!(middle, "aé".len());
}

#[test]
fn segmented_replacement_builds_independent_bounded_pages() {
    let mut authority = TextEditAuthority::default();
    let text = "🚀".repeat(TEXT_PAGE_BYTES / 2);
    assert_eq!(authority.replace_owned(text.clone()), Err(TextEditFault::ChunkTooLarge));
    let token = authority.begin(1, text.len(), 0, 0).unwrap();
    for chunk in text.as_bytes().chunks(TEXT_PAGE_BYTES) {
        authority.push(token, std::str::from_utf8(chunk).unwrap().to_string()).unwrap();
    }
    authority.commit(token).unwrap();
    let mut turns = 0;
    loop {
        turns += 1;
        if let TextEditProgress::Published { caret } = authority.step(1, 1, false).unwrap() {
            assert_eq!(caret, text.len());
            break;
        }
        assert!(turns < 128);
    }
    assert_eq!(authority.root().materialize(), text);
}
