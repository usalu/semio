//! 🏭️ Admitted factory tickets retain original aliases and publish each ownership currency.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,OriginalRetirementControl,OriginalRetirementFailure,OriginalRetirementOutcome,RetireOwned,factory_ticket_demands,close_factory_ticket};
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
    fn close_state_preparation_work_demand(&self,_state:&Self::CloseState,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory preparation lacks a native work quote"))}
    fn prepare_close_state_step(&self,state:&mut Self::CloseState,grant:RetainedCloneGrant)->RetainedWorkOutcome;
    fn close_state_preparation_is_complete(state:&Self::CloseState)->bool;
    fn transfer_payload(value: Self, state: &mut Self::CloseState) where Self: Sized;
    fn close_state_demands(state: &Self::CloseState, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError>;
    /// 🪪️ Reports whether the selected state child owns an original retirement loan.
    fn close_state_requires_original_control(_state:&Self::CloseState)->bool{false}
    /// ⛽️ Quotes the selected state action before any effect.
    fn close_state_work_demand(_state:&Self::CloseState,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory state lacks a native work quote"))}
    /// 🧷️ Forwards the original selected child control.
    fn close_state_step_controlled(_state:&mut Self::CloseState,_grant:RetainedCloneGrant,_control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->OriginalRetirementOutcome{RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory state lacks original control").into()),work:0}}
    /// 🧵️ Forwards the original selected child decoder loan.
    fn close_state_original_step(_state:&mut Self::CloseState,_original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,_grant:RetainedCloneGrant)->OriginalRetirementOutcome{RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory state lacks original loan").into()),work:0}}
    fn close_state_step(state: &mut Self::CloseState, grant: RetainedCloneGrant) -> OriginalRetirementOutcome;
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
        fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:RetainedCloneGrant)->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
        fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
        fn transfer_payload(value:Self,_:&mut Self::CloseState) {drop(value);}
        fn close_state_demands(_:&Self::CloseState,_:usize)->Result<RetirementDemand,ValueError> {Ok(RetirementDemand::default())}
        fn close_state_work_demand(_:&Self::CloseState,_:usize)->Result<u64,ValueError>{Ok(0)}
        fn close_state_step(_:&mut Self::CloseState,_:RetainedCloneGrant)->OriginalRetirementOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
        fn close_state_terminal_is_empty(_:&Self::CloseState)->bool {true}
    })*};
}
scalar_factory!(std::sync::atomic::AtomicUsize,std::sync::atomic::AtomicBool);

/// 📬️ Object-safe factory ownership admits its complete constructor before transferring any alias.
pub trait FactoryRetirement: Send + Sync {
    fn factory_retirement_work_demand(&self)->Result<u64,ValueError>;
    fn factory_retirement_birth_bytes(&self) -> usize;
    fn factory_retirement_copy_byte_demand(&self) -> usize;
    fn factory_retirement_depth_demand(&self) -> usize;
    fn preborn_factory_retirement(self:Arc<Self>,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Box<dyn FactoryRetirementTicket>,RetainedCloneProgress),FactoryRetirementAdmissionError>;
}

/// 🪆️ Child tickets occupy a fixed inline directory prepared within their parent's admitted birth.
pub struct FactoryChildTickets<const N:usize>{pub slots:[FactoryChildSlot;N],pub prepared:usize}
impl<const N:usize> FactoryChildTickets<N> {
    /// 🪪️ Reports whether the selected child owns an original ticket loan.
    pub fn requires_original_control(&self)->bool{self.slots.iter().find(|slot|!slot.terminal_is_empty()).is_some_and(FactoryChildSlot::requires_original_control)}
    /// ⛽️ Quotes the selected original child only.
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.slots.iter().find(|slot|!slot.terminal_is_empty()).map_or(Ok(0),|slot|slot.next_computational_work(body))}
    pub fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->OriginalRetirementOutcome{match self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()){Some(slot)=>{let turn=slot.close_step_controlled(grant,control);RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}},None=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}}
    pub fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementOutcome{match self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()){Some(slot)=>{let turn=slot.close_original_step(original,grant);RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}},None=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}}
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {self.slots.iter().find(|slot|!slot.terminal_is_empty()).map_or(Ok(Default::default()),|slot|slot.demands(copy))}
    pub fn terminal_is_empty(&self)->bool {self.slots.iter().all(FactoryChildSlot::terminal_is_empty)}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->OriginalRetirementOutcome{
        match self.slots.iter_mut().find(|slot|!slot.terminal_is_empty()){Some(slot)=>{let turn=slot.close_step(grant);RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}},None=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
    }
}

