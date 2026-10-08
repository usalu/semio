//! 🏭️ Admitted factory tickets retain original aliases and publish each ownership currency.
use crate::{ErasedSnapshotRetirement, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::{alloc::Layout, sync::Arc, mem::ManuallyDrop};

#[path="📦️owned/🦀️.rs"]
pub mod owned;

/// 🧬️ Concrete factory ownership supplies an inline state before any alias retires.
pub trait FactoryPayloadRetirement: Send + Sync + 'static {
    type CloseState: Send + 'static;
    fn close_state_birth_bytes(&self) -> usize;
    fn close_state_constructor_depth(&self) -> usize;
    fn prepare_close_state(&self) -> Self::CloseState;
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
        fn prepare_close_state(&self)->Self::CloseState {()}
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
    fn factory_retirement_depth_demand(&self) -> usize;
    fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<dyn FactoryRetirement>)>;
}

/// 🪆️ Child tickets occupy a fixed inline directory prepared within their parent's admitted birth.
pub struct FactoryChildTickets<const N:usize>(pub [Option<Box<dyn ErasedSnapshotRetirement>>;N]);
impl<const N:usize> FactoryChildTickets<N> {
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {self.0.iter().flatten().next().map_or(Ok(RetirementDemand::default()),|ticket|factory_ticket_demands(ticket,copy))}
    pub fn terminal_is_empty(&self)->bool {self.0.iter().all(Option::is_none)}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        match self.0.iter_mut().find(|slot|slot.is_some()) {
            Some(slot)=>match close_factory_ticket(slot,grant)? {RetainedCloneStep::Complete(progress)=>Ok(RetainedCloneStep::Progress(progress)),step=>Ok(step)},
            None=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
        }
    }
}

struct FactoryTicket<T:FactoryPayloadRetirement> {alias:ManuallyDrop<Option<Arc<T>>>,state:ManuallyDrop<T::CloseState>}
fn arc_extent<T>()->usize {Layout::new::<[usize;2]>().extend(Layout::new::<T>()).expect("factory Arc concrete layout").0.pad_to_align().size()}
pub fn factory_arc_birth_bytes<T:FactoryPayloadRetirement>()->usize {arc_extent::<T>()}
pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {std::mem::size_of::<FactoryTicket<T>>()}
pub fn factory_constructor_birth_bytes<T:FactoryPayloadRetirement>(child_constructor_bytes:usize)->usize {factory_arc_birth_bytes::<T>().checked_add(factory_retirement_frame_bytes::<T>()).and_then(|bytes|bytes.checked_add(child_constructor_bytes)).expect("factory constructor layout")}

impl<T:FactoryPayloadRetirement> FactoryRetirement for T {
    fn factory_retirement_birth_bytes(&self)->usize {factory_retirement_frame_bytes::<T>().checked_add(self.close_state_birth_bytes()).expect("factory ticket birth layout")}
    fn factory_retirement_depth_demand(&self)->usize {self.close_state_constructor_depth().checked_add(1).expect("factory constructor depth")}
    fn preborn_factory_retirement(self:Arc<Self>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<dyn FactoryRetirement>)> {
        if grant.maximum_items==0 {return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"factory ticket requires one admitted item"),self));}
        if grant.maximum_depth<self.factory_retirement_depth_demand() {return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"factory ticket constructor exceeds admitted depth"),self));}
        let bytes=self.factory_retirement_birth_bytes();
        if grant.maximum_capacity_bytes<bytes {return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"factory ticket constructor exceeds admitted capacity"),self));}
        let layout=Layout::new::<FactoryTicket<T>>();
        let Some(pointer)=std::ptr::NonNull::new(unsafe {std::alloc::alloc(layout)}.cast::<FactoryTicket<T>>()) else {return Err((ValueError::literal(ValueRefusalKind::AllocationFailed,"factory ticket frame allocation failed"),self));};
        let state=self.prepare_close_state();
        let owner=unsafe {pointer.as_ptr().write(FactoryTicket {alias:ManuallyDrop::new(Some(self)),state:ManuallyDrop::new(state)});Box::from_raw(pointer.as_ptr())};
        Ok((owner,RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
    }
}

