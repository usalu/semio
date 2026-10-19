//! 🏭️ Admitted factory tickets retain original aliases and publish each ownership currency.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,RetireOwned,factory_ticket_demands,close_factory_ticket};
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_derive::FactoryPayloadRetirement;
use std::{alloc::Layout, sync::Arc, mem::ManuallyDrop, marker::PhantomData};

#[path="📋️vector/🦀️.rs"]
pub mod vector;
pub use vector::FactoryVectorRetirement;

#[path="📦️owned/🦀️.rs"]
pub mod owned;
#[path="📦️boxed/🦀️.rs"]
pub mod boxed;

#[path="🚼️construction/🦀️.rs"]
pub mod construction;
pub use construction::{FactoryRetirementTicket,FactoryRetirementAdmissionError,FactoryChildSlot};
use construction::permits;

/// 🧬️ Concrete factory ownership supplies an inline state before any alias retires.
pub trait FactoryPayloadRetirement: Send + Sync + 'static {
    type CloseState: Send + 'static;
    fn close_state_birth_bytes(&self) -> usize;
    fn close_state_constructor_depth(&self) -> usize;
    fn close_state_constructor_copy_bytes(&self) -> usize;
    fn prepare_close_state(&self) -> Self::CloseState;
    fn close_state_preparation_demands(&self,state:&Self::CloseState,body:usize)->Result<RetirementDemand,ValueError>;
    fn prepare_close_state_step(&self,state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn close_state_preparation_is_complete(state:&Self::CloseState)->bool;
    fn transfer_payload(value: Self, state: &mut Self::CloseState) where Self: Sized;
    fn close_state_demands(state: &Self::CloseState, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError>;
    fn close_state_step(state: &mut Self::CloseState, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn close_state_terminal_is_empty(state: &Self::CloseState) -> bool;
}

macro_rules! scalar_factory {
    ($($ty:ty),*)=>{$(impl FactoryPayloadRetirement for $ty {
        type CloseState=();
        fn close_state_birth_bytes(&self)->usize {0}
        fn close_state_constructor_depth(&self)->usize {0}
        fn close_state_constructor_copy_bytes(&self)->usize {0}
        fn prepare_close_state(&self)->Self::CloseState {()}
        fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
        fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
        fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
        fn transfer_payload(value:Self,_:&mut Self::CloseState) {drop(value);}
        fn close_state_demands(_:&Self::CloseState,_:usize)->Result<RetirementDemand,ValueError> {Ok(RetirementDemand::default())}
        fn close_state_step(_:&mut Self::CloseState,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}
        fn close_state_terminal_is_empty(_:&Self::CloseState)->bool {true}
    })*};
}
scalar_factory!(std::sync::atomic::AtomicUsize,std::sync::atomic::AtomicBool);

/// 📬️ Object-safe factory ownership admits its complete constructor before transferring any alias.
pub trait FactoryRetirement: Send + Sync {
    fn factory_retirement_birth_bytes(&self) -> usize;
    fn factory_retirement_copy_byte_demand(&self) -> usize;
    fn factory_retirement_depth_demand(&self) -> usize;
    fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn FactoryRetirementTicket>, RetainedCloneProgress), FactoryRetirementAdmissionError>;
}

/// 🪆️ Child tickets occupy a fixed inline directory prepared within their parent's admitted birth.
pub struct FactoryChildTickets<const N:usize>{pub slots:[FactoryChildSlot;N],pub prepared:usize}
impl<const N:usize> FactoryChildTickets<N> {
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {self.slots.iter().find(|slot|!slot.terminal_is_empty()).map_or(Ok(Default::default()),|slot|slot.demands(copy))}
    pub fn terminal_is_empty(&self)->bool {self.slots.iter().all(FactoryChildSlot::terminal_is_empty)}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        match self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()) {
            Some(slot)=>match slot.close_step(grant)? {RetainedCloneStep::Complete(progress)=>Ok(RetainedCloneStep::Progress(progress)),step=>Ok(step)},
            None=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
        }
    }
}

