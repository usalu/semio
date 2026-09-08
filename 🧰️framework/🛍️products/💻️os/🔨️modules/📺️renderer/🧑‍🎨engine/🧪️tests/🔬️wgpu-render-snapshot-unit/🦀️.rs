
use super::*;

fn snapshot(revision: u64) -> RenderSnapshot {
    RenderSnapshot::new(revision, CursorRequest::Default, None)
}

#[test]
fn acquire_before_any_publish_returns_the_seed() {
    let sink = RenderSnapshotSink::new(snapshot(0));
    assert_eq!(sink.acquire().revision, 0);
}

#[test]
fn publish_then_acquire_sees_the_new_snapshot() {
    let sink = RenderSnapshotSink::new(snapshot(0));
    sink.publish(snapshot(1));
    assert_eq!(sink.acquire().revision, 1);
}

#[test]
fn acquiring_twice_with_no_new_publish_re_presents_the_same_snapshot() {
    let sink = RenderSnapshotSink::new(snapshot(0));
    sink.publish(snapshot(7));
    let first = sink.acquire();
    let second = sink.acquire();
    assert_eq!(first.revision, second.revision, "no newer snapshot landed — acquire must never block or invent one");
}

#[test]
fn next_revision_is_monotonic() {
    let sink = RenderSnapshotSink::new(snapshot(0));
    let a = sink.next_revision();
    let b = sink.next_revision();
    assert!(matches!((a, b), (Some(a), Some(b)) if b > a));
}

#[test]
fn revision_exhaustion_is_permanent_and_preserves_last_valid_snapshot() {
    let sink = RenderSnapshotSink::new(snapshot(9));
    sink.next_revision.store(u64::MAX - 1, Ordering::Release);
    assert_eq!(sink.next_revision(), Some(u64::MAX - 1));
    assert_eq!(sink.next_revision(), None);
    assert_eq!(sink.acquire().revision, 9);
}

#[test]
fn an_acquired_snapshot_survives_being_superseded() {
    let sink = RenderSnapshotSink::new(snapshot(0));
    let held = sink.acquire();
    sink.publish(snapshot(1));
    sink.publish(snapshot(2));
    assert_eq!(held.revision, 0, "a caller still holding an old Arc must keep reading its own snapshot, not a freed one");
    assert_eq!(sink.acquire().revision, 2);
}

#[test]
fn many_publishers_and_readers_never_tear_or_crash() {
    use std::sync::Arc as StdArc;
    let sink = StdArc::new(RenderSnapshotSink::new(snapshot(0)));
    std::thread::scope(|scope| {
        for writer in 0..4u64 {
            let sink = sink.clone();
            scope.spawn(move || {
                for i in 0..500u64 {
                    sink.publish(snapshot(writer * 1000 + i));
                }
            });
        }
        for _ in 0..4 {
            let sink = sink.clone();
            scope.spawn(move || {
                for _ in 0..500 {
                    let acquired = sink.acquire();
                    let _ = acquired.revision;
                }
            });
        }
    });
}
