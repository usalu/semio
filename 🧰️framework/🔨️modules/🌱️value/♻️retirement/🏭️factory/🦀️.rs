//! 🏭️ Admitted factory tickets retain original aliases and publish each ownership currency.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{retirement_fields_type,retirement_fields_seed,ErasedSnapshotRetirement,OriginalRetirementControl,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetireOwned,factory_ticket_demands,close_factory_ticket};
pub use semio_framework_value_retirement::retirement;
pub use semio_framework_value_derive::{FactoryPayloadRetirement,RetireOwned};
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
pub use construction::{FactoryRetirementTicket,FactoryProducerReply,FactoryProducerReplyComplete,FactoryChildSlot};
use construction::permits;

/// 🧬️ Concrete factory ownership supplies an inline state before any alias retires.
pub trait FactoryPayloadRetirement: Send + Sync + 'static {
    type CloseState: Send + 'static;
    fn close_state_birth_bytes(&self) -> usize;
    fn close_state_constructor_depth(&self) -> usize;
    fn close_state_constructor_copy_bytes(&self) -> usize;
    /// 🌱️ Moves only inline metadata; this seed cannot allocate, release, call out or unwind.
    fn prepare_close_state(&self) -> Self::CloseState;
    fn close_state_preparation_demands(&self,state:&Self::CloseState,body:usize)->Result<RetirementDemand,ValueError>;
    fn close_state_preparation_work_demand(&self,_state:&Self::CloseState,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory preparation lacks a native work quote"))}
    fn prepare_close_state_step(&self,state:&mut Self::CloseState,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll;
    fn close_state_preparation_is_complete(state:&Self::CloseState)->bool;
    /// 📦️ Transfers original fields into their prepared resident slots without allocation, release, callout or unwind.
    fn transfer_payload(value: Self, state: &mut Self::CloseState) where Self: Sized;
    fn close_state_demands(state: &Self::CloseState, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError>;
    /// 🪪️ Reports whether the selected state child owns an original retirement loan.
    fn close_state_requires_original_control(_state:&Self::CloseState)->bool{false}
    /// ⛽️ Quotes the selected state action before any effect.
    fn close_state_work_demand(_state:&Self::CloseState,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory state lacks a native work quote"))}
    /// 🧷️ Forwards the original selected child control.
    fn close_state_step_controlled(_state:&mut Self::CloseState,_effect:&mut RetainedAllocationEffect<'_>,_control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory state lacks original control").into()),work:0})}
    /// 🧵️ Forwards the original selected child decoder loan.
    fn close_state_original_step(_state:&mut Self::CloseState,_original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,_effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory state lacks original loan").into()),work:0})}
    fn close_state_step(state: &mut Self::CloseState, effect:&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll;
    /// 👶️ Selects the resident original birth before any general child effect.
    fn close_state_requires_original_birth(_state:&Self::CloseState)->bool{false}
    /// 🍼️ Preserves the original birth's independent marker and work proof.
    fn close_state_original_birth_step(_state:&mut Self::CloseState,_original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,_grant:RetainedCloneGrant)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory state has no selected original birth").into()),work:0})}
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
        fn close_state_preparation_work_demand(&self,_:&Self::CloseState,_:usize)->Result<u64,ValueError>{Ok(0)}
        fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
        fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
        fn transfer_payload(value:Self,_:&mut Self::CloseState) {drop(value);}
        fn close_state_demands(_:&Self::CloseState,_:usize)->Result<RetirementDemand,ValueError> {Ok(RetirementDemand::default())}
        fn close_state_work_demand(_:&Self::CloseState,_:usize)->Result<u64,ValueError>{Ok(0)}
        fn close_state_step(_:&mut Self::CloseState,_:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
        fn close_state_terminal_is_empty(_:&Self::CloseState)->bool {true}
    })*};
}
scalar_factory!(std::sync::atomic::AtomicUsize,std::sync::atomic::AtomicBool);

/// 📬️ Object-safe factory ownership admits its complete constructor before transferring any alias.
pub trait FactoryRetirement:Send+Sync{
 fn factory_retirement_work_demand(&self)->Result<u64,ValueError>;
 fn factory_retirement_birth_bytes(&self)->usize;
 fn factory_retirement_copy_byte_demand(&self)->usize;
 fn factory_retirement_depth_demand(&self)->usize;
 fn prepare_factory_retirement<'reply>(&self,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn FactoryRetirementTicket,ValueError>)->FactoryProducerReplyComplete<'reply,ValueError>;
 fn recover_factory_retirement<'reply>(&self,_effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn FactoryRetirementTicket,ValueError>)->FactoryProducerReplyComplete<'reply,ValueError>;
 /// 🪢️ Moves only the recorded original Arc into its prepared typed slot without allocation, release, callbacks or unwind.
 fn transfer_factory_original(self:Arc<Self>,ticket:&mut dyn FactoryRetirementTicket)->Result<(),Arc<dyn FactoryRetirement>>;
}
/// 🪆️ Child tickets occupy a fixed inline directory prepared within their parent's admitted birth.
pub struct FactoryChildTickets<const N:usize>{pub slots:[FactoryChildSlot;N],pub prepared:usize}
impl<const N:usize> FactoryChildTickets<N> {
    pub fn requires_original_birth(&self)->bool{self.slots.iter().find(|slot|!slot.terminal_is_empty()).is_some_and(FactoryChildSlot::requires_original_birth)}
    pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()).map_or(RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}),|slot|slot.close_original_birth_step(original,grant))}
    /// 🪪️ Reports whether the selected child owns an original ticket loan.
    pub fn requires_original_control(&self)->bool{self.slots.iter().find(|slot|!slot.terminal_is_empty()).is_some_and(FactoryChildSlot::requires_original_control)}
    /// ⛽️ Quotes the selected original child only.
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.slots.iter().find(|slot|!slot.terminal_is_empty()).map_or(Ok(0),|slot|slot.next_computational_work(body))}
    pub fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->OriginalRetirementPoll{let grant=effect.grant();match self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()){Some(slot)=>{let turn=slot.close_step_controlled(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"),control);turn.map(|step|RetainedCloneStep::Progress(step.progress()))},None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}}
    pub fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();match self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()){Some(slot)=>{let turn=slot.close_original_step(original,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));turn.map(|step|RetainedCloneStep::Progress(step.progress()))},None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}}
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {self.slots.iter().find(|slot|!slot.terminal_is_empty()).map_or(Ok(Default::default()),|slot|slot.demands(copy))}
    pub fn terminal_is_empty(&self)->bool {self.slots.iter().all(FactoryChildSlot::terminal_is_empty)}
    pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();
        match self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()){Some(slot)=>{let turn=slot.close_step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));turn.map(|step|RetainedCloneStep::Progress(step.progress()))},None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
    }
}