struct FactoryTicket<T:FactoryPayloadRetirement> {alias:ManuallyDrop<Option<Arc<T>>>,state:ManuallyDrop<T::CloseState>}
fn arc_extent<T>()->usize {Layout::new::<[usize;2]>().extend(Layout::new::<T>()).expect("factory Arc concrete layout").0.pad_to_align().size()}
pub fn factory_arc_birth_bytes<T:FactoryPayloadRetirement>()->usize {arc_extent::<T>()}
pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {size_of::<FactoryTicket<T>>()}
pub fn factory_constructor_birth_bytes<T:FactoryPayloadRetirement>(child_constructor_bytes:usize)->usize {factory_arc_birth_bytes::<T>().checked_add(factory_retirement_frame_bytes::<T>()).and_then(|bytes|bytes.checked_add(child_constructor_bytes)).expect("factory constructor layout")}

impl<T:FactoryPayloadRetirement> FactoryRetirement for T {
    fn factory_retirement_birth_bytes(&self)->usize {factory_retirement_frame_bytes::<T>().checked_add(self.close_state_birth_bytes()).expect("factory ticket birth layout")}
    fn factory_retirement_copy_byte_demand(&self)->usize {self.close_state_constructor_copy_bytes()}
    fn factory_retirement_depth_demand(&self)->usize {self.close_state_constructor_depth().checked_add(1).expect("factory constructor depth")}
    fn preborn_factory_retirement(self:Arc<Self>,grant:RetainedCloneGrant)->Result<(Box<dyn FactoryRetirementTicket>,RetainedCloneProgress),FactoryRetirementAdmissionError> {
        let copy=self.factory_retirement_copy_byte_demand();
        let bytes=self.factory_retirement_birth_bytes();
        let d=RetirementDemand{copy_bytes:copy,capacity_bytes:bytes,depth:self.factory_retirement_depth_demand(),..Default::default()};
        if !permits(d,grant){return Err(FactoryRetirementAdmissionError{error:ValueError::literal(ValueRefusalKind::OwnershipLimit,"factory original constructor exceeds admitted full grant"),original:Some(self),ticket:None,progress:Default::default()});}
        let layout=Layout::new::<FactoryTicket<T>>();
        let Some(pointer)=std::ptr::NonNull::new(unsafe {std::alloc::alloc(layout)}.cast::<FactoryTicket<T>>()) else {return Err(FactoryRetirementAdmissionError{error:ValueError::literal(ValueRefusalKind::AllocationFailed,"factory ticket frame allocation failed"),original:Some(self),ticket:None,progress:Default::default()});};
        let state=self.prepare_close_state();
        let owner=unsafe {pointer.as_ptr().write(FactoryTicket {alias:ManuallyDrop::new(Some(self)),state:ManuallyDrop::new(state)});Box::from_raw(pointer.as_ptr())};
        Ok((owner,RetainedCloneProgress {copied_items:1,copied_bytes:copy,retained_capacity_bytes:bytes,..Default::default()}))
    }
}

impl<T:FactoryPayloadRetirement> FactoryRetirementTicket for FactoryTicket<T>{
 fn preparation_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.preparation_is_complete(){return Ok(Default::default());}
  let source=self.alias.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"unprepared factory ticket lost its original source"))?;
  let mut d=source.close_state_preparation_demands(&self.state,body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory original preparation depth overflow"))?;Ok(d)
 }
 fn prepare_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.preparation_is_complete(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if !permits(self.preparation_demands(grant.maximum_copy_bytes)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let source=self.alias.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"factory preparation lost original source"))?;let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=source.prepare_close_state_step(&mut self.state,child)?;
  crate::retained_clone::admit_retained_clone_close(grant,if self.preparation_is_complete(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())},self.preparation_is_complete(),"original factory preparation")
 }
 fn preparation_is_complete(&self)->bool{T::close_state_preparation_is_complete(&self.state)}
}

