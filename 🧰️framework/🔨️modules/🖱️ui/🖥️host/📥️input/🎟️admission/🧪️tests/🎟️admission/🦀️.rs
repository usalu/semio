//! 🧪️ Actual input queue admission and terminal ownership laws before candidate production changes.

use super::*;

//#region 🧪️Admission
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
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
    let physical = queue.discrete.capacity() * size_of::<DiscreteEvent>();
    let expected = fixture()["physicalRetirement"]["initialQueueBackingBytes"].as_u64().unwrap();
    eprintln!("event-queue-constructor capacity={} slot-bytes={} physical={physical}", queue.discrete.capacity(), size_of::<DiscreteEvent>());
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
fn ordered_browser_input_preserves_pointer_samples_across_drain_pages() {
    use ui_render::{EventModifiers, PointerButton, PointerId, PointerInfo, PointerKind};
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    let pointer = PointerInfo { id: PointerId(7), kind: PointerKind::Mouse, pressure: None, tilt: None };
    let modifiers = EventModifiers::default();
    let down = DispatchEvent::PointerDown { pointer, x: 0.0, y: 0.0, button: PointerButton::Primary, modifiers };
    let movement = |x| DispatchEvent::PointerMove { pointer, x, y: 0.0, modifiers };
    let up = DispatchEvent::PointerUp { pointer, x: 4.0, y: 0.0, button: PointerButton::Primary, modifiers };
    let expected = [down, movement(1.0), movement(2.0), movement(3.0), up, movement(5.0)];
    assert!(!queue.prepare_ordered_batch(ui, DISCRETE_QUEUE_CAPACITY + 1, 0));
    assert_eq!(queue.current_generation(), InputGeneration(0));
    assert!(queue.prepare_ordered_batch(ui, expected.len(), 0));
    for event in expected.iter().cloned() {
        assert_eq!(queue.enqueue_ordered(ui, event), EnqueueOutcome::Accepted);
    }
    let generation = queue.current_generation();
    let first = queue.drain_page(WorkerContext::new(generation));
    let second = queue.drain_page(WorkerContext::new(generation));
    assert!(first.pointer_move.is_none() && second.pointer_move.is_none());
    let observed: Vec<_> = first.discrete.into_iter().chain(second.discrete).flatten().collect();
    assert_eq!(observed.len(), expected.len());
    for ((index, row), event) in observed.into_iter().enumerate().zip(expected) {
        assert_eq!(row.generation, InputGeneration(index as u64 + 1));
        assert_eq!(row.event, event);
    }
    assert!(queue.is_empty());
    drain(&mut queue);
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
    let physical = queue.discrete.capacity() * size_of::<DiscreteEvent>();
    assert_eq!(law["physicalRetirement"]["terminalRequiresEmptyBacking"], true);
    assert!(!terminal || physical == 0);
}
//#endregion 🧪️Admission
