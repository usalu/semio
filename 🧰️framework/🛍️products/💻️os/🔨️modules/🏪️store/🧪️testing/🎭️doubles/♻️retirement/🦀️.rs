//! ♻️ Retirement doubles of the demo snapshot and mutation owners.
//!
//! 🧭️ Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.

use semio_framework_os_store_testing_prelude::*;
use semio_framework_os_store_testing_fixtures::*;
use crate::*;
use semio_framework_value::RetirementDemand;
use semio_framework_os_store_testing_fixtures::demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};
use semio_framework_os_store_testing_fixtures::lossy::{LossyMutation, SetN as LossySetN};
use semio_framework_os_store_testing_fixtures::severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};
use semio_framework_os_store_testing_fixtures::timestamped::{SetN as TimestampedSetN, TimestampedMutation};
use semio_framework_os_store_testing_fixtures::validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_snapshot_clone::RetainedCloneEdit;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditCursor;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditStep;
use semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_pack_json_retained_schema::testing::{OWNED_SCHEMA_TEST_FIELDS, owned_schema_test_cursor, drive_owned_schema, schema_test_close_grant};
use semio_framework_value::testing::physical_test_close_grant;
use crate::store::ArtifactStore;


#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DemoSnapshotRetirementFactory;

impl SnapshotRetirementFactory<DemoSnapshot> for DemoSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, snapshot: &Arc<DemoSnapshot>) -> usize {
        semio_framework_value::factory::SharedValueRetirementFactory::<DemoSnapshot>::default().retirement_birth_bytes(snapshot)
    }
    fn retire(&self, snapshot: Arc<DemoSnapshot>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<DemoSnapshot>)> {
        semio_framework_value::factory::SharedValueRetirementFactory::<DemoSnapshot>::default().retire(snapshot, grant)
    }
}

pub struct ExactDemoSnapshotRetirement {
    pub owner: std::mem::ManuallyDrop<Option<Arc<DemoSnapshot>>>,
    pub value: std::mem::ManuallyDrop<Option<DemoSnapshot>>,
    pub child: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub completed: std::mem::ManuallyDrop<Option<Arc<std::sync::atomic::AtomicUsize>>>,
    pub completion_recorded: bool,
}

impl ExactDemoSnapshotRetirement {
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(owner) = self.owner.as_ref() { return Ok(RetirementDemand { release_bytes: semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<DemoSnapshot>(),depth: 1,..Default::default() }); }
        if let Some(value) = self.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: DemoInitialSnapshotRetirementFactory.retirement_birth_bytes(value),depth: 2,..Default::default() }); }
        if let Some(child) = self.child.as_ref() { let mut demand=artifact_retirement_box_demands(child,body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"counted original child depth overflow"))?;return Ok(demand); }
        if !self.completion_recorded { return Ok(RetirementDemand { depth: 1,..Default::default() }); }
        Ok(self.completed.as_ref().map_or(Default::default(),|counter|RetirementDemand { release_bytes: if Arc::strong_count(counter)==1 { semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<std::sync::atomic::AtomicUsize>() } else { 0 },depth: 1,..Default::default() }))
    }
}

impl ErasedSnapshotRetirement for ExactDemoSnapshotRetirement {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"counted original close requires supplied depth")); }
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if let Some(owner)=self.owner.as_ref() {
            if Arc::strong_count(owner)!=1||Arc::weak_count(owner)!=0 { return Ok(RetainedCloneStep::Progress(empty)); }
            match Arc::try_unwrap(self.owner.take().unwrap()) {
                Ok(value)=>{*self.value=Some(value);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1,released_bytes: demand.release_bytes,..empty }));},
                Err(original)=>{*self.owner=Some(original);return Ok(RetainedCloneStep::Progress(empty));}
            }
        }
        let child_grant=RetainedCloneGrant { maximum_items: 1,maximum_depth: grant.maximum_depth-1,..grant };
        if let Some(value)=self.value.take() {
            match DemoInitialSnapshotRetirementFactory.retire_owned(value,child_grant) {
                Ok((owner,progress))=>{*self.child=Some(owner);return Ok(RetainedCloneStep::Progress(progress));},
                Err((error,original))=>{*self.value=Some(original);return Err(error);}
            }
        }
        if self.child.is_some() { return artifact_retirement_box_close_step(&mut self.child,child_grant).map(|step|RetainedCloneStep::Progress(step.progress())); }
        if !self.completion_recorded { self.completed.as_ref().unwrap().fetch_add(1,std::sync::atomic::Ordering::SeqCst);self.completion_recorded=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1,..empty })); }
        let counter=self.completed.as_ref().unwrap();
        if Arc::weak_count(counter)!=0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if Arc::strong_count(counter)!=1 { self.completed.take();return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,..empty })); }
        match Arc::try_unwrap(self.completed.take().unwrap()) {
            Ok(counter)=>{drop(counter);Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,released_bytes: demand.release_bytes,..empty }))},
            Err(original)=>{*self.completed=Some(original);Ok(RetainedCloneStep::Progress(empty))}
        }
    }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none()&&self.value.is_none()&&self.child.is_none()&&self.completed.is_none()&&self.completion_recorded }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}