impl<T:FactoryPayloadRetirement> ErasedSnapshotRetirement for FactoryTicket<T> {
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.preparation_is_complete(){return self.preparation_demands(body);}
        if self.alias.is_some(){return Ok(RetirementDemand{release_bytes:arc_extent::<T>(),depth:1,..Default::default()});}
        let mut d=T::close_state_demands(&self.state,body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement nested depth overflow"))?;Ok(d)
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if !self.preparation_is_complete(){let step=self.prepare_step(grant)?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth"));}
        if let Some(alias)=self.alias.as_ref() {
            if Arc::weak_count(alias)!=0||grant.maximum_copy_bytes<self.next_copy_byte_demand()?||grant.maximum_release_bytes<arc_extent::<T>() {return Ok(RetainedCloneStep::Progress(empty));}
            let value=Arc::into_inner(self.alias.take().unwrap());
            let released_bytes=if let Some(value)=value {T::transfer_payload(value,&mut self.state);arc_extent::<T>()}else{0};
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:0,released_bytes,..empty}));
        }
        let step=T::close_state_step(&mut self.state,RetainedCloneGrant {maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant})?;
        if !step.progress().fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory payload exceeded its admitted full grant"));}
        if matches!(step,RetainedCloneStep::Complete(_))&&!T::close_state_terminal_is_empty(&self.state){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory payload returned a false terminal witness"));}
        Ok(step)
    }
    fn terminal_is_empty(&self)->bool {self.alias.is_none()&&T::close_state_terminal_is_empty(&self.state)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(0)?.copy_bytes);}if self.alias.is_some(){Ok(0)}else{Ok(T::close_state_demands(&self.state,0)?.copy_bytes)}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(copy)?.capacity_bytes);}if self.alias.is_some(){Ok(0)}else{Ok(T::close_state_demands(&self.state,copy)?.capacity_bytes)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(0)?.release_bytes);}if self.alias.is_some(){Ok(arc_extent::<T>())}else{Ok(T::close_state_demands(&self.state,0)?.release_bytes)}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(0)?.depth);}if self.alias.is_some(){Ok(1)}else{T::close_state_demands(&self.state,0)?.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement nested depth overflow"))}}
}
impl<T:FactoryPayloadRetirement> Drop for FactoryTicket<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"factory ticket abandoned its original alias and payload");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.alias);ManuallyDrop::drop(&mut self.state);}}}}

/// 📏️ Borrows each exact currency including the final admitted ticket-frame release.


/// 🧹️ Preserves the caller's independent grants through payload and final ticket-frame release.



/// 🔐️ Owns the original factory capability until its admitted typed payload ticket closes.
pub struct FactoryAuthority {
    source:ManuallyDrop<Option<Arc<dyn FactoryRetirement>>>,
    ticket:ManuallyDrop<Option<Box<dyn FactoryRetirementTicket>>>,
}
impl FactoryAuthority {
    pub fn new(source:Arc<dyn FactoryRetirement>)->Self {Self{source:ManuallyDrop::new(Some(source)),ticket:ManuallyDrop::new(None)}}
    pub fn factory_address(&self)->usize {self.source.as_ref().map_or(0,|source|Arc::as_ptr(source)as*const()as usize)}
    pub fn demands(&self,work:usize)->Result<RetirementDemand,ValueError>{
        if let Some(source)=self.source.as_ref(){return Ok(RetirementDemand{copy_bytes:source.factory_retirement_copy_byte_demand(),capacity_bytes:source.factory_retirement_birth_bytes(),depth:source.factory_retirement_depth_demand(),..Default::default()});}
        self.ticket.as_ref().map_or(Ok(Default::default()),|ticket|factory_ticket_demands(ticket,work))
    }
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory authority exceeds admitted depth"));}
        if !permits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.source.is_some(){
            let child=grant;
            let(ticket,receipt)=match self.source.take().unwrap().preborn_factory_retirement(child){Ok(value)=>value,Err(error)=>{*self.source=error.original;*self.ticket=error.ticket;return Err(error.error.with_retained_progress(error.progress));}};
            *self.ticket=Some(ticket);
            if !receipt.fits(grant)||receipt.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory authority changed its admitted constructor receipt").with_retained_progress(receipt));}
            return Ok(RetainedCloneStep::Progress(receipt));
        }
        let step=close_factory_ticket(&mut self.ticket,grant)?;
        Ok(if matches!(step,RetainedCloneStep::Complete(_))&&!self.terminal_is_empty(){RetainedCloneStep::Progress(step.progress())}else{step})
    }
    pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.ticket.is_none()}
}
impl crate::retirement::RetireOwned for FactoryAuthority{
    fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl crate::retirement::RetirementCursor for FactoryAuthority{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->crate::retirement::RetirementStep{match self.step(grant){Err(error)=>crate::retirement::RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>crate::retirement::RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>crate::retirement::RetirementStep::Progress(progress)}}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,work:usize)->Option<usize>{self.demands(work).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for FactoryAuthority{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"factory authority abandoned original typed payload");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.ticket);}}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

