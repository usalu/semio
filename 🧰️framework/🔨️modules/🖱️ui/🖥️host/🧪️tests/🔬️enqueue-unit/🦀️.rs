
use super::*;
use ui_render::{EventModifiers, PointerButton, PointerId, PointerInfo, PointerKind};

fn pointer() -> PointerInfo {
    PointerInfo { id: PointerId(1), kind: PointerKind::Mouse, pressure: None, tilt: None }
}

#[test]
fn pointer_move_storm_coalesces_to_one_sample() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for i in 0..1000 {
        queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: i as f32, y: 0.0 });
    }
    assert_eq!(queue.pending_discrete_len(), 0, "pointer move never touches the discrete queue");
    let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
    assert_eq!(drained.pointer_move.map(|sample| sample.x), Some(999.0), "only the latest position survives");
}

#[test]
fn scroll_storm_accumulates_delta_rather_than_overwriting() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for _ in 0..10 {
        queue.enqueue(ui, DispatchEvent::Scroll { x: 0.0, y: 0.0, delta_x: 0.0, delta_y: 1.0 });
    }
    let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
    assert_eq!(drained.scroll.map(|sample| sample.delta_y), Some(10.0), "10 wheel ticks of 1.0 each must sum, not overwrite");
}

#[test]
fn resize_storm_coalesces_to_the_latest_metrics() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for width in 100..2000u32 {
        queue.enqueue_metrics(ui, width, 600, 1.0);
    }
    let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
    assert_eq!(drained.metrics.map(|sample| sample.physical_width), Some(1999));
}

#[test]
fn discrete_events_are_never_dropped_under_capacity() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for _ in 0..DISCRETE_QUEUE_CAPACITY {
        let outcome = queue.enqueue(ui, DispatchEvent::KeyDown { key: "a".to_string(), modifiers: EventModifiers::default() });
        assert_eq!(outcome, EnqueueOutcome::Accepted);
    }
    let mut drained_count = 0;
    while !queue.is_empty() {
        let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
        drained_count += drained.discrete.into_iter().flatten().count();
    }
    assert_eq!(drained_count, DISCRETE_QUEUE_CAPACITY, "every discrete event up to capacity must survive paged drains");
    assert_eq!(queue.overflow_count(), 0);
}

#[test]
fn discrete_overflow_is_reported_not_silently_dropped() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for _ in 0..DISCRETE_QUEUE_CAPACITY {
        queue.enqueue(ui, DispatchEvent::KeyDown { key: "a".to_string(), modifiers: EventModifiers::default() });
    }
    let outcome = queue.enqueue(ui, DispatchEvent::KeyDown { key: "b".to_string(), modifiers: EventModifiers::default() });
    assert_eq!(outcome, EnqueueOutcome::Overflow);
    assert_eq!(queue.overflow_count(), 1, "overflow must be observable, never silent");
}

#[test]
fn discrete_drain_never_transfers_more_than_one_fixed_page() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for _ in 0..DISCRETE_QUEUE_CAPACITY {
        assert_eq!(queue.enqueue(ui, DispatchEvent::KeyDown { key: "x".repeat(DISCRETE_EVENT_BYTE_CAPACITY), modifiers: EventModifiers::default() }), EnqueueOutcome::Accepted);
    }
    let mut total = 0;
    while !queue.is_empty() {
        let page = queue.drain_page(WorkerContext::new(queue.current_generation()));
        let count = page.discrete.into_iter().flatten().count();
        assert!(count <= DISCRETE_DRAIN_PAGE_CAPACITY);
        total += count;
    }
    assert_eq!(total, DISCRETE_QUEUE_CAPACITY);
}

#[test]
fn one_variable_payload_over_the_page_credit_is_rejected() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    let outcome = queue.enqueue(ui, DispatchEvent::Paste { text: "x".repeat(DISCRETE_EVENT_BYTE_CAPACITY + 1) });
    assert_eq!(outcome, EnqueueOutcome::Overflow);
    assert!(queue.is_empty());
}

#[test]
fn pointer_down_up_are_lossless_and_ordered() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    queue.enqueue(ui, DispatchEvent::PointerDown { pointer: pointer(), x: 1.0, y: 1.0, button: PointerButton::Primary });
    queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: 5.0, y: 5.0 });
    queue.enqueue(ui, DispatchEvent::PointerUp { pointer: pointer(), x: 5.0, y: 5.0, button: PointerButton::Primary });
    let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
    let mut discrete = drained.discrete.into_iter().flatten();
    assert!(matches!(discrete.next().map(|event| event.event), Some(DispatchEvent::PointerDown { .. })));
    assert!(matches!(discrete.next().map(|event| event.event), Some(DispatchEvent::PointerUp { .. })));
    assert!(discrete.next().is_none());
}

#[test]
fn input_generation_increases_monotonically_and_survives_drain() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    let g0 = queue.current_generation();
    queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: 0.0, y: 0.0 });
    let g1 = queue.current_generation();
    assert!(g1 > g0, "enqueue must bump the generation");
    queue.enqueue(ui, DispatchEvent::KeyDown { key: "x".to_string(), modifiers: EventModifiers::default() });
    let g2 = queue.current_generation();
    assert!(g2 > g1);
    let drained = queue.drain_page(WorkerContext::new(g2));
    assert_eq!(drained.discrete.into_iter().flatten().next().expect("discrete event").generation, g2, "the discrete event carries the generation it was enqueued at");
}

#[test]
fn drain_leaves_the_queue_empty() {
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: 0.0, y: 0.0 });
    queue.enqueue(ui, DispatchEvent::KeyDown { key: "a".to_string(), modifiers: EventModifiers::default() });
    assert!(!queue.is_empty());
    while !queue.is_empty() {
        let _ = queue.drain_page(WorkerContext::new(queue.current_generation()));
    }
    assert!(queue.is_empty(), "paged drains must eventually leave the queue empty");
}
