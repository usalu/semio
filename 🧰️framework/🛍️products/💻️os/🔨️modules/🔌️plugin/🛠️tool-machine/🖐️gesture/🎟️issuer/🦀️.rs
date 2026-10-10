//! 🎟️ Original gesture issuance keeps the runtime and captured alias behind a strong-only authority.
use super::{GestureSlot,GestureSlotDecision};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::RetainedCloneGrant,retirement::{RetireOwned,shared::sealed::{SealedShared,SealedSharedBirth,SharedIssuer}}};

#[derive(semio_framework_value::RetireOwned)]
pub enum GestureCapture<M:Send+'static>{Detached(GestureSlot<M>),Issued(SealedShared<GestureSlot<M>>)}
impl<M:Send+'static> GestureCapture<M>{
 pub fn detached()->Self{Self::Detached(GestureSlot::detached())}
 pub fn get(&self)->&GestureSlot<M>{match self{Self::Detached(slot)=>slot,Self::Issued(slot)=>slot.get()}}
}
impl<M:Send+'static> std::ops::Deref for GestureCapture<M>{type Target=GestureSlot<M>;fn deref(&self)->&Self::Target{self.get()}}

#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct GestureOperation<M:Send+'static>{pub operation:u64,pub slot:SealedShared<GestureSlot<M>>,pub decision:Option<GestureSlotDecision<M>>,pub settled:bool}
impl<M:Send+'static> GestureOperation<M>{pub fn request_cancellation(&mut self){self.settled=true;}}

pub(crate) fn reserve_live<M:Send+'static>(available:bool,birth:RetainedCloneGrant,handoff:RetainedCloneGrant)->Result<SealedSharedBirth<GestureSlot<M>>,ValueError>{
 if !available{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"all original gesture operation slots remain occupied"));}
 if handoff.maximum_items==0||handoff.maximum_copy_bytes<size_of::<SealedShared<GestureSlot<M>>>()||handoff.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original gesture capture alias is not funded"));}
 SealedShared::prepare_birth(birth)
}
pub(crate) fn issue_live<M:RetireOwned+Send+'static>(birth:SealedSharedBirth<GestureSlot<M>>,slot:GestureSlot<M>,operation:u64,handoff:RetainedCloneGrant)->(GestureOperation<M>,GestureCapture<M>){
 let(slot,_)=birth.materialize(slot,SharedIssuer::owned());
 let(alias,_)=slot.try_duplicate(handoff).unwrap_or_else(|_|panic!("prevalidated original gesture handoff remains funded"));
 (GestureOperation{operation,slot,decision:None,settled:false},GestureCapture::Issued(alias))
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