struct FactoryTicket<T:FactoryPayloadRetirement> {alias:ManuallyDrop<Option<Arc<T>>>,state:ManuallyDrop<T::CloseState>}
impl<T:FactoryPayloadRetirement> FactoryTicket<T>{fn alias_demand(&self)->Result<RetirementDemand,ValueError>{let source=self.alias.as_ref().ok_or_else(||ValueError::invariant("factory alias absent"))?;let copy_bytes=size_of::<Option<Arc<T>>>().checked_add(size_of::<T>()).and_then(|n|n.checked_add(source.close_state_constructor_copy_bytes())).ok_or_else(||ValueError::invariant("factory original transfer extent overflow"))?;Ok(RetirementDemand{copy_bytes,release_bytes:arc_extent::<T>(),depth:1,..Default::default()})}}
fn arc_extent<T>()->usize {Layout::new::<[usize;2]>().extend(Layout::new::<T>()).expect("factory Arc concrete layout").0.pad_to_align().size()}
pub fn factory_arc_birth_bytes<T:FactoryPayloadRetirement>()->usize {arc_extent::<T>()}
pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {size_of::<FactoryTicket<T>>()}
pub fn factory_constructor_birth_bytes<T:FactoryPayloadRetirement>(child_constructor_bytes:usize)->usize {factory_arc_birth_bytes::<T>().checked_add(factory_retirement_frame_bytes::<T>()).and_then(|bytes|bytes.checked_add(child_constructor_bytes)).expect("factory constructor layout")}