impl Drop for ExactDemoSnapshotRetirement {
    fn drop(&mut self) { assert!(std::thread::panicking()||self.terminal_is_empty(),"counted original snapshot retains every payload and counter until funded terminal"); }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ExactDemoSnapshotRetirementFactory(#[factory_child] pub Arc<std::sync::atomic::AtomicUsize>);

impl SnapshotRetirementFactory<DemoSnapshot> for ExactDemoSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _: &Arc<DemoSnapshot>) -> usize { std::mem::size_of::<ExactDemoSnapshotRetirement>() }
    fn retire(&self, snapshot: Arc<DemoSnapshot>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<DemoSnapshot>)> {
        admit_artifact_retirement(snapshot,grant,|original|ExactDemoSnapshotRetirement { owner: std::mem::ManuallyDrop::new(Some(original)),value: std::mem::ManuallyDrop::new(None),child: std::mem::ManuallyDrop::new(None),completed: std::mem::ManuallyDrop::new(Some(Arc::clone(&self.0))),completion_recorded: false })
    }
}


#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DemoInitialSnapshotRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<DemoSnapshot> for DemoInitialSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, value: &DemoSnapshot) -> usize {
        semio_framework_value::factory::OwnedValueRetirementFactory::<DemoSnapshot>::default().retirement_birth_bytes(value)
    }
    fn retire_owned(&self, value: DemoSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, DemoSnapshot)> {
        semio_framework_value::factory::OwnedValueRetirementFactory::<DemoSnapshot>::default().retire_owned(value, grant)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DemoMutationRetirementFactory;

impl<Mutation: semio_framework_value::retirement::RetireOwned> ArtifactOwnedValueRetirementFactory<Mutation> for DemoMutationRetirementFactory {
    fn retirement_birth_bytes(&self, value: &Mutation) -> usize {
        semio_framework_value::factory::OwnedValueRetirementFactory::<Mutation>::default().retirement_birth_bytes(value)
    }
    fn retire_owned(&self, value: Mutation, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Mutation)> {
        semio_framework_value::factory::OwnedValueRetirementFactory::<Mutation>::default().retire_owned(value, grant)
    }
}

/// 🧹️ Retires a detached demo envelope a test compared against, through the store's own envelope retirement.
pub fn retire_demo_envelope<Mutation: semio_framework_value::retirement::RetireOwned>(envelope: ArtifactEnvelope<DemoSnapshot, Mutation>) {
    let grant=physical_test_close_grant();
    let(owner,progress)=retire_document_envelope(envelope,Arc::new(DemoInitialSnapshotRetirementFactory),Arc::new(DemoMutationRetirementFactory),grant).unwrap_or_else(|(error,_,_,_)|panic!("declared envelope birth refused: {error}"));
    assert!(progress.fits(grant));drive_retirement_terminal(owner);
}

/// 🎟️ The demo fixture admits an original alias frame with independent zero-copy constructor authority.
pub fn admit_demo_alias_retirement(store:&ArtifactStore<DemoSnapshot,DemoMutation>,alias:Arc<DemoSnapshot>)->Box<dyn ErasedSnapshotRetirement>{
    let mut original=Some(alias);let grant=RetainedCloneGrant{maximum_copy_bytes:0,maximum_release_bytes:0,..physical_test_close_grant()};
    let (owner,progress)=store.retire_snapshot_alias(&mut original,grant).expect("exact original demo alias issuer").expect("original alias is present");assert!(progress.fits(grant));assert!(original.is_none());owner
}

pub fn drive_retirement_terminal(retirement: Box<dyn ErasedSnapshotRetirement>) {
    let grant = physical_test_close_grant();
    let mut original = Some(retirement);
    for _ in 0..10_000 {
        let Some(owner) = original.as_ref() else { return; };
        let demand = artifact_retirement_box_demands(owner, grant.maximum_copy_bytes).expect("current original boxed owner frontier");
        let turn = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(grant.maximum_copy_bytes), maximum_capacity_bytes: demand.capacity_bytes.max(grant.maximum_capacity_bytes), maximum_release_bytes: demand.release_bytes.max(grant.maximum_release_bytes), maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
        let step = artifact_retirement_box_close_step(&mut original, turn).expect("original owner and Box close in distinct granted turns");
        assert!(step.progress().fits(turn));
    }
    panic!("retained original owner must reach terminal empty across interruptions");
}



pub struct DemoPublishedRootWitness {
    pub held: std::sync::atomic::AtomicBool,
    pub value: std::cell::UnsafeCell<Option<std::sync::Weak<DemoSnapshot>>>,
}

/// 🔐️ The fixed inline guard is the sole accessor to this fixture's snapshot Weak.
unsafe impl Sync for DemoPublishedRootWitness {}

impl DemoPublishedRootWitness {
    pub fn new() -> Self { Self { held: std::sync::atomic::AtomicBool::new(false), value: std::cell::UnsafeCell::new(None) } }
    pub fn lock(&self) -> Result<DemoPublishedRootGuard<'_>, std::convert::Infallible> {
        while self.held.compare_exchange(false, true, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed).is_err() { std::hint::spin_loop(); }
        Ok(DemoPublishedRootGuard(self))
    }
    pub fn publish(&self, root: &Arc<DemoSnapshot>) {
        let weak = Arc::downgrade(root);
        assert_eq!(std::sync::Weak::weak_count(&weak), 1, "fresh Demo publication has exactly its typed observer Weak");
        *self.lock().expect("root witness lock") = Some(weak);
    }
}

