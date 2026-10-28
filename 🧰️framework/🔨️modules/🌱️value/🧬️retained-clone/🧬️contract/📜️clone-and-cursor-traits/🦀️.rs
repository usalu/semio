//! 📜️ Clone and cursor traits of the retained clone contract: the retained clone trait with its bitwise hooks, the bulk page bytes, the bulk run helpers, the clone cursor trait, the close record and the binding close helper. See [`super`].

use super::*;

pub trait RetainedClone: RetireOwned + Send + Sync + Sized + 'static {
    type Cursor: RetainedCloneCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor;
    /// 🧬️ True for plain scalar values whose arrays clone in one funded bitwise copy turn.
    const BITWISE: bool = false;
    /// 📋️ Copies a whole array bitwise when [`Self::BITWISE`] is set; every other element clones one by one.
    fn bitwise_array<const N: usize>(_source: &[Self; N]) -> Option<[Self; N]> { None }
    /// 🚚️ Appends a contiguous run bitwise into already reserved vector capacity when [`Self::BITWISE`] is set; returns false for every element that must clone one by one.
    fn bitwise_extend(_target: &mut Vec<Self>, _source: &[Self]) -> bool { false }
}

/// 📃️ The largest payload one bulk vector copy turn moves, equal to the ABI page so a bitwise run is never longer than one page of work.
pub const RETAINED_CLONE_BULK_PAGE_BYTES: usize = 64 * 1024;

/// 📏️ The number of `width`-byte elements one bulk turn may process under `grant`: one item and at most one page of payload, zero when the grant admits neither one item nor one element.
pub fn bulk_run_elements(grant: RetainedCloneGrant, width: usize) -> usize {
    if grant.maximum_items == 0 { return 0; }
    if width == 0 { return RETAINED_CLONE_BULK_PAGE_ITEMS; }
    grant.maximum_copy_bytes.min(RETAINED_CLONE_BULK_PAGE_BYTES) / width
}

/// 🧾️ The honest receipt of one bulk turn that processed `elements` elements of `width` bytes: one item and exactly those payload bytes.
pub fn bulk_run_progress(elements: usize, width: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: elements.saturating_mul(width), ..Default::default() }
}

/// 🧷️ Transfers one native owner once; after `take`, callers must close the spent cursor before dropping it.
/// 🎟️ Borrows the actual selected child closure quote without advancing its typed owner.
pub fn clone_cursor_close_demand<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if cursor.terminal_is_empty(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}cursor.close_demands(body)}

pub trait RetainedCloneCursor<T: RetainedClone>: Send {
    fn output_ready(&self)->bool{false}
        /// 🚦️ Explicit cursor owners override their close lifecycle; absent closure intent is unstarted.
    fn close_started(&self)->bool{false}
    /// 🎟️ Quotes one close action from the actual paid currencies.
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>;
/// ⏩️ Quotes the actual next immutable-source turn before any work or allocation.
    fn advance_demands(&self, _source: RetainedCloneRef<'_, T>, _maximum_body_bytes: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor lacks an original-source advance quote"))
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError>;
    fn take(&mut self) -> Option<T>;
    fn begin_close(&mut self) -> bool;
    
    fn terminal_is_empty(&self) -> bool;
    /// 🪜️ Observes the depth of the next controlled close transition without changing ownership.
    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError>;
    /// 🧮️ Observes the next payload-work grant without allocating or releasing retained owners.
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close work demand"))
    }
    /// 📐️ Observes the next granted close birth; unsupported cursors retain their owners.
    fn next_close_capacity_byte_demand(&self, _maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close birth demand"))
    }
    /// 📪️ Observes the next body or scaffold release independently of payload copying.
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close release demand"))
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no capacity-bearing close authority"))
    }
}

#[derive(Default)]
pub struct RetainedCloneClose {pub(super) retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}

/// 🎯️ The retained clone owner uses the same exclusive original ticket selection as typed cursors.
pub use crate::RetirementSelection as RetainedCloseSelection;