impl<T:FactoryPayloadRetirement> FactoryRetirement for T{
 fn factory_retirement_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn factory_retirement_birth_bytes(&self)->usize{size_of::<FactoryTicket<T>>()}
 fn factory_retirement_copy_byte_demand(&self)->usize{self.close_state_constructor_copy_bytes()}
 fn factory_retirement_depth_demand(&self)->usize{self.close_state_constructor_depth().checked_add(1).expect("factory constructor depth")}
 fn prepare_factory_retirement<'reply>(&self,effect:&mut RetainedAllocationEffect<'_>,mut reply:FactoryProducerReply<'reply,dyn FactoryRetirementTicket,ValueError>)->FactoryProducerReplyComplete<'reply,ValueError>{
  let bytes=self.factory_retirement_birth_bytes();let demand=RetirementDemand{copy_bytes:self.factory_retirement_copy_byte_demand(),capacity_bytes:bytes,depth:self.factory_retirement_depth_demand(),..Default::default()};
  if reply.has_output()||!permits(demand,effect.grant())||effect.maximum_work_units()<1||effect.actual().is_some(){return reply.complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"factory original constructor exceeds its admitted effect")),work:0})}
  if !reply.performed(effect,1).expect("original factory work sink"){return reply.complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"factory producer exceeded its original work quote")),work:1})}let layout=Layout::new::<FactoryTicket<T>>();*effect.actual()=Some(0);let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<FactoryTicket<T>>())else{return reply.complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"factory ticket frame allocation failed")),work:1})};*effect.actual()=Some(bytes);
  let state=self.prepare_close_state();let owner=unsafe{pointer.as_ptr().write(FactoryTicket{alias:ManuallyDrop::new(None),state:ManuallyDrop::new(state)});Box::from_raw(pointer.as_ptr())};reply.publish(owner);
  reply.complete(RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:bytes,..Default::default()}),work:1})
 }
 fn recover_factory_retirement<'reply>(&self,_effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn FactoryRetirementTicket,ValueError>)->FactoryProducerReplyComplete<'reply,ValueError>{reply.pending()}
 fn transfer_factory_original(self:Arc<Self>,ticket:&mut dyn FactoryRetirementTicket)->Result<(),Arc<dyn FactoryRetirement>>{let Some(slot)=ticket.original_slot().and_then(|slot|slot.downcast_mut::<Option<Arc<T>>>())else{return Err(self)};if slot.is_some(){return Err(self)}*slot=Some(self);Ok(())}
}

