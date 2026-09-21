
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
        queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: i as f32, y: 0.0, modifiers: EventModifiers { shift: i == 0, ..Default::default() } });
    }
    assert_eq!(queue.pending_discrete_len(), 0, "pointer move never touches the discrete queue");
    let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
    assert_eq!(drained.pointer_move.map(|sample| sample.x), Some(999.0), "only the latest position survives");
    assert_eq!(drained.pointer_move.map(|sample| sample.modifiers.shift), Some(false), "the latest modifier snapshot replaces a stale held modifier");
}

fn ordered_scroll_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../📥️input/🎡️ordered-scroll/🧫️fixtures/🔣️.json")).unwrap()
}

fn fixture_modifiers(value: &serde_json::Value) -> EventModifiers {
    EventModifiers {
        shift: value["shift"].as_bool().unwrap(),
        ctrl: value["ctrl"].as_bool().unwrap(),
        alt: value["alt"].as_bool().unwrap(),
        meta: value["meta"].as_bool().unwrap(),
    }
}

fn fixture_event(value: &serde_json::Value) -> DispatchEvent {
    let number = |field: &str| value[field].as_f64().unwrap() as f32;
    match value["kind"].as_str().unwrap() {
        "scroll" => DispatchEvent::Scroll {
            x: number("x"),
            y: number("y"),
            delta_x: number("deltaX"),
            delta_y: number("deltaY"),
            modifiers: fixture_modifiers(&value["modifiers"]),
        },
        "pointer-move" => DispatchEvent::PointerMove { pointer: pointer(), x: number("x"), y: number("y"), modifiers: fixture_modifiers(&value["modifiers"]) },
        "key-down" => DispatchEvent::KeyDown { key: value["key"].as_str().unwrap().to_string(), modifiers: fixture_modifiers(&value["modifiers"]) },
        kind => panic!("unknown ordered Scroll fixture event {kind}"),
    }
}

fn assert_ordered_scroll_case(id: &str) {
    let fixture = ordered_scroll_fixture();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == id).unwrap();
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for event in row["physical"].as_array().unwrap() {
        assert_eq!(queue.enqueue(ui, fixture_event(event)), EnqueueOutcome::Accepted);
    }
    let expected = row["expectedQueue"].as_array().unwrap();
    let expected = expected.iter().map(fixture_event).collect::<Vec<_>>();
    let mut actual = Vec::new();
    while !queue.is_empty() {
        let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
        let mut page = drained.discrete.into_iter().flatten().map(|event| (event.generation, event.event)).collect::<Vec<_>>();
        if let Some(sample) = drained.pointer_move {
            page.push((sample.generation, DispatchEvent::PointerMove { pointer: sample.pointer, x: sample.x, y: sample.y, modifiers: sample.modifiers }));
        }
        page.sort_by_key(|(generation, _)| *generation);
        actual.extend(page.into_iter().map(|(_, event)| event));
    }
    assert_eq!(actual, expected, "{id}: ordered input and the retained pointer merge by their original generation across bounded pages");
}

#[test]
fn same_point_scrolls_keep_event_local_deltas_and_modifiers() {
    assert_ordered_scroll_case("same-point-modifier-snapshots");
}

#[test]
fn opposite_scrolls_remain_two_ordered_events() {
    assert_ordered_scroll_case("opposite-sign-scrolls");
}

#[test]
fn scroll_key_and_replaceable_pointer_keep_ingress_sequence() {
    assert_ordered_scroll_case("scroll-key-and-replaceable-pointer");
}

#[test]
fn retained_pointer_waits_for_every_older_ordered_page() {
    let fixture = ordered_scroll_fixture();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == "retained-pointer-after-an-older-page").unwrap();
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for event in row["physical"].as_array().unwrap() {
        assert_eq!(queue.enqueue(ui, fixture_event(event)), EnqueueOutcome::Accepted);
    }
    let first = queue.drain_page(WorkerContext::new(queue.current_generation()));
    assert!(first.pointer_move.is_none(), "a retained pointer sample cannot overtake older ordered input waiting beyond this fixed page");
    assert_eq!(first.discrete.into_iter().flatten().count(), DISCRETE_DRAIN_PAGE_CAPACITY);
}

#[test]
fn scroll_overflow_refuses_without_mutating_the_admitted_queue() {
    let fixture = ordered_scroll_fixture();
    let row = &fixture["overflow"];
    let mut queue = EventQueue::new();
    let ui = UiThreadToken::mint();
    for _ in 0..row["fillCount"].as_u64().unwrap() {
        assert_eq!(queue.enqueue(ui, fixture_event(&row["fill"])), EnqueueOutcome::Accepted);
    }
    let generation = queue.current_generation();
    assert_eq!(queue.enqueue(ui, fixture_event(&row["candidate"])), EnqueueOutcome::Overflow);
    assert_eq!(queue.current_generation(), generation);
    assert_eq!(queue.overflow_count(), row["expectedOverflowCount"].as_u64().unwrap());
    assert_eq!(queue.pending_discrete_len(), row["expectedPreservedCount"].as_u64().unwrap() as usize);
    let mut preserved = 0;
    while !queue.is_empty() {
        let drained = queue.drain_page(WorkerContext::new(queue.current_generation()));
        preserved += drained.discrete.into_iter().flatten().count();
    }
    assert_eq!(preserved, row["expectedPreservedCount"].as_u64().unwrap() as usize);
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
    queue.enqueue(ui, DispatchEvent::PointerDown { pointer: pointer(), x: 1.0, y: 1.0, button: PointerButton::Primary, modifiers: EventModifiers { shift: true, ..Default::default() } });
    queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: 5.0, y: 5.0, modifiers: EventModifiers::default() });
    queue.enqueue(ui, DispatchEvent::PointerUp { pointer: pointer(), x: 5.0, y: 5.0, button: PointerButton::Primary, modifiers: EventModifiers::default() });
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
    queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: 0.0, y: 0.0, modifiers: EventModifiers::default() });
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
    queue.enqueue(ui, DispatchEvent::PointerMove { pointer: pointer(), x: 0.0, y: 0.0, modifiers: EventModifiers::default() });
    queue.enqueue(ui, DispatchEvent::KeyDown { key: "a".to_string(), modifiers: EventModifiers::default() });
    assert!(!queue.is_empty());
    while !queue.is_empty() {
        let _ = queue.drain_page(WorkerContext::new(queue.current_generation()));
    }
    assert!(queue.is_empty(), "paged drains must eventually leave the queue empty");
}
