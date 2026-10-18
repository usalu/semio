//! 🎟️ Original gesture issuance keeps the runtime and captured alias behind a strong-only authority.
use semio_framework_plugin_decl_views::gesture_contract::GestureCapture;
use semio_framework_plugin_decl_views::gesture_contract::{GestureSlot, GestureSlotDecision};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::shared_issuance::{SealedShared, SealedSharedBirth, SharedIssuer};

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct GestureOperation<M:Send+'static>{pub operation:u64,pub slot:SealedShared<GestureSlot<M>>,pub decision:Option<GestureSlotDecision<M>>,pub settled:bool}
impl<M:Send+'static> GestureOperation<M>{pub fn request_cancellation(&mut self){self.settled=true;}}

#[doc(hidden)]
pub fn reserve_live<M:Send+'static>(available:bool,birth:RetainedCloneGrant,handoff:RetainedCloneGrant)->Result<SealedSharedBirth<GestureSlot<M>>,ValueError>{
 if !available{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"all original gesture operation slots remain occupied"));}
 if handoff.maximum_items==0||handoff.maximum_copy_bytes<size_of::<SealedShared<GestureSlot<M>>>()||handoff.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original gesture capture alias is not funded"));}
 SealedShared::prepare_birth(birth)
}
#[doc(hidden)]
pub fn issue_live<M:RetireOwned+Send+'static>(birth:SealedSharedBirth<GestureSlot<M>>,slot:GestureSlot<M>,operation:u64,handoff:RetainedCloneGrant)->(GestureOperation<M>,GestureCapture<M>){
 let(slot,_)=birth.materialize(slot,SharedIssuer::owned());
 let(alias,_)=slot.try_duplicate(handoff).unwrap_or_else(|_|panic!("prevalidated original gesture handoff remains funded"));
 (GestureOperation{operation,slot,decision:None,settled:false},GestureCapture::Issued(alias))
}

#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