impl<T:FactoryPayloadRetirement> FactoryRetirementTicket for FactoryTicket<T>{
 fn preparation_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.preparation_is_complete(){return Ok(0)}self.alias.as_ref().ok_or_else(||ValueError::invariant("factory preparation source absent"))?.close_state_preparation_work_demand(&self.state,body)}
 fn preparation_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.preparation_is_complete(){return Ok(Default::default());}
  let source=self.alias.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"unprepared factory ticket lost its original source"))?;
  let mut d=source.close_state_preparation_demands(&self.state,body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory original preparation depth overflow"))?;Ok(d)
 }
 fn original_slot(&mut self)->Option<&mut dyn std::any::Any>{Some(&mut *self.alias)}
 fn prepare_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll{let grant=effect.grant();
  if self.preparation_is_complete(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
  let demand=match self.preparation_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};if !permits(demand,grant){return RetainedWorkPoll::Pending{work:0}}
  let Some(source)=self.alias.as_ref()else{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory preparation lost original source")),work:0})};let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let work=match source.close_state_preparation_work_demand(&self.state,grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};if work>effect.maximum_work_units(){return RetainedWorkPoll::Pending{work:0}}let turn=source.prepare_close_state_step(&mut self.state,&mut effect.reborrow(child,effect.maximum_work_units()).expect("factory preparation exceeds original effect"));turn.and_then(|step|{let complete=self.preparation_is_complete();admit_retained_clone_close(grant,if complete{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())},complete,"original factory preparation")})
 }
 fn preparation_is_complete(&self)->bool{T::close_state_preparation_is_complete(&self.state)}
}

impl<T:FactoryPayloadRetirement> ErasedSnapshotRetirement for FactoryTicket<T> {
 fn requires_original_birth(&self)->bool{self.preparation_is_complete()&&self.alias.is_none()&&T::close_state_requires_original_birth(&self.state)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if !self.requires_original_birth(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("factory ticket has no selected original birth").into()),work:0})}let Some(depth)=grant.maximum_depth.checked_sub(1)else{return RetainedWorkPoll::Pending{work:0}};T::close_state_original_birth_step(&mut self.state,original,RetainedCloneGrant{maximum_depth:depth,..grant})}

    fn requires_original_control(&self)->bool{self.preparation_is_complete()&&self.alias.is_none()&&T::close_state_requires_original_control(&self.state)}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if !self.preparation_is_complete(){return self.preparation_work_demand(body)}if self.alias.is_some(){return Ok(1)}T::close_state_work_demand(&self.state,body)}
    fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->OriginalRetirementPoll{let grant=effect.grant();if !self.requires_original_control(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory ticket has no selected original child").into()),work:0})}let d=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if grant.maximum_depth<d.depth{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth").into()),work:0})}if !permits(d,grant){return RetainedWorkPoll::Pending{work:0}}let turn=T::close_state_step_controlled(&mut self.state,&mut effect.reborrow(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant},effect.maximum_work_units()).expect("factory child exceeds original effect"),control);turn.and_then(|step|{let complete=T::close_state_terminal_is_empty(&self.state);admit_retained_clone_close(grant,step,complete,"factory payload").map_err(Into::into)})}
    fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();if !self.requires_original_control(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory ticket has no selected original child").into()),work:0})}let d=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if grant.maximum_depth<d.depth{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth").into()),work:0})}if !permits(d,grant){return RetainedWorkPoll::Pending{work:0}}let turn=T::close_state_original_step(&mut self.state,original,&mut effect.reborrow(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant},effect.maximum_work_units()).expect("factory child exceeds original effect"));turn.and_then(|step|{let complete=T::close_state_terminal_is_empty(&self.state);admit_retained_clone_close(grant,step,complete,"factory payload").map_err(Into::into)})}

    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.preparation_is_complete(){return self.preparation_demands(body);}
        if self.alias.is_some(){return self.alias_demand();}
        let mut d=T::close_state_demands(&self.state,body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement nested depth overflow"))?;Ok(d)
    }
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();
        let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(empty)),work:0})}if grant.maximum_items==0{return RetainedWorkPoll::Pending{work:0}}let work=match self.next_computational_work(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if work>effect.maximum_work_units(){return RetainedWorkPoll::Pending{work:0}}
        if !self.preparation_is_complete(){let turn=self.prepare_step(effect);return turn.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)}
        let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if grant.maximum_depth<demand.depth{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth").into()),work:0})}if !permits(demand,grant){return RetainedWorkPoll::Pending{work:0}}
        if let Some(alias)=self.alias.as_ref(){if Arc::weak_count(alias)!=0{return RetainedWorkPoll::Pending{work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"factory alias transfer exceeded original work").into()),work:1}),Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let value=Arc::into_inner(match self.alias.take(){Some(v)=>v,None=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("alias absent").into()),work:0})});let released_bytes=if let Some(value)=value{T::transfer_payload(value,&mut self.state);arc_extent::<T>()}else{0};return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..empty})),work:1})}
        let turn=T::close_state_step(&mut self.state,&mut effect.reborrow(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant},effect.maximum_work_units()).expect("factory child exceeds original effect"));turn.and_then(|step|{let complete=T::close_state_terminal_is_empty(&self.state);admit_retained_clone_close(grant,step,complete,"factory payload").map_err(Into::into)})
    }
    fn terminal_is_empty(&self)->bool {self.alias.is_none()&&T::close_state_terminal_is_empty(&self.state)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(0)?.copy_bytes);}if self.alias.is_some(){Ok(self.alias_demand()?.copy_bytes)}else{Ok(T::close_state_demands(&self.state,0)?.copy_bytes)}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(copy)?.capacity_bytes);}if self.alias.is_some(){Ok(0)}else{Ok(T::close_state_demands(&self.state,copy)?.capacity_bytes)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(0)?.release_bytes);}if self.alias.is_some(){Ok(arc_extent::<T>())}else{Ok(T::close_state_demands(&self.state,0)?.release_bytes)}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.preparation_demands(0)?.depth);}if self.alias.is_some(){Ok(1)}else{T::close_state_demands(&self.state,0)?.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement nested depth overflow"))}}
}
impl<T:FactoryPayloadRetirement> Drop for FactoryTicket<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"factory ticket abandoned its original alias and payload");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.alias);ManuallyDrop::drop(&mut self.state);}}}}

