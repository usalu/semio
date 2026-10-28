//! 🪟️ Group visibility: the shared decision bit with which a staged history group becomes visible to every direct reader at once.
//!
//! Holds the `🧩️GroupHistoryVisibility` region of the module; the root re-exports its items.

use super::*;

//#region 🧩️GroupHistoryVisibility
/// 🪟 A shared decision bit used by prepared read roots; only its unique publisher can switch it.
pub struct ArtifactGroupVisibility {
    state: std::sync::atomic::AtomicU8,
}

pub struct ArtifactGroupReadDecision<'a> {
    pub(super) visibility: &'a ArtifactGroupVisibility,
    pub(super) committed: bool,
}

impl ArtifactGroupReadDecision<'_> {
    pub fn committed_for(&self, visibility: &ArtifactGroupVisibility) -> Result<bool, ()> {
        if std::ptr::eq(self.visibility, visibility) {
            Ok(self.committed)
        } else {
            Err(())
        }
    }
}

/// 🗝️ Unique low-level publication owner; preparation and freshness remain the Store coordinator's responsibility.
pub struct ArtifactGroupVisibilityOwner {
    view: std::sync::Arc<ArtifactGroupVisibility>,
}

impl ArtifactGroupVisibilityOwner {
    /// 📐️ Exact backing reserved for the shared visibility decision.
    pub const fn allocation_bytes() -> usize {
        let word=std::mem::size_of::<usize>();
        2*word+(std::mem::size_of::<ArtifactGroupVisibility>()+word-1)/word*word
    }
    pub fn new() -> Self {
        Self { view: std::sync::Arc::new(ArtifactGroupVisibility { state: std::sync::atomic::AtomicU8::new(0) }) }
    }

    pub fn view(&self) -> std::sync::Arc<ArtifactGroupVisibility> {
        std::sync::Arc::clone(&self.view)
    }

    pub fn commit(&mut self) -> bool {
        self.view.state.compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    pub fn abort(&mut self) -> bool {
        self.view.state.compare_exchange(0, 2, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    /// ♻️ Moves the unique publisher's exact decision alias into funded retirement after aborting any unpublished intent.
    pub fn into_retirement(mut self) -> semio_framework_value::FactoryAuthority {
        self.abort();
        let original = std::mem::ManuallyDrop::new(self);
        let view = unsafe { std::ptr::read(&original.view) };
        semio_framework_value::FactoryAuthority::new(view)
    }
}

impl Drop for ArtifactGroupVisibilityOwner {
    fn drop(&mut self) {
        let _ = self.abort();
    }
}

impl ArtifactGroupVisibility {
    pub fn capture(&self) -> ArtifactGroupReadDecision<'_> {
        ArtifactGroupReadDecision { visibility: self, committed: self.committed() }
    }

    /// 🟢️ Reads whether the shared owner published its complete staged group.
    pub fn committed(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Acquire) == 1
    }

    /// 🟡️ Reads whether the shared owner still permits private candidate construction.
    pub fn pending(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Acquire) == 0
    }
}

pub(super) struct ArtifactHistoryGroupSuffix {
    pub(super) visibility: std::sync::Arc<ArtifactGroupVisibility>,
    pub(super) head: Option<u32>,
    pub(super) tail: Option<u32>,
    pub(super) len: usize,
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️group-history-visibility/🦀️.rs"]
mod group_history_visibility_tests;
//#endregion 🧩️GroupHistoryVisibility

impl semio_framework_value::RetainedPublicationDecision for ArtifactGroupVisibility{
 fn committed(&self)->bool{Self::committed(self)}
 fn pending(&self)->bool{Self::pending(self)}
 fn into_retirement(self:std::sync::Arc<Self>)->semio_framework_value::FactoryAuthority{semio_framework_value::FactoryAuthority::new(self)}
}
impl semio_framework_value::FactoryPayloadRetirement for ArtifactGroupVisibility{
 type CloseState=();
 fn close_state_birth_bytes(&self)->usize{0}
 fn close_state_constructor_depth(&self)->usize{0}
 fn close_state_constructor_copy_bytes(&self)->usize{0}
 fn prepare_close_state(&self){}
 fn close_state_preparation_demands(&self,_:&(),_:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
 fn prepare_close_state_step(&self,_:&mut(),_:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))}
 fn close_state_preparation_is_complete(_:&())->bool{true}
 fn transfer_payload(_value:Self,_:&mut()){}
 fn close_state_demands(_:&(),_:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
 fn close_state_step(_:&mut(),_:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))}
 fn close_state_terminal_is_empty(_:&())->bool{true}
}

#[cfg(test)]
#[test]
fn original_group_publisher_retirement_keeps_the_same_decision_and_physically_closes_its_exact_alias(){
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
 for committed in [false,true]{
  let mut publisher=ArtifactGroupVisibilityOwner::new();let visibility=publisher.view();if committed{assert!(publisher.commit());}
  let (mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||publisher.into_retirement());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(visibility.committed(),committed);assert!(!visibility.pending());drop(visibility);
  for index in 0..10000{if owner.terminal_is_empty(){break}let d=owner.demands(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};for axis in 0..5{let amount=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,3=>d.release_bytes,_=>d.depth};if amount==0{continue}let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=amount-1,2=>denied.maximum_capacity_bytes=amount-1,3=>denied.maximum_release_bytes=amount-1,_=>denied.maximum_depth=amount-1}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(denied));let p=match step{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert_eq!(p,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(index<9999)}assert!(owner.terminal_is_empty());
 }
}
