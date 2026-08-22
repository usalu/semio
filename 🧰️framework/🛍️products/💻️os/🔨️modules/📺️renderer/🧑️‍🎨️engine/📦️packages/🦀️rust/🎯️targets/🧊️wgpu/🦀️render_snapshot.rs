//! @emoji 📸 `RenderSnapshot` — the immutable, atomically-published frame artifact ticket
//! `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR` Phase 3 (packet P3a) asks for, and
//! [`RenderSnapshotSink`] — its publish/acquire mechanism, mirroring the actor crate's already-proven
//! `SceneStore`/`SceneSnapshot` pattern (`🎭️actor/🦀️component.rs` ~1834-1918: `apply_patch`/
//! `commit_frame`, Arc-based copy-on-write, readers hold snapshots indefinitely). **Not** a raw
//! `Arc`+`AtomicPtr` scheme despite that being this packet's own first attempt — see
//! [`RenderSnapshotSink`]'s own doc for the real use-after-free bug a concurrent stress test caught in
//! that version and why a `Mutex<Arc<T>>` is the correct fix here, not a compromise.
//!
//! The mounted native renderer owns `AppRuntime` behind a bounded completion mailbox and submits
//! `AppRuntime::frame` through `FrameBuildHandle` to the process worker pool. The UI callback
//! non-blockingly polls the capacity-one completion channel, presents only an immutable prepared
//! packet, and keeps the last valid snapshot when the worker stalls. This sink carries the
//! cursor/IME scheduling subset of that publication. The retained hit-test index and damage regions
//! are still explicit optional gaps below; neither is represented by an invented placeholder.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use ui_render::{CursorRequest, ImeDirective};

//#region 🔖️RenderSnapshot

/// 📸️ One immutable, fully-prepared frame — everything [`crate::os_host::OsHost::redraw`] needs to
/// present without touching model state, running layout, or allocating substantially. Carries the
/// UI-thread-relevant subset the design doc's §4.2 sketch names (revision, generation, timestamp,
/// cursor/IME directives, damage regions); `dispatch_tree`/`wgpu_target`/texture-upload references from
/// that sketch are Element/FrameEngine-pipeline concepts this crate's still-`DrawList`-based renderer
/// does not yet have a real value for (see this file's own module docstring) — carried here as `None`-
/// capable `Option`s rather than invented placeholders, so a future packet's real values slot in without
/// a breaking field-shape change.
#[derive(Clone)]
pub struct RenderSnapshot {
    /// 🔢️ Bumped on every publish — lets a caller detect "this is the same snapshot I already
    /// presented" without comparing contents.
    pub revision: u64,
    /// 🔢️ P1e's own frame-generation counter, threaded through so a snapshot and the
    /// `semio_framework_trace::Watchdog` call that built it share one identity.
    pub generation: semio_framework_trace::Generation,
    pub timestamp_us: u64,
    /// 🖱️ The cursor this frame settled on — see `winit_app.rs`'s own `semio_cursor_to_request` for
    /// the narrowing from `AppRuntime`'s richer 13-variant `SemioCursor`.
    pub cursor: CursorRequest,
    pub ime: Option<ImeDirective>,
    /// 🌳️ Hit-test index for the frame this snapshot represents. This crate's actual hit-testing
    /// (`ui_wgpu::wgpu::InputState::hit_at`) is immediate-mode against live `AppRuntime` state, not a
    /// `ui_render::DispatchTree` — so there is no real value to carry here yet. `None` is an explicit
    /// remaining P3 input-contract gap, not a synthetic index.
    pub dispatch_tree: Option<Arc<()>>,
    /// 🩹️ Damage/dirty regions this frame touched — not tracked by the immediate-mode `DrawList`
    /// pipeline today (it always repaints the full surface); `None` until damage tracking exists.
    pub damage_regions: Option<Vec<()>>,
}

impl RenderSnapshot {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(revision: u64, generation: semio_framework_trace::Generation, timestamp_us: u64, cursor: CursorRequest, ime: Option<ImeDirective>) -> Self {
        Self { revision, generation, timestamp_us, cursor, ime, dispatch_tree: None, damage_regions: None }
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
    pub fn next_revision(&self) -> u64 {
        self.next_revision.fetch_add(1, Ordering::Relaxed)
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
        RenderSnapshot::new(revision, semio_framework_trace::Generation(revision), 0, CursorRequest::Default, None)
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
        assert!(b > a);
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