struct FactoryTicket<T:FactoryPayloadRetirement> {alias:ManuallyDrop<Option<Arc<T>>>,state:ManuallyDrop<T::CloseState>}
fn arc_extent<T>()->usize {Layout::new::<[usize;2]>().extend(Layout::new::<T>()).expect("factory Arc concrete layout").0.pad_to_align().size()}
pub fn factory_arc_birth_bytes<T:FactoryPayloadRetirement>()->usize {arc_extent::<T>()}
pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {size_of::<FactoryTicket<T>>()}
pub fn factory_constructor_birth_bytes<T:FactoryPayloadRetirement>(child_constructor_bytes:usize)->usize {factory_arc_birth_bytes::<T>().checked_add(factory_retirement_frame_bytes::<T>()).and_then(|bytes|bytes.checked_add(child_constructor_bytes)).expect("factory constructor layout")}

impl<T:FactoryPayloadRetirement> FactoryRetirement for T {
    fn factory_retirement_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
    fn factory_retirement_birth_bytes(&self)->usize {factory_retirement_frame_bytes::<T>().checked_add(self.close_state_birth_bytes()).expect("factory ticket birth layout")}
    fn factory_retirement_copy_byte_demand(&self)->usize {self.close_state_constructor_copy_bytes()}
    fn factory_retirement_depth_demand(&self)->usize {self.close_state_constructor_depth().checked_add(1).expect("factory constructor depth")}
    fn preborn_factory_retirement(self:Arc<Self>,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Box<dyn FactoryRetirementTicket>,RetainedCloneProgress),FactoryRetirementAdmissionError>{
        let copy=self.factory_retirement_copy_byte_demand();let bytes=self.factory_retirement_birth_bytes();let demand=RetirementDemand{copy_bytes:copy,capacity_bytes:bytes,depth:self.factory_retirement_depth_demand(),..Default::default()};
        if !permits(demand,grant){return RetainedWorkOutcome{result:Err(FactoryRetirementAdmissionError{error:ValueError::literal(ValueRefusalKind::OwnershipLimit,"factory original constructor exceeds admitted full grant"),original:Some(self),ticket:None,progress:Default::default()}),work:0}}
        let layout=Layout::new::<FactoryTicket<T>>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<FactoryTicket<T>>())else{return RetainedWorkOutcome{result:Err(FactoryRetirementAdmissionError{error:ValueError::literal(ValueRefusalKind::AllocationFailed,"factory ticket frame allocation failed"),original:Some(self),ticket:None,progress:Default::default()}),work:1}};
        let state=self.prepare_close_state();let owner=unsafe{pointer.as_ptr().write(FactoryTicket{alias:ManuallyDrop::new(Some(self)),state:ManuallyDrop::new(state)});Box::from_raw(pointer.as_ptr())};RetainedWorkOutcome{result:Ok((owner,RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:bytes,..Default::default()})),work:1}
    }
}

impl<T:FactoryPayloadRetirement> FactoryRetirementTicket for FactoryTicket<T>{
 fn preparation_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.preparation_is_complete(){return Ok(0)}self.alias.as_ref().ok_or_else(||ValueError::invariant("factory preparation source absent"))?.close_state_preparation_work_demand(&self.state,body)}
 fn preparation_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.preparation_is_complete(){return Ok(Default::default());}
  let source=self.alias.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"unprepared factory ticket lost its original source"))?;
  let mut d=source.close_state_preparation_demands(&self.state,body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory original preparation depth overflow"))?;Ok(d)
 }
 fn prepare_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.preparation_is_complete(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=match self.preparation_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !permits(demand,grant){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let Some(source)=self.alias.as_ref()else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory preparation lost original source")),work:0}};let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let turn=source.prepare_close_state_step(&mut self.state,child);let complete=self.preparation_is_complete();let result=turn.result.and_then(|step|admit_retained_clone_close(grant,if complete{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())},complete,"original factory preparation"));RetainedWorkOutcome{result,work:turn.work}
 }
 fn preparation_is_complete(&self)->bool{T::close_state_preparation_is_complete(&self.state)}
}

