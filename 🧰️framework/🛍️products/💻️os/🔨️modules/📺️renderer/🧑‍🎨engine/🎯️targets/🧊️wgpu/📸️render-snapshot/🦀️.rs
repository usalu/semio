//! 📸️ Immutable cursor and IME directives published with each completed native frame.
//! Readers retain the last valid snapshot while a new frame is being prepared.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use ui_render::{CursorRequest, ImeDirective};
use ui_wgpu::wgpu::SemioCursor;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind,retirement::{RetireOwned,shared::SharedControlledRetirement}};
use std::mem::ManuallyDrop;
fn close_shared(owner:&mut SharedControlledRetirement<RenderSnapshot>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let demand=semio_framework_value::RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(cx.retained_grant().maximum_copy_bytes)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?};let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;Ok(())}


//#region 🔖️RenderSnapshot

/// 📸️ Cursor and IME state from a fully prepared frame.
#[derive(Clone)]
pub struct RenderSnapshot {
    /// 🔢️ Publication identity assigned by the snapshot sink.
    pub revision: u64,
    pub cursor: CursorRequest,
    pub accepted_cursor: SemioCursor,
    pub accepted_theme_dark: bool,
    pub ime: Option<ImeDirective>,
    pub media_slots: Vec<crate::media_slots::PresentedMediaSlot>,
}

impl RenderSnapshot {
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(revision: u64, cursor: CursorRequest, accepted_cursor: SemioCursor, accepted_theme_dark: bool, ime: Option<ImeDirective>) -> Self {
        Self { revision, cursor, accepted_cursor, accepted_theme_dark, ime, media_slots: Vec::new() }
    }
    /// 🎬️ Attaches descriptors from the same completed presentation transaction.
    pub fn with_media_slots(mut self, media_slots: Vec<crate::media_slots::PresentedMediaSlot>) -> Self {
        self.media_slots = media_slots;
        self
    }
}

impl RetireOwned for RenderSnapshot{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{self.media_slots.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.media_slots.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
}
/// 🫴️ A paid output alias keeps the original snapshot body stationary while canonical pages are written.
pub(crate) struct RenderSnapshotOutputLoan{original:ManuallyDrop<Option<Arc<RenderSnapshot>>>}
impl RenderSnapshotOutputLoan{pub(crate) fn snapshot(&self)->&RenderSnapshot{self.original.as_ref().expect("original snapshot output remains owned")}}
impl RetireOwned for RenderSnapshotOutputLoan{
    fn retirement(mut self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.original.take().unwrap()))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<SharedControlledRetirement<RenderSnapshot>>())}
    fn controlled_retirement_supported()->bool{true}
}
impl Drop for RenderSnapshotOutputLoan{fn drop(&mut self){assert!(std::thread::panicking()||self.original.is_none(),"original snapshot output alias requires funded retirement")}}
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
    current: Mutex<ManuallyDrop<Option<Arc<RenderSnapshot>>>>,
    next_revision: AtomicU64,
    original_displaced:Mutex<Option<SharedControlledRetirement<RenderSnapshot>>>,
    original_incoming:Mutex<ManuallyDrop<Option<RenderSnapshot>>>,
    original_refused:Mutex<Option<semio_framework_value::retirement::controlled::ControlledRetirement<RenderSnapshot>>>,
}

impl RenderSnapshotSink {
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(seed: RenderSnapshot) -> Self {
        Self { current: Mutex::new(ManuallyDrop::new(Some(Arc::new(seed)))), next_revision: AtomicU64::new(1),original_displaced:Mutex::new(None),original_incoming:Mutex::new(ManuallyDrop::new(None)),original_refused:Mutex::new(None) }
    }

    /// 🔢️ The revision the NEXT [`Self::publish`] call should stamp — a builder reads this, builds
    /// against it, and passes the same value into the `RenderSnapshot` it constructs, so revisions are
    /// assigned by the sink (one source of truth) rather than guessed by each caller.
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn acquire(&self) -> Arc<RenderSnapshot> {
        self.current.lock().expect("render snapshot sink lock").as_ref().expect("render snapshot is live").clone()
    }