pub struct DemoPublishedRootGuard<'a>(pub &'a DemoPublishedRootWitness);

impl std::ops::Deref for DemoPublishedRootGuard<'_> {
    type Target = Option<std::sync::Weak<DemoSnapshot>>;
    fn deref(&self) -> &Self::Target { unsafe { &*self.0.value.get() } }
}

impl std::ops::DerefMut for DemoPublishedRootGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.0.value.get() } }
}

impl Drop for DemoPublishedRootGuard<'_> {
    fn drop(&mut self) { self.0.held.store(false, std::sync::atomic::Ordering::Release); }
}

pub struct DemoPublishedRootClose {
    pub weak: Option<std::sync::Weak<DemoSnapshot>>,
    pub root: Option<Arc<DemoSnapshot>>,
}

pub fn demo_snapshot_frame_bytes() -> usize {
    std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<DemoSnapshot>()).unwrap().0.pad_to_align().size()
}

impl semio_framework_value::FactoryPayloadRetirement for DemoPublishedRootWitness {
    type CloseState = DemoPublishedRootClose;
    fn close_state_birth_bytes(&self) -> usize { 0 }
    fn close_state_constructor_depth(&self)->usize { 0 }
    fn close_state_constructor_copy_bytes(&self)->usize { 0 }
    fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError> { Ok(Default::default()) }
    fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
    fn close_state_preparation_is_complete(_:&Self::CloseState)->bool { true }
    fn prepare_close_state(&self)->Self::CloseState { DemoPublishedRootClose { weak:None,root:None } }
    fn transfer_payload(value:Self,state:&mut Self::CloseState) { state.weak=value.value.into_inner(); }
    fn close_state_demands(state:&Self::CloseState,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        let release_bytes=if let Some(root)=state.root.as_ref() { if Arc::strong_count(root)==1 { demo_snapshot_frame_bytes() } else { 0 } } else if state.weak.as_ref().is_some_and(|weak|weak.strong_count()==0) { demo_snapshot_frame_bytes() } else { 0 };
        Ok(semio_framework_value::RetirementDemand { release_bytes,depth:usize::from(state.weak.is_some()||state.root.is_some()),..Default::default() })
    }
    fn close_state_step(state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if Self::close_state_terminal_is_empty(state) { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original witness requires supplied depth")); }
        let demand=Self::close_state_demands(state,grant.maximum_copy_bytes)?;
        if grant.maximum_release_bytes<demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if let Some(weak)=state.weak.as_ref() {
            if let Some(root)=weak.upgrade() { state.root=Some(root);drop(state.weak.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,..empty })); }
            drop(state.weak.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,released_bytes:demo_snapshot_frame_bytes(),..empty }));
        }
        if let Some(root)=state.root.as_ref() {
            if Arc::weak_count(root)!=0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original root remains witnessed by another Weak")); }
            let original=state.root.take().expect("funded original root remains present");
            let released_bytes=if let Some(DemoSnapshot { n })=Arc::into_inner(original) { drop(n);demo_snapshot_frame_bytes() } else { 0 };
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,released_bytes,..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool { state.weak.is_none()&&state.root.is_none() }
}

pub fn close(owner: &mut dyn ErasedSnapshotRetirement) -> usize {
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let (demands, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| RetirementDemand {
            copy_bytes: owner.next_copy_byte_demand().unwrap(),
            capacity_bytes: owner.next_capacity_byte_demand(owner.next_copy_byte_demand().unwrap()).unwrap(),
            release_bytes: owner.next_release_byte_demand().unwrap(), depth: owner.next_depth_demand().unwrap(),
        });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demands.copy_bytes, maximum_capacity_bytes: demands.capacity_bytes, maximum_release_bytes: demands.release_bytes, maximum_depth: demands.depth };
        for axis in 0..5 {
            let mut below = grant;
            match axis {
                0 => below.maximum_items = 0,
                1 if demands.copy_bytes > 0 => below.maximum_copy_bytes -= 1,
                2 if demands.capacity_bytes > 0 => below.maximum_capacity_bytes -= 1,
                3 if demands.release_bytes > 0 => below.maximum_release_bytes -= 1,
                4 if demands.depth > 0 => below.maximum_depth -= 1,
                _ => continue,
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(below).unwrap());
            assert_eq!(step.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes);
        assert_eq!(heap.released_bytes, step.progress().released_bytes);
        turns += 1;
        assert!(turns < 100000, "replay retirement stalled under its own quote {demands:?}; step {step:?}");
    }
    turns
}
