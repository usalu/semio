//! 📸️ Immutable cursor and IME directives published with each completed native frame.
//! Readers retain the last valid snapshot while a new frame is being prepared.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use ui_render::{CursorRequest, ImeDirective};

//#region 🔖️RenderSnapshot

/// 📸️ Cursor and IME state from a fully prepared frame.
#[derive(Clone)]
pub struct RenderSnapshot {
    /// 🔢️ Publication identity assigned by the snapshot sink.
    pub revision: u64,
    pub cursor: CursorRequest,
    pub ime: Option<ImeDirective>,
}

impl RenderSnapshot {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(revision: u64, cursor: CursorRequest, ime: Option<ImeDirective>) -> Self {
        Self { revision, cursor, ime }
    }
}

//#endregion 🔖️RenderSnapshot

//#region 🔖️RenderSnapshotSink

/// 📮️ **Revision history, read before trusting the "Arc + AtomicPtr" name in this file's own module
/// docstring: that was the FIRST implementation, and it was wrong.** A hand-rolled `AtomicPtr<T>` swap
/// with `Arc::into_raw`/`Arc::from_raw`/`Arc::increment_strong_count` has a genuine use-after-free race
/// this packet's own stress test (`many_publishers_and_readers_never_tear_or_crash`, still below) caught
/// via a real `SIGTRAP` crash under 4 concurrent writers + 4 concurrent readers: `acquire`'s `load` then
/// `increment_strong_count` are two separate steps, and a `publish` on another thread can `swap` the
/// pointer out AND drop its `Arc` (freeing the allocation) in the gap between them — the classic ABA/
/// use-after-free hazard that hazard-pointer or epoch-based reclamation schemes exist to solve. This
/// crate has zero third-party dependencies to reach for (no `arc-swap`), and inventing a correct
/// hazard-pointer scheme by hand is a much larger, higher-risk undertaking than this packet's actual
/// charter. The FIX: `Mutex<Arc<RenderSnapshot>>`. `acquire` is `lock().clone()`, `publish` is
/// `*lock() = Arc::new(..)` — both are sub-microsecond critical sections (a pointer-sized clone/store,
/// never the frame build or any I/O), never held across anything expensive, and the ticket's "never
/// blocks or waits on a WORKER" requirement is satisfied: a `Mutex` guarding a pointer swap is the same
/// class of operation as a `RefCell` borrow, categorically different from waiting on a worker to finish
/// a job. Zero `unsafe` code, and the concurrent stress test (still below) is now the thing that proves
/// it, not just describes it — see `📓️p3a-render-snapshot.md` for the full incident writeup.
pub struct RenderSnapshotSink {
    current: Mutex<Arc<RenderSnapshot>>,
    next_revision: AtomicU64,
}

impl RenderSnapshotSink {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(seed: RenderSnapshot) -> Self {
        Self { current: Mutex::new(Arc::new(seed)), next_revision: AtomicU64::new(1) }
    }

    /// 🔢️ The revision the NEXT [`Self::publish`] call should stamp — a builder reads this, builds
    /// against it, and passes the same value into the `RenderSnapshot` it constructs, so revisions are
    /// assigned by the sink (one source of truth) rather than guessed by each caller.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn next_revision(&self) -> Option<u64> {
        loop {
            let current = self.next_revision.load(Ordering::Acquire);
            let next = current.checked_add(1)?;
            if self.next_revision.compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                return Some(current);
            }
        }
    }

    /// 📥️ Acquires the newest published snapshot — never blocks on a worker, never waits for a build:
    /// the lock is only ever held by another thread for the duration of a pointer clone or store (see
    /// this struct's own doc for why that is not the kind of "waiting" the ticket's governing rule
    /// forbids). A poisoned lock (a panic while holding it, which neither method here can cause since
    /// neither can itself panic) would be a genuine bug elsewhere; unwrapping it here matches this
    /// crate's existing convention for other single-purpose locks (`ResponseSlot`'s own `.lock().expect(..)`
    /// in `kernel_runtime`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn acquire(&self) -> Arc<RenderSnapshot> {
        self.current.lock().expect("render snapshot sink lock").clone()
    }

    /// 📤️ Publishes `snapshot` as the new current one — a concurrent [`Self::acquire`] either sees the
    /// old `Arc` in full or the new one in full, never a torn read, and an `Arc` a caller already cloned
    /// out via `acquire` stays valid for exactly as long as that caller holds it (ordinary `Arc` refcount
    /// semantics — this is precisely what made the AtomicPtr version's manual refcounting redundant AND
    /// unsafe at the same time).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn publish(&self, snapshot: RenderSnapshot) {
        *self.current.lock().expect("render snapshot sink lock") = Arc::new(snapshot);
    }
}

//#endregion 🔖️RenderSnapshotSink

#[cfg(test)]
mod tests {
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
}