impl RetainedCloneClose {
    /// 🪑️ Seats one already admitted original retirement so custody laws can quote and fund its close without a typed owner.
    #[doc(hidden)] pub fn from_admitted_original(retirement:Box<dyn ErasedSnapshotRetirement>)->Self{Self{retirement:ManuallyDrop::new(Some(retirement))}}
    /// 📏️ Selects all currencies from the same actual body request forwarded to the retained child.
    pub fn demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.retirement.as_ref().map_or(Ok(Default::default()),|owner|crate::factory_ticket_demands(owner,body))}
    /// 🧷️ Loans the actual owner exclusively so the selected body cannot change before its grant is checked.
    pub fn select(&mut self,body:usize)->Result<RetainedCloseSelection<'_>,crate::ValueError>{crate::select_factory_ticket(&mut self.retirement,body)}
    pub fn next_owner_demands<T:RetireOwned>(&self,present:bool,binding:&Option<RetainedCloneBinding>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(owner)=self.retirement.as_ref(){return crate::factory_ticket_demands(owner,body);}if present{if !T::controlled_retirement_supported(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"typed clone owner lacks controlled close authority"));}return Ok(crate::RetirementDemand{capacity_bytes:crate::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()});}RetainedCloneBinding::close_demand(binding,body)}
    pub fn next_owner_depth_with_binding(&self,present:bool,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if !self.is_empty(){self.next_depth_demand()}else if present{Ok(1)}else{RetainedCloneBinding::depth_demand(binding)}}
    pub fn next_copy_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty(){RetainedCloneBinding::copy_demand(binding)}else{self.next_copy_byte_demand()}}
    pub fn next_release_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty(){RetainedCloneBinding::release_demand(binding)}else{self.next_release_byte_demand()}}
    pub fn next_owner_capacity_with_binding<T:RetireOwned>(&self,present:bool,body:usize,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty()&&!present{RetainedCloneBinding::capacity_demand(binding,body)}else{self.next_owner_capacity_byte_demand::<T>(present,body)}}
    pub fn next_copy_byte_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.copy_bytes))}
    pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,body)?.capacity_bytes))}
    pub fn next_release_byte_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.release_bytes))}
    pub fn next_depth_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.depth))}
    pub fn next_owner_capacity_byte_demand<T:RetireOwned>(&self,present:bool,body:usize)->Result<usize,crate::ValueError> {
        if !self.is_empty(){return self.next_capacity_byte_demand(body);}
        if present&&!T::controlled_retirement_supported(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"typed owner has no controlled close birth authority"));}
        Ok(if present{crate::owned_retirement_birth_bytes::<T>()}else{0})
    }
    pub fn begin_granted<T:RetireOwned>(&mut self,value:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,crate::ValueError> {
        if !self.is_empty()||value.is_none(){return Ok(None);}
        let bytes=crate::owned_retirement_birth_bytes::<T>();
        if grant.maximum_items==0||grant.maximum_depth==0||bytes>grant.maximum_capacity_bytes{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        match crate::admit_owned_retirement(value.take().unwrap(),grant) {
            Ok((owner,progress))=>{*self.retirement=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},
            Err((error,owner))=>{*value=Some(owner);Err(error)},
        }
    }
    pub fn begin_default_granted<T:RetireOwned+Default>(&mut self,value:&mut T,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,crate::ValueError> {
        if !self.is_empty(){return Ok(None);}
        let bytes=crate::owned_retirement_birth_bytes::<T>();
        if grant.maximum_items==0||grant.maximum_depth==0||bytes>grant.maximum_capacity_bytes{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let mut owner=Some(std::mem::take(value));
        let step=self.begin_granted(&mut owner,grant);
        if let Some(owner)=owner{*value=owner;}
        step
    }
    pub fn step_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError> {
        if let Some(owner)=self.retirement.as_ref(){let demand=crate::factory_ticket_demands(owner,grant.maximum_copy_bytes)?;if !crate::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}}
        let step=crate::close_factory_ticket(&mut self.retirement,grant)?;
        if !step.progress().fits(grant){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"clone retirement exceeded admitted full grant"));}
        Ok(step)
    }
    pub fn is_empty(&self)->bool {self.retirement.is_none()}
}

impl Drop for RetainedCloneClose {fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"clone retirement abandoned original ownership");if self.is_empty(){unsafe{ManuallyDrop::drop(&mut self.retirement);}}}}

pub fn close_retained_binding(binding: &mut Option<RetainedCloneBinding>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
    RetainedCloneBinding::close_one(binding, grant)
}