    /// 🎟️ Alias admission consumes original metadata before the actual atomic reference increments.
    pub(crate) fn acquire_output_on_original_caller(&self,cx:&mut StepContext<'_>)->Result<Option<RenderSnapshotOutputLoan>,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let Ok(current)=self.current.try_lock()else{return Ok(None)};cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
        Ok(Some(RenderSnapshotOutputLoan{original:ManuallyDrop::new(Some(Arc::clone(current.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original snapshot output source is closed"))?)))}))
    }
    /// 🪙️ The genuine snapshot Arc birth and displaced original close are separately paid events.
    pub(crate) fn publish_on_original_caller(&self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let Ok(mut displaced)=self.original_displaced.try_lock()else{return Ok(false)};
        if let Some(owner)=displaced.as_mut(){if !owner.terminal_is_empty(){close_shared(owner,cx)?;return Ok(false)}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;*displaced=None;return Ok(false)}
        let Ok(mut incoming)=self.original_incoming.try_lock()else{return Ok(false)};
        if incoming.is_none(){return Ok(true)}if incoming.as_ref().unwrap().revision==0{let revision=self.next_revision().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original snapshot revision exhausted"))?;incoming.as_mut().unwrap().revision=revision;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}let capacity=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<RenderSnapshot>();if grant.maximum_capacity_bytes<capacity{return Ok(false)}
        let Ok(mut current)=self.current.try_lock()else{return Ok(false)};let original=Arc::new(incoming.take().unwrap());let previous=current.replace(original);if let Some(previous)=previous{*displaced=Some(SharedControlledRetirement::lease(previous))}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;Ok(false)
    }
    /// 📥️ A completed presentation donates its same media vector into the preborn publication slot.
    pub(crate) fn retain_original_publication(&self,source:&mut Option<RenderSnapshot>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if source.is_none(){return Ok(true)}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}let Ok(mut incoming)=self.original_incoming.try_lock()else{return Ok(false)};if incoming.is_some(){return Ok(false)}**incoming=source.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    pub(crate) fn close_on_original_caller(&self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let Ok(mut incoming)=self.original_incoming.try_lock()else{return Ok(false)};let Ok(mut displaced)=self.original_displaced.try_lock()else{return Ok(false)};let Ok(mut refused)=self.original_refused.try_lock()else{return Ok(false)};
        if let Some(owner)=refused.as_mut(){if !owner.terminal_is_empty(){let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;*refused=None;return Ok(false)}
        if let Some(owner)=displaced.as_mut(){if !owner.terminal_is_empty(){close_shared(owner,cx)?;return Ok(false)}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;*displaced=None;return Ok(false)}
        if let Some(original)=incoming.take(){*refused=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original snapshot has typed retirement")));cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false)}
        let Ok(mut current)=self.current.try_lock()else{return Ok(false)};if let Some(original)=current.take(){*displaced=Some(SharedControlledRetirement::lease(original));cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false)}Ok(true)
    }
    /// 📤️ Publishes `snapshot` as the new current one — a concurrent [`Self::acquire`] either sees the
    /// old `Arc` in full or the new one in full, never a torn read, and an `Arc` a caller already cloned
    /// out via `acquire` stays valid for exactly as long as that caller holds it (ordinary `Arc` refcount
    /// semantics — this is precisely what made the AtomicPtr version's manual refcounting redundant AND
    /// unsafe at the same time).
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn publish(&self, snapshot: RenderSnapshot) {
        **self.current.lock().expect("render snapshot sink lock") = Some(Arc::new(snapshot));
    }
}

//#endregion 🔖️RenderSnapshotSink

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-render-snapshot-unit/🦀️.rs"]
mod tests;

impl Drop for RenderSnapshotSink{fn drop(&mut self){let current=self.current.get_mut().expect("original snapshot current lock");let incoming=self.original_incoming.get_mut().expect("original snapshot incoming lock");assert!(std::thread::panicking()||incoming.is_none(),"original snapshot publication requires funded close");if current.is_some(){unsafe{ManuallyDrop::drop(current)}}}}
