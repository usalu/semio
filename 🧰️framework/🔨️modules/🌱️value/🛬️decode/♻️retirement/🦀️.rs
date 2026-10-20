//! ♻️ Original retirement loans retain decoder custody after new semantic work is cancelled.
use super::{NativeDecodeControl,NativeDecodeProgress,allocation};
use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneStep,retained_clone_grant_admits}};
/// 🫴️ A borrowed original ledger permits only the paid capacity of one already-owned retirement hop.
pub struct NativeRetirementDecodeControl<'operation,'decode>{original:&'operation mut NativeDecodeControl<'decode>,capacity:usize,spent:usize,previous_maximum:usize}
fn invalid(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::literal(kind,message)}
impl<'operation,'decode> NativeRetirementDecodeControl<'operation,'decode>{
 fn begin(original:&'operation mut NativeDecodeControl<'decode>,grant:RetainedCloneGrant)->Result<Self,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Err(invalid(ValueRefusalKind::WorkLimit,"original retirement requires its paid caller turn"));}if original.retirement.is_none(){return Err(invalid(ValueRefusalKind::UnsupportedOwner,"original retirement requires its exact original recipient"));}let previous_maximum=original.maximum_bytes;original.admit_turn_capacity(grant.maximum_capacity_bytes)?;Ok(Self{original,capacity:grant.maximum_capacity_bytes,spent:0,previous_maximum})}
 /// 🪪️ Observes the unchanged original decoder issuance.
 pub fn ledger_identity(&self)->u64{self.original.ledger_identity()}
 /// 🧷️ Observes the exact original recipient retained by this decoder continuation.
 pub fn retirement_recipient_identity(&self)->Option<u64>{self.original.retirement_recipient_identity()}
 /// 📊️ Reads cumulative original allocation without issuing another capacity.
 pub fn owned_bytes(&self)->usize{self.original.owned_bytes}
 /// 🎟️ Admits one real cleanup birth against this original hop before reporting the original cancellation observer.
 pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{let spent=self.spent.checked_add(bytes).filter(|spent|*spent<=self.capacity).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original retirement exceeds its paid hop capacity"))?;let next=self.original.owned_bytes.checked_add(bytes).filter(|next|*next<=self.original.maximum_bytes).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original retirement exceeds its cumulative ownership ceiling"))?;if let allocation::Binding::Forwarded(port)=&mut self.original.allocation{port(super::NativeDecodeAllocation{bytes,owned_bytes:self.original.owned_bytes,next_owned_bytes:next,maximum_bytes:self.original.maximum_bytes})?;}(self.original.callback)(NativeDecodeProgress{completed:self.original.completed,total:self.original.total,owned_bytes:self.original.owned_bytes});self.spent=spent;self.original.owned_bytes=next;Ok(())}
 /// 👓️ Reports already-born ownership in this exact original recipient.
 pub fn has_retirement_owner(&self)->bool{self.original.has_retirement_owner()}
 /// 📏️ Quotes this original recipient once under the paid cleanup body span.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.original.retirement.as_ref().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement has no recipient"))?.next_demand(body)}
 /// 🚮️ Charges and closes the original returned owner after every independent caller currency admits it.
 pub fn close_retirement_recipient(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.original.retirement.as_ref().unwrap().terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}self.charge(demand.capacity_bytes)?;self.original.retirement.as_mut().unwrap().close_step(grant)}
}
impl Drop for NativeRetirementDecodeControl<'_, '_>{fn drop(&mut self){if !self.original.receiving{self.original.maximum_bytes=self.previous_maximum;}}}
impl<'decode> NativeDecodeControl<'decode>{
 /// 🛟️ Loans only retirement capacity while keeping this operation's original ledger, observer and recipient.
 pub fn with_retirement<T>(&mut self,grant:RetainedCloneGrant,operation:impl FnOnce(&mut NativeRetirementDecodeControl<'_, 'decode>)->Result<T,ValueError>)->Result<T,ValueError>{let mut loan=NativeRetirementDecodeControl::begin(self,grant)?;operation(&mut loan)}
}

#[path="🔒️close/🦀️.rs"]
mod close;
pub use close::NativeDecodeCloseControl;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
