
use super::*;

fn surface(value: &str) -> ui_contract::SurfaceId {
    ui_contract::SurfaceId::try_from(value).expect("bounded fixture surface")
}

#[test]
fn presenter_reading_a_not_b_wakes_only_on_a() {
    let mut tracker = DependencyTracker::default();
    let surface = surface("s");
    let a = EntityId(1);
    let b = EntityId(2);
    tracker.begin(surface.clone());
    tracker.record_read(a);
    tracker.finish(surface.clone());

    tracker.notify_entity(a);
    assert_eq!(tracker.drain_dirty().collect::<Vec<_>>(), vec![surface]);

    tracker.notify_entity(b);
    assert_eq!(tracker.drain_dirty().count(), 0);
}

#[test]
fn stale_edge_disappears_after_next_present_without_the_read() {
    let mut tracker = DependencyTracker::default();
    let surface = surface("s");
    let a = EntityId(1);
    tracker.begin(surface.clone());
    tracker.record_read(a);
    tracker.finish(surface.clone());
    assert_eq!(tracker.dirty_surfaces_for(a).collect::<Vec<_>>(), vec![surface.clone()]);

    tracker.begin(surface.clone());
    tracker.finish(surface);
    assert_eq!(tracker.dirty_surfaces_for(a).count(), 0);
}

#[test]
fn n_notifications_of_one_surface_coalesce_to_one_dirty_mark() {
    let mut tracker = DependencyTracker::default();
    let surface = surface("s");
    let a = EntityId(1);
    let b = EntityId(2);
    tracker.begin(surface.clone());
    tracker.record_read(a);
    tracker.record_read(b);
    tracker.finish(surface.clone());

    tracker.notify_entity(a);
    tracker.notify_entity(b);
    tracker.notify_entity(a);

    assert_eq!(tracker.drain_dirty().collect::<Vec<_>>(), vec![surface]);
}

#[test]
fn nested_present_scopes_attribute_reads_to_the_right_surface() {
    let mut tracker = DependencyTracker::default();
    let outer = surface("outer");
    let inner = surface("inner");
    let x = EntityId(1);
    let y = EntityId(2);
    let z = EntityId(3);

    tracker.begin(outer.clone());
    tracker.record_read(x);
    tracker.begin(inner.clone());
    tracker.record_read(y);
    tracker.finish(inner.clone());
    tracker.record_read(z);
    tracker.finish(outer.clone());

    assert_eq!(tracker.dirty_surfaces_for(x).collect::<Vec<_>>(), vec![outer.clone()]);
    assert_eq!(tracker.dirty_surfaces_for(y).collect::<Vec<_>>(), vec![inner]);
    assert_eq!(tracker.dirty_surfaces_for(z).collect::<Vec<_>>(), vec![outer]);
}

#[test]
fn reads_outside_a_present_scope_are_not_recorded() {
    let mut tracker = DependencyTracker::default();
    let a = EntityId(1);
    tracker.record_read(a);
    assert_eq!(tracker.dirty_surfaces_for(a).count(), 0);
}

#[test]
#[should_panic(expected = "present scope nesting mismatch")]
fn finish_rejects_mismatched_surface() {
    let mut tracker = DependencyTracker::default();
    tracker.begin(surface("a"));
    tracker.finish(surface("b"));
}
