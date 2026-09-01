//! 🧪️ Actual input queue admission and terminal ownership laws before candidate production changes.

use super::*;

//#region 🧪️Admission
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap()
}

fn drain(queue: &mut EventQueue) {
    for _ in 0..DISCRETE_QUEUE_CAPACITY + 2 {
        if queue.close_step() { return; }
    }
    panic!("fixture cleanup did not reach the old queue terminal");
}

#[test]
fn input_admission_constructor_has_no_unadmitted_backing() {
    let queue = EventQueue::new();
    let physical = queue.discrete.capacity() * std::mem::size_of::<DiscreteEvent>();
    let expected = fixture()["physicalRetirement"]["initialQueueBackingBytes"].as_u64().unwrap();
    eprintln!("[DEBUG] event-queue-constructor capacity={} slot-bytes={} physical={physical}", queue.discrete.capacity(), std::mem::size_of::<DiscreteEvent>());
    assert_eq!(physical as u64, expected);
}

#[test]
fn input_admission_full_refusal_preserves_generation() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for _ in 0..DISCRETE_QUEUE_CAPACITY {
        assert_eq!(queue.enqueue(ui, DispatchEvent::Paste { text: String::new() }), EnqueueOutcome::Accepted);
    }
    let before = queue.current_generation();
    let result = queue.enqueue(ui, DispatchEvent::Paste { text: String::new() });
    let after = queue.current_generation();
    drain(&mut queue);
    assert_eq!(result, EnqueueOutcome::Overflow);
    assert_eq!(after, before);
}

#[test]
fn input_admission_generation_exhaustion_does_not_wrap() {
    let mut queue = EventQueue::new();
    queue.generation = InputGeneration(u64::MAX);
    let result = queue.enqueue(UiThreadToken::mint(), DispatchEvent::Paste { text: String::new() });
    let after = queue.current_generation();
    let count = queue.pending_discrete_len();
    drain(&mut queue);
    assert_eq!(after, InputGeneration(u64::MAX));
    assert_eq!(result, EnqueueOutcome::Overflow);
    assert_eq!(count, 0);
}

#[test]
fn input_admission_metrics_generation_exhaustion_preserves_queue() {
    let mut queue = EventQueue::new();
    queue.generation = InputGeneration(u64::MAX);
    queue.enqueue_metrics(UiThreadToken::mint(), 800, 600, 2.0);
    let after = queue.current_generation();
    let empty = queue.coalesced.is_empty();
    drain(&mut queue);
    assert_eq!(after, InputGeneration(u64::MAX));
    assert!(empty);
}

#[test]
fn input_admission_terminal_requires_empty_backing() {
    let law = fixture();
    let mut queue = EventQueue::new();
    let mut text = String::new();
    text.try_reserve_exact(law["physicalRetirement"]["payloadMinimumCapacity"].as_u64().unwrap() as usize).unwrap();
    text.push_str(law["ownedEvent"]["text"].as_str().unwrap());
    let pointer = text.as_ptr();
    let payload_capacity = text.capacity();
    let logical = text.len();
    assert_eq!(queue.enqueue(UiThreadToken::mint(), DispatchEvent::Paste { text }), EnqueueOutcome::Accepted);
    let DispatchEvent::Paste { text } = &queue.discrete.front().unwrap().event else { unreachable!() };
    assert_eq!(text.as_ptr(), pointer);
    assert_eq!(text.capacity(), payload_capacity);
    assert!(payload_capacity > logical);
    drain(&mut queue);
    let terminal = queue.terminal_is_empty();
    let physical = queue.discrete.capacity() * std::mem::size_of::<DiscreteEvent>();
    eprintln!("[DEBUG] event-queue-terminal logical={logical} original-payload-capacity={payload_capacity} retained-queue-backing={physical} terminal={terminal}");
    assert_eq!(law["physicalRetirement"]["terminalRequiresEmptyBacking"], true);
    assert!(!terminal || physical == 0);
}
//#endregion 🧪️Admission