/// 📏️ Borrows each exact currency including the final admitted ticket-frame release.


/// 🧹️ Preserves the caller's independent grants through payload and final ticket-frame release.



/// 🔐️ Owns the original factory capability until its admitted typed payload ticket closes.
pub struct FactoryAuthority{owner:FactoryChildSlot}
impl FactoryAuthority{
 pub fn new(source:Arc<dyn FactoryRetirement>)->Self{Self{owner:FactoryChildSlot::new(source)}}
 pub fn original(&self)->Option<&Arc<dyn FactoryRetirement>>{self.owner.original()}
 pub fn has_ticket(&self)->bool{self.owner.has_ticket()}
 pub fn factory_address(&self)->usize{self.owner.original().map_or(0,|source|Arc::as_ptr(source)as*const()as usize)}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owner.demands(body)}
 pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.owner.close_step(effect)}
 pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.owner.next_computational_work(body)}
 pub fn requires_original_birth(&self)->bool{self.owner.requires_original_birth()}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.owner.close_original_birth_step(original,grant)}
 pub fn requires_original_control(&self)->bool{self.owner.requires_original_control()}
 pub fn step_controlled(&mut self,effect:&mut RetainedAllocationEffect<'_>,control:&mut dyn OriginalRetirementControl)->OriginalRetirementPoll{self.owner.close_step_controlled(effect,control)}
 pub fn step_original(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.owner.close_original_step(original,effect)}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
}
impl RetireOwned for FactoryAuthority{
 type RetirementCursor=Self;
 fn retirement_seed(self)->Self{self}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
 fn controlled_retirement_supported()->bool{true}
}
fn authority_cursor_turn(poll:OriginalRetirementPoll)->retirement::RetirementStep{retirement::erased_cursor_turn(poll)}
impl retirement::RetirementCursor for FactoryAuthority{
 fn requires_original_birth(&self)->bool{Self::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->retirement::RetirementStep{authority_cursor_turn(Self::close_original_birth_step(self,original,grant))}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{Self::next_computational_work(self,body)}
 fn requires_original_control(&self)->bool{Self::requires_original_control(self)}
 fn close_step_controlled(&mut self,effect:&mut RetainedAllocationEffect<'_>,control:&mut dyn OriginalRetirementControl)->retirement::RetirementStep{authority_cursor_turn(self.step_controlled(effect,control))}
 fn close_original_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>)->retirement::RetirementStep{authority_cursor_turn(self.step_original(original,effect))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->retirement::RetirementStep{authority_cursor_turn(self.step(effect))}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

pub trait SnapshotRetirementFactory<P>:FactoryRetirement+Send+Sync{
 fn retirement_birth_bytes(&self,snapshot:&Arc<P>)->usize;
 fn retire_demands(&self,snapshot:&Arc<P>)->Result<RetirementDemand,ValueError>;
 fn retire_work_demand(&self,snapshot:&Arc<P>)->Result<u64,ValueError>;
 fn retire<'reply>(&self,snapshot:&mut Option<Arc<P>>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>;
 fn recover_retire<'reply>(&self,snapshot:&mut Option<Arc<P>>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>;
}

pub trait ArtifactOwnedValueRetirementFactory<T>:FactoryRetirement+Send+Sync+std::any::Any{
 fn retirement_birth_bytes(&self,value:&T)->usize;
 fn retire_owned_work_demand(&self,value:&T)->Result<u64,ValueError>;
 fn retire_owned_demands(&self,value:&T)->Result<RetirementDemand,ValueError>;
 fn retire_owned<'reply>(&self,value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>;
 fn recover_retire_owned<'reply>(&self,value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>;
}

#[derive(semio_framework_value_derive::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub struct OwnedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for OwnedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned> ArtifactOwnedValueRetirementFactory<T> for OwnedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&T)->usize {retirement::owned_retirement_birth_bytes::<T>()}
    fn retire_owned_work_demand(&self,_:&T)->Result<u64,ValueError>{Ok(u64::from(T::controlled_retirement_supported()))}
    fn retire_owned_demands(&self,value:&T)->Result<RetirementDemand,ValueError>{retirement::retirement_cursor_birth_demand(value)}
    fn retire_owned<'reply>(&self,value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,mut reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{let boundary=effect.performed_boundary();let turn=retirement::admit_owned_retirement(value,effect);let result=match turn.result{Ok(Some((owner,progress)))=>{reply.publish(owner);Ok(progress)},Ok(None)=>Ok(Default::default()),Err(error)=>Err(error.into())};reply.complete_observed(effect,boundary,result)}
    fn recover_retire_owned<'reply>(&self,_value:&mut Option<T>,_effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{reply.pending()}
}

/// 🔗️ Retires one store-owned snapshot alias as a lease: the last alias to close retires the payload, every other alias only drops its count, so sequentially closed owners of one Arc never wait on each other.
#[derive(semio_framework_value_derive::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub struct SharedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for SharedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned+Sync> SnapshotRetirementFactory<T> for SharedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&Arc<T>)->usize {retirement::arc_custody::shared_retirement_birth_bytes::<T>()}
    fn retire_demands(&self,_:&Arc<T>)->Result<RetirementDemand,ValueError>{Ok(retirement::arc_custody::shared_retirement_birth_demand::<T>())}
    fn retire_work_demand(&self,_:&Arc<T>)->Result<u64,ValueError>{Ok(1)}
    fn retire<'reply>(&self,value:&mut Option<Arc<T>>,effect:&mut RetainedAllocationEffect<'_>,mut reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{let boundary=effect.performed_boundary();let turn=retirement::arc_custody::admit_shared_retirement(value,effect,true);let result=match turn.result{Ok(Some((owner,progress)))=>{reply.publish(owner);Ok(progress)},Ok(None)=>Ok(Default::default()),Err(error)=>Err(error.into())};reply.complete_observed(effect,boundary,result)}
    fn recover_retire<'reply>(&self,_value:&mut Option<Arc<T>>,_effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{reply.pending()}
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
