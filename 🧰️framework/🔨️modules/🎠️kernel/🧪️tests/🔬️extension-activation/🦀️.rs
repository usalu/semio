
use super::*;
use std::future::Future;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// 🚫️async: E5 executor bridge — `extensions_extending`/`scope_capabilities_to_parent` are
/// pure `Vec`/`String` work with zero suspension points, so they complete on the very first
/// poll by construction; this hand-rolled poll-once bridge is the sanctioned E5 shape
/// (`📌️important.md` R2/R4 clause 5 — a `#[test] fn` body is a sanctioned executor entry
/// point) rather than pulling a runtime dependency into three crates for two pure fns. One per
/// crate this file is compiled into, as R2 requires.
fn block_on<F: Future>(future: F) -> F::Output {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("block_on: future was not ready on first poll — this fn is documented I/O-free"),
    }
}

fn descriptor(extension_id: &str, extends: &str) -> ExtensionDescriptor {
    ExtensionDescriptor { extension_id: extension_id.into(), extends: extends.into(), version: "0.1.0".into(), content_hash: format!("hash-{extension_id}"), capabilities: Vec::new(), capability_requests: Vec::new() }
}

#[test]
fn activation_event_can_be_retained_by_manifest_owners() {
    let original = ActivationEvent::OnCommand { id: "command.test".into() };
    let retained = original.clone();

    assert_eq!(retained, original);
}

/// 🧫️ 64 synthetic descriptors, half extending `flow` and half `cad` — a smaller stand-in for
/// the scale fixture's 50×50 shape, proving `extensions_extending` is a plain filter with no
/// branch on `installed.len()`.
#[test]
fn extensions_extending_filters_by_extends_at_scale_and_returns_none_for_an_unknown_plugin() {
    let installed: Vec<ExtensionDescriptor> = (0..64).map(|i| descriptor(&format!("ext-{i}"), if i % 2 == 0 { "flow" } else { "cad" })).collect();

    let matched = block_on(extensions_extending("flow", &installed));
    assert_eq!(matched.len(), 32, "half of 64 synthetic descriptors extend `flow`");
    assert!(matched.iter().all(|d| d.extends == "flow"));

    let none = block_on(extensions_extending("nonexistent-plugin", &installed));
    assert!(none.is_empty());
}

#[test]
fn scope_capabilities_to_parent_intersects_and_drops_what_the_parent_lacks() {
    let parent = vec![CapabilityId("storage.read".into()), CapabilityId("http:example.com".into())];
    let requested = vec![CapabilityId("storage.read".into()), CapabilityId("storage.write".into())];

    let scoped = block_on(scope_capabilities_to_parent(&parent, &requested));
    assert_eq!(scoped, vec![CapabilityId("storage.read".into())], "storage.write is not in the parent's effective set, so it must be dropped");
}

#[test]
fn scope_capabilities_to_parent_is_empty_when_the_parent_grants_nothing() {
    let requested = vec![CapabilityId("storage.read".into())];
    assert!(block_on(scope_capabilities_to_parent(&[], &requested)).is_empty());
}

fn presence_driver(pages: &[&[u8]], item_count: usize) -> CommandBatchDriver {
    let mut page_set = CommandPageSet::try_new().unwrap();
    if pages.is_empty() {
        page_set.try_push(FixedCommandPage::try_copy_from(&[]).unwrap()).unwrap();
    } else {
        for page in pages {
            page_set.try_push(FixedCommandPage::try_copy_from(page).unwrap()).unwrap();
        }
    }
    let command = PagedCommand::try_from_presence_pages(Some(7), page_set, item_count).unwrap();
    let mut commands = CommandEnvelopeSet::try_new().unwrap();
    commands.try_push(CommandEnvelope { instance: 3, seq: 9, command }).unwrap();
    let batch = CommandBatch::try_new(11, commands).unwrap();
    CommandBatchDriver::new(5, batch)
}

fn generic_driver(first: &[u8], last: &[u8]) -> CommandBatchDriver {
    let mut page_set = CommandPageSet::try_new().unwrap();
    page_set.try_push(FixedCommandPage::try_copy_from(first).unwrap()).unwrap();
    page_set.try_push(FixedCommandPage::try_copy_from(last).unwrap()).unwrap();
    let command = PagedCommand::try_from_pages(page_set).unwrap();
    let mut commands = CommandEnvelopeSet::try_new().unwrap();
    commands.try_push(CommandEnvelope { instance: 4, seq: 12, command }).unwrap();
    CommandBatchDriver::new(6, CommandBatch::try_new(13, commands).unwrap())
}