pub trait SnapshotRetirementFactory<P>: crate::FactoryRetirement + Send + Sync {
    /// 📦️ Publishes the retained cursor birth allocation before snapshot ownership transfers.
    fn retirement_birth_bytes(&self, snapshot: &Arc<P>) -> usize;
    fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)>;
}

pub trait ArtifactOwnedValueRetirementFactory<T>: crate::FactoryRetirement + Send + Sync {
    fn retirement_birth_bytes(&self, value: &T) -> usize;
    fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)>;
}

pub fn admit_original_owned_retirement<T>(original:&mut Option<T>,factory:&dyn ArtifactOwnedValueRetirementFactory<T>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,crate::retained_clone::RetainedCloneProgress)>,crate::ValueError> {
    let Some(value)=original.as_ref()else{return Ok(None)};
    let refusal=if grant.maximum_items==0{Some((crate::ValueRefusalKind::WorkLimit,"owned retirement requires one admitted item"))}else if grant.maximum_depth==0{Some((crate::ValueRefusalKind::DepthLimit,"owned retirement requires admitted depth"))}else if factory.retirement_birth_bytes(value)>grant.maximum_capacity_bytes{Some((crate::ValueRefusalKind::OwnershipLimit,"owned retirement exceeds admitted frame capacity"))}else{None};
    if let Some((kind,message))=refusal{return Err(crate::ValueError::literal(kind,message));}
    match factory.retire_owned(original.take().expect("observed exact original owned value"),grant){Ok(admitted)=>Ok(Some(admitted)),Err((error,value))=>{*original=Some(value);Err(error)}}
}
#[derive(semio_framework_value_derive::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub struct OwnedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for OwnedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned> ArtifactOwnedValueRetirementFactory<T> for OwnedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&T)->usize {retirement::owned_retirement_birth_bytes::<T>()}
    fn retire_owned(&self,value:T,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,crate::retained_clone::RetainedCloneProgress),(crate::ValueError,T)> {retirement::admit_owned_retirement(value,grant)}
}

/// 🔗️ Retires one store-owned snapshot alias as a lease: the last alias to close retires the payload, every other alias only drops its count, so sequentially closed owners of one Arc never wait on each other.
#[derive(semio_framework_value_derive::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub struct SharedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for SharedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned+Sync> SnapshotRetirementFactory<T> for SharedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&Arc<T>)->usize {retirement::arc_custody::shared_retirement_birth_bytes::<T>()}
    fn retire(&self,value:Arc<T>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,crate::retained_clone::RetainedCloneProgress),(crate::ValueError,Arc<T>)> {retirement::arc_custody::admit_shared_retirement(value,grant,true)}
}



#[path="../🔗️arc-custody/🏭️factory/🦀️.rs"]
pub mod shared;
pub use shared::FactorySharedRetirement;

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[cfg(any(test, feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
pub mod original_factory_fixture;