impl<T:FactoryPayloadRetirement> ErasedSnapshotRetirement for FactoryTicket<T> {
    fn requires_original_control(&self)->bool{self.preparation_is_complete()&&self.alias.is_none()&&T::close_state_requires_original_control(&self.state)}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if !self.preparation_is_complete(){return self.preparation_work_demand(body)}if let Some(alias)=self.alias.as_ref(){return Ok(u64::from(Arc::weak_count(alias)==0))}T::close_state_work_demand(&self.state,body)}
    fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->OriginalRetirementOutcome{if !self.requires_original_control(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory ticket has no selected original child").into()),work:0}}let d=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if grant.maximum_depth<d.depth{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth").into()),work:0}}if !permits(d,grant){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let turn=T::close_state_step_controlled(&mut self.state,RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant},control);let complete=T::close_state_terminal_is_empty(&self.state);RetainedWorkOutcome{result:turn.result.and_then(|step|admit_retained_clone_close(grant,step,complete,"factory payload").map_err(Into::into)),work:turn.work}}
    fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementOutcome{if !self.requires_original_control(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory ticket has no selected original child").into()),work:0}}let d=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if grant.maximum_depth<d.depth{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth").into()),work:0}}if !permits(d,grant){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let turn=T::close_state_original_step(&mut self.state,original,RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant});let complete=T::close_state_terminal_is_empty(&self.state);RetainedWorkOutcome{result:turn.result.and_then(|step|admit_retained_clone_close(grant,step,complete,"factory payload").map_err(Into::into)),work:turn.work}}

    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.preparation_is_complete(){return self.preparation_demands(body);}
        if self.alias.is_some(){return Ok(RetirementDemand{release_bytes:arc_extent::<T>(),depth:1,..Default::default()});}
        let mut d=T::close_state_demands(&self.state,body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement nested depth overflow"))?;Ok(d)
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->OriginalRetirementOutcome{
        let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(empty)),work:0}}if grant.maximum_items==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0}}
        if !self.preparation_is_complete(){let turn=self.prepare_step(grant);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into),work:turn.work}}
        let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if grant.maximum_depth<demand.depth{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth").into()),work:0}}if !permits(demand,grant){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0}}
        if let Some(alias)=self.alias.as_ref(){if Arc::weak_count(alias)!=0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(empty)),work:0}}let value=Arc::into_inner(match self.alias.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("alias absent").into()),work:0}});let released_bytes=if let Some(value)=value{T::transfer_payload(value,&mut self.state);arc_extent::<T>()}else{0};return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty})),work:1}}
        let turn=T::close_state_step(&mut self.state,RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant});let complete=T::close_state_terminal_is_empty(&self.state);let result=turn.result.and_then(|step|admit_retained_clone_close(grant,step,complete,"factory payload").map_err(Into::into));RetainedWorkOutcome{result,work:turn.work}
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
    pub fn step(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if grant.maximum_items==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
        let demand=match self.demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if grant.maximum_depth<demand.depth{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory authority exceeds admitted depth").into()),work:0}}if !permits(demand,grant){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
        if self.source.is_some(){let turn=match self.source.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("source absent").into()),work:0}}.preborn_factory_retirement(grant);let result=match turn.result{Ok((ticket,receipt))=>{*self.ticket=Some(ticket);if !receipt.fits(grant)||receipt.retained_capacity_bytes!=demand.capacity_bytes{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory authority changed its admitted constructor receipt").with_retained_progress(receipt))}else{Ok(RetainedCloneStep::Progress(receipt))}},Err(error)=>{*self.source=error.original;*self.ticket=error.ticket;Err(error.error.with_retained_progress(error.progress))}};return RetainedWorkOutcome{result:result.map_err(Into::into),work:turn.work}}
        let turn=close_factory_ticket(&mut self.ticket,grant);let result=turn.result.map(|step|if matches!(step,RetainedCloneStep::Complete(_))&&!self.terminal_is_empty(){RetainedCloneStep::Progress(step.progress())}else{step});RetainedWorkOutcome{result,work:turn.work}
    }
    /// ⛽️ Quotes the selected original ticket action; unsupported constructors refuse before effects.
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(source)=self.source.as_ref(){return source.factory_retirement_work_demand()}match self.ticket.as_ref(){Some(ticket)if ticket.terminal_is_empty()=>Ok(1),Some(ticket)=>ticket.next_computational_work(body),None=>Ok(0)}}
    pub fn requires_original_control(&self)->bool{self.source.is_none()&&self.ticket.is_some()}
    pub fn step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->semio_framework_value_retirement::OriginalRetirementOutcome{if !self.requires_original_control(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory authority has no selected original ticket").into()),work:0}}semio_framework_value_retirement::close_factory_ticket_controlled(&mut self.ticket,grant,control)}
    pub fn step_original(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{if !self.requires_original_control(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"factory authority has no selected original ticket").into()),work:0}}semio_framework_value_retirement::close_factory_ticket_original(&mut self.ticket,original,grant)}
    pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.ticket.is_none()}
}
impl RetireOwned for FactoryAuthority{
    fn retirement(self)->Box<dyn retirement::RetirementCursor>{Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
fn authority_cursor_turn(turn:semio_framework_value_retirement::OriginalRetirementOutcome)->retirement::RetirementStep{match turn.result{Err(error)=>retirement::RetirementStep::Failure(error,turn.work),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>retirement::RetirementStep::Complete(turn.work),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>retirement::RetirementStep::Progress(progress,turn.work)}}
impl retirement::RetirementCursor for FactoryAuthority{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{FactoryAuthority::next_computational_work(self,body)}
 fn requires_original_control(&self)->bool{self.source.is_none()&&self.ticket.is_some()}
 fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->retirement::RetirementStep{authority_cursor_turn(self.step_controlled(grant,control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->retirement::RetirementStep{authority_cursor_turn(self.step_original(original,grant))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->retirement::RetirementStep{let turn=self.step(grant);match turn.result{Err(error)=>retirement::RetirementStep::Failure(error,turn.work),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>retirement::RetirementStep::Complete(turn.work),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>retirement::RetirementStep::Progress(progress,turn.work)}}
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

pub trait SnapshotRetirementFactory<P>: FactoryRetirement + Send + Sync {
    /// 📦️ Publishes the retained cursor birth allocation before snapshot ownership transfers.
    fn retirement_birth_bytes(&self, snapshot: &Arc<P>) -> usize;
    fn retire(&self,snapshot:Arc<P>,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<P>)>;
}

pub trait ArtifactOwnedValueRetirementFactory<T>: FactoryRetirement + Send + Sync + std::any::Any {
    fn retirement_birth_bytes(&self, value: &T) -> usize;
    /// ⛽️ Quotes the provider's work before the original value transfers.
    fn retire_owned_work_demand(&self, value: &T) -> Result<u64, ValueError>;
    fn retire_owned(&self,value:T,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(OriginalRetirementFailure,T)>;
}

pub fn admit_original_owned_retirement<T:'static>(original:&mut Option<T>,factory:&dyn ArtifactOwnedValueRetirementFactory<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,OriginalRetirementFailure>{
 let Some(value)=original.as_ref()else{return RetainedWorkOutcome{result:Ok(None),work:0}};
 let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"owned retirement requires one admitted item"))}else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"owned retirement requires admitted depth"))}else if factory.retirement_birth_bytes(value)>grant.maximum_capacity_bytes{Some((ValueRefusalKind::OwnershipLimit,"owned retirement exceeds admitted frame capacity"))}else{None};if let Some((kind,message))=refusal{return RetainedWorkOutcome{result:Err(ValueError::literal(kind,message).into()),work:0}}
 let turn=factory.retire_owned(original.take().expect("observed exact original owned value"),grant);let result=match turn.result{Ok(admitted)=>Ok(Some(admitted)),Err((error,value))=>{*original=Some(value);Err(error)}};RetainedWorkOutcome{result,work:turn.work}
}
#[derive(semio_framework_value_derive::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub struct OwnedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for OwnedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned> ArtifactOwnedValueRetirementFactory<T> for OwnedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&T)->usize {retirement::owned_retirement_birth_bytes::<T>()}
    fn retire_owned_work_demand(&self,_:&T)->Result<u64,ValueError>{Ok(u64::from(T::controlled_retirement_supported()))}
    fn retire_owned(&self,value:T,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(OriginalRetirementFailure,T)>{let turn=retirement::admit_owned_retirement(value,grant);RetainedWorkOutcome{result:turn.result.map_err(|(error,value)|(error.into(),value)),work:turn.work}}
}

/// 🔗️ Retires one store-owned snapshot alias as a lease: the last alias to close retires the payload, every other alias only drops its count, so sequentially closed owners of one Arc never wait on each other.
#[derive(semio_framework_value_derive::FactoryPayloadRetirement)]
#[value(crate="crate")]
pub struct SharedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for SharedValueRetirementFactory<T> { fn default()->Self {Self(PhantomData)} }
impl<T:RetireOwned+Sync> SnapshotRetirementFactory<T> for SharedValueRetirementFactory<T> {
    fn retirement_birth_bytes(&self,_:&Arc<T>)->usize {retirement::arc_custody::shared_retirement_birth_bytes::<T>()}
    fn retire(&self,value:Arc<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<T>)>{retirement::arc_custody::admit_shared_retirement(value,grant,true)}
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