#[test]
fn generic_multi_page_owner_advances_only_after_each_exact_ack() {
    let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
    let mut driver = generic_driver(&first, b"tail");
    let (first_cursor, first_page) = driver.next_page().unwrap().unwrap();
    assert_eq!(first_cursor.page_count, 2);
    assert_eq!(first_page.len(), COMMAND_PAGE_MAXIMUM_BYTES);
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(first_cursor), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
    let (last_cursor, last_page) = driver.next_page().unwrap().unwrap();
    assert_eq!(last_cursor.page_index, 1);
    assert_eq!(last_page.as_slice(), b"tail");
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(last_cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Waiting);
    let mut terminal = last_cursor;
    terminal.page_index += 1;
    assert_eq!(driver.observe(&CommandIngressStatus::CommandComplete(terminal), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Complete);
}

#[test]
fn generic_backpressure_retains_the_exact_page_and_retry_cursor() {
    let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
    let mut driver = generic_driver(&first, b"tail");
    let (cursor, page) = driver.next_page().unwrap().unwrap();
    assert_eq!(driver.observe(&CommandIngressStatus::Backpressure(cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
    let (retry_cursor, retry_page) = driver.next_page().unwrap().unwrap();
    assert_eq!(retry_cursor, cursor);
    assert_eq!(retry_page, page);
}

#[test]
fn generic_stale_generation_status_is_rejected_without_releasing_the_owner() {
    let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
    let mut driver = generic_driver(&first, b"tail");
    let (mut stale, page) = driver.next_page().unwrap().unwrap();
    stale.generation -= 1;
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(stale), COMMAND_PAGE_MAXIMUM_BYTES).unwrap_err().code.0, "plugin.command-cursor-mismatch");
    assert_eq!(driver.next_page().unwrap().unwrap().1, page);
}

#[test]
fn generic_cancel_after_first_page_releases_untouched_tail_in_one_bounded_close_step() {
    let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
    let mut driver = generic_driver(&first, b"tail");
    let (cursor, _) = driver.next_page().unwrap().unwrap();
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
    assert_eq!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 4));
    assert!(driver.terminal_is_empty());
}

#[test]
fn fixed_command_driver_registry_returns_exact_colliding_owner_without_replacement() {
    let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
    let mut registry = CommandDriverRegistry::<2>::new();
    registry.try_insert(1, 9, generic_driver(&first, b"one")).unwrap();
    let (_, mut rejected) = registry.try_insert(3, 10, generic_driver(&first, b"rejected")).unwrap_err();
    assert_eq!(rejected.next_page().unwrap().unwrap().1.as_slice(), &first);
    registry.begin_close(1, 9).unwrap();
    while !registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0 {}
    assert!(registry.terminal_is_empty());
}

#[test]
fn suspended_command_driver_becomes_bounded_close_authority_if_caller_does_not_resume() {
    let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
    let mut registry = CommandDriverRegistry::<2>::new();
    registry.try_insert(1, 9, generic_driver(&first, b"tail")).unwrap();
    registry.prepare_suspend(1, 9).unwrap();
    assert_eq!(registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (false, 1, COMMAND_PAGE_MAXIMUM_BYTES));
    assert_eq!(registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 1, 4));
    assert!(registry.terminal_is_empty());
}

#[test]
fn stale_command_driver_resume_cannot_reanimate_a_reused_direct_slot() {
    let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
    let mut registry = CommandDriverRegistry::<1>::new();
    registry.try_insert(1, 9, generic_driver(&first, b"tail")).unwrap();
    registry.prepare_suspend(1, 9).unwrap();
    assert_eq!(registry.resume(1, 8).unwrap_err().code.0, "plugin.command-driver-stale");
    registry.begin_close(1, 9).unwrap();
    while !registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0 {}
}

