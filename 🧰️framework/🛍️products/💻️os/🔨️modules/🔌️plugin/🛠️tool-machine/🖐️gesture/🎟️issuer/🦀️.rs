//! 🎟️ Original gesture issuance keeps the runtime and captured alias behind a strong-only authority.
use semio_plugin_decl_views::gesture_contract::GestureCapture;
use semio_plugin_decl_views::gesture_contract::{GestureSlot, GestureSlotDecision};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneStep, RetainedWorkOutcome, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::shared_issuance::{SealedShared, SealedSharedBirth, SharedIssuer};

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct GestureOperation<M:Send+'static>{pub operation:u64,pub slot:SealedShared<GestureSlot<M>>,pub decision:Option<GestureSlotDecision<M>>,pub settled:bool}
impl<M:Send+'static> GestureOperation<M>{pub fn request_cancellation(&mut self){self.settled=true;}}

/// 🪙️ Funds one live gesture issuance (empty birth, publication and strong-only handoff) as the sum of its two host limits.
#[doc(hidden)]
pub fn gesture_issuance_grant(birth:RetainedCloneGrant,handoff:RetainedCloneGrant)->RetainedCloneGrant{
 RetainedCloneGrant{maximum_items:birth.maximum_items+handoff.maximum_items,maximum_copy_bytes:birth.maximum_copy_bytes+handoff.maximum_copy_bytes,maximum_capacity_bytes:birth.maximum_capacity_bytes+handoff.maximum_capacity_bytes,maximum_release_bytes:birth.maximum_release_bytes+handoff.maximum_release_bytes,maximum_depth:birth.maximum_depth.max(handoff.maximum_depth)}
}

/// 🧱️ Publishes one actual empty sealed backing for the next live gesture inside the caller's original effect.
#[doc(hidden)]
pub fn reserve_live<M:Send+'static>(available:bool,output:&mut Option<SealedSharedBirth<GestureSlot<M>>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
 if !available{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"all original gesture operation slots remain occupied")),work:0};}
 SealedShared::<GestureSlot<M>>::prepare_birth(output,effect)
}
/// 📤️ Moves the live gesture slot into its reserved sealed backing.
#[doc(hidden)]
pub fn publish_live<M:RetireOwned+Send+'static>(birth:&mut SealedSharedBirth<GestureSlot<M>>,slot:&mut Option<GestureSlot<M>>,output:&mut Option<SealedShared<GestureSlot<M>>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
 birth.publish(slot,SharedIssuer::owned(),output,effect)
}
/// 🎟️ Issues the strong operation and its captured alias once the alias is funded by the effect.
#[doc(hidden)]
pub fn issue_live<M:RetireOwned+Send+'static>(slot:&mut Option<SealedShared<GestureSlot<M>>>,operation:u64,output:&mut Option<(GestureOperation<M>,GestureCapture<M>)>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
 let Some(published)=slot.as_ref()else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original gesture slot is not published")),work:0}};
 let mut alias=None;let turn=published.duplicate_into(&mut alias,effect);
 if let Some(alias)=alias{*output=Some((GestureOperation{operation,slot:slot.take().expect("published slot checked"),decision:None,settled:false},GestureCapture::Issued(alias)))}
 turn
}

#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