impl<T:FactoryPayloadRetirement> ErasedSnapshotRetirement for FactoryTicket<T> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement exceeds admitted depth"));}
        if let Some(alias)=self.alias.as_ref() {
            if Arc::weak_count(alias)!=0||grant.maximum_release_bytes<arc_extent::<T>() {return Ok(RetainedCloneStep::Progress(empty));}
            let value=Arc::into_inner(self.alias.take().unwrap());
            let released_bytes=if let Some(value)=value {T::transfer_payload(value,&mut self.state);arc_extent::<T>()}else{0};
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes,..empty}));
        }
        let step=T::close_state_step(&mut self.state,RetainedCloneGrant {maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant})?;
        if !step.progress().fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory payload exceeded its admitted full grant"));}
        if matches!(step,RetainedCloneStep::Complete(_))&&!T::close_state_terminal_is_empty(&self.state){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory payload returned a false terminal witness"));}
        Ok(step)
    }
    fn terminal_is_empty(&self)->bool {self.alias.is_none()&&T::close_state_terminal_is_empty(&self.state)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if self.alias.is_some(){Ok(0)}else{Ok(T::close_state_demands(&self.state,0)?.copy_bytes)}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if self.alias.is_some(){Ok(0)}else{Ok(T::close_state_demands(&self.state,copy)?.capacity_bytes)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if self.alias.is_some(){Ok(arc_extent::<T>())}else{Ok(T::close_state_demands(&self.state,0)?.release_bytes)}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {if self.alias.is_some(){Ok(1)}else{T::close_state_demands(&self.state,0)?.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"factory retirement nested depth overflow"))}}
}
impl<T:FactoryPayloadRetirement> Drop for FactoryTicket<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"factory ticket abandoned its original alias and payload");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.alias);ManuallyDrop::drop(&mut self.state);}}}}

/// 📏️ Borrows each exact currency including the final admitted ticket-frame release.
pub fn factory_ticket_demands(ticket:&Box<dyn ErasedSnapshotRetirement>,copy:usize)->Result<RetirementDemand,ValueError> {
    if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:std::mem::size_of_val(ticket.as_ref()),depth:1,..Default::default()});}
    Ok(RetirementDemand {copy_bytes:ticket.next_copy_byte_demand()?,capacity_bytes:ticket.next_capacity_byte_demand(copy)?,release_bytes:ticket.next_release_byte_demand()?,depth:ticket.next_depth_demand()?})
}

/// 🧹️ Preserves the caller's independent grants through payload and final ticket-frame release.
pub fn close_factory_ticket(slot:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
    let empty=RetainedCloneProgress::default();
    let Some(ticket)=slot.as_mut()else{return Ok(RetainedCloneStep::Complete(empty));};
    if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
    if ticket.terminal_is_empty() {
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"terminal ticket requires admitted depth"));}
        let bytes=std::mem::size_of_val(ticket.as_ref());
        if grant.maximum_release_bytes<bytes{return Ok(RetainedCloneStep::Progress(empty));}
        drop(slot.take());
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
    }
    let step=ticket.close_step(grant)?;
    let step=crate::retained_clone::admit_retained_clone_close(grant,step,ticket.terminal_is_empty(),"factory ticket payload")?;
    Ok(RetainedCloneStep::Progress(step.progress()))
}


/// 🔐️ Owns the original factory capability until its admitted typed payload ticket closes.
pub struct FactoryAuthority {
    source:ManuallyDrop<Option<Arc<dyn FactoryRetirement>>>,
    ticket:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}
impl FactoryAuthority {
    pub fn new(source:Arc<dyn FactoryRetirement>)->Self {Self{source:ManuallyDrop::new(Some(source)),ticket:ManuallyDrop::new(None)}}
    pub fn factory_address(&self)->usize {self.source.as_ref().map_or(0,|source|Arc::as_ptr(source)as*const()as usize)}
    pub fn demands(&self,work:usize)->Result<RetirementDemand,ValueError>{
        if let Some(source)=self.source.as_ref(){return Ok(if self.ticket.is_some(){RetirementDemand{depth:1,..Default::default()}}else{RetirementDemand{capacity_bytes:source.factory_retirement_birth_bytes(),depth:source.factory_retirement_depth_demand(),..Default::default()}});}
        self.ticket.as_ref().map_or(Ok(Default::default()),|ticket|factory_ticket_demands(ticket,work))
    }
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"factory authority exceeds admitted depth"));}
        if demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(source)=self.source.as_ref(){
            if self.ticket.is_none(){
                let(ticket,receipt)=match Arc::clone(source).preborn_factory_retirement(grant){Ok(value)=>value,Err((error,alias))=>{drop(alias);return Err(error);}};
                if !receipt.fits(grant)||receipt.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"factory authority changed its admitted constructor receipt"));}
                *self.ticket=Some(ticket);return Ok(RetainedCloneStep::Progress(receipt));
            }
            drop(self.source.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        let step=close_factory_ticket(&mut self.ticket,grant)?;
        Ok(if matches!(step,RetainedCloneStep::Complete(_))&&!self.terminal_is_empty(){RetainedCloneStep::Progress(step.progress())}else{step})
    }
    pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.ticket.is_none()}
}
impl crate::retirement::RetireOwned for FactoryAuthority{
    fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl crate::retirement::RetirementCursor for FactoryAuthority{
    fn close_step(&mut self,grant:RetainedCloneGrant)->crate::retirement::RetirementStep{match self.step(grant){Err(error)=>crate::retirement::RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(_))=>crate::retirement::RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress))=>crate::retirement::RetirementStep::Progress(progress)}}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {Ok(self.demands(0)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,work:usize)->Option<usize>{self.demands(work).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for FactoryAuthority{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"factory authority abandoned original typed payload");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.ticket);}}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