#[test]
fn retained_batch_arena_has_no_nested_page_or_descriptor_destructor() {
    assert!(!std::mem::needs_drop::<FixedCommandPage>());
    let mut commands = CommandEnvelopeSet::try_new().unwrap();
    for seq in 0..COMMAND_BATCH_MAXIMUM_ITEMS as u64 {
        let mut pages = CommandPageSet::try_new().unwrap();
        pages.try_push(FixedCommandPage::try_copy_from(&[3]).unwrap()).unwrap();
        commands.try_push(CommandEnvelope { instance: 1, seq, command: PagedCommand::try_from_pages(pages).unwrap() }).unwrap();
    }
    let mut driver = CommandBatchDriver::new(7, CommandBatch::try_new(8, commands).unwrap());
    for _ in 0..COMMAND_BATCH_MAXIMUM_ITEMS {
        assert_eq!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES).1, 1);
    }
    assert!(driver.terminal_is_empty());
}

#[test]
fn fault_after_last_page_ack_closes_the_empty_descriptor_shell_without_page_release_theater() {
    let mut driver = presence_driver(&[b"one"], 1);
    let (cursor, _) = driver.next_page().unwrap().unwrap();
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Waiting);
    assert_eq!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 0));
    assert!(driver.terminal_is_empty());
}

#[test]
fn rejected_command_build_registry_retains_collision_and_releases_one_exact_page() {
    let mut rejected_pages = CommandPageSet::try_new().unwrap();
    rejected_pages.try_push(FixedCommandPage::try_copy_from(b"rejected").unwrap()).unwrap();
    let rejected = CommandEnvelope { instance: 1, seq: 2, command: PagedCommand::try_from_pages(rejected_pages).unwrap() };
    let mut registry = RejectedCommandBuildRegistry::<1>::new();
    registry.try_insert(1, RejectedCommandBuild::new(CommandEnvelopeSet::try_new().unwrap(), rejected)).unwrap();

    let mut colliding_pages = CommandPageSet::try_new().unwrap();
    colliding_pages.try_push(FixedCommandPage::try_copy_from(b"collision").unwrap()).unwrap();
    let collision = RejectedCommandBuild::new(CommandEnvelopeSet::try_new().unwrap(), CommandEnvelope { instance: 1, seq: 3, command: PagedCommand::try_from_pages(colliding_pages).unwrap() });
    let (_, mut collision) = registry.try_insert(2, collision).unwrap_err();
    assert_eq!(collision.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 9));
    assert_eq!(registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 1, 8));
    assert!(registry.terminal_is_empty());
}

#[test]
fn fixed_page_rejects_nonzero_padding_outside_declared_length() {
    let mut bytes = [0; COMMAND_PAGE_MAXIMUM_BYTES];
    bytes[7] = 1;
    assert_eq!(FixedCommandPage::try_from_array(bytes, 7).unwrap_err().code.0, "plugin.command-page-padding");
}

#[test]
fn zero_presence_page_ack_releases_the_present_empty_owner_then_completes() {
    let mut driver = presence_driver(&[], 0);
    let (cursor, bytes) = driver.next_page().unwrap().unwrap();
    assert!(bytes.is_empty());
    assert_eq!(cursor.kind, 28);
    assert_eq!(cursor.item_count, 0);
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Waiting);
    let mut terminal = cursor;
    terminal.page_index += 1;
    assert_eq!(driver.observe(&CommandIngressStatus::CommandComplete(terminal), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Complete);
    assert!(driver.terminal_is_empty());
}

#[test]
fn malformed_first_presence_fault_retains_then_closes_each_untouched_page() {
    let mut driver = presence_driver(&[b"bad-first", b"untouched-second"], 2);
    let (cursor, _) = driver.next_page().unwrap().unwrap();
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
    let mut fault_cursor = cursor;
    fault_cursor.page_index += 1;
    assert_eq!(driver.observe(&CommandIngressStatus::Fault { cursor: fault_cursor, fault: b"malformed".to_vec() }, COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Faulted);
    assert!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0);
    assert!(driver.terminal_is_empty());
}

#[test]
fn malformed_middle_presence_fault_preserves_fifo_tail_for_bounded_close() {
    let mut driver = presence_driver(&[b"first", b"bad-middle", b"tail"], 3);
    let (first, _) = driver.next_page().unwrap().unwrap();
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(first), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
    let (middle, _) = driver.next_page().unwrap().unwrap();
    assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(middle.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
    let mut fault_cursor = middle;
    fault_cursor.page_index += 1;
    assert_eq!(driver.observe(&CommandIngressStatus::Fault { cursor: fault_cursor, fault: b"malformed".to_vec() }, COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Faulted);
    assert!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0);
}
